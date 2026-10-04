//! Replays the golden fixtures in `tests/fixtures/engine/` through the
//! Rust engine. Each case runs on a fresh `Engine`; the first mismatching step
//! of every failing case is reported.

use std::fmt::Write as _;
use std::path::PathBuf;

use druti_core::{Action, Engine};
use serde::Deserialize;
use serde_json::Value;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/engine")
        .join(name)
}

fn load<T: for<'de> Deserialize<'de>>(name: &str) -> T {
    let path = fixture_path(name);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("cannot parse {}: {e}", path.display()))
}

#[derive(Deserialize)]
struct CaseFile<T> {
    cases: Vec<T>,
}

#[derive(Deserialize)]
struct EngineCase {
    name: String,
    steps: Vec<Step>,
}

#[derive(Deserialize)]
struct Step {
    k: Option<String>,
    c: Option<String>,
    bs: Option<u8>,
    en: Option<u8>,
    resume: Option<u8>,
    set: Option<String>,
    a: Vec<Value>,
    o: String,
    b: String,
}

fn decode_action(value: &Value) -> Action {
    let parts = value.as_array().expect("action is an array");
    let text = |i: usize| parts[i].as_str().expect("text").to_owned();
    let count =
        |i: usize| usize::try_from(parts[i].as_u64().expect("count")).expect("count fits usize");
    match parts[0].as_str().expect("action tag") {
        "i" => Action::Insert { text: text(1) },
        "r" => Action::Replace {
            chars_back: count(1),
            text: text(2),
        },
        "d" => Action::Delete {
            chars_back: count(1),
        },
        "s" => Action::SplitBlock,
        other => panic!("unknown action tag {other}"),
    }
}

fn describe_op(step: &Step) -> String {
    if let Some(key) = &step.k {
        match &step.c {
            Some(c) => format!("key {key:?} with textBeforeCaret {c:?}"),
            None => format!("key {key:?}"),
        }
    } else if step.bs.is_some() {
        "backspace".into()
    } else if step.en.is_some() {
        "toggle English".into()
    } else if step.resume.is_some() {
        "resume cluster".into()
    } else {
        format!("set output {:?}", step.set.as_deref().unwrap_or_default())
    }
}

/// Replays one case; returns a description of the first mismatch.
fn replay(case: &EngineCase) -> Result<(), String> {
    let mut engine = Engine::new();
    for (index, step) in case.steps.iter().enumerate() {
        let actions = if let Some(key) = &step.k {
            engine.process(key, step.c.as_deref())
        } else if step.bs.is_some() {
            engine.process_backspace()
        } else if step.en.is_some() {
            engine.toggle_english_mode();
            Vec::new()
        } else if step.resume.is_some() {
            engine.resume_cluster();
            Vec::new()
        } else {
            engine.set_output(step.set.as_deref().unwrap_or_default());
            Vec::new()
        };
        let expected: Vec<Action> = step.a.iter().map(decode_action).collect();
        if actions != expected || engine.output() != step.o || engine.buffer() != step.b {
            return Err(format!(
                "step {index} ({}):\n    actions  expected {expected:?}\n             actual   {actions:?}\n    output   expected {:?} actual {:?}\n    buffer   expected {:?} actual {:?}",
                describe_op(step),
                step.o,
                engine.output(),
                step.b,
                engine.buffer(),
            ));
        }
    }
    Ok(())
}

fn replay_file(name: &str) {
    let file: CaseFile<EngineCase> = load(name);
    let total = file.cases.len();
    let failures: Vec<(String, String)> = file
        .cases
        .iter()
        .filter_map(|case| replay(case).err().map(|e| (case.name.clone(), e)))
        .collect();
    if failures.is_empty() {
        return;
    }
    let mut report = format!("{name}: {} of {total} cases failed\n", failures.len());
    for (case, error) in failures.iter().take(10) {
        let _ = writeln!(report, "  case {case:?} {error}");
    }
    if failures.len() > 10 {
        let _ = writeln!(report, "  ... and {} more", failures.len() - 10);
    }
    panic!("{report}");
}

