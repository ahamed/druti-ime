//! The keystroke engine.
//!
//! `output` and `buffer` are stored as UTF-16 code units so that slicing and
//! the back counts in [`Action`] match the hosts' string ranges (`NSString` on
//! Apple platforms, JavaScript strings in the browser). Most suffix-based rules
//! inspect the end of the output; quote balancing scans the full prior text.

use unicode_general_category::{GeneralCategory, get_general_category};

use crate::data::{
    self, ASPIRATED_CONSONANT_BY_BASE, CAPITAL_ROMAN_TO_CONSONANT, CHONDROBINDU, DARI,
    DEFAULT_CONSONANT_BY_ROMAN_KEY, DOUBLE_DASH, ENTER_KEY, FULL_STOP, HASANT, NO_JOIN_AFTER,
    NUMBER_MAP, ONTOSTHO_RO, PHOLA_AFTER_NO_JOIN, ROMAN_TO_PHONETIC_VOWELS, SPACE,
    SPECIAL_CHARACTER_INPUTS, SPECIAL_CHARACTERS_MAP, TYPOGRAPHIC_DOUBLE_QUOTE_CLOSE,
    TYPOGRAPHIC_DOUBLE_QUOTE_OPEN, TYPOGRAPHIC_SINGLE_QUOTE_CLOSE, TYPOGRAPHIC_SINGLE_QUOTE_OPEN,
    is_ascii_or_bengali_digit, is_kar_taking_consonant, lookup,
};
use crate::vowel_attach::{
    ends_with_consonant_and_chandrabindu_units, ends_with_kar_taking_consonant_units,
};
use crate::{letters, rules};

/// One edit the host applies at the caret, in order. Back counts are UTF-16
/// code units.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Insert `text` at the caret.
    Insert {
        /// The text to insert.
        text: String,
    },
    /// Delete `chars_back` units before the caret, then insert `text`.
    Replace {
        /// UTF-16 units to delete before the caret.
        chars_back: usize,
        /// The text to insert in their place.
        text: String,
    },
    /// Delete `chars_back` units before the caret.
    Delete {
        /// UTF-16 units to delete before the caret.
        chars_back: usize,
    },
    /// Start a new paragraph (Enter).
    SplitBlock,
}

pub(crate) fn utf16(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

/// JavaScript `s.slice(0, count * -1)`. Note `slice(0, -0)` is empty.
fn drop_last(units: &[u16], count: usize) -> Vec<u16> {
    if count == 0 {
        return Vec::new();
    }
    units[..units.len().saturating_sub(count)].to_vec()
}

/// The last few UTF-16 units of `text`, always cut at a code point boundary.
/// Every "ends with" rule looks at most three units back.
fn tail_units(text: &str) -> Vec<u16> {
    let start = text.char_indices().rev().nth(7).map_or(0, |(i, _)| i);
    utf16(&text[start..])
}

fn balanced_typographic_quote(
    prior: &str,
    open: &'static str,
    close: &'static str,
) -> &'static str {
    let opens = prior.matches(open).count();
    let closes = prior.matches(close).count();
    if opens > closes { close } else { open }
}

/// `/(\p{L}|\p{N})(?:\p{M})*$/u`
fn prior_ends_with_word_char_for_apostrophe(prior: &str) -> bool {
    use GeneralCategory::{
        DecimalNumber, EnclosingMark, LetterNumber, LowercaseLetter, ModifierLetter,
        NonspacingMark, OtherLetter, OtherNumber, SpacingMark, TitlecaseLetter, UppercaseLetter,
    };
    let mut chars = prior.chars().rev().skip_while(|c| {
        matches!(
            get_general_category(*c),
            NonspacingMark | SpacingMark | EnclosingMark
        )
    });
    chars.next().is_some_and(|c| {
        matches!(
            get_general_category(c),
            UppercaseLetter
                | LowercaseLetter
                | TitlecaseLetter
                | ModifierLetter
                | OtherLetter
                | DecimalNumber
                | LetterNumber
                | OtherNumber
        )
    })
}

