//! UniFFI surface of `druti-core` for Swift (the macOS input method, and
//! later an iOS keyboard extension). Nothing platform-specific lives here.
//!
//! Every call is guarded: if the engine ever panics, the composer is reset
//! and the key is reported as not handled, so the host app still receives it
//! and typing never stops.

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

uniffi::setup_scaffolding!();

/// Output options; see `druti_core::Config`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Record)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "independent on/off settings, one per menu item"
)]
pub struct Config {
    /// `1` → `১`. Off: digits stay ASCII.
    pub bengali_digits: bool,
    /// `.` → `।`. Off: `.` stays `.`.
    pub dari_for_period: bool,
    /// `"` and `'` become typographic quotes. Off: they stay ASCII.
    pub smart_quotes: bool,
    /// Corrects a finished word from the Autocorrect list, which only removes
    /// hasants (`আম্রা` → `আমরা`). Off by default.
    pub autocorrect: bool,
}

impl From<Config> for druti_core::Config {
    fn from(c: Config) -> Self {
        Self {
            bengali_digits: c.bengali_digits,
            dari_for_period: c.dari_for_period,
            smart_quotes: c.smart_quotes,
            autocorrect: c.autocorrect,
        }
    }
}

impl From<druti_core::Config> for Config {
    fn from(c: druti_core::Config) -> Self {
        Self {
            bengali_digits: c.bengali_digits,
            dari_for_period: c.dari_for_period,
            smart_quotes: c.smart_quotes,
            autocorrect: c.autocorrect,
        }
    }
}

/// The defaults: the output options on, Autocorrect off.
#[uniffi::export]
pub fn default_config() -> Config {
    druti_core::Config::default().into()
}

/// What the host applies after a key; see `druti_core::Update`. It never
/// changes text committed earlier.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct Update {
    /// Text that replaces the marked text and becomes final.
    pub commit: String,
    /// The new marked text (empty: none).
    pub pending: String,
    /// Whether the key was consumed; if not, the app also processes it.
    pub handled: bool,
}

impl From<druti_core::Update> for Update {
    fn from(u: druti_core::Update) -> Self {
        Self {
            commit: u.commit,
            pending: u.pending,
            handled: u.handled,
        }
    }
}

/// One composer per text-input session (IMK creates one controller per client).
#[derive(Debug, uniffi::Object)]
pub struct Composer {
    inner: Mutex<druti_core::Composer>,
}

impl Composer {
    fn lock(&self) -> MutexGuard<'_, druti_core::Composer> {
        self.inner.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn guarded(&self, f: impl FnOnce(&mut druti_core::Composer) -> druti_core::Update) -> Update {
        let mut composer = self.lock();
        if let Ok(update) = catch_unwind(AssertUnwindSafe(|| f(&mut composer))) {
            return update.into();
        }
        let config = composer.config();
        *composer = druti_core::Composer::new(config);
        // Nothing to apply, and `handled: false` lets the key through to the app.
        druti_core::Update::default().into()
    }
}

#[uniffi::export]
#[expect(
    clippy::needless_pass_by_value,
    reason = "UniFFI lifts arguments into owned values"
)]
impl Composer {
    /// A composer with nothing pending.
    #[uniffi::constructor]
    pub fn new(config: Config) -> Arc<Self> {
        Arc::new(Self {
            inner: Mutex::new(druti_core::Composer::new(config.into())),
        })
    }

    /// One typed character (or `"Enter"`); `text_before_caret` is the committed
    /// text before the pending text when the app exposes it.
    pub fn key(&self, key: String, text_before_caret: Option<String>) -> Update {
        self.guarded(|c| c.key(&key, text_before_caret.as_deref()))
    }

    /// Backspace: removes one letter of the pending text; with nothing
    /// pending it is left to the app. See `druti_core::Composer::backspace`.
    pub fn backspace(&self) -> Update {
        self.guarded(druti_core::Composer::backspace)
    }

    /// Commits all pending text and ends the cluster.
    pub fn flush(&self) -> Update {
        self.guarded(druti_core::Composer::flush)
    }

    /// Forgets all state without committing (the caret moved).
    pub fn reset(&self, text_before_caret: Option<String>) -> Update {
        self.guarded(|c| c.reset(text_before_caret.as_deref()))
    }

    /// Whether the text before the caret still matches what the composer
    /// typed since its last reset; see
    /// `druti_core::Composer::matches_text_before_caret`. Hosts ask this when
    /// the caret isn't where they expected it.
    pub fn matches_text_before_caret(&self, text_before_caret: String) -> bool {
        self.lock().matches_text_before_caret(&text_before_caret)
    }

