# ime-composer Specification

## Purpose
Turns the engine's rewriting edit actions into what a marked-text input method can show: text that
is final in the document, plus the Bengali word still being typed, which later keys and Backspace may
still change. Committed text is never changed afterwards, so every host shows the same text. Every
keystroke shows its exact Bengali immediately.
## Requirements
### Requirement: Commit and pending split
After every key, the composer SHALL report (a) text to commit, which becomes final in the document,
and (b) the complete pending text that replaces any previous pending text. Committed text plus
pending text SHALL always equal the engine output accumulated since the last reset.

The pending text SHALL be the Bengali word being typed: the Bengali letters, signs, kars, hasant,
nukta and joiners that end that output since the last flush or reset. It always contains the
engine's buffer. Anything else ends the word and SHALL be committed with everything before it: a
space, a digit, punctuation, a symbol or a quote. The only exception is a held `-` or `।` (see
"Holding a trailing hyphen or dari").

#### Scenario: Consonant stays pending
- **WHEN** `k` is pressed
- **THEN** nothing is committed and the pending text is `ক`

#### Scenario: Aspiration rewrites only the pending text
- **WHEN** `k` then `h` are pressed
- **THEN** after `h` nothing is committed and the pending text is `খ`

#### Scenario: Kar stays in the pending word
- **WHEN** `k`, `h`, `u` are pressed
- **THEN** after `u` nothing is committed and the pending text is `খু`

#### Scenario: Space commits the word
- **WHEN** `k`, `h`, `u`, `b`, space are pressed
- **THEN** after `b` the pending text is `খুব`, and after space `খুব ` is committed with nothing pending

#### Scenario: Vowel sign change stays pending
- **WHEN** `k`, `O` are pressed and then `i`
- **THEN** after `O` the pending text is `কো`, and after `i` the pending text is `কৈ` with nothing committed

#### Scenario: Digit commits the word
- **WHEN** `k` then `1` are pressed
- **THEN** after `1`, `ক১` is committed with nothing pending

### Requirement: Keys the engine does not map are typed literally
A single-character key with no Bengali mapping (for example `?`, `!`, `/`, `(`, `)`, `@`, `;`) SHALL
commit any pending text followed by the key's own character, and end the cluster, so later keys
never rewrite across it. This is the engine's own behaviour; the composer passes it through.

#### Scenario: Question mark after a word
- **WHEN** `k`, `i`, `?` are pressed
- **THEN** the total committed text is `কি?` and nothing is pending

#### Scenario: Symbol while a cluster is pending
- **WHEN** `k` is pressed (pending `ক`) and then `(`
- **THEN** `ক(` is committed and nothing is pending, and a following `h` produces `হ`, not `খ`

### Requirement: Holding a trailing hyphen or dari
When a key inserts a `-` or a `।` and leaves the engine buffer empty, the composer SHALL hold that
character as pending instead of committing it, because the next key may rewrite it (`--` → `—`,
`।` + `.` → `..`). Any other key SHALL commit the held character along with that key's own result.

#### Scenario: Double hyphen becomes an em dash
- **WHEN** `-` then `-` are pressed
- **THEN** after the first key the pending text is `-`, and after the second key `—` is committed with nothing pending

#### Scenario: Single hyphen followed by a letter
- **WHEN** `-` then `k` are pressed
- **THEN** after `k`, `-` is committed and the pending text is `ক`

#### Scenario: Ellipsis after a dari
- **WHEN** `a`, `.`, `.`, `.` are pressed
- **THEN** after the first `.` the pending text is `।`, and the total committed text at the end is `আ...` with nothing pending

### Requirement: Letter backspace
Backspace SHALL remove the last letter of the pending text, as the engine's letter backspace
defines a letter: a consonant goes with its nukta and with the hasant joining it to the consonant
before, so a hasant is never left dangling. Nothing SHALL be committed. The engine's cluster SHALL
end, so the next consonant starts a new letter (see "Typing after Backspace or a caret move").

When nothing is pending, the composer SHALL report Backspace as not handled, so the host deletes by
its own rules, and SHALL reset itself.

#### Scenario: Remove one consonant from a conjunct
- **WHEN** `d`, `m` are pressed (pending `দ্ম`) and then Backspace
- **THEN** nothing is committed and the pending text is `দ`

#### Scenario: Remove the last consonant of ক্ষ
- **WHEN** `k`, `k`, `h` are pressed (pending `ক্ষ`) and then Backspace
- **THEN** nothing is committed and the pending text is `ক`

