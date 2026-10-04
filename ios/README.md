# Druti for iPhone and iPad

A keyboard that types Bengali phonetically in any app, with the same Rust engine and the same rules
as the [Mac input source](../macos/README.md) and the [web playground](https://ahamed.github.io/druti-ime/).
You type roman letters and the Bengali appears right away: `k` shows `ক`, `h` turns it into `খ`, and
`u` makes `খু`.

It needs iOS 17 or later. Like the Mac app it runs on Apple silicon only: the app is arm64, and the
simulator build needs an Apple Silicon Mac.

## Install on your iPhone or iPad

There's no App Store release: that needs a paid Apple Developer account. You build Druti on your
Mac and install it with your own Apple account instead.

1. Set up the Mac once: see [Prerequisites](#prerequisites-once).
2. Run `make -C ios project`. It builds the Rust engine and writes `ios/Druti.xcodeproj`.
3. Open `ios/Druti.xcodeproj` in Xcode. For both the **Druti** and **DrutiKeyboard** targets, open
   **Signing & Capabilities** and choose your team (sign in under Xcode → Settings → Accounts
   first; a free Apple account works).
4. If Xcode says the bundle identifier isn't available, change `DRUTI_BUNDLE_ID_PREFIX` (project →
   Build Settings → User-Defined) from `com.ahamed` to something of your own, such as
   `com.yourname`.
5. Connect the iPhone or iPad, choose it as the run destination and press Run (⌘R). The first
   time, turn on Developer Mode on the device (Settings → Privacy & Security → Developer Mode) and
   trust your developer certificate (Settings → General → VPN & Device Management).
6. On the device: **Settings → Druti → Keyboards** and turn on **Druti**. (Or Settings → General →
   Keyboard → Keyboards → Add New Keyboard… → Druti.)
7. In any app, touch and hold 🌐 (or the emoji key) and choose **Druti**.

With a free Apple account the install works for 7 days; after that, run it from Xcode again. Your
settings are kept.

Druti doesn't ask for Full Access, so it has no network access, and what you type stays on the
device. iOS switches to its own keyboard in password fields.

## Typing

The phonetic rules are the same as on the Mac. See the [main README](../README.md#how-typing-works).

| You tap | You get |
|---|---|
| Letters | Bengali, shown right away. The word you're typing is still pending and may change with the next key (`ক` becomes `খ` after `h`). Apps highlight it, as they do for other languages that compose words. |
| ⇧ Shift | The next letter in upper case, which matters for the rules (`t` is ত, `T` is ট). Double-tap for Caps Lock. Shift never turns itself on. |
| Space | Ends the word and commits it. |
| return | Commits the word, then types a line break. |
| ⌫ Delete | In the word you're typing, deletes one letter (`দ্ম` becomes `দ`, `করতে` becomes `করত`). In text that is already final, one character. Hold it to repeat. |
| 123, #+= | Digits and punctuation: `^` for chandrabindu, `:` for bisarga, `.` for `।`. Space takes you back to the letters. |
| ⚙ | The options: Bengali digits, দাঁড়ি (।) for full stop, and smart quotes, all on by default. They are remembered. |
| 🌐 | The next keyboard (only shown when iOS asks for it). |

Tapping somewhere else in the text keeps the word you were typing as it is. In apps that let
keyboards read their text, a vowel typed right after an existing consonant becomes a kar: put the
caret after `ক` and type `i` to get `কি`.

## Build from source

For developing Druti.

### Prerequisites (once)

1. A Mac with Apple Silicon.
2. **Xcode** from the App Store, with the iOS platform (Xcode → Settings → Components). Open it once
   to accept the license, then run `sudo xcode-select -s /Applications/Xcode.app`.
3. **Rust**: install from <https://rustup.rs>, then run
   `rustup target add aarch64-apple-darwin aarch64-apple-ios aarch64-apple-ios-sim`.
4. **XcodeGen**: `brew install xcodegen`.

### Commands

| Command | What it does |
|---|---|
| `make -C ios project` | Builds the Rust engine and generates `Druti.xcodeproj`. |
| `make -C ios test` | Runs the `BengaliIMECore` tests (bindings, the keyboard session and layout) on an iOS Simulator. `SIMULATOR_ID=<udid>` picks the simulator. |
| `make -C ios app` | Builds the app and keyboard for iOS devices without signing (CI runs this). |
| `make -C ios lint` / `format` | Checks / rewrites the Swift sources with `swift-format` (rules in the repo's root `.swift-format`). CI runs `lint`. |
| `make -C ios version` | Prints the app version, `MARKETING_VERSION` in `project.yml`. |
| `make -C ios clean` | Removes the build products. |

Layout:
- `Druti/` is the app that carries the keyboard (SwiftUI). It shows how to turn the keyboard on and
  has a field to try it in.
- `DrutiKeyboard/` is the keyboard extension. `KeyboardViewController.swift` connects the keys to a
  `KeyboardSession` and the session to the app's text field (`UITextDocumentProxy`).
  `KeyboardView.swift` draws the keys and the options panel.
- The typing logic is in the shared Swift package
  [`macos/BengaliIMECore`](../macos/BengaliIMECore/): `KeyboardSession` (the composer and the
  marked text) and `KeyboardState` (layers and Shift), with unit tests that run on macOS and on
  the iOS Simulator. The Bengali rules themselves are in Rust, in [`crates/`](../crates/).
- `project.yml` is the XcodeGen spec. The `.xcodeproj` is generated and not committed.

To debug the keyboard, run the **DrutiKeyboard** scheme from Xcode and pick an app to type in
(Notes or Safari); Xcode then attaches to the keyboard process.
