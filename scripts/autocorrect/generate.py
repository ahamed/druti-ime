#!/usr/bin/env python3
"""Builds crates/druti-core/data/autocorrect.tsv (autocorrect design D3).

For each of the most frequent Bengali words:
1. Spell it in Druti keys, with `o` between every two consonants that are not joined (the full
   spelling). The engine must give the word back from it exactly, or the word is skipped.
2. Find where native typists leave that `o` out, from the attested romanizations in Google's
   Dakshina lexicon (`amra` for আমরা). Words the lexicon doesn't romanize are skipped.
3. Type the spelling without those vowels (the typed spelling). If the engine then gives the word
   with extra hasants and nothing else changed, that output is a correction entry.
4. Drop entries whose engine output is itself a word, and outputs shared by two words.

Only the standard library is used. The engine runs through `cargo run --example transpile_lines`.
See README.md in this directory for the inputs and how to run it.
"""

import argparse
import gzip
import re
import subprocess
import sys
import unicodedata
from collections import Counter, defaultdict
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
DEFAULT_OUT = REPO / "crates/druti-core/data/autocorrect.tsv"

HASANT = "\u09cd"
NUKTA_LETTERS = {"ড\u09bc": "\u09dc", "ঢ\u09bc": "\u09dd", "য\u09bc": "\u09df"}  # letter + nukta -> U+09DC, U+09DD, U+09DF
ZW = "\u200c\u200d"  # ZWNJ, ZWJ

# Druti keys for each letter (the engine's key tables, read backwards).
CONSONANT_KEYS = {
    "ক": "k", "খ": "kh", "গ": "g", "ঘ": "gh", "ঙ": "Ng", "চ": "c", "ছ": "ch", "জ": "j",
    "ঝ": "jh", "ঞ": "NG", "ট": "T", "ঠ": "Th", "ড": "D", "ঢ": "Dh", "ণ": "N", "ত": "t",
    "থ": "th", "দ": "d", "ধ": "dh", "ন": "n", "প": "p", "ফ": "f", "ব": "b", "ভ": "v",
    "ম": "m", "য": "z", "র": "r", "ল": "l", "শ": "S", "ষ": "Sh", "স": "s", "হ": "h",
    "\u09dc": "R", "\u09dd": "Rh", "\u09df": "y", "ৎ": "tH",
}
# Conjuncts with their own keys (the engine's rules: kkhiyo, jo-plus-nyo, nyo-plus-*).
CONJUNCT_KEYS = {"ক\u09cdষ": "kkh", "জ\u09cdঞ": "gg", "ঞ\u09cdচ": "nc", "ঞ\u09cdছ": "nch", "ঞ\u09cdজ": "nj"}
# The second letter of a ফলা, typed with its ফলা key.
PHOLA_KEYS = {"য": "y", "ব": "w"}
KAR_KEYS = {
    "া": "a", "ি": "i", "ী": "I", "ু": "u", "ূ": "U", "ৃ": "rri", "ে": "e", "ৈ": "Oi",
    "ো": "O", "ৌ": "Ou",
}
VOWEL_KEYS = {
    "অ": "o", "আ": "a", "ই": "i", "ঈ": "I", "উ": "u", "ঊ": "U", "ঋ": "rri", "এ": "e",
    "ঐ": "Oi", "ও": "O", "ঔ": "Ou",
}
SIGN_KEYS = {"ং": "ng", "ঃ": ":", "\u0981": "^"}

