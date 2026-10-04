## Why

Bengali spelling often leaves the inherent vowel between two consonants unwritten, without a hasant:
করতে is spoken /kɔrte/. Druti joins every consonant typed after `r` into a reph, so `korte` gives
`কর্তে`, `korbo` gives `কর্ব`, and `dorkar` gives `দর্কার`. To get the right word, people must type an
`o` they neither say nor write (`korote`, `korobo`, `dorokar`).

Reph and the unwritten vowel sound the same (কর্তা /kɔrta/, করতে /kɔrte/), so no rule can tell them
apart from the letters alone. A rule that waits for later keys, or that uses a word list, changes text
already on screen (`কর্তা` → `করতাম`). The typist then has to re-read every word, which defeats fast
typing from muscle memory. So the typist has to mark one of the two readings, and the mark should go
on the less frequent one.

In a 2.4-million-word Bengali corpus (subtitles, close to everyday writing), র followed directly by a
consonant mid-word is a reph in 40,825 tokens and an unwritten vowel in 67,306. These include the most
common verb forms in the language: করতে, করবে, করছি, পারবে, দরকার.

## What Changes

- **BREAKING** A single `r` never joins the consonant after it: `korte` → `করতে`, `korbo` → `করব`,
  `korlam` → `করলাম`, `dorkar` → `দরকার`. A র typed after another consonant (র-ফলা: `pr` → `প্র`)
  is unchanged.
- **BREAKING** Reph is typed as `rr`. The second `r` shows a hasant right away (`korr` → `কর্`), and
  the next consonant completes the reph without changing any letter: `korrta` → `কর্তা`, `orrtho` →
  `অর্থ`, `dhorrmo` → `ধর্ম`.
- A vowel after `rr` cancels the reph: `korra` → `করা`, `korro` → `কর`. The exception is `i`, which
  keeps making ঋ / ঋ-kar as today: `rriN` → `ঋণ`, `krriShi` → `কৃষি`.
- **BREAKING** `y` after a single `r` is a visible য-ফলা, never reph: the engine writes র + ZWJ + `্য`
  (`poryonto` → `পর‍্যন্ত`, `ryab` → `র‍্যাব`). Reph over য is typed with `rr` like every other reph
  (`porryonto` or `porrzonto` → `পর্যন্ত`, `karryo` → `কার্য`). After র-ফলা, `y` gives plain `্য` as
  today (`bryanD` → `ব্র্যান্ড`). `z` is the letter য and follows the `r` rule like any consonant
  (`porzonto` → `পরযন্ত`).
- **BREAKING** Nothing joins after a breathy letter (খ ঘ ছ ঝ ঠ ঢ থ ধ ফ ভ), হ, ড়, ঢ় or য়, except the
  ফলা keys `r`, `l`, `m`, `n`/`N`, `w` and `y`: `dekhte` → `দেখতে`, `bujhte` → `বুঝতে`, `dekhbe` →
  `দেখবে`, `poRte` → `পড়তে`, `jayga` → `জায়গা`, while `bhromoN` → `ভ্রমণ`, `cihno` → `চিহ্ন`,
  `dhwoni` → `ধ্বনি` still join. Breathy sounds can't be pronounced in a cluster, so typists don't
  have to learn it as a list.
- `o` still separates consonants everywhere (`korote` → `করতে`), so the old spellings that use `o`
  keep working.
- This becomes the default and only behaviour, with no setting. It ships as Druti 2.0.0. It
  deliberately overrides the project rule that new behaviour defaults to the current one: these are
  core typing rules, and two algorithms would split users' muscle memory.
- Every decision is made by the key being typed, using only the letters since the last vowel. Text
  before the last vowel never changes.

## Capabilities

### New Capabilities
<!-- None -->

### Modified Capabilities
- `rust-engine-core`: new requirements for when `r` joins: a single `r` never forms reph; `rr` forms
  reph; a vowel after `rr`; ঋ from `rr` + `i`; visible য-ফলা after `r`; nothing joins after a breathy
  letter except ফলা keys.
- `ime-composer`: new requirements for the armed reph in the pending word (Backspace and committing)
  and for Backspace on `র‍্য`.

## Impact

- **Rust**: `druti-core`, in `engine.rs` (`process_consonant`, `process_vowel`; the breathy-letter set
  in `data.rs`) and `rules.rs`
  (`rassaw_ri` now reads `র্`, not `র্র`; a new rule for `r` after `r`). The public API and the
  bindings don't change.
- **Fixtures**: the engine fixtures that type `r` before a consonant key, `rr`, `ry`, or a consonant
  after a breathy letter, হ, ড়, ঢ় or য়. Before those last two rules that was 5 unit cases, 18 curated words, 201 of the 2,400 random sequences, and 9 bulk
  conversion cases; the regeneration script reports the final counts. Each is
  updated in the same commit as the engine change. No other fixture changes.
- **Hosts**: no code changes in Swift or TypeScript. Both pick up the new engine.
- **Docs**: the "How typing works" section of `README.md`, the `rri` note in
  `docs/macos-input-source.md`, and release notes with a table from old to new spellings.
- **Release**: Druti 2.0.0 (`macos/project.yml`).
- **Not in this change**: `conjunct-rules` (a 107-pair joining list) is parked: it needs a list to be
  memorised, which works against typing without looking. Words like আমরা, একটা, আপনি, বলতে are
  handled instead by the optional `autocorrect` change, which leaves the engine untouched.
