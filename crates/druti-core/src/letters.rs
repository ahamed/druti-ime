//! What one Backspace removes, which cluster resumes before the caret
//! (design D1 of the letter-backspace-and-caret-cluster change), and where the
//! word being typed starts (whole-word-pending design D1).
//!
//! They work on UTF-16 units, the engine's storage, and only ever cut at code
//! point boundaries. The engine uses them for its Backspace and for
//! [`Engine::resume_cluster`](crate::Engine::resume_cluster); the composer
//! uses them for the pending word.

use std::ops::RangeInclusive;

use unicode_segmentation::UnicodeSegmentation;

use crate::data::{HASANT, NUKTA, ZWJ, is_bengali_consonant_letter, is_kar_taking_consonant};

/// The Bengali Unicode block. Text outside it is deleted by grapheme cluster.
const BENGALI: RangeInclusive<char> = '\u{0980}'..='\u{09FF}';

/// How far back a grapheme cluster is looked for. Real clusters (an emoji
/// with modifiers, a flag) are far shorter.
const GRAPHEME_LOOKBACK_CODE_POINTS: usize = 32;

/// The last code point of `units` and the index it starts at. A lone
/// surrogate reads as U+FFFD, which is also one unit long.
fn last_char(units: &[u16]) -> Option<(usize, char)> {
    let start = match units {
        [] => return None,
        [.., high, low] if (0xD800..=0xDBFF).contains(high) && (0xDC00..=0xDFFF).contains(low) => {
            units.len() - 2
        }
        _ => units.len() - 1,
    };
    let ch = char::decode_utf16(units[start..].iter().copied())
        .next()
        .and_then(Result::ok)
        .unwrap_or(char::REPLACEMENT_CHARACTER);
    Some((start, ch))
}

/// Whether `ch` is the single code point `s`.
fn is(ch: char, s: &str) -> bool {
    let mut chars = s.chars();
    chars.next() == Some(ch) && chars.next().is_none()
}

fn test(ch: char, predicate: fn(&str) -> bool) -> bool {
    predicate(ch.encode_utf8(&mut [0; 4]))
}

/// Where the consonant that ends `units` starts, counting a combining nukta
/// after it as part of it. `None` when `units` doesn't end in a consonant
/// that satisfies `predicate`.
fn consonant_start(units: &[u16], predicate: fn(&str) -> bool) -> Option<usize> {
    let (mut start, mut ch) = last_char(units)?;
    if is(ch, NUKTA) {
        (start, ch) = last_char(&units[..start])?;
    }
    test(ch, predicate).then_some(start)
}

/// When `before` ends in a hasant joined to a consonant that can carry it,
/// the start of that hasant and of that consonant. A ZWJ between the
/// consonant and the hasant (র + ZWJ + ্য, rr-reph design D4) belongs to the
/// hasant, so the joined letter takes it along.
fn joined_hasant(before: &[u16]) -> Option<(usize, usize)> {
    let (mut hasant, ch) = last_char(before)?;
    if !is(ch, HASANT) {
        return None;
    }
    if let Some((zwj, ch)) = last_char(&before[..hasant])
        && is(ch, ZWJ)
    {
        hasant = zwj;
    }
    let consonant = consonant_start(&before[..hasant], is_kar_taking_consonant)?;
    Some((hasant, consonant))
}

/// The UTF-16 length of the last extended grapheme cluster of `units`.
fn last_grapheme_len(units: &[u16]) -> usize {
    let mut start = units.len();
    for _ in 0..GRAPHEME_LOOKBACK_CODE_POINTS {
        match last_char(&units[..start]) {
            Some((index, _)) => start = index,
            None => break,
        }
    }
    String::from_utf16_lossy(&units[start..])
        .graphemes(true)
        .next_back()
        .map_or(0, |g| g.encode_utf16().count())
}

/// The UTF-16 length of the last letter of `units`: what one Backspace
/// removes (rust-engine-core spec, "Letter backspace").
///
/// A consonant goes with its nukta and with the hasant joining it to the
/// consonant before, so a hasant is never left dangling (`দ্ম` → `দ`). Any
/// other Bengali code point (a kar, a sign, a vowel, a digit, a lone hasant)
/// is a letter by itself. Text outside the Bengali block loses one extended
/// grapheme cluster. Returns 0 for empty input.
pub(crate) fn last_letter_len_utf16(units: &[u16]) -> usize {
    let Some((start, ch)) = last_char(units) else {
        return 0;
    };
    if !BENGALI.contains(&ch) {
        return last_grapheme_len(units);
    }
    let Some(consonant) = consonant_start(units, is_bengali_consonant_letter) else {
        return units.len() - start;
    };
    let start = joined_hasant(&units[..consonant]).map_or(consonant, |(hasant, _)| hasant);
    units.len() - start
}