# How people romanize each letter casually (for matching Dakshina's romanizations).
CASUAL = {
    "ক": "k|c|q|ck|kh", "খ": "kh|k", "গ": "g|gh", "ঘ": "gh|g", "ঙ": "ng|n|g", "চ": "ch|c|s",
    "ছ": "chh|ch|ss|s|c", "জ": "j|z|g|jh", "ঝ": "jh|j|z", "ঞ": "n|y|ng|", "ট": "t|tt|th",
    "ঠ": "th|t", "ড": "d|r|dd|dh", "ঢ": "dh|d|r", "ণ": "n|nn", "ত": "t|th|tt", "থ": "th|t",
    "দ": "d|dh|th", "ধ": "dh|d", "ন": "n|nn", "প": "p|f|ph", "ফ": "ph|f|p", "ব": "b|v|w|bh",
    "ভ": "bh|v|b", "ম": "m|mm", "য": "j|z|y", "র": "r|rr", "ল": "l|ll", "শ": "sh|s|ss|x",
    "ষ": "sh|s", "স": "s|sh|ss|c", "হ": "h|", "\u09dc": "r|rh|d", "\u09dd": "r|rh",
    "\u09df": "y|w|i|e|", "ৎ": "t|d|th",
    "া": "a|aa|e", "ি": "i|e|ee|y", "ী": "i|ee|e|y", "ু": "u|oo|o", "ূ": "u|oo", "ৃ": "ri|r",
    "ে": "e|a|ae|ay|i|ee", "ৈ": "oi|oy|ai|y", "ো": "o|oo|u", "ৌ": "ou|ow|au|o",
    "অ": "o|a", "আ": "a|aa|e", "ই": "i|e|ee", "ঈ": "i|ee|e", "উ": "u|oo|o", "ঊ": "u|oo",
    "ঋ": "ri|r", "এ": "e|a|ae|ay|i|ye", "ঐ": "oi|oy|ai", "ও": "o|w|u", "ঔ": "ou|ow|au",
    "ং": "ng|n|m|g", "ঃ": "h|", "\u0981": "n|",
}
CASUAL_CONJUNCT = {"ক\u09cdষ": "kkh|kh|ksh|kk", "জ\u09cdঞ": "gg|gy|ggy|gn|jn|g"}
CASUAL_PHOLA = {"য": "y|e|a|", "ব": "w|b|v|"}
INHERENT = "o|a|w|e|u"

TOKEN = re.compile(r"[\u0981-\u09e3\u09f0\u09f1\u200c\u200d]+")


def normalize(text):
    """NFC, then the nukta letters precomposed (NFC decomposes them: composition exclusions)."""
    text = unicodedata.normalize("NFC", text)
    for decomposed, composed in NUKTA_LETTERS.items():
        text = text.replace(decomposed, composed)
    return text


def tokens(text):
    for token in TOKEN.findall(normalize(text)):
        token = token.strip(ZW)
        if token and not any(c in ZW for c in token):
            yield token


def is_consonant(c):
    return c in CONSONANT_KEYS


def units(word):
    """Splits a word into (kind, text) units, or None for words the generator can't spell.

    Kinds: "cons" (a consonant, or a conjunct with its own keys), "join" (a hasant between two
    consonants), "kar", "vowel", "sign".
    """
    out = []
    i = 0
    while i < len(word):
        three = word[i:i + 3]
        c = word[i]
        if three in CONJUNCT_KEYS:
            out.append(("cons", three))
            i += 3
        elif is_consonant(c):
            out.append(("cons", c))
            i += 1
        elif c == HASANT:
            if not out or out[-1][0] != "cons" or i + 1 >= len(word) or not is_consonant(word[i + 1]):
                return None
            out.append(("join", c))
            i += 1
        elif c in KAR_KEYS:
            if not out or out[-1][0] != "cons":
                return None
            out.append(("kar", c))
            i += 1
        elif c in VOWEL_KEYS:
            out.append(("vowel", c))
            i += 1
        elif c in SIGN_KEYS:
            if not out:
                return None
            out.append(("sign", c))
            i += 1
        else:
            return None
    return out


def spell(word_units):
    """The full Druti spelling: a list of key strings, with "o" items at each unwritten vowel
    between two consonants (the apart positions, numbered by their index in the list)."""
    keys = []
    apart = []
    prev = None
    for n, (kind, text) in enumerate(word_units):
        if kind == "cons":
            if prev and prev[0] == "join":
                before = word_units[n - 2][1]
                if before == "র":
                    keys.append("r")  # reph: `rr` + consonant (rr-reph)
                keys.append(PHOLA_KEYS.get(text, CONJUNCT_KEYS.get(text, CONSONANT_KEYS.get(text))))
            else:
                if prev and prev[0] in ("cons",) and prev[1] != "ৎ":
                    apart.append(len(keys))
                    keys.append("o")
                keys.append(CONJUNCT_KEYS.get(text) or CONSONANT_KEYS[text])
        elif kind == "kar":
            keys.append(KAR_KEYS[text])
        elif kind == "vowel":
            if prev and prev[0] == "cons":
                keys.append("o")  # a silent `o` keeps the vowel independent (কই: koi)
            keys.append(VOWEL_KEYS[text])
        elif kind == "sign":
            if text == "ং" and prev and prev[0] == "cons":
                apart.append(len(keys))
                keys.append("o")
            keys.append(SIGN_KEYS[text])
        prev = (kind, text)
    return keys, apart


