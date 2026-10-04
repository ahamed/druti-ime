## Why

The author wants Druti on their iPhone and iPad, typing the same way it does on the Mac. The engine
was built for this (design D7 of `add-macos-input-source` planned iOS slices), but there is no iOS
host yet.

At the same time the Intel half of the Mac build has never been tested on Intel hardware, and it
costs every build and CI run an x86_64 compile plus Swift and Rust tests under Rosetta. The author
has decided to support Apple silicon only, on the Mac and on iOS.

## What Changes

- **BREAKING** The macOS app is arm64 only. It no longer runs on Intel Macs. The XCFramework drops
  its x86_64 slice, and CI and the release workflow drop the Rosetta tests and check that every
  binary is arm64 only.
- The XCFramework gains `ios-arm64` (iPhone and iPad) and `ios-arm64-simulator` (the simulator on
  Apple Silicon Macs) slices. There is no x86_64 simulator slice.
- New `ios/`: an iOS app with a custom keyboard extension, **Druti**, for iOS 17 or later. It types
  with the same Rust `Composer` as the Mac: each key shows its Bengali right away, the word being
  typed is marked text, Backspace removes one letter of it, and committed text is never changed.
  It has QWERTY letter, number and symbol layers, Shift and Caps Lock, a globe key when iOS asks
  for one, and an options panel with the same three toggles as the Mac input menu. It doesn't ask
  for Full Access.
- The iOS session logic (`KeyboardSession`, behind a small `TextDocument` protocol) and the layout
  rules (`KeyboardState`) go into the shared `BengaliIMECore` package, so they are unit tested on
  macOS and on the iOS Simulator. `Settings` moves there too, shared by both apps.
- The iOS app is built from source and installed from Xcode with the developer's own Apple account
  (a free one works). No App Store, TestFlight or downloadable build in this change.
- `.swift-format` moves to the repository root, so both Apple projects use it.
- Documentation: `ios/README.md`, an "iPhone and iPad" section in the top-level README, Apple
  silicon wording in the Mac README, DMG Read Me and release notes, and CLAUDE.md.
- No engine or composer behaviour changes; no fixtures change.

## Capabilities

### New Capabilities
- `ios-keyboard`: the Druti keyboard for iPhone and iPad: typing, key layout, options, the container
  app, privacy, and building and installing it from source.

### Modified Capabilities
- `macos-distribution`: the release is Apple silicon only. "Universal build for macOS 14 or later"
  is replaced by "Apple Silicon build for macOS 14 or later", and the release notes no longer carry
  the Intel caveat.
- `rust-engine-core`: "Platform independence" now requires the engine to build for the iOS device and
  simulator targets.

## Impact

- **Build**: `scripts/build-xcframework.sh` (three arm64 slices, no `lipo`), `macos/project.yml`
  (`ARCHS: arm64`), `macos/BengaliIMECore/Package.swift` (iOS 17 platform), new `ios/project.yml`
  and `ios/Makefile`. Developers need `rustup target add aarch64-apple-ios aarch64-apple-ios-sim`
  and no longer need `x86_64-apple-darwin`.
- **Swift**: new `KeyboardSession.swift`, `KeyboardLayout.swift` and tests in `BengaliIMECore`;
  `Settings.swift` moves from `macos/Druti` into the package (public API, same behaviour); new
  `ios/Druti` (SwiftUI) and `ios/DrutiKeyboard` (UIKit).
- **CI**: the `macos` job lints, tests and builds both apps, runs the package tests on an iOS
  Simulator, and checks that all binaries are arm64 only. The release workflow drops Rosetta.
- **Users**: Intel Mac users can no longer install new releases; the Universal releases already on
  GitHub Releases keep working for them.
