# Druti for macOS

A macOS input source that types Bengali phonetically in any app. It uses Druti's Rust engine in
[`crates/`](../crates/), the same engine as the [web playground](https://ahamed.github.io/druti-ime/). The Bengali text appears as you type each key: `k` shows `ক`, `h`
turns it into `খ`, and `u` makes `খু`.

It's free and signed ad hoc, so it needs no Apple Developer account, and it installs into your own
`~/Library/Input Methods` without an administrator password. It runs on Apple Silicon and Intel
Macs with macOS 14 or later.

## Install

1. Download the latest **Druti-X.Y.Z.dmg** from
   [Releases](https://github.com/ahamed/druti-ime/releases/latest) and open it. (There's also a
   **Druti-X.Y.Z.app.zip** with the same app: unzip it and open Druti from there instead.)
2. Double-click **Druti**. It copies itself into `~/Library/Input Methods`, turns itself on, and
   tells you when it's done. You can eject the disk image afterwards.
3. If macOS says it can't verify Druti: open **System Settings → Privacy & Security**, scroll down,
   click **Open Anyway** next to Druti, and open Druti again. macOS asks once, because Druti isn't
   notarized by Apple. (Optionally, check the download first with `shasum -a 256`, and compare
   the output with the `.sha256` file on the release page.)
4. Choose Druti in the input menu in the menu bar, or press Control-Space (or the 🌐 key) to switch
   input sources.

If macOS doesn't let Druti turn itself on, the installer says so and offers to open Keyboard
settings. There, go to Text Input → Input Sources → **Edit…** → **+**, pick **Bengali** →
**Druti**, and click **Add**. If it isn't listed, log out and back in once.

**Keep ABC (or U.S.) enabled as well.** You need it for passwords, which macOS always types in
ASCII. You also need it to keep typing if the input method ever misbehaves.

**Updating:** download the newer DMG and double-click Druti again. The next key you type uses the
new version, and your settings are kept.

**Uninstalling:** choose **Uninstall Druti…** from Druti's input menu. It removes Druti from your
input sources, moves it to the Trash and deletes its settings.

Intel Macs haven't been tested on real hardware yet. If you use one, please report whether Druti
works in [Issues](https://github.com/ahamed/druti-ime/issues).

## Typing

The phonetic rules are the same as the web version. See the [main README](../README.md#determinism-rules).

| You press | You get |
|---|---|
| Letters, digits, punctuation | Bengali, shown right away. The word you're typing is still pending and may change with the next key (`ক` becomes `খ` after `h`). |
| Space | Ends the word and commits it. |
| Return | Commits, then the app gets Return, so chat apps send the whole word. |
| Backspace | In the word you're typing, deletes one letter: `দ্ম` becomes `দ`, never `দ্`, and `করতে` becomes `করত`. The same in every app, including Chrome, Cursor and Terminal. A consonant typed next starts a new letter (`ka`, Backspace, `k` gives `কক`). In text that is already final, the app deletes as usual, one character at a time. |
| Arrow keys, Tab, Esc, Home/End, Page Up/Down | Commits, then the app handles the key. |
| ⌘, ⌃ or ⌥ shortcuts | Commits, then the shortcut works as usual. |
| Keys with no Bengali mapping (`? ! ( ) @ /` …) | Typed as-is. |

Some apps draw an underline under the word you're typing. It becomes normal text at the next space,
punctuation mark, click or arrow key.

In apps that let input methods read their text (TextEdit, Notes, Safari, Pages, ...), a vowel typed
right after an existing consonant becomes a kar: click after `ক` and type `i` to get `কি`. Apps
that don't expose their text, like Terminal, give `কই` there, because a vowel after a click starts a
new syllable. Right after typing, `ki` still gives `কি` in every app.

## Input menu

Click the **দ্রু** icon in the menu bar while Druti is active:

- **Bengali Digits (১২৩)**: on by default. Turn it off to type `2024` as ASCII digits.
- **দাঁড়ি (।) for Full Stop**: on by default. Turn it off to type `.` as `.`.
- **Smart Quotes (“ ”)**: on by default. Turn it off for straight `"` and `'`.
- **Autocorrect**: off by default. When it's on, a common word typed without its hidden vowel is
  corrected when the word ends (`amra` + space → আমরা). Backspace right after undoes it. See
  [Autocorrect](../README.md#autocorrect).
- **Convert Selection to Bengali**: replaces the selected roman text with Bengali, keeping line
  breaks and using the settings above. In apps that don't expose the selection, it does nothing.
- **Uninstall Druti…**: asks first, then removes Druti (see [Install](#install)).

The toggles are remembered across restarts and updates.

## Build from source

For developing Druti. Users don't need any of this.

### Prerequisites (once)

1. **Xcode** from the App Store. Open it once to accept the license, then run
   `sudo xcode-select -s /Applications/Xcode.app`.
2. **Rust**: install from <https://rustup.rs>, then run
   `rustup target add aarch64-apple-darwin x86_64-apple-darwin`.
3. **XcodeGen**: `brew install xcodegen`.
4. Only for `make dmg`: **cargo-about** (`cargo install cargo-about --locked --features cli`) and
   **dmgbuild** (`pip3 install -r macos/Packaging/requirements.txt`, for example in a virtualenv).

### Install from source

```sh
make -C macos install
```

This builds the Rust engine and the Universal app, signs it ad hoc, and runs the app's own
installer (`Druti --install`), the same code path as the DMG:
1. Copies the app to `~/Library/Input Methods/Druti.app` and stops any running copy.
2. Registers it with Launch Services and the Text Input system, and enables it.
3. Restarts the menu bar's input menu and System Settings, so the new name and icon show up.

The first build takes a few minutes; later builds are faster. After a code change, run it again:
the next key you type uses the new build, without logging out.

```sh
make -C macos uninstall
```

This runs `Druti --uninstall`: it disables and deletes the installed copy and its settings. It may
stay listed in System Settings until you log out.

### Commands

| Command | What it does |
|---|---|
| `make -C macos install` / `uninstall` | See above. |
| `make -C macos test` | Builds the Rust core and runs the Swift tests (bindings, key routing, context helpers, install location). |
| `make -C macos lint` / `format` | Checks / rewrites the Swift sources with `swift-format` (bundled with Xcode; rules in `.swift-format`). CI runs `lint`. |
| `make -C macos app` | Builds and signs `macos/build/.../Druti.app` without installing it (CI runs this). |
| `make -C macos dmg` | Builds `macos/build/Druti-X.Y.Z.dmg`, the release DMG (needs cargo-about and dmgbuild). |
| `make -C macos app-zip` | Zips the built app into `macos/build/Druti-X.Y.Z.app.zip` with `ditto`, keeping its signature. Run it after `make dmg`. |
| `make -C macos licenses` | Writes `Licenses.txt` (Druti's license and the Rust crates' notices) for the app bundle. Without cargo-about, it lists only Druti's license. |
| `make -C macos version` | Prints the app version, `MARKETING_VERSION` in `project.yml`. |
| `make -C macos project` | Generates `Druti.xcodeproj` for browsing the code in Xcode. |
| `make -C macos clean` | Removes all build products. |

Layout:
- `BengaliIMECore/` is a Swift package. It holds the UniFFI bindings (generated by
  [`scripts/build-xcframework.sh`](../scripts/build-xcframework.sh)) and AppKit-free helpers with
  unit tests.
- `Druti/` is the input method app. `InputController.swift` routes keys and applies the
  composer's updates, `ClientText.swift` reads from the app's text field, `Settings.swift` holds
  the menu toggles, and `Installer.swift` / `InstallerUI.swift` install and uninstall it. Opened
  from anywhere other than an Input Methods folder, the app installs itself instead of running as
  the input method.
- `Packaging/` has the DMG layout (`dmg-settings.py`), the Read Me and release notes templates, and
  the cargo-about config. `Tools/` renders the menu icon and the DMG background.
- `project.yml` is the XcodeGen spec. The `.xcodeproj` is generated and not committed.

Bengali logic belongs in Rust (`crates/druti-core`), where it's tested on any OS. The Swift
side only routes keys and talks to InputMethodKit.

### Debugging

Logs:

```sh
log stream --predicate 'subsystem == "com.ahamed.inputmethod.Druti"' --level debug
```

If typing stops working, switch to ABC, then run `killall Druti`. macOS starts it again on
the next key press.

### Releasing

1. Set `MARKETING_VERSION` in `macos/project.yml` to the new version, for example `1.0.1`, and merge
   it to `main`.
2. Tag that commit and push the tag:
   ```sh
   git tag macos-v1.0.1 && git push origin macos-v1.0.1
   ```
   The [Release macOS](../.github/workflows/release-macos.yml) workflow checks that the tag matches
   the version, runs the tests on both architectures, builds the DMG and the zipped app
   (`make dmg app-zip`) and creates a **draft** release with both and their `.sha256` files.
3. Download the DMG and the zip from the draft with a browser and test them: a fresh install in a
   clean macOS user account, an upgrade over the previous version, and the pass in
   [COMPATIBILITY.md](COMPATIBILITY.md).
4. Write the changes into the draft's notes and publish it.

Never move a published tag. To fix a bad release, delete the draft (or unpublish it) and release
a new patch version.

## Compatibility

Per-app results for the fixed test paragraph are in [COMPATIBILITY.md](COMPATIBILITY.md).
