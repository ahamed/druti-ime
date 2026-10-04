//! Lookup tables.
//!
//! Table order matters (earlier entries win), and `tests/fixtures.rs`
//! compares each one entry by entry with `tests/fixtures/engine/data.json`.
//! The nukta letters are written as escapes (U+09DC, U+09DD, U+09DF) because
//! Unicode normalization would otherwise decompose them.

/// Hasant (virama) ্, which stacks two consonants.
pub const HASANT: &str = "\u{09CD}";
/// The `.` key, and the ASCII full stop it stays after a digit or another dot.
pub const FULL_STOP: &str = ".";
/// The space key.
pub const SPACE: &str = " ";
/// The key name hosts pass for Return/Enter.
pub const ENTER_KEY: &str = "Enter";
/// দাঁড়ি (।), the Bengali full stop.
pub const DARI: &str = "\u{0964}";
/// The `-` key; two in a row become [`DOUBLE_DASH`].
pub const DASH: &str = "-";
/// Em dash (—), typed as `--`.
pub const DOUBLE_DASH: &str = "\u{2014}";
/// Left double quotation mark (“).
pub const TYPOGRAPHIC_DOUBLE_QUOTE_OPEN: &str = "\u{201C}";
/// Right double quotation mark (”).
pub const TYPOGRAPHIC_DOUBLE_QUOTE_CLOSE: &str = "\u{201D}";
/// Left single quotation mark (‘).
pub const TYPOGRAPHIC_SINGLE_QUOTE_OPEN: &str = "\u{2018}";
/// Right single quotation mark (’), also the apostrophe.
pub const TYPOGRAPHIC_SINGLE_QUOTE_CLOSE: &str = "\u{2019}";

/// The named symbols above, by name, for the fixture comparison.
pub const SYMBOLS: &[(&str, &str)] = &[
    ("FULL_STOP", FULL_STOP),
    ("SPACE", SPACE),
    ("ENTER_KEY", ENTER_KEY),
    ("DARI", DARI),
    ("DASH", DASH),
    ("DOUBLE_DASH", DOUBLE_DASH),
    ("CAP", "^"),
    ("COLON", ":"),
    (
        "TYPOGRAPHIC_DOUBLE_QUOTE_OPEN",
        TYPOGRAPHIC_DOUBLE_QUOTE_OPEN,
    ),
    (
        "TYPOGRAPHIC_DOUBLE_QUOTE_CLOSE",
        TYPOGRAPHIC_DOUBLE_QUOTE_CLOSE,
    ),
    (
        "TYPOGRAPHIC_SINGLE_QUOTE_OPEN",
        TYPOGRAPHIC_SINGLE_QUOTE_OPEN,
    ),
    (
        "TYPOGRAPHIC_SINGLE_QUOTE_CLOSE",
        TYPOGRAPHIC_SINGLE_QUOTE_CLOSE,
    ),
];

// phoneticVowels
/// অ
pub const SWAR_E_O: &str = "\u{0985}";
/// আ
pub const SWAR_E_A: &str = "\u{0986}";
/// ই
pub const RASSAW_E: &str = "\u{0987}";
/// ঈ
pub const DIRGHA_E: &str = "\u{0988}";
/// উ
pub const RASSAW_U: &str = "\u{0989}";
/// ঊ
pub const DIRGHA_U: &str = "\u{098A}";
/// ঋ
pub const RASSAW_RI: &str = "\u{098B}";
/// এ
pub const VOWEL_A: &str = "\u{098F}";
/// ঐ
pub const VOWEL_OI: &str = "\u{0990}";
/// ও
pub const VOWEL_O: &str = "\u{0993}";
/// ঔ
pub const VOWEL_OU: &str = "\u{0994}";

// phoneticKar
/// া (aa-kar).
pub const A_KAR: &str = "\u{09BE}";
/// ি (i-kar).
pub const RASSAW_E_KAR: &str = "\u{09BF}";
/// ী (ii-kar).
pub const DIRGHA_E_KAR: &str = "\u{09C0}";
/// ু (u-kar).
pub const RASSAW_U_KAR: &str = "\u{09C1}";
/// ূ (uu-kar).
pub const DIRGHA_U_KAR: &str = "\u{09C2}";
/// ৃ (ri-kar).
pub const RASSAW_RI_KAR: &str = "\u{09C3}";
/// ে (e-kar).
pub const E_KAR: &str = "\u{09C7}";
/// ৈ (oi-kar).
pub const OI_KAR: &str = "\u{09C8}";
/// ো (o-kar).
pub const O_KAR: &str = "\u{09CB}";
/// ৌ (ou-kar).
pub const OU_KAR: &str = "\u{09CC}";