def casual_regex(word_units):
    """A regex for casual romanizations of the word, with a group `v<n>` at each apart position
    that captures the vowel typed there (empty when it was left out)."""
    parts = []
    slots = 0
    prev = None

    def slot():
        nonlocal slots
        slots += 1
        return f"(?P<v{slots - 1}>{INHERENT}|)"

    for kind, text in word_units:
        if kind == "cons":
            joined = prev and prev[0] == "join"
            if not joined and prev and prev[0] == "cons" and prev[1] != "ৎ":
                parts.append(slot())
            if joined and text in CASUAL_PHOLA:
                alts = CASUAL_PHOLA[text]
            elif text in CASUAL_CONJUNCT:
                alts = CASUAL_CONJUNCT[text]
            else:
                alts = "".join(f"(?:{CASUAL[c]})" for c in text if c != HASANT)
            parts.append(f"(?:{alts})")
        elif kind == "join":
            pass
        else:
            if kind == "sign" and text == "ং" and prev and prev[0] == "cons":
                parts.append(slot())
            elif kind == "vowel" and prev and prev[0] == "cons":
                parts.append(f"(?:{INHERENT}|)")
            parts.append(f"(?:{CASUAL[text]})")
        prev = (kind, text)
    if prev and prev[0] == "cons":
        parts.append("(?:o|a|)")
    return re.compile("".join(parts))


def read_frequency_words(path):
    counts = Counter()
    with open(path, encoding="utf-8") as f:
        for line in f:
            text, _, count = line.rstrip("\n").rpartition(" ")
            if count.isdigit():
                for token in tokens(text):
                    counts[token] += int(count)
    return counts


def read_wikipedia(dakshina):
    counts = Counter()
    path = dakshina / "bn/native_script_wikipedia/bn.wiki-filt.train.text.sorted.tsv.gz"
    with gzip.open(path, "rt", encoding="utf-8") as f:
        for line in f:
            counts.update(tokens(line.rstrip("\n").split("\t")[-1]))
    return counts


def read_lexicon(dakshina):
    """Native word -> Counter of attested romanizations (lowercased) with annotator counts."""
    lexicon = defaultdict(Counter)
    for split in ("train", "dev", "test"):
        path = dakshina / f"bn/lexicons/bn.translit.sampled.{split}.tsv"
        with open(path, encoding="utf-8") as f:
            for line in f:
                native, roman, count = line.rstrip("\n").split("\t")
                lexicon[normalize(native)][roman.lower()] += int(count)
    return lexicon


def common_words(subtitles, wikipedia, size):
    """The `size` words with the highest mean frequency per million across both corpora."""
    totals = (sum(subtitles.values()), sum(wikipedia.values()))
    score = Counter()
    for counts, total in zip((subtitles, wikipedia), totals):
        for word, count in counts.items():
            score[word] += count / total
    return sorted(score, key=lambda w: (-score[w], w))[:size]


def omitted_positions(word_units, apart, romanizations):
    """Apart positions where most annotations leave the vowel out (strictly more than type it)."""
    if not apart:
        return set()
    regex = casual_regex(word_units)
    left_out = Counter()
    typed = Counter()
    for roman, count in romanizations.items():
        match = regex.fullmatch(roman)
        if not match:
            continue
        for slot, position in enumerate(apart):
            if match.group(f"v{slot}"):
                typed[position] += count
            else:
                left_out[position] += count
    if not left_out and not typed:
        return None
    return {p for p in apart if left_out[p] > typed[p]}


def run_engine(lines):
    """Types each line on a fresh engine (the transpile_lines example)."""
    result = subprocess.run(
        ["cargo", "run", "-q", "--release", "-p", "druti-core", "--example", "transpile_lines"],
        input="\n".join(lines) + "\n",
        capture_output=True,
        text=True,
        cwd=REPO,
        check=True,
    )
    out = result.stdout.split("\n")[: len(lines)]
    if len(out) != len(lines):
        sys.exit("the engine returned fewer lines than it was given")
    return out


