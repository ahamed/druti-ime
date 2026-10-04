//! Replays the hand-written composer fixtures in `tests/fixtures/composer/`, which
//! encode the ime-composer spec scenarios, through a simulated host document.

use std::path::PathBuf;

use druti_core::{Composer, Config, Update};
use serde::Deserialize;

#[derive(Deserialize)]
struct CaseFile {
    cases: Vec<Case>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct ConfigPatch {
    bengali_digits: Option<bool>,
    dari_for_period: Option<bool>,
    smart_quotes: Option<bool>,
    autocorrect: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Case {
    name: String,
    config: Option<ConfigPatch>,
    /// Committed text already in the host document before the first step.
    document: Option<String>,
    /// Pass the host's committed text as `ctx` to every key, Backspace and
    /// reset without its own `ctx`, as the playground does.
    host_context: Option<bool>,
    steps: Vec<Step>,
    committed: Option<String>,
}

#[derive(Deserialize)]
struct Step {
    key: Option<String>,
    ctx: Option<String>,
    backspace: Option<u8>,
    flush: Option<u8>,
    reset: Option<u8>,
    expect: Option<Expected>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Expected {
    commit: String,
    pending: String,
    handled: bool,
}

impl From<&Update> for Expected {
    fn from(u: &Update) -> Self {
        Self {
            commit: u.commit.clone(),
            pending: u.pending.clone(),
            handled: u.handled,
        }
    }
}

/// The host side: committed document text. The pending text is marked text
/// the host shows after it; the composer never touches the committed text.
#[derive(Default)]
struct Host {
    committed: Vec<u16>,
}

impl Host {
    fn apply(&mut self, update: &Update) {
        self.committed.extend(update.commit.encode_utf16());
    }
}

fn run(case: &Case) -> Result<(), String> {
    let mut config = Config::default();
    if let Some(patch) = &case.config {
        config.bengali_digits = patch.bengali_digits.unwrap_or(config.bengali_digits);
        config.dari_for_period = patch.dari_for_period.unwrap_or(config.dari_for_period);
        config.smart_quotes = patch.smart_quotes.unwrap_or(config.smart_quotes);
        config.autocorrect = patch.autocorrect.unwrap_or(config.autocorrect);
    }
    let mut composer = Composer::new(config);
    let mut host = Host {
        committed: case
            .document
            .as_deref()
            .unwrap_or_default()
            .encode_utf16()
            .collect(),
    };
    for (index, step) in case.steps.iter().enumerate() {
        let ctx = step.ctx.clone().or_else(|| {
            case.host_context
                .unwrap_or_default()
                .then(|| String::from_utf16_lossy(&host.committed))
        });
        let ctx = ctx.as_deref();
        let (label, update) = if let Some(key) = &step.key {
            (format!("key {key:?}"), composer.key(key, ctx))
        } else if step.backspace.is_some() {
            ("backspace".to_owned(), composer.backspace())
        } else if step.flush.is_some() {
            ("flush".to_owned(), composer.flush())
        } else if step.reset.is_some() {
            ("reset".to_owned(), composer.reset(ctx))
        } else {
            return Err(format!("step {index}: no operation"));
        };
        host.apply(&update);
        if let Some(expected) = &step.expect {
            let actual = Expected::from(&update);
            if &actual != expected {
                return Err(format!(
                    "step {index} ({label}): expected {expected:?}, actual {actual:?}"
                ));
            }
        }
        if update.pending != composer.pending() {
            return Err(format!(
                "step {index} ({label}): update pending differs from composer state"
            ));
        }
    }
    if let Some(expected) = &case.committed {
        let actual = String::from_utf16_lossy(&host.committed);
        if &actual != expected {
            return Err(format!(
                "committed text: expected {expected:?}, actual {actual:?}"
            ));
        }
    }
    Ok(())
}

fn replay(name: &str) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/composer")
        .join(name);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let file: CaseFile =
        serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let failures: Vec<String> = file
        .cases
        .iter()
        .filter_map(|case| run(case).err().map(|e| format!("  {:?}: {e}", case.name)))
        .collect();
    assert!(
        failures.is_empty(),
        "{name}: {} of {} failed\n{}",
        failures.len(),
        file.cases.len(),
        failures.join("\n")
    );
}

#[test]
fn split() {
    replay("split.json");
}

#[test]
fn hold() {
    replay("hold.json");
}

#[test]
fn context() {
    replay("context.json");
}

#[test]
fn backspace() {
    replay("backspace.json");
}

#[test]
fn config() {
    replay("config.json");
}

#[test]
fn autocorrect() {
    replay("autocorrect.json");
}

#[derive(Deserialize)]
struct RandomFile {
    cases: Vec<RandomCase>,
}

#[derive(Deserialize)]
struct RandomCase {
    name: String,
    steps: Vec<RandomStep>,
}

#[derive(Deserialize)]
struct RandomStep {
    k: Option<String>,
}

/// Property: typing any key sequence through the composer (without host
/// context) keeps committed + pending equal to the engine's output, and keeps
/// the pending text equal to the word being typed: the Bengali letters that
/// end the output, or a held `-` / `।` (ime-composer spec, "Commit and
/// pending split").
#[test]
fn composer_invariants_over_random_sequences() {
    use druti_core::Engine;
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/engine/random.json");
    let file: RandomFile = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let mut failures = Vec::new();
    let mut keys_checked = 0;
    for case in &file.cases {
        let mut composer = Composer::default();
        let mut reference = Engine::new();
        let mut host = Host::default();
        // Enter commits the word, so the next word starts after it.
        let mut word_start = 0;
        for key in case.steps.iter().filter_map(|s| s.k.as_deref()) {
            keys_checked += 1;
            let update = composer.key(key, None);
            if key == "Enter" {
                reference.end_cluster();
                word_start = reference.output().len();
            } else {
                reference.process(key, None);
            }
            host.apply(&update);
            let shown = format!(
                "{}{}",
                String::from_utf16_lossy(&host.committed),
                update.pending
            );
            let output = reference.output();
            let word = trailing_word(&output[word_start..]);
            let pending_ok = update.pending == word
                || (word.is_empty() && (update.pending == "-" || update.pending == "।"));
            if shown != output || !pending_ok {
                failures.push(format!(
                    "  {:?} key {key:?}: shown {shown:?} engine {output:?} pending {:?} word {word:?}",
                    case.name, update.pending,
                ));
                break;
            }
        }
    }
    assert!(keys_checked > 10_000, "only {keys_checked} keys checked");
    assert!(
        failures.is_empty(),
        "{} sequences failed\n{}",
        failures.len(),
        failures
            .iter()
            .take(10)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// The Bengali letters, signs and joiners that end `text`: the word being typed.
fn trailing_word(text: &str) -> String {
    let is_word = |ch: char| {
        ('\u{0980}'..='\u{09E3}').contains(&ch)
            || matches!(ch, '\u{09F0}' | '\u{09F1}' | '\u{200C}' | '\u{200D}')
    };
    let start = text
        .char_indices()
        .rev()
        .take_while(|&(_, ch)| is_word(ch))
        .last()
        .map_or(text.len(), |(index, _)| index);
    text[start..].to_owned()
}

/// `Composer::key_reads_document` must name every key whose result can depend
/// on the text before the caret, or hosts would skip reading it. Checked by
/// typing each key after a range of document endings on a fresh composer and
/// comparing with an empty document.
#[test]
fn key_reads_document_covers_every_context_sensitive_key() {
    let contexts = [
        "\u{0995}",                 // ক
        "\u{0995}\u{09BC}",         // ক + nukta
        "\u{0995}\u{0981}",         // কঁ
        "\u{0995}\u{09CD}\u{09B7}", // ক্ষ
        "\u{0995}\u{09BF}",         // কি
        "-",
        "\u{0964}", // ।
        ".",
        "\u{09E7}", // ১
        "1",
        "\u{201C}\u{0995}", // “ক
        "\u{2018}",         // ‘
        "\"",
        "'",
        "a ",
        "\u{1F600}", // emoji (surrogate pair)
    ];
    let mut keys: Vec<String> = (0x20u8..0x7F).map(|b| (b as char).to_string()).collect();
    keys.push("\t".into());
    let mut missing = Vec::new();
    for key in &keys {
        let baseline = Composer::default().key(key, Some(""));
        for ctx in contexts {
            let update = Composer::default().key(key, Some(ctx));
            if update != baseline && !Composer::key_reads_document(key) {
                missing.push(format!("{key:?} after {ctx:?}"));
            }
        }
    }
    assert!(
        missing.is_empty(),
        "context-sensitive keys not reported: {missing:?}"
    );
    assert!(Composer::key_reads_document("i"));
    assert!(Composer::key_reads_document("A"));
}

/// ime-composer spec, "Typing after Backspace or a caret move": a consonant starts a new
/// letter wherever the caret is, so hosts need not read the document for it.
#[test]
fn consonant_keys_do_not_read_the_document() {
    assert!(
        Composer::key_reads_document("i"),
        "a vowel attaches as a kar"
    );
    for key in ["h", "k", "^", "1", " "] {
        assert!(
            !Composer::key_reads_document(key),
            "{key:?} should not read"
        );
    }
}

/// ime-composer spec, "Checking the text before the caret".
#[test]
fn text_before_caret_is_checked_against_the_composer_output() {
    let mut composer = Composer::new(Config::default());
    for key in ["p", "o", " "] {
        composer.key(key, None);
    }
    assert!(
        composer.matches_text_before_caret("প "),
        "only the text near the caret"
    );
    assert!(
        !composer.matches_text_before_caret("আমি "),
        "the caret is elsewhere"
    );

    composer.reset(None);
    for key in ["k", "o", " "] {
        composer.key(key, None);
    }
    assert!(
        composer.matches_text_before_caret("আমি ক "),
        "a longer text before the caret"
    );

    composer.reset(None);
    assert!(
        !composer.matches_text_before_caret("প"),
        "nothing typed since the reset"
    );
}

/// ime-composer spec, "Document context with fallback": context supplied with
/// one key is kept for later keys sent without it, as the Mac sends them while
/// a word is pending.
#[test]
fn context_is_kept_for_later_keys() {
    let mut kept = Composer::default();
    kept.reset(None);
    kept.key("a", Some("ক"));
    let without = kept.key("'", None);

    let mut given = Composer::default();
    given.reset(None);
    given.key("a", Some("ক"));
    let with = given.key("'", Some("ক"));
    assert_eq!(without, with);
}