// phoneticConsonants
/// ক
pub const KONTHYO_KO: &str = "\u{0995}";
/// খ
pub const KONTHYO_KHO: &str = "\u{0996}";
/// গ
pub const KONTHYO_GO: &str = "\u{0997}";
/// ঘ
pub const KONTHYO_GHO: &str = "\u{0998}";
/// ঙ
pub const KONTHYO_UNGO: &str = "\u{0999}";
/// চ
pub const TALOBBO_CHO: &str = "\u{099A}";
/// ছ
pub const TALOBBO_CHHO: &str = "\u{099B}";
/// জ
pub const BORGIYO_JO: &str = "\u{099C}";
/// ঝ
pub const BORGIYO_JHO: &str = "\u{099D}";
/// ঞ
pub const TALOBBO_NYO: &str = "\u{099E}";
/// ট
pub const MURDHONNO_TO: &str = "\u{099F}";
/// ঠ
pub const MURDHONNO_THO: &str = "\u{09A0}";
/// ড
pub const MURDHONNO_DO: &str = "\u{09A1}";
/// ঢ
pub const MURDHONNO_DHO: &str = "\u{09A2}";
/// ণ
pub const MURDHONNO_NO: &str = "\u{09A3}";
/// ত
pub const DONTO_TO: &str = "\u{09A4}";
/// থ
pub const DONTO_THO: &str = "\u{09A5}";
/// দ
pub const DONTO_DO: &str = "\u{09A6}";
/// ধ
pub const DONTO_DHO: &str = "\u{09A7}";
/// ন
pub const DONTO_NO: &str = "\u{09A8}";
/// প
pub const OSHTHO_PO: &str = "\u{09AA}";
/// ফ
pub const OSHTHO_PHO: &str = "\u{09AB}";
/// ব
pub const OSHTHO_BO: &str = "\u{09AC}";
/// ভ
pub const OSHTHO_BHO: &str = "\u{09AD}";
/// ম
pub const OSHTHO_MO: &str = "\u{09AE}";
/// য
pub const ONTOSTHO_JO: &str = "\u{09AF}";
/// র
pub const ONTOSTHO_RO: &str = "\u{09B0}";
/// ল
pub const ONTOSTHO_LO: &str = "\u{09B2}";
/// শ
pub const TALOBBO_SHO: &str = "\u{09B6}";
/// ষ
pub const MURDHONNO_SHO: &str = "\u{09B7}";
/// স
pub const DONTO_SHO: &str = "\u{09B8}";
/// হ
pub const USHMO_HO: &str = "\u{09B9}";
/// ড়
pub const D_E_SHUNNO_RO: &str = "\u{09DC}";
/// ঢ়
pub const DH_E_SHUNNO_RO: &str = "\u{09DD}";
/// য়
pub const ONTOSTHO_YO: &str = "\u{09DF}";
/// ৎ
pub const KHONDO_TO: &str = "\u{09CE}";
/// ঁ
pub const CHONDROBINDU: &str = "\u{0981}";
/// ং
pub const ONUSHWAR: &str = "\u{0982}";
/// ঃ
pub const BISHORGO: &str = "\u{0983}";

/// `Object.values(phoneticConsonants)`, in declaration order.
pub const PHONETIC_CONSONANT_GRAPHEMES: &[&str] = &[
    KONTHYO_KO,
    KONTHYO_KHO,
    KONTHYO_GO,
    KONTHYO_GHO,
    KONTHYO_UNGO,
    TALOBBO_CHO,
    TALOBBO_CHHO,
    BORGIYO_JO,
    BORGIYO_JHO,
    TALOBBO_NYO,
    MURDHONNO_TO,
    MURDHONNO_THO,
    MURDHONNO_DO,
    MURDHONNO_DHO,
    MURDHONNO_NO,
    DONTO_TO,
    DONTO_THO,
    DONTO_DO,
    DONTO_DHO,
    DONTO_NO,
    OSHTHO_PO,
    OSHTHO_PHO,
    OSHTHO_BO,
    OSHTHO_BHO,
    OSHTHO_MO,
    ONTOSTHO_JO,
    ONTOSTHO_RO,
    ONTOSTHO_LO,
    TALOBBO_SHO,
    MURDHONNO_SHO,
    DONTO_SHO,
    USHMO_HO,
    D_E_SHUNNO_RO,
    DH_E_SHUNNO_RO,
    ONTOSTHO_YO,
    KHONDO_TO,
    CHONDROBINDU,
    ONUSHWAR,
    BISHORGO,
];

/// Chandrabindu, onushwar and bishorgo: signs that follow a letter but are not letters.
pub const MODIFIER_GRAPHEME_CHARS: &[&str] = &[CHONDROBINDU, ONUSHWAR, BISHORGO];