#[test]
fn unit_fixtures() {
    replay_file("unit.json");
}

#[test]
fn word_fixtures() {
    replay_file("words.json");
}

#[test]
fn random_fixtures() {
    replay_file("random.json");
}

fn string_pairs(value: &Value) -> Vec<(String, String)> {
    value
        .as_array()
        .expect("pair list")
        .iter()
        .map(|pair| {
            let pair = pair.as_array().expect("pair");
            (
                pair[0].as_str().unwrap().to_owned(),
                pair[1].as_str().unwrap().to_owned(),
            )
        })
        .collect()
}

fn owned_pairs(table: &[(&str, &str)]) -> Vec<(String, String)> {
    table
        .iter()
        .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
        .collect()
}

fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("string list")
        .iter()
        .map(|s| s.as_str().unwrap().to_owned())
        .collect()
}

fn owned(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| (*s).to_owned()).collect()
}

#[test]
fn data_tables_match_golden() {
    use druti_core::data::*;
    let ts: Value = load("data.json");
    assert_eq!(ts["hasant"].as_str().unwrap(), HASANT);
    assert_eq!(
        string_pairs(&ts["symbols"]),
        owned_pairs(SYMBOLS),
        "symbols"
    );
    assert_eq!(
        string_pairs(&ts["numberMap"]),
        owned_pairs(NUMBER_MAP),
        "numberMap"
    );
    assert_eq!(
        string_pairs(&ts["specialCharactersMap"]),
        owned_pairs(SPECIAL_CHARACTERS_MAP),
        "specialCharactersMap"
    );
    assert_eq!(
        strings(&ts["specialCharacterInputs"]),
        owned(SPECIAL_CHARACTER_INPUTS),
        "specialCharacterInputs"
    );
    let vowels: Vec<(String, String, String)> = ts["romanToPhoneticVowels"]
        .as_array()
        .unwrap()
        .iter()
        .map(|pair| {
            let key = pair[0].as_str().unwrap().to_owned();
            let ind = pair[1]["ind"].as_str().unwrap().to_owned();
            let kar = pair[1]["kar"].as_str().unwrap().to_owned();
            (key, ind, kar)
        })
        .collect();
    let rust_vowels: Vec<(String, String, String)> = ROMAN_TO_PHONETIC_VOWELS
        .iter()
        .map(|(k, v)| ((*k).to_owned(), v.ind.to_owned(), v.kar.to_owned()))
        .collect();
    assert_eq!(vowels, rust_vowels, "romanToPhoneticVowels");
    assert_eq!(
        string_pairs(&ts["defaultConsonantByRomanKey"]),
        owned_pairs(DEFAULT_CONSONANT_BY_ROMAN_KEY),
        "defaultConsonantByRomanKey"
    );
    assert_eq!(
        string_pairs(&ts["capitalRomanToConsonant"]),
        owned_pairs(CAPITAL_ROMAN_TO_CONSONANT),
        "capitalRomanToConsonant"
    );
    assert_eq!(
        string_pairs(&ts["aspiratedConsonantByBase"]),
        owned_pairs(ASPIRATED_CONSONANT_BY_BASE),
        "aspiratedConsonantByBase"
    );
    assert_eq!(
        strings(&ts["noJoinAfter"]),
        owned(NO_JOIN_AFTER),
        "noJoinAfter"
    );
    assert_eq!(
        strings(&ts["pholaAfterNoJoin"]),
        owned(PHOLA_AFTER_NO_JOIN),
        "pholaAfterNoJoin"
    );
    assert_eq!(ts["zwj"].as_str().unwrap(), ZWJ);
    assert_eq!(
        strings(&ts["phoneticConsonantGraphemes"]),
        owned(PHONETIC_CONSONANT_GRAPHEMES),
        "phoneticConsonantGraphemes"
    );
    assert_eq!(
        strings(&ts["modifierGraphemeChars"]),
        owned(MODIFIER_GRAPHEME_CHARS),
        "modifierGraphemeChars"
    );
    assert_eq!(
        strings(&ts["bengaliConsonantLetterGraphemes"]),
        owned(&bengali_consonant_letter_graphemes()),
        "bengaliConsonantLetterGraphemes"
    );
    assert_eq!(
        strings(&ts["karTakingConsonantGraphemes"]),
        owned(&kar_taking_consonant_graphemes()),
        "karTakingConsonantGraphemes"
    );
    assert_eq!(
        strings(&ts["independentVowelGraphemes"]),
        owned(INDEPENDENT_VOWEL_GRAPHEMES),
        "independentVowelGraphemes"
    );
    assert_eq!(
        strings(&ts["dependentVowelGraphemes"]),
        owned(DEPENDENT_VOWEL_GRAPHEMES),
        "dependentVowelGraphemes"
    );
}

