## Context

- The Mac input source is a thin Swift host over the Rust `Composer` (UniFFI). It shows the word
  being typed as marked text and never changes committed text (whole-word-pending D1 and D2). Its
  AppKit-free helpers live in the `BengaliIMECore` Swift package.
- `scripts/build-xcframework.sh` built `aarch64-apple-darwin` and `x86_64-apple-darwin` and merged
  them with `lipo` into one `macos-arm64_x86_64` slice. CI ran the Rust and Swift tests again as
  x86_64 under Rosetta. Intel was never tested on real hardware.
- No paid Apple Developer account: no App Store, TestFlight, App Groups or notarization.
- A custom iOS keyboard is an app extension (`UIInputViewController`) inside an app. It reaches the
  text field only through `UITextDocumentProxy`: `insertText`, `deleteBackward`,
  `setMarkedText(_:selectedRange:)`, `unmarkText`, and `documentContextBeforeInput` (the text
  before the caret, often only the current sentence). It never learns the caret offset. It hears
  about changes it didn't make through `textWillChange` / `textDidChange`.

## Goals / Non-Goals

**Goals:**
- Druti on iPhone and iPad with the same output as the Mac for the same keys.
- The iOS-specific decisions (how a composer update becomes proxy calls, caret moves, Shift and
  layers) are unit tested without a device.
- One build of the engine for both platforms, Apple silicon only.

**Non-Goals:**
- App Store, TestFlight or a downloadable IPA (all need a paid account or a sideloading tool).
- Word suggestions, autocorrect, swipe typing, a Bengali-letter layout, key sounds and haptics
  (those need Full Access).
- Sharing settings between the Mac and iOS, or between the iOS app and its keyboard.
- An iPad-specific layout beyond the scaled iPhone one.

## Decisions

### D1. Apple silicon only
Every Apple binary is arm64: `ARCHS: arm64` in both XcodeGen specs, and the XCFramework has
exactly three slices, `macos-arm64`, `ios-arm64` and `ios-arm64-simulator`, one static library each
(no `lipo`). CI checks `lipo -archs` prints only `arm64` for each library and app binary. The
Rosetta steps are gone from CI and from the release workflow.

The bindings are generated from the `aarch64-apple-darwin` dylib on any Mac: UniFFI's library mode
reads metadata from the file without loading it, and the interface is the same for every target.
- *Alternative:* keep an x86_64 simulator slice for Intel Macs running Xcode. Rejected: the
  development Mac is Apple silicon, and the point is to drop x86_64 everywhere.

### D2. One Swift package for both platforms
`macos/BengaliIMECore` declares macOS 14 and iOS 17 and is used by both apps; `ios/project.yml`
refers to it as `../macos/BengaliIMECore`. It stays in `macos/` to avoid moving the package and
every path that names it; the iOS docs point to it.

New in the package, free of UIKit:
- `TextDocument`, a `@MainActor` protocol with the five proxy operations the keyboard uses.
- `KeyboardSession`, which owns a `Composer` and turns its updates into `TextDocument` calls:
  `commit` replaces the marked text (`setMarkedText(commit)` then `unmarkText()`, or `insertText`
  when nothing is marked), then `pending` becomes the marked text. This is the macOS `apply` with
  the proxy's operations.
- `KeyboardState` with `KeyboardKey`, `KeyboardLayer` and `ShiftState`: which keys are on screen,
  and how Shift and the layers change.

Tests use a fake document that keeps committed text and marked text apart and fails the test if
`insertText` or `deleteBackward` is called while text is marked. They run with `swift test` on
macOS and with `xcodebuild test` on an iOS Simulator.

### D3. Caret moves without a caret position
macOS compares the caret offset with where the last update left it (input-source D9). A keyboard
has no offset, so the session uses the text instead:
- **Nothing pending, before a key:** if `documentContextBeforeInput` (clipped to the paragraph) is
  not consistent with the composer's output (`matchesTextBeforeCaret`, whole-word-pending D4), the
  caret moved or the app changed the text: reset, so the key starts a new word. Vowels and the
  other context keys then read that text, as on macOS.
- **Word pending, on `textDidChange`:** if the text before the caret no longer ends with the
  pending word, the app ended the composition (a tap elsewhere keeps marked text as normal text in
  UIKit), so the session forgets the word without writing. If it still ends with the word, the
  change was the keyboard's own and nothing happens.
- Apps that expose no text (`nil`) never cause a reset, like apps without text access on macOS.

This assumes `documentContextBeforeInput` includes the marked text. Task 4.3 checks it on a
device; if it doesn't, the pending check compares against the text before the marked text instead.

### D4. Keys are drawn by the extension, in code
`KeyboardView` builds rows of `UIButton`s from `KeyboardState.rows(showsNextKeyboard:)`, with widths
in tenths of the row like the system keyboard, and the system keyboard's light and dark colours. It
redraws only when the state, the globe key or the appearance changes, so a held Backspace keeps its
button while it repeats (a `Task` that sleeps 500 ms, then 100 ms, until the touch ends). The globe
key is wired to `handleInputModeList(from:with:)` for all touch events, as Apple requires. Buttons
use target-action, which is main-actor safe in Swift 6.
- *Alternative:* SwiftUI for the keys. Rejected: touch-down handling for Backspace repeat and the
  globe key's touch forwarding are simpler and more predictable in UIKit, and `UIInputViewController`
  is UIKit anyway. The container app, which is a plain form, uses SwiftUI.

### D5. Options live in the keyboard
The keyboard can't read the container app's preferences without an App Group, which a free
account can't use, so the toggles are in a panel the gear key opens, stored in the extension's own
`UserDefaults`. `Settings` moves into `BengaliIMECore` so the Mac input menu and the iOS panel use
the same keys and defaults; each process has its own domain. Every toggle defaults to on, Autocorrect
included (the author's decision, amending autocorrect design D6); the engine's own default stays
off, so the fixtures and the playground don't change.

### D6. Build and install from source with any Apple account
`make -C ios project` builds the engine and generates `ios/Druti.xcodeproj`. The developer picks
their team in Xcode and runs on a device. Bundle IDs come from a `DRUTI_BUNDLE_ID_PREFIX` build
setting (default `com.ahamed`), so someone else changes one value. The keyboard uses Xcode's
generated Info.plist merged with a small file holding only the `NSExtension` dictionary
(`com.apple.keyboard-service`, `PrimaryLanguage` `bn`, `RequestsOpenAccess` false). CI builds for
`generic/platform=iOS` with signing off, and checks the bundle.

### D7. `.swift-format` at the repository root
swift-format finds its configuration by walking up from each file, so one root file serves
`macos/` and `ios/`.

## Risks / Trade-offs

- [Marked text looks different on iOS] → UIKit highlights marked text. It's the documented way for
  keyboards to compose and matches CJK keyboards; the README says the word being typed is
  highlighted.
- [Apps that handle keyboard marked text badly] → Some web views or custom editors may mishandle
  `setMarkedText`. Task 4.4 records results per app, as `macos/COMPATIBILITY.md` does for the Mac.
- [D3's assumption about the context] → Verified on a device in task 4.3 before the change is
  archived.
- [Intel users lose updates] → Said in the README, the Read Me and the release notes; old
  Universal releases stay downloadable.
- [No compiler on Linux] → The Swift code was written on Linux; the first compile is CI's macOS job
  or the author's Mac (tasks in group 3).
