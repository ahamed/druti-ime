//! Host adapter for marked-text input methods (macOS, the web playground, and
//! later iOS).
//!
//! The engine rewrites recent output (`k` then `h`: ক → খ, Backspace:
//! `দ্ম` → `দ`). A host can reliably change only its own marked (pending)
//! text: some apps ignore requests to replace text that is already committed
//! (Chromium-based apps, editors built on `EditContext`), and some expose no
//! text at all (terminals). So the composer keeps the whole Bengali word being
//! typed pending, and never asks a host to change committed text
//! (whole-word-pending design D1 and D2). Every host then shows the same text.
//!
//! After every key the composer splits the result into text to commit, which
//! is final in the document, and pending text: the word being typed, or a lone
//! `-` / `।` that the next key may rewrite (`--` → `—`, `।.` → `..`).
//!
//! With [`Config::autocorrect`] on, a word found in the Autocorrect list is
//! corrected when it ends, while it is still pending, and stays pending with
//! the key that ended it until the next key, so Backspace can undo it
//! (autocorrect design D4 and D5). Committed text is still never changed.

use crate::autocorrect;
use crate::data::{DARI, DASH, ENTER_KEY};
use crate::engine::{Action, Config, Engine, is_vowel, utf16};
use crate::letters::{last_letter_len_utf16, trailing_word_len_utf16};

/// What the host applies after a key, in this order:
/// 1. Replace the current pending text with `commit`, as final text.
/// 2. Show `pending` as the new pending text (empty: none).
///
/// Text committed earlier is never changed (whole-word-pending design D2).
/// When `handled` is false the host lets the application process the key
/// itself, after applying steps 1–2.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Update {
    /// Text that replaces the pending text and becomes final (step 1).
    pub commit: String,
    /// The new pending text (step 2).
    pub pending: String,
    /// Whether the key was consumed; if not, the application also processes it.
    pub handled: bool,
}

/// Wraps an [`Engine`] and tracks what is pending in the host.
#[derive(Debug, Clone)]
pub struct Composer {
    engine: Engine,
    /// The pending (marked) text currently shown by the host, in UTF-16. It is
    /// always the end of the engine's output.
    pending: Vec<u16>,
    /// The correction made by the last key, while its word is still pending:
    /// the next Backspace undoes it (autocorrect design D5).
    correction: Option<Correction>,
    /// The pending word is one a Backspace un-corrected: it is not corrected
    /// again when it ends (autocorrect spec, "Backspace undoes a correction").
    kept: bool,
}

/// What undoing a correction restores.
#[derive(Debug, Clone)]
struct Correction {
    /// The engine as it was before the key that ended the word.
    engine: Engine,
    /// The word as the engine wrote it, before the correction.
    word: Vec<u16>,
}

impl Default for Composer {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl Composer {
    /// A composer with nothing pending.
    pub fn new(config: Config) -> Self {
        Self {
            engine: Engine::with_config(config),
            pending: Vec::new(),
            correction: None,
            kept: false,
        }
    }

    /// The current output options.
    pub fn config(&self) -> Config {
        self.engine.config()
    }

    /// Changes the output options; applies from the next key.
    pub fn set_config(&mut self, config: Config) {
        self.engine.set_config(config);
    }

    /// The pending text the host should currently be showing.
    pub fn pending(&self) -> String {
        String::from_utf16_lossy(&self.pending)
    }

    /// Handles one typed character. `text_before_caret` is the committed
    /// document text before the pending text, when the host can read it; with
    /// `None` the engine uses what it has produced since the last reset.
    ///
    /// The document text only decides how the key is written (a kar after a
    /// consonant, a decimal point after a digit, which quote); it is never
    /// rewritten. A rule that would rewrite it (`-` after a committed `-`)
    /// applies as if the word started at the caret (whole-word-pending design
    /// D2).
    ///
    /// Return/Enter is not a Bangla key: `"Enter"` flushes and is not handled.
    ///
    /// With [`Config::autocorrect`], a key that ends a word in the Autocorrect
    /// list leaves the corrected word and the key's text pending, for the next
    /// key to commit or Backspace to undo. Enter commits the corrected word at
    /// once.
    pub fn key(&mut self, key: &str, text_before_caret: Option<&str>) -> Update {
        // Any key but Backspace settles the last correction; a word break or
        // the next letter then commits it as usual.
        let corrected = self.correction.take().is_some();
        if key == ENTER_KEY {
            if !corrected {
                self.correct_pending_word();
            }
            return Update {
                handled: false,
                ..self.flush()
            };
        }

        let context = text_before_caret.map(|text| format!("{text}{}", self.pending()));
        if let Some(text) = &context {
            // The host's text is the truth: later keys it sends without
            // context (while a word is pending) read this instead of only what
            // was typed since the last reset.
            self.engine.set_output(text);
        }
        let before = self.engine.clone();
        let mut actions = self.engine.process(key, context.as_deref());
        if actions.is_empty() {
            // Not a key the engine knows (e.g. a multi-character key name).
            // The application acts on it at once, so a correction can no
            // longer be undone: commit it first (autocorrect design D4).
            if corrected {
                return Update {
                    handled: false,
                    ..self.flush()
                };
            }
            return Update {
                handled: false,
                pending: self.pending(),
                ..Update::default()
            };
        }
        if self.reaches_committed_text(&actions) {
            self.engine = before.clone();
            actions = self.engine.process(key, Some(&self.pending()));
        }
        let word = self.pending.clone();
        let update = self.apply_actions(key, &actions);
        self.correct_ended_word(word, &update.commit, before)
            .unwrap_or(update)
    }

