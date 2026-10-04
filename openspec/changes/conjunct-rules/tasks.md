> **Status: parked (2026-10-03). Do not apply.** The author chose not to ship a joining list: it
> has to be memorised, which works against typing without looking at the screen. `rr-reph` takes only
> its cheapest, audible rule (nothing joins after a breathy letter), and words such as আমরা, একটা,
> আপনি are left to the optional `autocorrect` change. This folder is kept for its research
> (`research.md`) and in case a joining list is offered later as an opt-in setting.

# Tasks

Implement after `rr-reph` has landed. Evidence for every number is in `research.md`.

## 1. Joining table (Linux)

- [ ] 1.1 Add `data::JOINING_PAIRS` with the 107 pairs of design D3, grouped by family with a comment per group citing "conjunct-rules design D3". Write nukta letters as `\u{...}` escapes. Add `joining_pairs` to `engine/data.json` and to the data fixture test. Verify that `cargo test -p druti-core --test fixtures` passes and the "Table matches the fixture" scenario holds.

## 2. Engine rules (Linux)

- [ ] 2.1 In `engine.rs` `process_consonant`, decide each join per design D1 and D2: word start from the output before the buffer, otherwise `JOINING_PAIRS`; write the consonant apart and restart the buffer when it doesn't join. Verify with new `engine/unit.json` cases for "Loanword clusters", "Pair that only joins at the start", "After a space", "Pairs that stay apart", "Listed pairs join", "o still separates a listed pair" and "Existing conjunct keys are unchanged".
- [ ] 2.2 In `rules::aspirated_consonant`, remove the hasant when the aspirated pair is not in the table inside a word (design D4). Verify with `engine/unit.json` cases for "ক্ত becomes কথ" (output after every key) and "Aspirated pair that is listed".
- [ ] 2.3 Add the backtick rule in `rules.rs` (design D5): hasant after a consonant, same as a second `r` after র, ZWNJ on a second backtick, removed by a vowel, passed through with no consonant before it. Verify with `engine/unit.json` cases for "Forced join, key by key", "Other forced joins", "Visible hasant", "Vowel after the backtick", "Backtick after r is reph" and "Backtick without a consonant", plus a letter-Backspace case on `ডক্`.
- [ ] 2.4 Add the `w` ব-ফলা rule and make `z` never join (design D6). Verify with `engine/unit.json` cases for "ব-ফলা outside the table", "b follows the table", "z after a consonant" and "Backtick before z".
- [ ] 2.5 Update doc comments for `process_consonant` and the new rules, citing "conjunct-rules design D1–D6". Verify that `cargo clippy --all-targets -- -D warnings` and `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --workspace` are clean.

## 3. Golden fixtures (Linux)

- [ ] 3.1 Regenerate the affected cases in `engine/unit.json`, `engine/words.json`, `engine/random.json` and `engine/transpile.json` with a one-off script kept out of the repo. The script must fail if any case would change that has no consonant key after a consonant inside a word, no backtick, no `w` and no `z` after a consonant. Verify that `cargo test -p druti-core` passes and record the per-file counts in the PR description.
- [ ] 3.2 Add `engine/words.json` entries for every word in the specs and in the design D7 user text (`glas`, `skul`, `slip`, `asle`, `dekhte`, `amra`, `ekTa`, `apni`, `bolle`, `jayga`, `kintu`, `rasta`, `pakka`, `buddhi`, `potro`, `Sokti`, `raShTro`, `janote`, ``Dok`Tor``, ``al`lah``, ``som`raT``, ``sbop`no``, ``ud``dIn``, `pokw`, `somonwoy`, `dekhbe`, `biSbas`, `iccha`, `atma`, `Sobdo`, `golpo`, `koShTo`, `niScoy`, `kompon`, `sottor`, `Sukla`, `bhokto`). Add `engine/transpile.json` cases such as `ami dekhte cai, amra ekTa glas nebo` → `আমি দেখতে চাই, আমরা একটা গ্লাস নেব`. Verify that `word_fixtures` and `transpile_fixtures` pass.

## 4. Composer (Linux)

- [ ] 4.1 Add `composer/backspace.json` and `composer/*.json` cases for "Backspace on a backtick hasant", "Word break commits the hasant" and "Joining inside the pending word". Verify that `cargo test -p druti-core --test composer` passes with no change to composer code.
- [ ] 4.2 Run `tests/hosts.rs` on the regenerated `random.json`, and add scripts typing ``Dok`Tor`` with a Backspace after the backtick. Verify that the playground, Mac and terminal models show identical text.

## 5. Bindings and hosts (Linux, then Mac)

- [ ] 5.1 Run `cargo test -p druti-ffi -p druti-wasm` and the WASM clippy target (Linux). Verify that the parity tests pass with no binding changes.
- [ ] 5.2 Run `make -C macos test`, including under Rosetta (Mac). Verify that it passes with no Swift changes.
- [ ] 5.3 Manually type `dekhte`, `amra`, `glas`, ``Dok`Tor``, ``ud``dIn`` and `pokw` in TextEdit and one Chromium-based app with `make -C macos app` (Mac). Verify that the results match the spec scenarios, that the backtick key reaches the input method on a US and a UK layout, and record the results in the PR.

## 6. Documentation and release (Linux, then Mac)

- [ ] 6.1 Add "How consonants join" to `README.md` using the design D7 text, and an appendix table of the 107 pairs with one example each, generated from `JOINING_PAIRS` by a test or script so it cannot drift. Verify that every example in the docs is a `words.json` fixture.
- [ ] 6.2 Extend the 2.0.0 release notes from `rr-reph` with this change's spellings (`dekhte` now gives দেখতে; ``Dok`Tor`` for ডক্টর; `w` for ব-ফলা; backtick rules). Verify on the Mac that `make -C macos version` prints 2.0.0.
- [ ] 6.3 Run the full CLAUDE.md command list: fmt, clippy (native and WASM), tests, docs, MSRV, and the Swift targets on the Mac. Verify there are no warnings or failures.