/// `Object.values(phoneticVowels)`.
pub const INDEPENDENT_VOWEL_GRAPHEMES: &[&str] = &[
    SWAR_E_O, SWAR_E_A, RASSAW_E, DIRGHA_E, RASSAW_U, DIRGHA_U, RASSAW_RI, VOWEL_A, VOWEL_OI,
    VOWEL_O, VOWEL_OU,
];

/// `Object.values(phoneticKar)`.
pub const DEPENDENT_VOWEL_GRAPHEMES: &[&str] = &[
    A_KAR,
    RASSAW_E_KAR,
    DIRGHA_E_KAR,
    RASSAW_U_KAR,
    DIRGHA_U_KAR,
    RASSAW_RI_KAR,
    E_KAR,
    OI_KAR,
    O_KAR,
    OU_KAR,
];

/// ASCII digit → Bengali digit.
pub const NUMBER_MAP: &[(&str, &str)] = &[
    ("1", "\u{09E7}"),
    ("2", "\u{09E8}"),
    ("3", "\u{09E9}"),
    ("4", "\u{09EA}"),
    ("5", "\u{09EB}"),
    ("6", "\u{09EC}"),
    ("7", "\u{09ED}"),
    ("8", "\u{09EE}"),
    ("9", "\u{09EF}"),
    ("0", "\u{09E6}"),
];

/// Punctuation keys with a fixed Bengali output.
pub const SPECIAL_CHARACTERS_MAP: &[(&str, &str)] = &[
    (".", DARI),
    ("^", CHONDROBINDU),
    (":", BISHORGO),
    (",", ","),
];

/// Keys handled as punctuation rather than letters.
pub const SPECIAL_CHARACTER_INPUTS: &[&str] = &[".", "^", ":", ",", "-", "\"", "'"];

/// Independent vowel and its dependent sign (kar) for a roman vowel key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VowelData {
    /// The independent vowel, used at the start of a syllable (আ).
    pub ind: &'static str,
    /// The dependent sign, used after a consonant (া). Empty for `o`, the inherent vowel.
    pub kar: &'static str,
}

const fn vowel(ind: &'static str, kar: &'static str) -> VowelData {
    VowelData { ind, kar }
}

/// Roman vowel key → its independent vowel and kar.
pub const ROMAN_TO_PHONETIC_VOWELS: &[(&str, VowelData)] = &[
    ("o", vowel(SWAR_E_O, "")),
    ("a", vowel(SWAR_E_A, A_KAR)),
    ("A", vowel(SWAR_E_A, A_KAR)),
    ("i", vowel(RASSAW_E, RASSAW_E_KAR)),
    ("I", vowel(DIRGHA_E, DIRGHA_E_KAR)),
    ("u", vowel(RASSAW_U, RASSAW_U_KAR)),
    ("U", vowel(DIRGHA_U, DIRGHA_U_KAR)),
    ("e", vowel(VOWEL_A, E_KAR)),
    ("E", vowel(VOWEL_A, E_KAR)),
    ("O", vowel(VOWEL_O, O_KAR)),
];

/// Lowercase roman key → consonant.
pub const DEFAULT_CONSONANT_BY_ROMAN_KEY: &[(&str, &str)] = &[
    ("q", KONTHYO_KO),
    ("k", KONTHYO_KO),
    ("g", KONTHYO_GO),
    ("c", TALOBBO_CHO),
    ("j", BORGIYO_JO),
    ("t", DONTO_TO),
    ("d", DONTO_DO),
    ("n", DONTO_NO),
    ("p", OSHTHO_PO),
    ("b", OSHTHO_BO),
    ("m", OSHTHO_MO),
    ("r", ONTOSTHO_RO),
    ("l", ONTOSTHO_LO),
    ("s", DONTO_SHO),
    ("z", ONTOSTHO_JO),
    ("y", ONTOSTHO_YO),
    ("v", OSHTHO_BHO),
    ("h", USHMO_HO),
    ("w", OSHTHO_BO),
    ("f", OSHTHO_PHO),
    ("x", "\u{0995}\u{09CD}\u{09B8}"), // ক্স
];

/// Capital roman keys with their own consonant (retroflex and nukta letters).
pub const CAPITAL_ROMAN_TO_CONSONANT: &[(&str, &str)] = &[
    ("T", MURDHONNO_TO),
    ("D", MURDHONNO_DO),
    ("N", MURDHONNO_NO),
    ("R", D_E_SHUNNO_RO),
    ("S", TALOBBO_SHO),
];

/// ঞ্চ
pub const NYO_CHO: &str = "\u{099E}\u{09CD}\u{099A}";
/// ঞ্ছ
pub const NYO_CHHO: &str = "\u{099E}\u{09CD}\u{099B}";

