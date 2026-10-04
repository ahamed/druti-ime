//! wasm-bindgen surface of `druti-core` for the web playground. It mirrors the
//! UniFFI surface in `druti-ffi`, so the browser types exactly like the macOS
//! input method. Nothing platform-specific lives here.
//!
//! A panic traps on `wasm32-unknown-unknown` (there is no unwinding), so the
//! JavaScript host catches the exception, creates a new `Composer` and lets
//! the key through.

use wasm_bindgen::prelude::*;

/// Output options; see `druti_core::Config`. `new Config()` has the output
/// options on and Autocorrect off.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "independent on/off settings, one per menu item"
)]
pub struct Config {
    /// `1` → `১`. Off: digits stay ASCII.
    #[wasm_bindgen(js_name = bengaliDigits)]
    pub bengali_digits: bool,
    /// `.` → `।`. Off: `.` stays `.`.
    #[wasm_bindgen(js_name = dariForPeriod)]
    pub dari_for_period: bool,
    /// `"` and `'` become typographic quotes. Off: they stay ASCII.
    #[wasm_bindgen(js_name = smartQuotes)]
    pub smart_quotes: bool,
    /// Corrects a finished word from the Autocorrect list, which only removes
    /// hasants (`আম্রা` → `আমরা`). Off by default.
    #[wasm_bindgen(js_name = autocorrect)]
    pub autocorrect: bool,
}

#[wasm_bindgen]
impl Config {
    /// The defaults: the output options on, Autocorrect off.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        druti_core::Config::default().into()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
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

/// What the host applies after a key; see `druti_core::Update`. It never
/// changes text committed earlier.
// `getter_with_clone` only on the `String` fields: on the whole struct it
// would also clone the `bool`, which clippy's `clone_on_copy` rejects.
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update {
    /// Text that replaces the pending text and becomes final.
    #[wasm_bindgen(getter_with_clone)]
    pub commit: String,
    /// The new pending text (empty: none).
    #[wasm_bindgen(getter_with_clone)]
    pub pending: String,
    /// Whether the key was consumed; if not, the editor also processes it.
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

/// One composer per editor.
#[wasm_bindgen]
#[derive(Debug)]
pub struct Composer {
    inner: druti_core::Composer,
}

#[wasm_bindgen]
#[expect(
    clippy::needless_pass_by_value,
    reason = "wasm-bindgen has no borrowed form of `Option<String>`"
)]
impl Composer {
    /// A composer with nothing pending.
    #[wasm_bindgen(constructor)]
    pub fn new(config: &Config) -> Self {
        Self {
            inner: druti_core::Composer::new((*config).into()),
        }
    }

    /// One typed character (or `"Enter"`); `textBeforeCaret` is the committed
    /// text before the pending text.
    pub fn key(
        &mut self,
        key: &str,
        #[wasm_bindgen(js_name = textBeforeCaret)] text_before_caret: Option<String>,
    ) -> Update {
        self.inner.key(key, text_before_caret.as_deref()).into()
    }

    /// Backspace: removes one letter of the pending text; with nothing
    /// pending it is left to the editor. See `druti_core::Composer::backspace`.
    pub fn backspace(&mut self) -> Update {
        self.inner.backspace().into()
    }

    /// Commits all pending text and ends the cluster.
    pub fn flush(&mut self) -> Update {
        self.inner.flush().into()
    }

    /// Forgets all state without committing (the caret moved).
    pub fn reset(
        &mut self,
        #[wasm_bindgen(js_name = textBeforeCaret)] text_before_caret: Option<String>,
    ) -> Update {
        self.inner.reset(text_before_caret.as_deref()).into()
    }

    /// Whether the text before the caret still matches what the composer
    /// typed since its last reset; see
    /// `druti_core::Composer::matches_text_before_caret`. Hosts ask this when
    /// the caret isn't where they expected it.
    #[wasm_bindgen(js_name = matchesTextBeforeCaret)]
    pub fn matches_text_before_caret(
        &self,
        #[wasm_bindgen(js_name = textBeforeCaret)] text_before_caret: &str,
    ) -> bool {
        self.inner.matches_text_before_caret(text_before_caret)
    }

    /// The pending text the host should currently be showing.
    #[wasm_bindgen(getter)]
    pub fn pending(&self) -> String {
        self.inner.pending()
    }

    /// The current output options.
    #[wasm_bindgen(getter)]
    pub fn config(&self) -> Config {
        self.inner.config().into()
    }

