# Autocorrect list generator

`generate.py` builds `crates/druti-core/data/autocorrect.tsv`, the list that the optional
Autocorrect setting uses (autocorrect design D3). Run it by hand after any change to the engine's
rules; the consistency test in `druti-core` fails until the list matches the engine again.

## Inputs

Download these once (both CC BY-SA 4.0):

- FrequencyWords, built from OpenSubtitles 2018:
  `https://raw.githubusercontent.com/hermitdave/FrequencyWords/master/content/2018/bn/bn_full.txt`
- Google's Dakshina dataset v1.0 (about 2 GB, of which the generator reads the Bengali lexicon and
  Wikipedia text): `https://storage.googleapis.com/gresearch/dakshina/dakshina_dataset_v1.0.tar`.
  Extract it.

## Running

```sh
python3 scripts/autocorrect/generate.py \
  --frequency-words path/to/bn_full.txt \
  --dakshina path/to/dakshina_dataset_v1.0
```

It needs Python 3 and Cargo. It runs the engine through
`cargo run --example transpile_lines`, takes about half a minute, writes the list (sorted, so diffs
are reviewable) and prints how many words it skipped and why. The same inputs always give the same
file.

## What goes in

1. The 20,000 words with the highest mean frequency across the subtitles and Wikipedia.
2. Each word is spelled in Druti keys with `o` at every unwritten vowel between two consonants
   (আমরা: `amora`). Words whose spelling doesn't give the word back are skipped.
3. Dakshina's attested romanizations say where people leave that vowel out (`amra`). A vowel is
   left out when more annotations leave it out than type it. Words Dakshina doesn't romanize are
   skipped.
4. The spelling without those vowels is typed. If the engine gives the word with extra hasants and
   nothing else changed (আম্রা), that is an entry. Words where it changes a letter (`n` + `c` → ঞ্চ)
   are skipped: Autocorrect only removes hasants.
5. Entries are dropped when the engine's output is itself a word (আস্তে, "slowly", is never
   corrected to আসতে): a word Dakshina romanizes, or one seen at least 5 times in the subtitles or
   20 times in Wikipedia. Outputs that two words share are dropped too.

Each line is `engine spelling<TAB>correct spelling<TAB>roman spelling`, where `_` in the roman
spelling marks the left-out vowel: `am_ra`.