/// Consonant → the consonant that `h` turns it into.
pub const ASPIRATED_CONSONANT_BY_BASE: &[(&str, &str)] = &[
    (KONTHYO_KO, KONTHYO_KHO),
    (KONTHYO_GO, KONTHYO_GHO),
    (TALOBBO_CHO, TALOBBO_CHHO),
    (BORGIYO_JO, BORGIYO_JHO),
    (DONTO_TO, DONTO_THO),
    (MURDHONNO_TO, MURDHONNO_THO),
    (DONTO_DO, DONTO_DHO),
    (MURDHONNO_DO, MURDHONNO_DHO),
    (OSHTHO_PO, OSHTHO_PHO),
    (OSHTHO_BO, OSHTHO_BHO),
    (D_E_SHUNNO_RO, DH_E_SHUNNO_RO),
    (DONTO_SHO, TALOBBO_SHO),
    (TALOBBO_SHO, MURDHONNO_SHO),
    (NYO_CHO, NYO_CHHO),
];

/// Letters that no consonant joins after, except a ফলা (rr-reph design D4b):
/// the breathy letters, হ, ড়, ঢ় and য়. A breathy sound can't close a cluster,
/// so `dekhte` gives দেখতে, not দেখ্তে.
pub const NO_JOIN_AFTER: &[&str] = &[
    KONTHYO_KHO,
    KONTHYO_GHO,
    TALOBBO_CHHO,
    BORGIYO_JHO,
    MURDHONNO_THO,
    MURDHONNO_DHO,
    DONTO_THO,
    DONTO_DHO,
    OSHTHO_PHO,
    OSHTHO_BHO,
    USHMO_HO,
    D_E_SHUNNO_RO,
    DH_E_SHUNNO_RO,
    ONTOSTHO_YO,
];

/// ফলা letters that still join after [`NO_JOIN_AFTER`] (rr-reph design D4b):
/// চিহ্ন, ব্রাহ্মণ, ভ্রমণ, ফ্লাইট. ব joins only when typed with `w` (ধ্বনি, but
/// দেখবে), and `y` makes য-ফলা by its own rule.
pub const PHOLA_AFTER_NO_JOIN: &[&str] =
    &[ONTOSTHO_RO, ONTOSTHO_LO, OSHTHO_MO, DONTO_NO, MURDHONNO_NO];

/// Zero-width joiner. Between র and য-ফলা (র + ZWJ + ্য) it keeps the ফলা
/// visible rather than letting fonts draw reph (rr-reph design D4).
pub const ZWJ: &str = "\u{200D}";

/// `Map.get` over one of the pair tables above.
pub fn lookup<V: Copy>(table: &[(&str, V)], key: &str) -> Option<V> {
    table.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
}

/// True for any of [`PHONETIC_CONSONANT_GRAPHEMES`].
pub fn is_phonetic_consonant(s: &str) -> bool {
    PHONETIC_CONSONANT_GRAPHEMES.contains(&s)
}

/// True for any of [`MODIFIER_GRAPHEME_CHARS`].
pub fn is_modifier(s: &str) -> bool {
    MODIFIER_GRAPHEME_CHARS.contains(&s)
}

/// `bengaliConsonantLetterGraphemes`: phonetic consonants minus the modifiers.
pub fn is_bengali_consonant_letter(s: &str) -> bool {
    is_phonetic_consonant(s) && !is_modifier(s)
}

/// The `bengaliConsonantLetterGraphemes` set, in insertion order.
pub fn bengali_consonant_letter_graphemes() -> Vec<&'static str> {
    PHONETIC_CONSONANT_GRAPHEMES
        .iter()
        .copied()
        .filter(|s| !is_modifier(s))
        .collect()
}

/// Combining nukta, as in a decomposed ড + ়.
pub const NUKTA: &str = "\u{09BC}";

/// `karTakingConsonantGraphemes`: consonants that can carry a kar or a hasant
/// (every consonant letter except khanda ta ৎ).
pub fn is_kar_taking_consonant(s: &str) -> bool {
    is_bengali_consonant_letter(s) && s != KHONDO_TO
}

/// The `karTakingConsonantGraphemes` set, in insertion order.
pub fn kar_taking_consonant_graphemes() -> Vec<&'static str> {
    bengali_consonant_letter_graphemes()
        .into_iter()
        .filter(|s| *s != KHONDO_TO)
        .collect()
}

/// `numberMap.has(c) || bengaliDigits.has(c)`.
pub fn is_ascii_or_bengali_digit(s: &str) -> bool {
    NUMBER_MAP
        .iter()
        .any(|(ascii, bengali)| *ascii == s || *bengali == s)
}