    /// Applies from the next key.
    #[wasm_bindgen(js_name = setConfig)]
    pub fn set_config(&mut self, config: &Config) {
        self.inner.set_config((*config).into());
    }
}

/// Bulk conversion of a roman document.
#[wasm_bindgen(js_name = transpileRomanDocument)]
pub fn transpile_roman_document(
    document: &str,
    #[wasm_bindgen(js_name = preserveLineBreaks)] preserve_line_breaks: bool,
    config: &Config,
) -> String {
    druti_core::transpile_roman_document_with_config(
        document,
        preserve_line_breaks,
        (*config).into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn type_keys(composer: &mut Composer, keys: &str) -> Vec<Update> {
        keys.chars()
            .map(|ch| composer.key(&ch.to_string(), None))
            .collect()
    }

    #[test]
    fn aspiration_in_the_browser() {
        let mut composer = Composer::new(&Config::new());
        let updates = type_keys(&mut composer, "kh");
        assert_eq!(updates[0].pending, "ক");
        assert_eq!(updates[0].commit, "");
        assert_eq!(updates[1].pending, "খ");
        assert_eq!(updates[1].commit, "");
    }

    #[test]
    fn commit_on_a_word_break() {
        let mut composer = Composer::new(&Config::new());
        let updates = type_keys(&mut composer, "ami ");
        let committed: String = updates.iter().map(|u| u.commit.as_str()).collect();
        assert_eq!(committed, "আমি ");
        assert_eq!(updates.last().unwrap().pending, "");
    }

    #[test]
    fn default_settings() {
        let config = Config::new();
        assert!(config.bengali_digits && config.dari_for_period && config.smart_quotes);
        assert!(!config.autocorrect);
        assert_eq!(
            druti_core::Config::from(config),
            druti_core::Config::default()
        );
    }

    #[test]
    fn same_updates_as_the_core_composer() {
        let mut wasm = Composer::new(&Config::new());
        let mut core = druti_core::Composer::new(druti_core::Config::default());
        for key in [
            "k", "k", "h", "i", " ", "-", "-", "1", ".", "5", "\"", "Enter",
        ] {
            assert_eq!(
                wasm.key(key, Some("ক".into())),
                core.key(key, Some("ক")).into()
            );
        }
        assert_eq!(wasm.backspace(), core.backspace().into(), "nothing pending");
        for key in ["p", "o", "d", "m", "o"] {
            assert_eq!(wasm.key(key, None), core.key(key, None).into());
        }
        assert_eq!(
            wasm.backspace(),
            core.backspace().into(),
            "backspace in the word"
        );
        assert_eq!(
            wasm.key("h", Some("করত".into())),
            core.key("h", Some("করত")).into()
        );
        for context in ["ধ", "আমি "] {
            assert_eq!(
                wasm.matches_text_before_caret(context),
                core.matches_text_before_caret(context),
                "matches {context:?}"
            );
        }
        assert_eq!(wasm.flush(), core.flush().into());
        assert_eq!(wasm.reset(Some("ক".into())), core.reset(Some("ক")).into());
    }

    #[test]
    fn config_changes_apply_from_the_next_key() {
        let mut composer = Composer::new(&Config::new());
        let mut config = composer.config();
        config.bengali_digits = false;
        composer.set_config(&config);
        assert_eq!(composer.key("2", None).commit, "2");
    }

    #[test]
    fn autocorrect_matches_the_core_composer() {
        let mut config = Config::new();
        config.autocorrect = true;
        let mut wasm = Composer::new(&config);
        let mut core = druti_core::Composer::new(config.into());
        for key in ["a", "m", "r", "a", " "] {
            assert_eq!(wasm.key(key, None), core.key(key, None).into());
        }
        assert_eq!(wasm.pending(), "আমরা ");
        assert_eq!(wasm.backspace(), core.backspace().into(), "undo");
        assert_eq!(wasm.pending(), "আম্রা");
        assert_eq!(transpile_roman_document("amra", true, &config), "আমরা");
    }

    #[test]
    fn multi_line_document() {
        let input = "ami banglay gan gai\nami banglar gan gai";
        let config = Config::new();
        let expected = druti_core::transpile_roman_document_with_config(input, true, config.into());
        assert_eq!(transpile_roman_document(input, true, &config), expected);
        assert_eq!(expected.lines().count(), 2);
    }
}
