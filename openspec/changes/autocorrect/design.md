## Context

After `rr-reph`, the engine joins consonants typed together except after র (single `r`) and after a
breathy letter, হ, ড়, ঢ় or য়. Words with an unwritten vowel between two other consonants (আমরা,
একটা, আপনি, বলতে, জানতে, থাকতে) need the typist to type `o`. The research in
`conjunct-rules/research.md` showed:
- no rule that only sees the keys typed so far can tell these words from real conjuncts (একটা vs
  ডক্টর, আপনি vs স্বপ্ন);
- a joining list inside the engine (`conjunct-rules`, parked) reaches about 92% of words but must be
  memorised, which works against typing without looking.

The author's idea: keep the engine as it is, and add an optional layer that knows words. It turns
`amra` into what `amora` would have given.

The composer already keeps the whole word pending until it ends (whole-word-pending D1) and never
changes committed text (D2). It already holds a trailing `-` or `।` pending ("Holding a trailing
hyphen or dari"). Those two mechanisms make a word-end correction possible without touching committed
text.

## Goals / Non-Goals

**Goals:**
- The engine, and every engine fixture, stays exactly as it is; Autocorrect is off by default.
- With Autocorrect on, the result is still a pure function of keys + settings + list version.
- One predictable moment of change: when the word ends, and at most once per word.
- An easy way out of a wrong correction (Backspace).

**Non-Goals:**
- Suggestions, ranking, or learning from the user.
- Adding conjuncts or reph (`korta` → কর্তা). Autocorrect only removes hasants.
- Correcting text already committed, or words outside the list.
- A user-editable list (could follow later).

## Decisions

### D1. A layer in `druti-core`, applied by the composer at word end
Autocorrect lives in Rust like all behaviour (CLAUDE.md), in a new `autocorrect` module used by the
composer and by `transpile`. Hosts only show a toggle.

The composer applies it when a key ends the pending word. That is the same moment it already commits
the word today, so a typist learns one rule: "a word may change once, when I finish it".

*Alternative:* rewrite the roman keys mid-word (`amr` → `amor`) before they reach the engine, as the
author first sketched. Rejected: at `amr` the layer can't know whether আমরা or আম্রপালি is coming, so
it would have to guess and later undo the guess on screen. Deciding mid-word "as soon as certain" was
offered and declined in review as harder to predict.

### D2. The list is keyed by the engine's output, not by keys
Entries map the Bengali the engine produced to the correct Bengali: `আম্রা` → `আমরা`. Reasons:
- It is independent of how the word was typed: Backspace inside the word, `K` vs `kh`, resumed
  clusters. The roman keys of a word are not even known after a Backspace.
- The same lookup serves bulk conversion and the composer.
- The correction is checkable: the test removes the listed hasants and compares.

This is equivalent to the author's "insert `o` and send it to the engine": every entry's correct
spelling is exactly what the engine gives with `o` typed there (enforced by the consistency test).

### D3. What goes in the list
The generator (run by hand, kept in the repo under `scripts/autocorrect/`) builds it:
1. Take the ~20,000 most frequent words from the two corpora (subtitles, Wikipedia), normalised (nukta
   letters precomposed, ZWJ/ZWNJ handled as in the research).
2. For each word, take its native romanizations from Google's Dakshina lexicon (how people really
   type it, e.g. আমরা → "amra"). Run each through the current engine.
3. Keep the word when the engine's output differs from it only by extra hasants. That is exactly a
   hidden vowel the typist didn't type. Drop it if the output differs in any other way (a different
   letter, a missing reph): those are spelling differences, not hidden vowels.
4. Drop the entry if the output is itself a lexicon word, or if two words would share an output
   (ambiguous).
5. Write `crates/druti-core/data/autocorrect.tsv` (engine spelling, correct spelling, roman spelling),
   sorted, one entry per line, so diffs are reviewable.

Expected size: 3,000–5,000 entries. In the research corpora, the top 20,000 words cover about 89% of
hidden-vowel occurrences in subtitles and 71% in Wikipedia (heuristic upper bound; Dakshina filtering
removes words whose vowel is actually typed).

