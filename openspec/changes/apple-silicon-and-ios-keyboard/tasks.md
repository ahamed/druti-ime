# Tasks

No task here changes engine or composer behaviour, and no fixture changes.

## 1. M0 — Apple silicon only (Linux)

- [x] 1.1 `scripts/build-xcframework.sh`: build `aarch64-apple-darwin`, `aarch64-apple-ios` and `aarch64-apple-ios-sim`, one slice each, no `lipo`; generate the bindings from the `aarch64-apple-darwin` dylib (design D1). Verify `cargo check -p druti-ffi` passes for all three targets
- [x] 1.2 `macos/project.yml`: `ARCHS: arm64`. Update the Makefile's prerequisites, `about.toml`'s targets, the README, `macos/README.md`, the DMG Read Me and the release notes (Apple silicon, no Intel caveat), and CLAUDE.md
- [x] 1.3 CI `macos` job and the release workflow: drop Rosetta and the x86_64 tests, install the iOS Rust targets, and check that every library slice and app binary is arm64 only
- [x] 1.4 Update `openspec/config.yaml`'s context (Apple silicon, iOS keyboard)

## 2. M1 — Shared Swift package (Linux, verified on Mac)

- [x] 2.1 `Package.swift`: add the iOS 17 platform. Move `Settings` into the package as public API (design D5) and move `.swift-format` to the repository root (design D7)
- [x] 2.2 Add `TextDocument` and `KeyboardSession` (design D2, D3) with tests through a fake document: each key as marked text, Space and Return commit, letter Backspace, Backspace on committed text, kar after a document consonant, apps without text, caret moves, the app ending the word, config changes
- [x] 2.3 Add `KeyboardState` with tests: QWERTY rows, globe only when asked, one-shot Shift, Caps Lock, `^` and `:` reachable, Space returns to letters, rows fit ten keys
- [ ] 2.4 (Mac) `make -C macos test` and `make -C macos lint` pass, and `make -C macos install` still types `khub` → `খুব` in TextEdit with the menu toggles working (Settings moved), and `amra` + space gives `আমরা ` with Autocorrect never touched
- [x] 2.5 Merge `main` (Autocorrect): add the Autocorrect toggle to the shared `Settings` and the iOS options panel, on by default on both platforms. Verify with the `Settings` default test and a `KeyboardSession` test for correction and Backspace undo

## 3. M2 — iOS app and keyboard (Linux, verified on Mac)

- [x] 3.1 `ios/project.yml` (app + keyboard extension, arm64, iOS 17, `DRUTI_BUNDLE_ID_PREFIX`), `ios/Makefile` (`project`, `app`, `test`, `lint`, `format`, `version`, `clean`) and the keyboard's `NSExtension` Info.plist (design D6)
- [x] 3.2 `KeyboardViewController` and `ProxyDocument`, `KeyboardView` (keys, Backspace repeat, globe, options panel) and the SwiftUI container app
- [x] 3.3 CI: lint `ios/`, run `make -C ios test` and `make -C ios app`, check the `.appex` is embedded, arm64 only, and doesn't request Full Access
- [ ] 3.4 (Mac) `make -C ios lint`, `make -C ios test` and `make -C ios app` pass with no warnings; fix whatever the first real compile finds
- [x] 3.5 `ios/README.md` and the top-level README's "iPhone and iPad" section

## 4. M3 — On a device (Mac + iPhone)

- [ ] 4.1 Install from Xcode with a free Apple account as `ios/README.md` says, turn the keyboard on, and confirm iOS never asks about Full Access
- [ ] 4.2 In Notes, type `khub` (each key shows `ক`, `খ`, `খু`, `খুব`), `podmo` + Backspace (`পদ`), Shift `t` `a` (`টা`), `2024` with Bengali digits on and off, `ami` + Return, and hold Backspace
- [ ] 4.3 Check design D3's assumption: with a word pending, `documentContextBeforeInput` ends with the marked text, and tapping elsewhere mid-word keeps the word and starts a new one at the new caret. If the context excludes marked text, change `KeyboardSession.documentDidChange` and its test, and record it in design D3
- [ ] 4.4 Type the `macos/COMPATIBILITY.md` paragraph in Notes, Messages, Safari (a web form), Mail, WhatsApp and Google Docs; record the results in `ios/COMPATIBILITY.md`
- [ ] 4.5 Check the keyboard in dark mode, in landscape, and on an iPad
