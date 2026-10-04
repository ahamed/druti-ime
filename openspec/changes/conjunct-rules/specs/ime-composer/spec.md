# Spec Delta

## ADDED Requirements

### Requirement: Backtick hasant in the pending word
The hasant shown after a backtick SHALL be part of the pending word, like the armed reph of
`rr-reph`. Backspace SHALL remove only that hasant; the next consonant then starts a new letter, as
after any Backspace. A word break SHALL commit the hasant as shown. The start of the pending text
counts as the start of a word for the joining rules, because the composer does not read the document
for consonant keys.

#### Scenario: Backspace on a backtick hasant
- **WHEN** `D`, `o`, `k`, `` ` `` are pressed, then Backspace
- **THEN** the pending text is `ডক`

#### Scenario: Word break commits the hasant
- **WHEN** `D`, `o`, `k`, `` ` ``, space are pressed
- **THEN** `ডক্ ` is committed and nothing is pending

#### Scenario: Joining inside the pending word
- **WHEN** `d`, `e`, `k`, `h`, `t`, `e` are pressed
- **THEN** the pending text is `দেখতে`
