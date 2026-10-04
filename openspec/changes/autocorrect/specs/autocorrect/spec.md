# Spec Delta

## ADDED Requirements

### Requirement: Correction list
`druti-core` SHALL ship a fixed correction list that maps a Bengali word as the engine produces it to
its correct spelling. Every entry SHALL satisfy all of:
- the correct spelling equals the engine's spelling with one or more hasants removed, and nothing
  else changed;
- the correct spelling is what the engine produces when `o` is typed at each removed hasant;
- the engine's spelling is not itself a word in the source lexicon;
- no other entry has the same engine spelling.

The list SHALL be generated from the source corpora by a generator kept in the repository, and SHALL
change only when it is regenerated in a reviewed commit.

#### Scenario: Entries remove hasants only
- **WHEN** the list is checked by the test suite
- **THEN** every entry's correct spelling is its engine spelling minus one or more `্`, and typing the stored roman spelling with `o` at those places gives the correct spelling

#### Scenario: Real words are never keys
- **WHEN** the list is checked against the source lexicon
- **THEN** no entry's engine spelling is a lexicon word (for example, `ভক্ত` and `শক্তি` are not keys)

#### Scenario: List matches the current engine
- **WHEN** the engine rules change and the list is not regenerated
- **THEN** the consistency test fails, naming the first entry whose roman spelling no longer gives its engine spelling

### Requirement: Lookup by engine output
Correcting a word SHALL depend only on the finished word's text and the list: a word found in the list
SHALL be replaced by its correct spelling, any other word SHALL be left unchanged. The lookup SHALL
match the whole word exactly, after Unicode normalization of nukta letters to their precomposed
forms (ড় U+09DC, ঢ় U+09DD, য় U+09DF).

#### Scenario: Common words
- **WHEN** the words `আম্রা`, `এক্টা`, `আপ্নি`, `বল্তে` and `থাক্তে` are looked up
- **THEN** the results are `আমরা`, `একটা`, `আপনি`, `বলতে` and `থাকতে`

#### Scenario: Words not in the list
- **WHEN** the words `ভক্ত`, `কিন্তু` and `দেখতে` are looked up
- **THEN** each is returned unchanged

### Requirement: Autocorrect in bulk conversion
When the autocorrect setting is on, bulk transpilation SHALL correct each word of its output with the
list. With the setting off, bulk transpilation SHALL be unchanged.

#### Scenario: Converting a sentence
- **WHEN** `amra ekTa jinis dekhte cai` is transpiled with autocorrect on
- **THEN** the output is `আমরা একটা জিনিস দেখতে চাই`

#### Scenario: Setting off
- **WHEN** `amra` is transpiled with default settings
- **THEN** the output is `আম্রা`
