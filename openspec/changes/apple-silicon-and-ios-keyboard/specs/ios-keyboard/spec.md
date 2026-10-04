## ADDED Requirements

### Requirement: Typing in any app
While the Druti keyboard is active, every character key SHALL go to the Rust composer and SHALL
immediately show its Bengali result in the text field, following the composer's commit/pending split.
Committed text SHALL be inserted as normal text. Pending text (the word being typed) SHALL be shown as
marked text. The keyboard SHALL never change text it has committed.

#### Scenario: Typing a word in Notes
- **WHEN** the user types `khub` in Notes with the Druti keyboard
- **THEN** the field shows `ক`, then `খ`, then `খু`, then `খুব` after each key, and no roman letters appear at any point

#### Scenario: Space commits the word
- **WHEN** the user types `khub` and then Space
- **THEN** `খুব ` is normal text and nothing is marked

### Requirement: Keys that end or edit the word
- Space goes to the composer, and its result is committed.
- Return commits the pending word, then types a line break.
- Backspace removes one letter of the pending word (`দ্ম` → `দ`); with nothing pending it deletes one
  character of committed text. Holding Backspace repeats it.
- When the keyboard is dismissed or switched away, the pending word is committed.

#### Scenario: Backspace in a conjunct
- **WHEN** the user types `podmo` and then Backspace
- **THEN** the field shows `পদ`, never `পদ্`

#### Scenario: Return in a chat app
- **WHEN** the user types `ami` and taps Return
- **THEN** `আমি` is committed before the line break or send

### Requirement: Caret moves and text the app changes
With nothing pending, if the text before the caret no longer ends the way the composer's own output
does, the keyboard SHALL treat it as a caret move and start a new word there. While a word is pending,
if the app reports a change and the text before the caret no longer ends with the pending word, the
keyboard SHALL forget the pending word without writing anything (the app has already kept it as
normal text). When the app exposes its text, a vowel typed right after an existing consonant SHALL
become a kar.

#### Scenario: Tapping elsewhere mid-word
- **WHEN** the user types `kh`, taps after `আমি ` elsewhere in the text, and types `k`
- **THEN** `খ` stays where it was, and `ক` is typed after `আমি `

#### Scenario: Kar after an existing consonant
- **WHEN** the user puts the caret right after an existing `ক` in Notes and types `i`
- **THEN** the field shows `কি`

### Requirement: Keyboard layout
The keyboard SHALL have a QWERTY letters layer, a numbers layer and a symbols layer like the system
keyboard's, which together hold every character the phonetic rules use (including `^` and `:`). Shift
SHALL type the next letter in upper case, a double tap SHALL lock it, and it SHALL never turn on by
itself. Space on the numbers or symbols layer SHALL return to the letters. The globe key SHALL be shown
only when iOS asks for it, and SHALL open the system's keyboard switcher.

#### Scenario: Upper case matters
- **WHEN** the user taps Shift, then `t`, then `a`
- **THEN** the field shows `টা`, and Shift is off again

### Requirement: Options
A key on the keyboard SHALL open an options panel with the same four toggles as the macOS input menu:
Bengali digits, দাঁড়ি for full stop, smart quotes and Autocorrect, all on by default. A change SHALL apply from the
next key and SHALL be remembered when the keyboard is opened again.

#### Scenario: ASCII digits
- **WHEN** the user turns off Bengali digits and types `2024`
- **THEN** the field shows `2024`

#### Scenario: Autocorrect out of the box
- **WHEN** the user types `amra` then space on a fresh install, without opening the options
- **THEN** the field shows `আমরা `, and Backspace right after gives `আম্রা`

### Requirement: Container app and privacy
The keyboard SHALL ship inside an iOS app named Druti for iOS 17 or later that explains how to turn the
keyboard on and offers a field to try it in. The keyboard SHALL NOT request Full Access
(`RequestsOpenAccess` is false), so it has no network access and no shared container.

#### Scenario: Turning the keyboard on
- **WHEN** the user installs the app and follows its steps
- **THEN** Druti appears under Settings → General → Keyboard → Keyboards, and iOS never asks to allow Full Access

### Requirement: Apple silicon build from source
The app and the keyboard SHALL be built for arm64 only (iPhones, iPads, and the arm64 simulator on Apple
Silicon Macs). A developer SHALL be able to generate the Xcode project with one command, and install
the app on their own device from Xcode with their own Apple account, including a free one. CI SHALL
build the app for devices without signing, run the `BengaliIMECore` tests on an iOS Simulator, and
check that the app and the keyboard binaries are arm64 only.

#### Scenario: Installing with a free Apple account
- **WHEN** a developer runs `make -C ios project`, chooses their personal team for both targets in Xcode and runs on their iPhone
- **THEN** the Druti app installs and the keyboard can be turned on, for as long as the free provisioning profile lasts (7 days)