    /// The pending text the host should currently be showing.
    pub fn pending(&self) -> String {
        self.lock().pending()
    }

    /// The current output options.
    pub fn config(&self) -> Config {
        self.lock().config().into()
    }

    /// Changes the output options; applies from the next key.
    pub fn set_config(&self, config: Config) {
        self.lock().set_config(config.into());
    }
}

/// Whether `key` can depend on the document text before the caret. Read the
/// document (and pass it as `text_before_caret`) only for these keys, and only
/// while nothing is pending.
#[uniffi::export]
#[expect(
    clippy::needless_pass_by_value,
    reason = "UniFFI lifts arguments into owned values"
)]
pub fn key_reads_document(key: String) -> bool {
    druti_core::Composer::key_reads_document(&key)
}

/// Bulk conversion of a roman document (the "Convert selection" command).
#[uniffi::export]
pub fn transpile_roman_document(
    document: String,
    preserve_line_breaks: bool,
    config: Config,
) -> String {
    catch_unwind(|| {
        druti_core::transpile_roman_document_with_config(
            &document,
            preserve_line_breaks,
            config.into(),
        )
    })
    .unwrap_or(document)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn type_keys(composer: &Composer, keys: &str) -> String {
        let mut committed = String::new();
        for ch in keys.chars() {
            let update = composer.key(ch.to_string(), None);
            committed.push_str(&update.commit);
        }
        committed
    }

    #[test]
    fn typing_through_the_ffi_surface() {
        let composer = Composer::new(default_config());
        assert_eq!(type_keys(&composer, "khub "), "খুব ");
        assert_eq!(composer.pending(), "");
    }

    #[test]
    fn document_keys() {
        assert!(key_reads_document("i".into()));
        assert!(key_reads_document("-".into()));
        assert!(!key_reads_document("k".into()));
        assert!(!key_reads_document("1".into()));
    }

    #[test]
    fn pending_and_backspace() {
        let composer = Composer::new(default_config());
        assert_eq!(composer.key("d".into(), None).pending, "দ");
        assert_eq!(composer.key("m".into(), None).pending, "দ্ম");
        let update = composer.backspace();
        assert!(update.handled);
        assert_eq!(update.pending, "দ");
        assert_eq!(composer.backspace().pending, "");
        assert!(!composer.backspace().handled);
    }

    #[test]
    fn same_updates_as_the_core_composer() {
        let ffi = Composer::new(default_config());
        let mut core = druti_core::Composer::new(druti_core::Config::default());
        for key in ["p", "o", "d", "m", "o"] {
            assert_eq!(ffi.key(key.into(), None), core.key(key, None).into());
        }
        assert_eq!(
            ffi.backspace(),
            core.backspace().into(),
            "backspace in the word"
        );
        assert_eq!(ffi.key(" ".into(), None), core.key(" ", None).into());
        assert_eq!(
            ffi.backspace(),
            core.backspace().into(),
            "backspace after the word"
        );
        assert_eq!(
            ffi.key("h".into(), Some("করত".into())),
            core.key("h", Some("করত")).into()
        );
        for context in ["ধ", "আমি "] {
            assert_eq!(
                ffi.matches_text_before_caret(context.into()),
                core.matches_text_before_caret(context),
                "matches {context:?}"
            );
        }
    }

    #[test]
    fn config_round_trip() {
        let composer = Composer::new(default_config());
        let config = Config {
            bengali_digits: false,
            ..default_config()
        };
        composer.set_config(config);
        assert_eq!(composer.config(), config);
        assert_eq!(composer.key("2".into(), None).commit, "2");
    }

    #[test]
    fn autocorrect_matches_the_core_composer() {
        let config = Config {
            autocorrect: true,
            ..default_config()
        };
        assert!(!default_config().autocorrect);
        let ffi = Composer::new(config);
        let mut core = druti_core::Composer::new(config.into());
        for key in ["a", "m", "r", "a", " "] {
            assert_eq!(ffi.key(key.into(), None), core.key(key, None).into());
        }
        assert_eq!(ffi.pending(), "আমরা ");
        assert_eq!(ffi.backspace(), core.backspace().into(), "undo");
        assert_eq!(ffi.pending(), "আম্রা");
        assert_eq!(transpile_roman_document("amra".into(), true, config), "আমরা");
    }

    #[test]
    fn transpile_applies_config() {
        let config = Config {
            dari_for_period: false,
            ..default_config()
        };
        assert_eq!(
            transpile_roman_document("ami banglay gan gai.".into(), true, default_config()),
            "আমি বাংলা\u{9DF} গান গাই।"
        );
        assert_eq!(
            transpile_roman_document("ami.".into(), true, config),
            "আমি."
        );
    }
}