    /// When the last key committed (`commit`) the pending word `word` and the
    /// list has it, puts the corrected word back in the pending text, before
    /// the key's own text, and remembers how to undo it (autocorrect design
    /// D4). `before` is the engine before that key.
    fn correct_ended_word(
        &mut self,
        word: Vec<u16>,
        commit: &str,
        before: Engine,
    ) -> Option<Update> {
        let is_word = !word.is_empty() && trailing_word_len_utf16(&word) == word.len();
        if !self.config().autocorrect || !is_word {
            return None;
        }
        // The pending text is a word or a held `-` / `।`, so the word is
        // committed exactly when a key ended it.
        let committed = utf16(commit);
        let rest = committed.strip_prefix(word.as_slice())?;
        if std::mem::take(&mut self.kept) {
            return None;
        }
        let corrected = utf16(autocorrect::lookup(&String::from_utf16_lossy(&word))?);

        // Keep "committed + pending = engine output" true of the corrected text,
        // so later keys and `matches_text_before_caret` see what the host shows.
        let tail = word.len() + rest.len() + self.pending.len();
        let start = self.engine.output.len().checked_sub(tail)?;
        let in_step = self.engine.output[start..].starts_with(&word);
        debug_assert!(in_step, "the engine output ends with the ended word");
        if !in_step {
            return None;
        }
        self.engine
            .output
            .splice(start..start + word.len(), corrected.iter().copied());

        let mut pending = corrected;
        pending.extend_from_slice(rest);
        pending.append(&mut self.pending);
        self.pending = pending;
        self.correction = Some(Correction {
            engine: before,
            word,
        });
        Some(Update {
            commit: String::new(),
            pending: self.pending(),
            handled: true,
        })
    }

    /// Corrects the pending word in place, before Enter commits it.
    fn correct_pending_word(&mut self) {
        let is_word = !self.pending.is_empty()
            && trailing_word_len_utf16(&self.pending) == self.pending.len();
        if !self.config().autocorrect || !is_word || self.kept {
            return;
        }
        let Some(corrected) = autocorrect::lookup(&self.pending()) else {
            return;
        };
        let Some(start) = self.engine.output.len().checked_sub(self.pending.len()) else {
            debug_assert!(false, "the pending text is the end of the engine output");
            return;
        };
        let corrected = utf16(corrected);
        self.engine
            .output
            .splice(start.., corrected.iter().copied());
        self.pending = corrected;
    }

    /// Whether the document can change the result of `key`: vowels (a kar or
    /// an independent vowel), `-` (em dash), `.` (decimal point, ellipsis) and
    /// quotes (balancing). Hosts read the document only for these keys, and
    /// only while nothing is pending. A consonant always starts a new letter
    /// after committed text, so it never needs the document.
    pub fn key_reads_document(key: &str) -> bool {
        matches!(key, "-" | "." | "\"" | "'") || is_vowel(key)
    }

    /// Whether replaying `actions` on the pending text would delete more than
    /// the pending text.
    fn reaches_committed_text(&self, actions: &[Action]) -> bool {
        let mut len = self.pending.len();
        actions.iter().any(|action| {
            let (chars_back, insert) = match action {
                Action::Insert { text } => (0, text.as_str()),
                Action::Replace { chars_back, text } => (*chars_back, text.as_str()),
                Action::Delete { chars_back } => (*chars_back, ""),
                Action::SplitBlock => (0, ""),
            };
            let reaches = chars_back > len;
            len = len.saturating_sub(chars_back) + utf16(insert).len();
            reaches
        })
    }