#[derive(Deserialize)]
struct AttachCase {
    text: String,
    #[serde(rename = "karTaking")]
    kar_taking: bool,
    #[serde(rename = "consonantChandrabindu")]
    consonant_chandrabindu: bool,
}

#[test]
fn vowel_attach_fixtures() {
    use druti_core::{ends_with_consonant_and_chandrabindu, ends_with_kar_taking_consonant};
    let file: CaseFile<AttachCase> = load("vowel-attach.json");
    let failures: Vec<String> = file
        .cases
        .iter()
        .filter(|case| {
            ends_with_kar_taking_consonant(&case.text) != case.kar_taking
                || ends_with_consonant_and_chandrabindu(&case.text) != case.consonant_chandrabindu
        })
        .map(|case| {
            let points: Vec<String> = case
                .text
                .chars()
                .map(|c| format!("U+{:04X}", c as u32))
                .collect();
            format!(
                "  {:?} {points:?}: expected karTaking {} consonantChandrabindu {}",
                case.text, case.kar_taking, case.consonant_chandrabindu
            )
        })
        .collect();
    assert!(
        failures.is_empty(),
        "vowel-attach.json: {} of {} failed\n{}",
        failures.len(),
        file.cases.len(),
        failures.join("\n")
    );
}

#[derive(Deserialize)]
struct TranspileCase {
    input: String,
    #[serde(rename = "preserveLineBreaks")]
    preserve_line_breaks: Option<bool>,
    output: String,
}

#[test]
fn transpile_fixtures() {
    let file: CaseFile<TranspileCase> = load("transpile.json");
    let failures: Vec<String> = file
        .cases
        .iter()
        .filter_map(|case| {
            let preserve = case.preserve_line_breaks.unwrap_or(true);
            let actual = druti_core::transpile_roman_document(&case.input, preserve);
            (actual != case.output).then(|| {
                format!(
                    "  {:?} (preserve {preserve}): expected {:?} actual {actual:?}",
                    case.input, case.output
                )
            })
        })
        .collect();
    assert!(
        failures.is_empty(),
        "transpile.json: {} of {} failed\n{}",
        failures.len(),
        file.cases.len(),
        failures.join("\n")
    );
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AutocorrectTranspileCase {
    name: String,
    input: String,
    preserve_line_breaks: Option<bool>,
    autocorrect: Option<bool>,
    output: String,
}

/// autocorrect spec, "Autocorrect in bulk conversion".
#[test]
fn autocorrect_transpile_fixtures() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/autocorrect/transpile.json");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let file: CaseFile<AutocorrectTranspileCase> =
        serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let failures: Vec<String> = file
        .cases
        .iter()
        .filter_map(|case| {
            let mut config = druti_core::Config::default();
            if let Some(autocorrect) = case.autocorrect {
                config.autocorrect = autocorrect;
            }
            let preserve = case.preserve_line_breaks.unwrap_or(true);
            let actual =
                druti_core::transpile_roman_document_with_config(&case.input, preserve, config);
            (actual != case.output).then(|| {
                format!(
                    "  {:?}: expected {:?} actual {actual:?}",
                    case.name, case.output
                )
            })
        })
        .collect();
    assert!(
        failures.is_empty(),
        "autocorrect/transpile.json: {} of {} failed\n{}",
        failures.len(),
        file.cases.len(),
        failures.join("\n")
    );
}
