# Tasks

Implement after `rr-reph` has landed.

## 0. Licensing (author)

- [x] 0.1 Decide the data licence (design D7). Record the decision and the attribution text in `design.md`.
- [x] 0.2 Add `crates/druti-core/data/LICENSE-DATA` (CC BY-SA 4.0 text plus the attribution) with the list, and the attribution to the README and the macOS About text (the app has no About panel: the bundled `Licenses.txt` and the DMG's Read Me). Verify that `cargo package -p druti-core --list` includes both files.

## 1. Correction list (Linux)

- [x] 1.1 Write the generator in `scripts/autocorrect/` (design D3). It reads downloaded corpora and the Dakshina lexicon, runs the engine through `cargo run`, and writes `crates/druti-core/data/autocorrect.tsv`, sorted. Verify by running it twice and checking that the output is byte-identical.
- [x] 1.2 Generate the list and review a sample of 200 entries by hand, including the 50 most frequent. Record the entry count and the coverage figures in the PR description.

## 2. Core (Linux)

- [x] 2.1 Add the `autocorrect` module: parse the embedded list into a sorted table, `lookup(word) -> Option<&str>` with nukta normalisation (design D2, D3). Verify with unit tests for the "Common words" and "Words not in the list" scenarios.
- [x] 2.2 Add the consistency test: for every entry, typing the roman spelling gives the engine spelling, and typing it with `o` at each removed hasant gives the correct spelling (design D2). Verify that it passes, and that changing one engine rule makes it fail with the entry's name.
- [x] 2.3 Add `Config::autocorrect` (default `false`) and apply it in `transpile` (design D6). Verify with `engine/transpile.json`-style cases for "Converting a sentence" and "Setting off" in a new `autocorrect/transpile.json` fixture.
- [x] 2.4 Correct the pending word at word end in `Composer`, keep it and the break pending for one key, commit at once on Enter and unhandled keys, and keep the engine output in step (design D4). Verify with a new `composer/autocorrect.json` covering "Default is off", "Correction at space", "Next key commits the correction", "Uncorrected word commits as usual", "Correction before a held dari" and "Enter commits at once".
- [x] 2.5 Add Backspace undo and the "kept" mark (design D5). Verify with `composer/autocorrect.json` cases "Undo", "Undone word is kept" and "Second Backspace edits as usual".
- [x] 2.6 Run every existing composer and engine fixture with Autocorrect off. Verify that none changes.

## 3. Bindings and hosts (Linux, then Mac)

- [x] 3.1 Add `autocorrect` to the `Config` of `druti-ffi` and `druti-wasm` with matching docs. Verify that the parity tests pass and that the WASM clippy target is clean (Linux).
- [x] 3.2 Add the playground toggle for typing and conversion (Linux). Verify the "Toggle in the playground" scenario in the browser.
- [ ] 3.3 Add the macOS input-menu toggle, stored with the others and off after upgrade; apply it to Convert selection (Mac). Verify with `make -C macos test` and by hand for "Turning on Autocorrect" and "Upgrade keeps it off". (Swift written on Linux: `Settings`, the menu item and a binding test; not yet compiled.)
- [ ] 3.4 Manually check word-end correction and Backspace undo in TextEdit, Notes, Safari and one Chromium-based app, watching the pending trailing space (Mac). Record the results in the PR.

## 4. Documentation and release (Linux)

- [x] 4.1 Document Autocorrect in `README.md`: off by default, what it changes (hidden vowels only, at word end), Backspace to undo, and that the engine's own rules are unchanged. Add the data attribution from task 0.1. Verify that every example is a fixture.
- [ ] 4.2 Run the full CLAUDE.md command list (fmt, clippy native and WASM, tests, docs, MSRV; Swift targets on the Mac). Verify there are no warnings or failures. (Linux part done and clean; Swift targets pending on the Mac.)