fn single_quote_from_prior(prior: &str) -> &'static str {
    if prior_ends_with_word_char_for_apostrophe(prior) {
        return TYPOGRAPHIC_SINGLE_QUOTE_CLOSE;
    }
    balanced_typographic_quote(
        prior,
        TYPOGRAPHIC_SINGLE_QUOTE_OPEN,
        TYPOGRAPHIC_SINGLE_QUOTE_CLOSE,
    )
}

/// Output options (the macOS input menu and the playground toggles).
/// `Config::default()` turns the output toggles on and Autocorrect off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "independent on/off settings, one per menu item"
)]
pub struct Config {
    /// `1` → `১`. Off: digits stay ASCII.
    pub bengali_digits: bool,
    /// `.` → `।` (with the decimal point and ellipsis rules). Off: `.` stays `.`.
    pub dari_for_period: bool,
    /// `"` and `'` become typographic quotes. Off: they stay ASCII.
    pub smart_quotes: bool,
    /// Corrects a finished word from the Autocorrect list, which only removes
    /// hasants (`আম্রা` → `আমরা`). The [`Engine`] ignores it: the
    /// [`Composer`](crate::Composer) applies it when a word ends, and bulk
    /// conversion to every word. Off by default (autocorrect design D6).
    pub autocorrect: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bengali_digits: true,
            dari_for_period: true,
            smart_quotes: true,
            autocorrect: false,
        }
    }
}

/// Phonetic roman-to-Bengali state machine. See the crate docs for how its
/// behaviour is pinned.
#[derive(Debug, Default, Clone)]
pub struct Engine {
    pub(crate) buffer: Vec<u16>,
    pub(crate) output: Vec<u16>,
    english_mode: bool,
    config: Config,
    actions: Vec<Action>,
    skip_document_kar_for_next_vowel: bool,
}

impl Engine {
    /// An engine with the default [`Config`] and empty output.
    pub fn new() -> Self {
        Self::default()
    }

    /// An engine with the given output options and empty output.
    pub fn with_config(config: Config) -> Self {
        Self {
            config,
            ..Self::default()
        }
    }

    /// The current output options.
    pub fn config(&self) -> Config {
        self.config
    }