**Result (first generation, 2026-10-03):** 1,269 entries (75 KB). The estimate above was too high
because 5,826 of the 20,000 words have no Dakshina romanization, so there is no evidence of how they
are typed, and they are skipped. Against every word in the corpora with that evidence (3,852
entries), the 20,000-word list covers 98% of the hidden-vowel occurrences in subtitles and 88% in
Wikipedia: 5.4% and 3.7% of all words. A hand review of 210 entries (the 60 most frequent and 150
random) found no wrong correction.

The data is embedded with `include_str!` and parsed once into a sorted table; lookup is a binary
search. No new dependency.

### D4. Correcting the pending word
When a key ends the word and Autocorrect is on:
1. Look up the pending word (just the Bengali word; a held `-` or `।` is not part of it).
2. Not found: behave exactly as today.
3. Found: replace the word with its correction, and assign the corrected text to the engine's output
   (the engine already supports assigning output and resuming from it), so later document-aware keys
   and `matches_text_before_caret` see what the host shows.
4. Keep the corrected word plus the key's text pending until the next key (D5). For Enter and keys the
   composer does not handle (passed to the app), commit at once instead, because the app acts on that
   key immediately.

For a corrected word, "committed plus pending equals the engine output" holds against the corrected
output. With Autocorrect off, nothing changes, so every existing composer fixture still passes.

### D5. Backspace undoes once
The pending "corrected word + break" state lasts exactly one key:
- Backspace: replace it with the uncorrected word as pending text, restore the engine's output, and
  mark that word "kept". The next word end commits it unchanged (the mark clears then).
- Any other key: commit the corrected word and break, then process the key normally.

This mirrors phone autocorrect, and it keeps D2 of whole-word-pending: committed text is never
touched, because the correction is still pending when it is undone.

### D6. Setting and bindings
- `Config::autocorrect: bool`, default `false`. That follows the `openspec/config.yaml` rule (new
  options default to current behaviour), unlike `rr-reph`.
- `druti-ffi` and `druti-wasm` add the field to their `Config` together, with the same doc, and their
  parity tests cover it.
- macOS: an input-menu toggle stored with the others; off after upgrade. Playground: a toggle.
- `transpile` takes the same config: Convert selection and the playground converter correct words when
  it is on.

### D7. Data licensing (decided: CC BY-SA 4.0 for the list)
The sources are CC BY-SA 4.0: Google Dakshina, and the FrequencyWords lists built from
OpenSubtitles. Druti is MIT. Options:
- (a) Ship `autocorrect.tsv` under CC BY-SA 4.0 with attribution in the README and the app's About
  text; the code stays MIT. This is the common practice for word lists.
- (b) Build the lexicon only from public-domain or MIT-compatible sources. That means smaller coverage
  and more work.

**Decision (author, 2026-10-03): (a).** `crates/druti-core/data/autocorrect.tsv` is licensed CC BY-SA
4.0, with a `LICENSE-DATA` file next to it and this attribution in the README and the macOS About
text:

> The Autocorrect word list is derived from the Dakshina dataset (Google Research, CC BY-SA 4.0) and
> from FrequencyWords by Hermit Dave (built from OpenSubtitles, CC BY-SA 4.0). It is licensed CC BY-SA
> 4.0. The rest of Druti is MIT.

The code that reads the list stays MIT.

## Risks / Trade-offs

- **[Trade-off] The word changes on screen once, at word end** (আম্রা → আমরা). This is the price of a
  word list. → Off by default; one fixed moment; Backspace undoes.
- **[Risk] A wrong correction in a rare word.** → Only hasant removal, only unambiguous entries, only
  frequent words; Backspace undo; the generator's output is reviewed as a sorted diff.
- **[Risk] A trailing space in marked text** looks different in some apps for one keystroke. → Covered
  by the manual app checks; if an app misbehaves, commit immediately there (no undo), as for Enter.
- **[Risk] The list drifts from the engine** when rules change. → The consistency test fails until
  the list is regenerated.
- **[Trade-off] App and WASM size grow** by about 75 KB (the first list).
- **[Trade-off] The list file is CC BY-SA**, not MIT (D7). Anyone redistributing it must keep the
  attribution and licence; the code is unaffected.

## Migration Plan

1. Land `rr-reph` (engine rules final), then this change.
2. Ship in a minor release after 2.0.0 (or with it); off by default, so nobody's typing changes
   unless they turn it on.
3. Rollback: turn the setting off, or revert; no stored data beyond the toggle.