    /// Replays the engine's actions on the pending text and splits the result
    /// into text to commit and the new pending text: the word being typed, or
    /// a held `-` / `।`.
    fn apply_actions(&mut self, key: &str, actions: &[Action]) -> Update {
        let mut text = self.pending.clone();
        for action in actions {
            let (chars_back, insert) = match action {
                Action::Insert { text } => (0, text.as_str()),
                Action::Replace { chars_back, text } => (*chars_back, text.as_str()),
                Action::Delete { chars_back } => (*chars_back, ""),
                Action::SplitBlock => (0, ""),
            };
            debug_assert!(chars_back <= text.len(), "an action reached committed text");
            text.truncate(text.len().saturating_sub(chars_back));
            text.extend(insert.encode_utf16());
        }

        let held = self.held_len(key, &text, actions);
        let commit = text[..text.len() - held].to_vec();
        self.pending = text[text.len() - held..].to_vec();
        Update {
            commit: String::from_utf16_lossy(&commit),
            pending: self.pending(),
            handled: true,
        }
    }

    /// How much of the end of `text` stays pending: the word being typed, which
    /// always contains the engine's cluster, or a lone `-` / `।` that the next
    /// key may rewrite.
    fn held_len(&self, key: &str, text: &[u16], actions: &[Action]) -> usize {
        let word = trailing_word_len_utf16(text)
            .max(self.engine.buffer_len_utf16())
            .min(text.len());
        if word > 0 {
            return word;
        }
        let inserted =
            |s: &str| matches!(actions.last(), Some(Action::Insert { text }) if text == s);
        match key {
            DASH if inserted(DASH) => utf16(DASH).len(),
            "." if inserted(DARI) => utf16(DARI).len(),
            _ => 0,
        }
    }

    /// Backspace: removes the last letter of the pending text (ime-composer
    /// spec, "Letter backspace"). A consonant goes with the hasant joining it,
    /// so `দ্ম` becomes `দ`, never `দ্`. The next consonant starts a new letter:
    /// `দ্ম`, Backspace, `h` gives `দহ` (whole-word-pending design D3).
    ///
    /// With nothing pending the key is not handled: the application deletes
    /// committed text by its own rules, and the composer resets.
    ///
    /// Directly after an Autocorrect correction, Backspace undoes it instead:
    /// the word as typed comes back as the pending text, without the key that
    /// ended it, and is not corrected again (autocorrect design D5).
    pub fn backspace(&mut self) -> Update {
        if let Some(Correction { engine, word }) = self.correction.take() {
            self.engine = engine;
            // As after any Backspace, the next consonant starts a new letter
            // (whole-word-pending design D3).
            self.engine.end_cluster();
            self.pending = word;
            self.kept = true;
            return Update {
                commit: String::new(),
                pending: self.pending(),
                handled: true,
            };
        }
        if self.pending.is_empty() {
            self.reset(None);
            return Update {
                handled: false,
                ..Update::default()
            };
        }
        let count = last_letter_len_utf16(&self.pending);
        debug_assert!(
            self.engine.output.ends_with(&self.pending),
            "the pending text is the end of the engine output"
        );
        self.engine.pop(count);
        self.pending.truncate(self.pending.len() - count);
        if self.pending.is_empty() {
            self.kept = false;
        }
        Update {
            commit: String::new(),
            pending: self.pending(),
            handled: true,
        }
    }

    /// Commits all pending text and ends the cluster (focus change, arrow keys,
    /// shortcuts, switching input source).
    pub fn flush(&mut self) -> Update {
        self.engine.end_cluster();
        self.correction = None;
        self.kept = false;
        let commit = self.pending();
        self.pending.clear();
        Update {
            commit,
            pending: String::new(),
            handled: true,
        }
    }

    /// Forgets all state without committing anything (the caret moved, or the
    /// document changed underneath). The host drops its pending text.
    ///
    /// `text_before_caret`, when known, becomes the engine's view of the
    /// document, which later keys read when the host passes no context. The
    /// next consonant starts a new letter either way.
    pub fn reset(&mut self, text_before_caret: Option<&str>) -> Update {
        self.engine = Engine::with_config(self.engine.config());
        if let Some(text) = text_before_caret {
            self.engine.set_output(text);
        }
        self.pending.clear();
        self.correction = None;
        self.kept = false;
        Update {
            handled: true,
            ..Update::default()
        }
    }

    /// Whether `text_before_caret`, followed by the pending text, is consistent
    /// with what the composer produced since its last reset: both are
    /// non-empty and one ends with the other (ime-composer spec, "Checking the
    /// text before the caret"). Changes nothing.
    ///
    /// Hosts ask this when the caret isn't where they expected, to tell a real
    /// caret move from an app that reports the caret late or exposes only the
    /// text near it (whole-word-pending design D4).
    pub fn matches_text_before_caret(&self, text_before_caret: &str) -> bool {
        let output = &self.engine.output;
        let text: Vec<u16> = text_before_caret
            .encode_utf16()
            .chain(self.pending.iter().copied())
            .collect();
        !output.is_empty()
            && !text.is_empty()
            && (output.ends_with(&text) || text.ends_with(output))
    }
}
