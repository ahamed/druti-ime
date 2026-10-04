# Spec Delta

## ADDED Requirements

### Requirement: Armed reph in the pending word
The `র্` shown after a double `r` SHALL be part of the pending word. Backspace SHALL remove only its
hasant. After that Backspace, the next consonant starts a new letter, as after any Backspace
(including another `r`). A word break SHALL commit the `র্` as shown.

#### Scenario: Backspace on an armed reph
- **WHEN** `k`, `o`, `r`, `r` are pressed, then Backspace
- **THEN** the pending text is `কর`

#### Scenario: Consonant after that Backspace
- **WHEN** `k`, `o`, `r`, `r` are pressed, then Backspace, then `t`
- **THEN** the pending text is `করত`

#### Scenario: Re-typing reph after Backspace
- **WHEN** `k`, `o`, `r`, `r` are pressed, then Backspace twice, then `r`, `r`, `t`
- **THEN** the pending text is `কর্ত`

#### Scenario: Word break commits what is shown
- **WHEN** `k`, `o`, `r`, `r`, space are pressed
- **THEN** `কর্ ` is committed and nothing is pending

### Requirement: Visible য-ফলা after r in the pending word
The ZWJ in `র‍্য` SHALL belong to the য-ফলা letter. Backspace SHALL remove ZWJ, hasant and য
together, and lengths SHALL count the ZWJ as one UTF-16 code unit.

#### Scenario: Backspace on র‍্য
- **WHEN** `p`, `o`, `r`, `y` are pressed, then Backspace
- **THEN** the pending text is `পর`
