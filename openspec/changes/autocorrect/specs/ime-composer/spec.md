# Spec Delta

## ADDED Requirements

### Requirement: Autocorrect setting
The composer settings SHALL include Autocorrect, off by default. With it off, the composer SHALL
behave exactly as before this change.

#### Scenario: Default is off
- **WHEN** `a`, `m`, `r`, `a`, space are pressed with default settings
- **THEN** `আম্রা ` is committed

### Requirement: Correcting the word when it ends
With Autocorrect on, when a key ends the pending word (a space, digit, punctuation, symbol, quote or
Enter, as in "Commit and pending split"), the composer SHALL look the pending word up in the
correction list before committing it. If it is found, the composer SHALL use the corrected word in
place of the pending word, and SHALL keep the engine's output in step with it so later keys read the
corrected text. For a corrected word, committed plus pending text equals the engine output with that
correction applied.

The corrected word and the key's own text SHALL stay pending until the next key, so that Backspace
can undo the correction; the next key that is not Backspace commits them first. Enter and keys the
composer does not handle SHALL commit immediately instead, and their correction cannot be undone.
A word that is not corrected SHALL be committed exactly as without Autocorrect.

#### Scenario: Correction at space
- **WHEN** `a`, `m`, `r`, `a`, space are pressed with Autocorrect on
- **THEN** after `a` the pending text is `আম্রা`, and after space the pending text is `আমরা ` with nothing committed

#### Scenario: Next key commits the correction
- **WHEN** `a`, `m`, `r`, `a`, space, `k` are pressed with Autocorrect on
- **THEN** after `k`, `আমরা ` is committed and the pending text is `ক`

#### Scenario: Uncorrected word commits as usual
- **WHEN** `b`, `h`, `o`, `k`, `t`, space are pressed with Autocorrect on
- **THEN** after space, `ভক্ত ` is committed with nothing pending

#### Scenario: Correction before a held dari
- **WHEN** `a`, `m`, `r`, `a`, `.` are pressed with Autocorrect on
- **THEN** the pending text is `আমরা।`

#### Scenario: Enter commits at once
- **WHEN** `a`, `m`, `r`, `a`, Enter are pressed with Autocorrect on
- **THEN** `আমরা` is committed, nothing is pending, and Enter passes to the application

### Requirement: Backspace undoes a correction
With Autocorrect on, a Backspace pressed directly after a correction SHALL replace the pending
corrected word and word break with the uncorrected word, as pending text, and SHALL do nothing else.
That word SHALL NOT be corrected again when it next ends. Any other Backspace SHALL behave as
before.

#### Scenario: Undo
- **WHEN** `a`, `m`, `r`, `a`, space, Backspace are pressed with Autocorrect on
- **THEN** the pending text is `আম্রা` and nothing is committed

#### Scenario: Undone word is kept
- **WHEN** `a`, `m`, `r`, `a`, space, Backspace, space are pressed with Autocorrect on
- **THEN** `আম্রা ` is committed with nothing pending

#### Scenario: Second Backspace edits as usual
- **WHEN** `a`, `m`, `r`, `a`, space, Backspace, Backspace are pressed with Autocorrect on
- **THEN** the pending text is `আম্র`
