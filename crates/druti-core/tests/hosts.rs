//! Types key scripts through the composer into simulated hosts that differ in
//! what they support, and checks that every host ends with the same text
//! (ime-composer spec, "Same result in every host").
//!
//! The hosts differ in when they can pass the text before the caret: the web
//! playground with every key, a Mac app only while nothing is marked, a
//! terminal never. None can be asked to change committed text, because
//! [`Update`] cannot express it: that is what makes Cocoa apps, Chromium-based
//! apps (the Claude app, Cursor's `EditContext` editor) and the playground
//! behave the same. When the composer does not handle a Backspace, the host
//! deletes one code point, as Chromium does for Bengali.

use std::path::PathBuf;

use druti_core::{Composer, Config, Update};
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    /// `examples/playground`: passes the text before the caret with every key.
    Playground,
    /// An app with text access on the Mac input source (Cocoa, Chromium).
    Mac,
    /// A terminal on the Mac input source: no text access.
    Terminal,
}

const TEXT_HOSTS: [Kind; 2] = [Kind::Playground, Kind::Mac];
const ALL_HOSTS: [Kind; 3] = [Kind::Playground, Kind::Mac, Kind::Terminal];

/// One step of a script.
#[derive(Clone, Copy, Debug)]
enum Op<'a> {
    /// Types each character as one key.
    Keys(&'a str),
    /// Presses Backspace this many times.
    Backspace(usize),
    /// Clicks at this UTF-16 offset of the visible text.
    MoveTo(usize),
}

struct Host {
    kind: Kind,
    composer: Composer,
    /// Committed document text.
    text: Vec<u16>,
    /// UTF-16 offset in `text` where the marked (pending) text sits.
    caret: usize,
    marked: Vec<u16>,
}

impl Host {
    fn with_config(kind: Kind, config: Config) -> Self {
        Self {
            kind,
            composer: Composer::new(config),
            text: Vec::new(),
            caret: 0,
            marked: Vec::new(),
        }
    }

    fn visible(&self) -> String {
        let mut units = self.text[..self.caret].to_vec();
        units.extend(&self.marked);
        units.extend(&self.text[self.caret..]);
        String::from_utf16_lossy(&units)
    }

    fn before_caret(&self) -> String {
        String::from_utf16_lossy(&self.text[..self.caret])
    }

    /// What the host passes as `text_before_caret` with `key`: the playground
    /// always; the Mac only while nothing is marked, for keys that read it.
    fn context(&self, key: &str) -> Option<String> {
        match self.kind {
            Kind::Playground => Some(self.before_caret()),
            Kind::Mac if self.marked.is_empty() && Composer::key_reads_document(key) => {
                Some(self.before_caret())
            }
            _ => None,
        }
    }

    /// The composer's contract has no way to change committed text, so the
    /// hosts differ only in what they can read.
    fn apply(&mut self, update: &Update) {
        let commit: Vec<u16> = update.commit.encode_utf16().collect();
        self.text
            .splice(self.caret..self.caret, commit.iter().copied());
        self.caret += commit.len();
        self.marked = update.pending.encode_utf16().collect();
    }

    /// The host's own Backspace: one code point before the caret.
    fn delete_code_point(&mut self) {
        let Some(last) = self.text[..self.caret].last() else {
            return;
        };
        let len = if (0xDC00..=0xDFFF).contains(last) && self.caret >= 2 {
            2
        } else {
            1
        };
        self.text.drain(self.caret - len..self.caret);
        self.caret -= len;
    }

    fn key(&mut self, key: &str) {
        let context = self.context(key);
        let update = self.composer.key(key, context.as_deref());
        self.apply(&update);
        if !update.handled && key == "Enter" {
            self.text.insert(self.caret, u16::from(b'\n'));
            self.caret += 1;
        }
    }

    fn backspace(&mut self) {
        let update = self.composer.backspace();
        self.apply(&update);
        if !update.handled {
            assert!(
                self.marked.is_empty(),
                "{:?}: unhandled Backspace with marked text",
                self.kind
            );
            self.delete_code_point();
            if self.kind == Kind::Playground {
                // The playground re-reads the document after the browser edits it.
                let before = self.before_caret();
                self.composer.reset(Some(&before));
            }
        }
    }

