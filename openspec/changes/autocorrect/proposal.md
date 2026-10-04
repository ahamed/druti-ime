## Why

Bengali writes many words with an unwritten vowel between two consonants that are pronounced
together: আমরা is said "amra", একটা "ekTa", আপনি "apni", বলতে "bolte". Druti's engine joins the
consonants a typist types together, so `amra` gives `আম্রা`, and the typist has to type the hidden
vowel (`amora`). After `rr-reph` (a single `r` never joins; nothing joins after a breathy letter),
about 38% of everyday words that have two consonants in a row (27% in formal text) still need
that `o`.

No engine rule can fix these without a word list. A list inside the engine makes its output depend
on words, which is what the author rejected in the smart-hasant draft and in `conjunct-rules`. A list
*outside* the engine can keep the engine exactly as it is: deterministic and learnable, with the same
keys always giving the same letters. The typist who wants natural spellings turns on an extra layer.

## What Changes

- **New, off by default:** an Autocorrect setting. When it is on and a word ends (space,
  punctuation, digit, Enter), the composer looks the finished word up in a fixed list. If the list
  has it, the word is replaced once by its correct form before it is committed: `amra` + space →
  `আমরা `, `ekTa` → `একটা`, `apni` → `আপনি`, `bolte` → `বলতে`, `thakte` → `থাকতে`.
- Corrections only ever **remove a hasant**, which is the "hidden o" the author described: the
  corrected word is exactly what the engine gives when the `o` is typed (`amora` → `আমরা`). Autocorrect
  never adds a conjunct, a reph or any other letter.
- The list is keyed by the **Bengali word the engine produced** (`আম্রা` → `আমরা`), so it works however
  the word was typed, including after Backspace inside it.
- **Backspace undoes a correction:** as the very next key, it restores the uncorrected word as
  pending text and removes the word break. That word is not corrected again when it ends.
- **Ambiguous words are never corrected.** If the engine's output is itself a real word, or two list
  words would come from the same output, there is no entry; the engine's output stands.
- The list holds the words among the ~20,000 most frequent in two corpora that need a hidden-vowel
  fix and whose typing Dakshina attests (1,269 entries in the first generation; see design D3 for
  coverage). It ships inside `druti-core` as data, versioned with
  the engine.
- Everything stays deterministic: the result is a function of the keys, the settings and the list
  version. Nothing is learned from the user and nothing is suggested.
- Bulk conversion (Convert selection on macOS, the playground's converter) applies the same list
  when the setting is on.
- Committed text still never changes (whole-word-pending D2): the correction happens to the pending
  word, at the moment it ends.

## Capabilities

### New Capabilities
- `autocorrect`: the correction list, its contents rules (hasant removal only, no ambiguous entries),
  lookup by engine output, and its use in bulk conversion.

### Modified Capabilities
- `ime-composer`: correcting the pending word when it ends, undoing with Backspace, the setting.
- `macos-input-source`: an Autocorrect toggle in the input menu.
- `web-playground`: an Autocorrect toggle, and the setting in the WASM bindings.

## Impact

- **Depends on** `rr-reph`: the list is generated from the engine's output, so it is built after the
  engine rules are final, and regenerated whenever they change (a test enforces this).
- **Rust**: `druti-core` gains an `autocorrect` module and data file, a `Config` field
  (`autocorrect: bool`, default `false`), and composer handling at word end and on Backspace.
  `druti-ffi` and `druti-wasm` expose the new `Config` field together (binding parity).
- **Engine**: unchanged. No engine fixture changes.
- **Fixtures**: new `composer/autocorrect.json`; a test that every list entry matches the current
  engine; `data.json` unchanged.
- **Hosts**: macOS input menu toggle (Swift, settings only); playground toggle (TypeScript).
- **Tooling**: a generator (kept in the repo, run by hand) that builds the list from downloaded
  corpora by running the engine.
- **Licensing (decided)**: the word list is derived from CC BY-SA 4.0 corpora (Google Dakshina,
  OpenSubtitles via FrequencyWords), so the list file ships under CC BY-SA 4.0 with attribution; the
  code stays MIT. See design D7.
- **Size**: about 75 KB of data in the app and the WASM bundle.