def only_extra_hasants(engine, correct):
    """Whether removing some hasants from `engine` gives `correct`, and at least one is removed."""
    if len(engine) <= len(correct):
        return False
    i = 0
    for c in engine:
        if i < len(correct) and c == correct[i]:
            i += 1
        elif c != HASANT:
            return False
    return i == len(correct)


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("--frequency-words", type=Path, required=True,
                        help="FrequencyWords content/2018/bn/bn_full.txt")
    parser.add_argument("--dakshina", type=Path, required=True,
                        help="the extracted dakshina_dataset_v1.0 directory")
    parser.add_argument("--size", type=int, default=20000, help="how many common words to check")
    parser.add_argument("--out", type=Path, default=DEFAULT_OUT)
    args = parser.parse_args()

    subtitles = read_frequency_words(args.frequency_words)
    wikipedia = read_wikipedia(args.dakshina)
    lexicon = read_lexicon(args.dakshina)
    # A real word: romanized in the lexicon, or seen often enough in a corpus not to be a typo.
    real_words = set(lexicon) | {w for w, n in subtitles.items() if n >= 5} | {
        w for w, n in wikipedia.items() if n >= 20}

    candidates = []  # (word, full keys, typed keys, positions left out)
    stats = Counter()
    for word in common_words(subtitles, wikipedia, args.size):
        word_units = units(word)
        if word_units is None:
            stats["not spellable"] += 1
            continue
        keys, apart = spell(word_units)
        if word not in lexicon:
            stats["not in lexicon"] += 1
            candidates.append((word, keys, None, None))
            continue
        omitted = omitted_positions(word_units, apart, lexicon[word])
        if omitted is None:
            stats["romanizations not matched"] += 1
            candidates.append((word, keys, None, None))
            continue
        typed = [k for n, k in enumerate(keys) if n not in omitted]
        candidates.append((word, keys, typed, omitted))

    # Round trip: the full spelling must give the word back.
    full_out = run_engine(["".join(c[1]) for c in candidates])
    checked = []
    for (word, keys, typed, omitted), out in zip(candidates, full_out):
        if out != word:
            stats["full spelling does not round-trip"] += 1
        elif typed is not None:
            checked.append((word, keys, typed, omitted))
    typed_out = run_engine(["".join(c[2]) for c in checked])

    # Mark only the left-out vowels that the engine needs: `_` where `o` goes back.
    found = []
    probes = []
    for (word, keys, typed, omitted), engine in zip(checked, typed_out):
        if engine == word:
            continue
        if not only_extra_hasants(engine, word):
            stats["typed spelling differs by more than hasants"] += 1
            continue
        found.append((word, keys, omitted, engine))
        for position in sorted(omitted):
            probes.append("".join(k for n, k in enumerate(keys) if n not in omitted or n == position))
    probe_out = iter(run_engine(probes))
    entries = defaultdict(list)
    for word, keys, omitted, engine in found:
        needed = {p for p in sorted(omitted) if next(probe_out) != engine}
        roman = "".join("_" if n in needed else k for n, k in enumerate(keys)
                        if n not in omitted or n in needed)
        entries[engine].append((word, roman))

    # The consistency test's check, before anything is written.
    restored = run_engine([words[0][1].replace("_", "o") for words in entries.values()])
    rows = []
    for (engine, words), out in zip(entries.items(), restored):
        if len(words) > 1:
            stats["ambiguous"] += len(words)
        elif out != words[0][0]:
            stats["restored spelling does not round-trip"] += 1
        elif engine in real_words:
            stats["engine output is a word"] += 1
        else:
            rows.append((engine, words[0][0], words[0][1]))
    rows.sort()

    header = [
        "# Druti Autocorrect list: engine spelling, correct spelling, roman spelling.",
        "# In the roman spelling, `_` marks a vowel left out: typing it as written gives the engine",
        "# spelling, and typing `o` at each `_` gives the correct spelling.",
        "# Generated by scripts/autocorrect/generate.py; do not edit by hand.",
        "# The Autocorrect word list is derived from the Dakshina dataset (Google Research,",
        "# CC BY-SA 4.0) and from FrequencyWords by Hermit Dave (built from OpenSubtitles,",
        "# CC BY-SA 4.0). It is licensed CC BY-SA 4.0 (see LICENSE-DATA). The rest of Druti is MIT.",
    ]
    args.out.parent.mkdir(parents=True, exist_ok=True)
    with open(args.out, "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(header) + "\n")
        for row in rows:
            f.write("\t".join(row) + "\n")
    for key, value in sorted(stats.items()):
        print(f"{key}: {value}", file=sys.stderr)
    print(f"entries: {len(rows)}", file=sys.stderr)


if __name__ == "__main__":
    main()