/// The UTF-16 length of the consonant run that ends `units`: the last
/// consonant that can carry a kar (with its nukta), plus every consonant
/// joined to it by hasants before it (`ন্ত্র`). Returns 0 when `units`
/// doesn't end in such a consonant, as after a kar, `ং`, `ৎ` or a space.
pub(crate) fn trailing_consonant_run_len_utf16(units: &[u16]) -> usize {
    let Some(mut start) = consonant_start(units, is_kar_taking_consonant) else {
        return 0;
    };
    while let Some((_, consonant)) = joined_hasant(&units[..start]) {
        start = consonant;
    }
    units.len() - start
}

/// Whether `ch` belongs to a Bengali word: a letter, a sign, a kar, the hasant
/// or nukta, or a joiner. Digits, currency signs and punctuation end a word.
pub(crate) fn is_word_char(ch: char) -> bool {
    ('\u{0980}'..='\u{09E3}').contains(&ch)
        || matches!(ch, '\u{09F0}' | '\u{09F1}' | '\u{200C}' | '\u{200D}')
}

/// The UTF-16 length of the Bengali word that ends `units`: what the
/// composer keeps pending (whole-word-pending design D1). Returns 0 when
/// `units` ends in anything else, such as a space, a digit or punctuation.
pub(crate) fn trailing_word_len_utf16(units: &[u16]) -> usize {
    let mut start = units.len();
    while let Some((index, ch)) = last_char(&units[..start]) {
        if !is_word_char(ch) {
            break;
        }
        start = index;
    }
    units.len() - start
}

#[cfg(test)]
mod tests {
    use super::*;

    fn units(s: &str) -> Vec<u16> {
        s.encode_utf16().collect()
    }

    fn letter(s: &str) -> usize {
        last_letter_len_utf16(&units(s))
    }

    fn run(s: &str) -> usize {
        trailing_consonant_run_len_utf16(&units(s))
    }

    #[test]
    fn decomposed_nukta_goes_with_its_consonant() {
        // য + ় (decomposed য়), joined to ক by a hasant.
        assert_eq!(letter("\u{0995}\u{09CD}\u{09AF}\u{09BC}"), 3);
        assert_eq!(run("\u{0995}\u{09CD}\u{09AF}\u{09BC}"), 4);
    }

    #[test]
    fn precomposed_nukta_letter_is_one_consonant() {
        // য় U+09DF after an a-kar.
        assert_eq!(letter("\u{09BE}\u{09DF}"), 1);
        assert_eq!(run("\u{09BE}\u{09DF}"), 1);
    }

    #[test]
    fn lone_hasant_and_lone_nukta_are_letters_by_themselves() {
        assert_eq!(letter("দ্"), 1);
        assert_eq!(run("দ্"), 0);
        assert_eq!(letter("\u{09BC}"), 1);
    }

    #[test]
    fn surrogate_pairs_are_never_split() {
        assert_eq!(letter("ক😀"), 2);
        assert_eq!(letter("😀"), 2);
    }

    #[test]
    fn emoji_with_a_skin_tone_is_one_letter() {
        assert_eq!(letter("ক👍🏽"), 4);
    }

    #[test]
    fn lone_surrogate_is_one_unit() {
        assert_eq!(last_letter_len_utf16(&[0x0995, 0xD83D]), 1);
    }

    #[test]
    fn visible_ja_phala_goes_with_its_zwj() {
        // র + ZWJ + ্য (rr-reph design D4): one letter, and part of the run.
        assert_eq!(letter("প\u{09B0}\u{200D}\u{09CD}\u{09AF}"), 3);
        assert_eq!(run("প\u{09B0}\u{200D}\u{09CD}\u{09AF}"), 4);
    }

    #[test]
    fn khondo_to_is_a_letter_but_never_starts_a_run() {
        assert_eq!(letter("কৎ"), 1);
        assert_eq!(run("কৎ"), 0);
    }

    #[test]
    fn word_ends_at_spaces_digits_and_punctuation() {
        let word = |s: &str| trailing_word_len_utf16(&units(s));
        assert_eq!(word("আমি পদ্ম"), 4);
        assert_eq!(word("কি?"), 0);
        assert_eq!(word("ক১"), 0);
        assert_eq!(word("১ক"), 1);
        assert_eq!(word("ক।"), 0);
        assert_eq!(word("ক\u{200C}ষ"), 3);
        assert_eq!(word(""), 0);
    }

    #[test]
    fn empty_input() {
        assert_eq!(letter(""), 0);
        assert_eq!(run(""), 0);
    }
}