    /// A click: the marked text is committed where it is, then the caret moves.
    fn move_to(&mut self, offset: usize) {
        let update = self.composer.flush();
        self.apply(&update);
        self.caret = offset.min(self.text.len());
        let before = self.before_caret();
        let context = (self.kind == Kind::Playground).then_some(before.as_str());
        self.composer.reset(context);
    }

    fn run(&mut self, script: &[Op<'_>]) {
        for op in script {
            match *op {
                Op::Keys(keys) => {
                    for key in keys.chars() {
                        self.key(&key.to_string());
                    }
                }
                Op::Backspace(count) => {
                    for _ in 0..count {
                        self.backspace();
                    }
                }
                Op::MoveTo(offset) => self.move_to(offset),
            }
        }
    }
}

/// Runs `script` on each of `hosts` and checks the visible text.
fn check(hosts: &[Kind], script: &[Op<'_>], expected: &str) {
    check_with(Config::default(), hosts, script, expected);
}

fn check_with(config: Config, hosts: &[Kind], script: &[Op<'_>], expected: &str) {
    for &kind in hosts {
        let mut host = Host::with_config(kind, config);
        host.run(script);
        assert_eq!(host.visible(), expected, "{kind:?}: {script:?}");
    }
}

#[test]
fn backspace_in_the_word_being_typed_removes_the_last_consonant_of_a_conjunct() {
    check(&ALL_HOSTS, &[Op::Keys("podmo"), Op::Backspace(1)], "পদ");
    check(
        &ALL_HOSTS,
        &[Op::Keys("ekoTa podmo"), Op::Backspace(1)],
        "একটা পদ",
    );
    check(&ALL_HOSTS, &[Op::Keys("kkh"), Op::Backspace(1)], "ক");
}

#[test]
fn backspace_in_the_word_being_typed_removes_one_letter_at_a_time() {
    check(&ALL_HOSTS, &[Op::Keys("korote"), Op::Backspace(1)], "করত");
    check(&ALL_HOSTS, &[Op::Keys("korote"), Op::Backspace(2)], "কর");
    check(&ALL_HOSTS, &[Op::Keys("korote"), Op::Backspace(3)], "ক");
    check(&ALL_HOSTS, &[Op::Keys("korote"), Op::Backspace(4)], "");
}

#[test]
fn a_consonant_after_backspace_starts_a_new_letter() {
    check(
        &ALL_HOSTS,
        &[Op::Keys("ka"), Op::Backspace(1), Op::Keys("k")],
        "কক",
    );
    check(
        &ALL_HOSTS,
        &[Op::Keys("dm"), Op::Backspace(1), Op::Keys("h")],
        "দহ",
    );
    check(
        &ALL_HOSTS,
        &[Op::Keys("korote"), Op::Backspace(1), Op::Keys("h")],
        "করতহ",
    );
}

#[test]
fn a_reph_typed_with_rr_and_its_backspace_look_the_same_everywhere() {
    // rr-reph design D2 and D5: the armed hasant shows at once, and Backspace
    // removes only it.
    check(&ALL_HOSTS, &[Op::Keys("korrta")], "কর্তা");
    check(&ALL_HOSTS, &[Op::Keys("korr"), Op::Backspace(1)], "কর");
    check(
        &ALL_HOSTS,
        &[Op::Keys("korr"), Op::Backspace(1), Op::Keys("ta")],
        "করতা",
    );
    // Like দ্ম → দ, a consonant goes with the hasant joining it.
    check(&ALL_HOSTS, &[Op::Keys("korrta"), Op::Backspace(2)], "কর");
    check(&ALL_HOSTS, &[Op::Keys("pory"), Op::Backspace(1)], "পর");
}

#[test]
fn autocorrect_looks_the_same_everywhere() {
    // autocorrect design D4 and D5: the correction is pending text, so no
    // host is asked to change committed text.
    let config = Config {
        autocorrect: true,
        ..Config::default()
    };
    let check =
        |script: &[Op<'_>], expected: &str| check_with(config, &ALL_HOSTS, script, expected);
    check(&[Op::Keys("amra ekTa ")], "আমরা একটা ");
    check(&[Op::Keys("amra "), Op::Backspace(1)], "আম্রা");
    check(
        &[Op::Keys("amra "), Op::Backspace(1), Op::Keys(" ")],
        "আম্রা ",
    );
    check(&[Op::Keys("amra "), Op::Backspace(2)], "আম্র");
    check(&[Op::Keys("amra."), Op::Keys("ami")], "আমরা।আমি");
    check(&[Op::Keys("ekTa "), Op::MoveTo(0), Op::Keys("o")], "অএকটা ");
    check(&[Op::Keys("amra"), Op::MoveTo(0)], "আম্রা");
}

#[test]
fn a_vowel_after_backspace_still_attaches_as_a_kar() {
    check(
        &ALL_HOSTS,
        &[Op::Keys("ka"), Op::Backspace(1), Op::Keys("i")],
        "কি",
    );
    check(
        &ALL_HOSTS,
        &[Op::Keys("dm"), Op::Backspace(1), Op::Keys("a")],
        "দা",
    );
}

#[test]
fn backspace_in_committed_text_is_left_to_the_host() {
    // One code point per Backspace, as the app deletes it.
    check(&ALL_HOSTS, &[Op::Keys("podmo "), Op::Backspace(1)], "পদ্ম");
    check(&ALL_HOSTS, &[Op::Keys("podmo "), Op::Backspace(2)], "পদ্");
    check(&ALL_HOSTS, &[Op::Keys("podmo "), Op::Backspace(3)], "পদ");
    check(&ALL_HOSTS, &[Op::Keys("korote "), Op::Backspace(2)], "করত");
}

#[test]
fn a_consonant_after_a_caret_move_starts_a_new_letter() {
    check(
        &ALL_HOSTS,
        &[Op::Keys("kor"), Op::MoveTo(0), Op::Keys("k")],
        "ককর",
    );
    check(
        &ALL_HOSTS,
        &[Op::Keys("kor "), Op::MoveTo(2), Op::Keys("k")],
        "করক ",
    );
    check(
        &ALL_HOSTS,
        &[Op::Keys("korot "), Op::MoveTo(3), Op::Keys("h")],
        "করতহ ",
    );
}

#[test]
fn a_vowel_after_a_caret_move_attaches_where_the_app_exposes_its_text() {
    check(
        &TEXT_HOSTS,
        &[Op::Keys("korote "), Op::MoveTo(2), Op::Keys("i")],
        "করিতে ",
    );
    // A terminal can't say what is before the caret.
    check(
        &[Kind::Terminal],
        &[Op::Keys("korote "), Op::MoveTo(2), Op::Keys("i")],
        "করইতে ",
    );
}

#[test]
fn words_are_committed_at_word_breaks() {
    check(&ALL_HOSTS, &[Op::Keys("khub ")], "খুব ");
    check(&ALL_HOSTS, &[Op::Keys("a--")], "আ—");
    check(&ALL_HOSTS, &[Op::Keys("1.5")], "১.৫");
    check(&ALL_HOSTS, &[Op::Keys("ami.")], "আমি।");
    check(&ALL_HOSTS, &[Op::Keys("ki?")], "কি?");
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
    bs: Option<u8>,
}

/// Property: over the seeded random key and Backspace sequences, with
/// Autocorrect off and on, the hosts that expose their text show the same
/// text after every step.
#[test]
fn every_host_shows_the_same_text_over_random_sequences() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/engine/random.json");
    let file: RandomFile = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let mut steps_checked = 0;
    let autocorrect = Config {
        autocorrect: true,
        ..Config::default()
    };
    for (config, case) in [Config::default(), autocorrect]
        .into_iter()
        .flat_map(|config| file.cases.iter().map(move |case| (config, case)))
    {
        let mut hosts: Vec<Host> = ALL_HOSTS
            .iter()
            .map(|&kind| Host::with_config(kind, config))
            .collect();
        for (index, step) in case.steps.iter().enumerate() {
            for host in &mut hosts {
                if let Some(key) = &step.k {
                    host.key(key);
                } else if step.bs.is_some() {
                    host.backspace();
                }
            }
            steps_checked += 1;
            let shown: Vec<String> = hosts.iter().map(Host::visible).collect();
            assert!(
                shown[..TEXT_HOSTS.len()].iter().all(|s| s == &shown[0]),
                "{:?} step {index} ({config:?}): hosts differ: {shown:?}",
                case.name
            );
        }
    }
    assert!(steps_checked > 10_000, "only {steps_checked} steps checked");
}