    /// Changes the output options; applies from the next key.
    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }

    /// Backspace: deletes the last letter of the output and resumes the
    /// cluster before it (rust-engine-core spec, "Letter backspace").
    ///
    /// A letter is a consonant with its nukta and the hasant joining it to the
    /// consonant before (`দ্ম` → `দ`, never `দ্`), any other single Bengali
    /// code point (a kar, a sign, a vowel), or one grapheme cluster of other
    /// text. Only visible text counts, so a keystroke that produced nothing
    /// (the silent `o`) never uses up a Backspace. Returns one delete action,
    /// or none on an empty output.
    pub fn process_backspace(&mut self) -> Vec<Action> {
        self.actions.clear();
        let count = letters::last_letter_len_utf16(&self.output);
        if count > 0 {
            self.pop(count);
            self.resume_cluster();
        }
        std::mem::take(&mut self.actions)
    }

    /// `process(char, { textBeforeCaret })`.
    pub fn process(&mut self, key: &str, text_before_caret: Option<&str>) -> Vec<Action> {
        self.actions.clear();
        self.process_keystroke(key, text_before_caret);
        std::mem::take(&mut self.actions)
    }

    /// Makes the consonant run at the end of the output the current cluster,
    /// so the next consonant continues it as if the run had just been typed:
    /// after `করত`, `h` gives `থ` (design D3). The run is consonants joined by
    /// hasants; after anything else (a kar, `ং`, a space) the cluster is empty.
    ///
    /// Hosts call this when the engine lost track of the caret. The engine
    /// never resumes on its own after ending a cluster itself, so `kom` stays
    /// `কম`. Returns no actions and leaves the output unchanged.
    pub fn resume_cluster(&mut self) {
        let run = letters::trailing_consonant_run_len_utf16(&self.output);
        self.buffer = self.output[self.output.len() - run..].to_vec();
        self.skip_document_kar_for_next_vowel = false;
    }

    /// Flips English mode and ends the current Bangla cluster.
    pub fn toggle_english_mode(&mut self) {
        self.english_mode = !self.english_mode;
        self.flush_buffer();
        self.skip_document_kar_for_next_vowel = false;
    }

    /// Whether keys are currently passed through as English.
    pub fn is_english_mode(&self) -> bool {
        self.english_mode
    }

    /// Assigns `output` directly (a host resync). The buffer is kept; call
    /// [`Engine::resume_cluster`] to take the cluster from the new output.
    pub fn set_output(&mut self, output: &str) {
        self.output = utf16(output);
    }

    /// Everything the engine has produced (or was given by [`Engine::set_output`]).
    pub fn output(&self) -> String {
        String::from_utf16_lossy(&self.output)
    }

    /// The current cluster: the end of the output that the next key may still rewrite.
    pub fn buffer(&self) -> String {
        String::from_utf16_lossy(&self.buffer)
    }

    /// Ends the current cluster, as toggling English mode does: the next key
    /// cannot rewrite what came before.
    pub fn end_cluster(&mut self) {
        self.flush_buffer();
        self.skip_document_kar_for_next_vowel = false;
    }

    /// Length of `output` in UTF-16 code units.
    pub fn output_len_utf16(&self) -> usize {
        self.output.len()
    }

    /// Length of `buffer` in UTF-16 code units.
    pub fn buffer_len_utf16(&self) -> usize {
        self.buffer.len()
    }

    /// `spliceTail`: replaces the last `count` units of the output with `text`.
    /// Keeps JavaScript's `slice(0, negative)` behaviour when `count` exceeds
    /// the output length (only possible after a resync shortened the output).
    fn splice_tail(&mut self, count: usize, text: &[u16]) {
        let len = self.output.len();
        let keep = if count <= len {
            len - count
        } else {
            (2 * len).saturating_sub(count)
        };
        self.output.truncate(keep);
        self.output.extend_from_slice(text);
    }

    fn process_keystroke(&mut self, key: &str, text_before_caret: Option<&str>) {
        if self.english_mode {
            self.process_english_keystroke(key);
            return;
        }

        if key == SPACE || key == ENTER_KEY {
            self.skip_document_kar_for_next_vowel = false;
            self.process_word_boundary(key);
            return;
        }

        if is_number(key) {
            self.skip_document_kar_for_next_vowel = false;
            self.process_number(key);
            return;
        }

        if is_special_character(key) {
            // Chandrabindu sits on the syllable, so a silent `o` before it still counts.
            if key != "^" {
                self.skip_document_kar_for_next_vowel = false;
            }
            self.process_special_characters(key, text_before_caret);
            return;
        }

        if is_vowel(key) {
            self.process_vowel(key, text_before_caret);
            return;
        }

        self.skip_document_kar_for_next_vowel = false;
        self.process_consonant(key);
    }

    fn process_number(&mut self, key: &str) {
        if !self.config.bengali_digits {
            self.append_and_flush_buffer(key);
            return;
        }
        if let Some(digit) = lookup(NUMBER_MAP, key) {
            self.append_and_flush_buffer(digit);
        }
    }

    fn process_english_keystroke(&mut self, key: &str) {
        if key == SPACE {
            self.append_and_flush_buffer(SPACE);
            return;
        }
        if key == ENTER_KEY {
            self.actions.push(Action::SplitBlock);
            self.flush_buffer();
            return;
        }
        self.append_and_flush_buffer(key);
    }

    /// `textBeforeCaret ?? this.recent`, reduced to the units the rules read.
    fn prior_tail(&self, text_before_caret: Option<&str>) -> Vec<u16> {
        match text_before_caret {
            Some(text) => tail_units(text),
            None => self.output[self.output.len().saturating_sub(16)..].to_vec(),
        }
    }

    fn process_vowel(&mut self, key: &str, text_before_caret: Option<&str>) {
        let lower = key.to_lowercase();
        let vowel_direct = lookup(ROMAN_TO_PHONETIC_VOWELS, key);
        let Some(vowel) = vowel_direct.or_else(|| lookup(ROMAN_TO_PHONETIC_VOWELS, &lower)) else {
            return;
        };
        let vowel_key = if vowel_direct.is_some() {
            key
        } else {
            lower.as_str()
        };

        if rules::rassaw_ri(self, key) || rules::oi(self, key) || rules::ou(self, key) {
            return;
        }

        // Reph never comes before a vowel, so a vowel after `rr` drops the
        // armed hasant and acts as after র (rr-reph design D3). The host's text
        // still ends in that hasant, so read it without it.
        let text_before_caret = if self.reph_is_armed() {
            self.drop_armed_hasant();
            text_before_caret.map(|text| text.strip_suffix(HASANT).unwrap_or(text))
        } else {
            text_before_caret
        };

        // A kar only ever attaches to a consonant directly before the caret; after
        // anything else the vowel is independent. `o` after a consonant is the
        // silent inherent vowel, so the next vowel starts a new syllable.
        let prior = self.prior_tail(text_before_caret);
        let after_silent_o = self.skip_document_kar_for_next_vowel;
        self.skip_document_kar_for_next_vowel = false;

        // `O` keeps the buffer so a following `i`/`u` can form ঐ/ঔ.
        let keep_buffer = vowel_key == "O";

        if !after_silent_o && ends_with_kar_taking_consonant_units(&prior) {
            if keep_buffer {
                self.append(vowel.kar, true);
            } else {
                self.append_and_flush_buffer(vowel.kar);
            }
            self.skip_document_kar_for_next_vowel = vowel_key == "o";
            return;
        }

        // Chandrabindu typed before the vowel (`k^a`): the kar goes before ঁ.
        if !after_silent_o && ends_with_consonant_and_chandrabindu_units(&prior) {
            self.replace_last(&format!("{}{CHONDROBINDU}", vowel.kar), !keep_buffer, 1);
            self.skip_document_kar_for_next_vowel = vowel_key == "o";
            return;
        }

        if keep_buffer {
            self.append(vowel.ind, true);
        } else {
            self.append_and_flush_buffer(vowel.ind);
        }
    }

    fn process_consonant(&mut self, key: &str) {
        let last_in_buffer = self.last_in_buffer();

        if rules::reph(self, key)
            || rules::kkhiyo(self, key)
            || rules::ho(self, key)
            || rules::khanda_to(self, key)
            || rules::ja_fala(self, key)
            || rules::aspirated_consonant(self, key)
        {
            return;
        }

        if self.process_nasal_connectors(key) {
            return;
        }

        let lower = key.to_lowercase();
        let direct = lookup(DEFAULT_CONSONANT_BY_ROMAN_KEY, key)
            .or_else(|| lookup(CAPITAL_ROMAN_TO_CONSONANT, key));

        let consonant = if direct.is_some() {
            direct
        } else if key != lower {
            lookup(DEFAULT_CONSONANT_BY_ROMAN_KEY, &lower)
                .or_else(|| lookup(CAPITAL_ROMAN_TO_CONSONANT, &lower))
                .map(|base| lookup(ASPIRATED_CONSONANT_BY_BASE, base).unwrap_or(base))
        } else {
            None
        };

        let Some(consonant) = consonant else {
            self.process_unmapped_key(key);
            return;
        };

        match last_in_buffer.as_deref() {
            Some(last) if is_kar_taking_consonant(last) => {
                if joins_after(last, key, consonant) {
                    self.append(&format!("{HASANT}{consonant}"), true);
                } else {
                    // Written side by side: the consonant starts a new cluster,
                    // so no later rule reads across the gap (rr-reph design D1).
                    self.flush_buffer();
                    self.append(consonant, true);
                }
            }
            _ => self.append(consonant, true),
        }
    }

    /// Whether the buffer ends in a reph armed by `rr`: `র্`, the only hasant
    /// the engine leaves at the end of its buffer (rr-reph design D2).
    pub(crate) fn reph_is_armed(&self) -> bool {
        self.buffer_ends_with(&format!("{ONTOSTHO_RO}{HASANT}"))
    }

    /// Deletes the hasant of an armed reph, keeping `র` as the end of the buffer.
    fn drop_armed_hasant(&mut self) {
        let len = utf16(HASANT).len();
        self.splice_tail(len, &[]);
        self.buffer.truncate(self.buffer.len() - len);
        self.actions.push(Action::Delete { chars_back: len });
    }

    /// Any other single code point is written as-is and ends the cluster.
    fn process_unmapped_key(&mut self, key: &str) {
        if key.chars().count() != 1 {
            return;
        }
        self.append_and_flush_buffer(key);
    }

    fn process_nasal_connectors(&mut self, key: &str) -> bool {
        rules::nyo_plus_cha(self, key)
            || rules::onushwar(self, key)
            || rules::ungo(self, key)
            || rules::jo_plus_nyo(self, key)
            || rules::nyo(self, key)
            || rules::nyo_plus_borgiyo_jo(self, key)
    }

    fn process_special_characters(&mut self, key: &str, text_before_caret: Option<&str>) -> bool {
        if key == data::DASH {
            let prior = self.prior_tail(text_before_caret);
            if prior.ends_with(&utf16(data::DASH)) {
                self.actions.push(Action::Replace {
                    chars_back: 1,
                    text: DOUBLE_DASH.to_owned(),
                });
                if self.output.ends_with(&utf16(data::DASH)) {
                    self.splice_tail(1, &utf16(DOUBLE_DASH));
                }
                self.flush_buffer();
                return true;
            }
            self.append_and_flush_buffer(data::DASH);
            return true;
        }

        if key == FULL_STOP {
            let prior = self.prior_tail(text_before_caret);
            self.process_full_stop(&prior);
            return true;
        }

        if key == "^" {
            // Kept in the buffer so a following vowel can slot its kar before ঁ,
            // and O + ^ + i can still become ৈঁ.
            self.append(CHONDROBINDU, true);
            return true;
        }

        if !self.config.smart_quotes && (key == "\"" || key == "'") {
            self.append_and_flush_buffer(key);
            return true;
        }

        if key == "\"" {
            let prior = text_before_caret.map_or_else(|| self.output(), str::to_owned);
            let quote = balanced_typographic_quote(
                &prior,
                TYPOGRAPHIC_DOUBLE_QUOTE_OPEN,
                TYPOGRAPHIC_DOUBLE_QUOTE_CLOSE,
            );
            self.append_and_flush_buffer(quote);
            return true;
        }

        if key == "'" {
            let prior = text_before_caret.map_or_else(|| self.output(), str::to_owned);
            self.append_and_flush_buffer(single_quote_from_prior(&prior));
            return true;
        }

        let Some(character) = lookup(SPECIAL_CHARACTERS_MAP, key) else {
            return false;
        };
        self.append_and_flush_buffer(character);
        true
    }

    /// `.` after a digit stays a decimal point, `.` after `।` turns it into `..`
    /// (so `...` is an ellipsis), and `.` after `.` stays `.`; otherwise `।`.
    fn process_full_stop(&mut self, prior: &[u16]) {
        if !self.config.dari_for_period {
            self.append_and_flush_buffer(FULL_STOP);
            return;
        }
        let ends_with_digit = prior
            .last()
            .is_some_and(|unit| is_ascii_or_bengali_digit(&String::from_utf16_lossy(&[*unit])));
        if ends_with_digit || prior.ends_with(&utf16(FULL_STOP)) {
            self.append_and_flush_buffer(FULL_STOP);
            return;
        }

        let dari = utf16(DARI);
        if prior.ends_with(&dari) {
            let dots = format!("{FULL_STOP}{FULL_STOP}");
            self.actions.push(Action::Replace {
                chars_back: dari.len(),
                text: dots.clone(),
            });
            if self.output.ends_with(&dari) {
                self.splice_tail(dari.len(), &utf16(&dots));
            }
            self.flush_buffer();
            return;
        }

        self.append_and_flush_buffer(DARI);
    }

    fn process_word_boundary(&mut self, key: &str) {
        if key == ENTER_KEY {
            self.actions.push(Action::SplitBlock);
            self.flush_buffer();
            return;
        }
        self.append_and_flush_buffer(key);
    }

    pub(crate) fn flush_buffer(&mut self) {
        self.buffer.clear();
    }

    /// `buffer.at(-1)`: the last UTF-16 code unit as a string.
    pub(crate) fn last_in_buffer(&self) -> Option<String> {
        self.buffer
            .last()
            .map(|unit| String::from_utf16_lossy(&[*unit]))
    }

    pub(crate) fn buffer_ends_with(&self, s: &str) -> bool {
        self.buffer.ends_with(&utf16(s))
    }

    /// `replaceLast(next, flushBuffer, count)`.
    pub(crate) fn replace_last(&mut self, next: &str, flush_buffer: bool, count: usize) {
        let next_units = utf16(next);
        self.splice_tail(count, &next_units);
        self.actions.push(Action::Replace {
            chars_back: count,
            text: next.to_owned(),
        });

        if flush_buffer {
            self.flush_buffer();
        } else {
            self.buffer = drop_last(&self.buffer, count);
            self.buffer.extend_from_slice(&next_units);
        }
    }

    /// `pop(count)`.
    pub(crate) fn pop(&mut self, count: usize) {
        self.skip_document_kar_for_next_vowel = false;
        self.splice_tail(count, &[]);
        self.actions.push(Action::Delete { chars_back: count });
        self.flush_buffer();
    }

    /// `append(text, updateBuffer)`.
    pub(crate) fn append(&mut self, text: &str, update_buffer: bool) {
        let units = utf16(text);
        self.output.extend_from_slice(&units);
        self.actions.push(Action::Insert {
            text: text.to_owned(),
        });
        if update_buffer {
            self.buffer.extend_from_slice(&units);
        }
    }

    /// `appendAndFlushBuffer(text)`.
    pub(crate) fn append_and_flush_buffer(&mut self, text: &str) {
        self.append(text, false);
        self.flush_buffer();
    }
}

/// Whether `consonant`, typed with `key`, joins the kar-taking consonant
/// `last` that ends the buffer. A single র never joins what follows (reph is
/// typed `rr`, rr-reph design D1); after a breathy letter, হ, ড়, ঢ় or য় only a
/// ফলা joins, and ব only when typed with `w` (design D4b). Everything else
/// joins, and `o` keeps letters apart.
fn joins_after(last: &str, key: &str, consonant: &str) -> bool {
    if last == ONTOSTHO_RO {
        return false;
    }
    if NO_JOIN_AFTER.contains(&last) {
        return PHOLA_AFTER_NO_JOIN.contains(&consonant) || key.eq_ignore_ascii_case("w");
    }
    true
}

/// `isVowel`: true for roman vowel keys (and their lowercase forms).
pub(crate) fn is_vowel(key: &str) -> bool {
    lookup(ROMAN_TO_PHONETIC_VOWELS, key).is_some()
        || lookup(ROMAN_TO_PHONETIC_VOWELS, &key.to_lowercase()).is_some()
}

fn is_number(key: &str) -> bool {
    lookup(NUMBER_MAP, key).is_some()
}

fn is_special_character(key: &str) -> bool {
    SPECIAL_CHARACTER_INPUTS.contains(&key)
}