#### Scenario: Remove an aspirated consonant
- **WHEN** `k`, `h` are pressed (pending `খ`) and then Backspace
- **THEN** the pending text is empty

#### Scenario: Conjunct at the end of a word
- **WHEN** `p`, `o`, `d`, `m`, `o` are pressed (pending `পদ্ম`) and then Backspace
- **THEN** nothing is committed and the pending text is `পদ`

#### Scenario: One letter at a time through a word
- **WHEN** `korote` is typed (pending `করতে`) and Backspace is pressed five times
- **THEN** the pending text is `করত`, then `কর`, then `ক`, then empty, and the fifth Backspace is not handled

#### Scenario: Remaining vowel stays in the word
- **WHEN** `O`, `C` are pressed (pending `ওছ`) and then Backspace
- **THEN** nothing is committed and the pending text is `ও`

#### Scenario: Held hyphen
- **WHEN** `-` is pressed (pending `-`) and then Backspace
- **THEN** nothing is committed and the pending text is empty

#### Scenario: Nothing pending
- **WHEN** `k`, `h`, `u`, space are pressed and then Backspace
- **THEN** the update reports the key as not handled and changes nothing

### Requirement: Document context with fallback
When the host supplies the text before the pending text, the composer SHALL give the engine that text
followed by the pending text, and SHALL keep it as its view of the document, so later keys without
context read the same text. When the host has never supplied it since the last reset, the text the
composer has itself produced since then SHALL be used instead (the engine's own output).

#### Scenario: Host provides context
- **WHEN** after a reset the host supplies text-before-caret `ক` and `i` is pressed
- **THEN** nothing is committed and the pending text is `ি`

#### Scenario: Host cannot provide context mid-typing
- **WHEN** the host never supplies context and the keys `a`, `m`, space, `i` are pressed
- **THEN** `আম ` is committed and the pending text is `ই`, the same text the engine gives

#### Scenario: Host cannot provide context after reset
- **WHEN** the composer is reset without context and `i` is pressed
- **THEN** the pending text is `ই`

#### Scenario: Context is kept for later keys
- **WHEN** after a reset without context, `a` is pressed with text-before-caret `ক`, and then `'` without context
- **THEN** `'` is decided from `কা`, the same result as when the context is supplied with it

### Requirement: Flush and reset
Flush SHALL commit all pending text and leave the engine ready for a new cluster. Reset SHALL discard
engine state without committing anything, optionally seeding it with the text before the caret.

#### Scenario: Flush on focus change
- **WHEN** pending text is `খ` and flush is requested
- **THEN** `খ` is committed and the pending text is empty

#### Scenario: Reset after caret move
- **WHEN** reset is requested while pending text is `খ`
- **THEN** the update commits nothing, and the next key starts a new cluster

### Requirement: Output toggles
The composer SHALL support three settings, all on by default: Bengali digits, দাঁড়ি for `.`, and
typographic quotes. With all three on, output SHALL be identical to the engine with default
behaviour. With a setting off, the matching key SHALL produce its ASCII character instead: the digit,
`.`, `"` or `'`.

#### Scenario: Defaults
- **WHEN** `1`, space, `.`, space are pressed with default settings
- **THEN** `১ । ` is committed

#### Scenario: ASCII digits
- **WHEN** Bengali digits is off and `2` is pressed
- **THEN** `2` is committed

#### Scenario: Plain full stop
- **WHEN** দাঁড়ি for `.` is off and `.` is pressed
- **THEN** `.` is committed

#### Scenario: Straight quotes
- **WHEN** typographic quotes is off and `"` is pressed
- **THEN** `"` is committed

### Requirement: Committed text is never changed
The composer SHALL NOT ask a host to change text it has committed, or text that was in the document
before the caret. An update SHALL consist only of text that replaces the pending text, the new
pending text, and whether the key was handled.

When an engine rule would rewrite committed text, the composer SHALL process the key as if the word
started at the caret: with the pending text as the only context. This covers `-` after a committed
`-`, `.` after a committed `।`, and a kar before a committed `ঁ`.

#### Scenario: Hyphen already in the document is not rewritten
- **WHEN** after a reset, `-` is pressed with text-before-caret `ক-`
- **THEN** nothing is committed and the pending text is `-`

#### Scenario: Dari already in the document is not rewritten
- **WHEN** after a reset, `.` is pressed with text-before-caret `ক।`
- **THEN** nothing is committed and the pending text is `।`

#### Scenario: Chandrabindu already in the document is not rewritten
- **WHEN** after a reset, `a` is pressed with text-before-caret `কঁ`
- **THEN** nothing is committed and the pending text is `আ`

### Requirement: Typing after Backspace or a caret move
After a Backspace, a reset or a new composer, the next consonant SHALL start a new letter. It SHALL
NOT join or rewrite a consonant before it, whether that consonant is pending or committed. Vowels
SHALL still read the text before the caret and attach as kars. The keys that read the document
(`key_reads_document`) SHALL be vowels, `-`, `.`, `"` and `'`, and not consonants or `^`.

#### Scenario: Consonant after a removed kar
- **WHEN** `k`, `a` are pressed, then Backspace (pending `ক`), then `k`
- **THEN** the pending text is `কক`

#### Scenario: Consonant after a removed conjunct letter
- **WHEN** `d`, `m` are pressed, then Backspace, then `h`
- **THEN** the pending text is `দহ`

#### Scenario: Vowel after Backspace is a kar
- **WHEN** `d`, `m` are pressed, then Backspace, then `a`
- **THEN** the pending text is `দা`

#### Scenario: Consonant after a caret move
- **WHEN** the composer is reset with text-before-caret `করত` and `h` is pressed with that context
- **THEN** nothing is committed and the pending text is `হ`

#### Scenario: Vowel in the middle of a word
- **WHEN** the composer is reset with text-before-caret `কর` and `i` is pressed with that context
- **THEN** nothing is committed and the pending text is `ি`

#### Scenario: Silent o is not undone by context
- **WHEN** `k`, `o`, `m` are pressed with the text before the caret supplied for every key
- **THEN** the pending text is `কম`

#### Scenario: Consonant keys do not read the document
- **WHEN** a host asks whether `i`, `h`, `k`, `^`, `1` or space read the document
- **THEN** the answer is yes for `i` and no for the others

### Requirement: Checking the text before the caret
The composer SHALL answer whether a given text before the caret, followed by the pending text, is
consistent with the output it has produced since its last reset: both are non-empty and one ends
with the other. The check SHALL NOT change any state. Hosts use it to tell a real caret move from an
app that reports the caret late or exposes only the text near the caret.

#### Scenario: App exposes only the text near the caret
- **WHEN** `p`, `o`, space are typed and the host reports text-before-caret `প `
- **THEN** the text matches

#### Scenario: Longer text before the caret
- **WHEN** after a reset `k`, `o`, space are typed and the host reports text-before-caret `আমি ক `
- **THEN** the text matches

#### Scenario: Caret moved elsewhere
- **WHEN** `p`, `o`, space are typed and the host reports text-before-caret `আমি `
- **THEN** the text does not match

#### Scenario: Nothing typed since the reset
- **WHEN** the composer was just reset without context and the host reports text-before-caret `প`
- **THEN** the text does not match

### Requirement: Same result in every host
The composer's updates SHALL produce the same visible text in every host that applies them as
specified, whether or not the host supports replacement ranges. Hosts that can read the text before
the caret SHALL end with identical text, whether they pass it with every key (the web playground)
or only for keys that read the document, and only while nothing is pending (the Mac input source).
A host without text access MAY differ only where a vowel follows text it could not read. This SHALL
be verified by typing key scripts and the seeded random key and Backspace sequences into modelled
hosts, where an unhandled Backspace deletes one code point.

#### Scenario: Backspace after podmo in every host
- **WHEN** `podmo` is typed and Backspace is pressed in each modelled host
- **THEN** every host shows `পদ`

#### Scenario: Backspace after ekoTa podmo in every host
- **WHEN** `ekoTa podmo` is typed and Backspace is pressed in each modelled host
- **THEN** every host shows `একটা পদ`

#### Scenario: Consonant at the start of existing text
- **WHEN** `kor` is typed, the caret is moved to the start, and `k` is typed in each modelled host
- **THEN** every host shows `ককর`

#### Scenario: Backspace in committed text
- **WHEN** `podmo ` is typed and Backspace is pressed twice in each modelled host
- **THEN** every host shows `পদ্`, and a third Backspace gives `পদ`

#### Scenario: Random sequences
- **WHEN** every seeded random key and Backspace sequence is typed into the modelled hosts
- **THEN** the hosts that read the text show the same text after every step

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
