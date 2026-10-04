# Druti

Phonetic **roman-to-Bengali** typing: you type roman letters (Avro-style) and get Bengali Unicode
text. This is transliteration, not English-to-Bengali translation: `ami banglay gan gai` →
`আমি বাংলায় গান গাই`.

**[Try it in your browser →](https://ahamed.github.io/druti-ime/)** The playground runs the same
engine as the Mac app, compiled to WebAssembly.

## Download for macOS

**Druti** is a Mac input source that types Bengali in any app. It needs macOS 14 or later on a Mac
with Apple Silicon (M1 or later); it doesn't run on Intel Macs.

1. Download the latest **Druti-X.Y.Z.dmg** from [Releases](https://github.com/ahamed/druti-ime/releases/latest)
   and open it. (Or download **Druti-X.Y.Z.app.zip**, unzip it and use the Druti app inside it.)
2. Double-click **Druti**. It installs itself into `~/Library/Input Methods` and turns itself on.
3. If macOS says it can't verify Druti, open **System Settings → Privacy & Security**, scroll down,
   click **Open Anyway** next to Druti, and open Druti again. macOS asks once, because Druti is free
   and isn't notarized by Apple.
4. Choose Druti in the input menu in the menu bar, or press Control-Space (or 🌐) to switch to it.
   Keep ABC or U.S. enabled as well, for passwords.

**Update:** open a newer DMG and double-click Druti again. It keeps your settings.
**Uninstall:** choose **Uninstall Druti…** from Druti's input menu.

More in [macos/README.md](macos/README.md).

## iPhone and iPad

Druti is also an iOS keyboard, with the same engine and the same typing rules. It needs iOS 17 or
later. There's no App Store release yet: you build it on an Apple Silicon Mac with Xcode and
install it on your iPhone or iPad with your own Apple account (a free one works). See
[ios/README.md](ios/README.md).

## How typing works

Each key shows its Bengali right away. The last few characters (the cluster you're still typing)
stay **pending**, shown underlined, because the next key may change them: `k` gives `ক`, then `h`
turns it into `খ`. A space, punctuation, a click or an arrow key makes the pending text final.

**Backspace** removes one letter: a consonant goes together with the hasant that joins it, so `দ্ম`
becomes `দ` (never `দ্`), and a kar goes on its own (`করতে` → `করত` → `কর` → `ক`). What is left
is still being typed: `dm`, Backspace, `h` gives `ধ`. **Placing the caret** after a word picks it
up again: a vowel attaches as a kar (`i` after `র` in `করতে` gives `করিতে`), and a consonant
continues the cluster (`h` after `করত` gives `করথ`).

The same keys always produce the same text. A few rules make sure the output is always well-formed
Bangla:

- **Kars** attach only to a consonant directly before the caret (`Oa` → `ওআ`, `tHa` → `ৎআ`, `k  a` → `ক  আ`).
- **Hasant** is only placed between two consonants that can carry it: `kng` → `কং`, `ktH` → `কৎ`, `tHy` → `ৎয়`.
- **Unmapped keys** (`? ! ; ( ) @ /`, tab, emoji, …) are written as-is and end the cluster: `k?k` → `ক?ক`.
- **`.`** is `।`, except right after a digit (`1.5` → `১.৫`) or another dot; `...` → `...`.
- **`^`** (chandrabindu) may be typed before or after the vowel: `k^a` and `ka^` both give `কাঁ`.

Three options can be turned off in the Mac input menu and in the playground: Bengali digits,
দাঁড়ি for `.`, and smart quotes.

## Repository

| Path                                          | What it is                                                                                                                                           |
| --------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`crates/druti-core`](crates/druti-core/)     | The engine, in Rust: keystroke rules, the pending/commit composer, bulk conversion. Its behaviour is pinned by golden fixtures in `tests/fixtures/`. |
| [`crates/druti-ffi`](crates/druti-ffi/)       | UniFFI bindings for Swift, used by the Mac app and the iOS keyboard.                                                                                 |
| [`crates/druti-wasm`](crates/druti-wasm/)     | wasm-bindgen bindings for JavaScript, used by the playground.                                                                                        |
| [`examples/playground`](examples/playground/) | The web playground (Vite + TypeScript), deployed to GitHub Pages from `main`.                                                                        |
| [`macos`](macos/)                             | The Druti input source (Swift, AppKit, InputMethodKit), and `BengaliIMECore`, the Swift package shared with iOS.                                     |
| [`ios`](ios/)                                 | The Druti keyboard for iPhone and iPad (Swift, UIKit keyboard extension).                                                                            |
| [`openspec`](openspec/)                       | Specs and the design history of each change.                                                                                                         |

## Development

Rust engine:

```sh
cargo test --all
```

Playground (needs `rustup target add wasm32-unknown-unknown`, [wasm-pack](https://rustwasm.github.io/wasm-pack/)
and Node with Yarn 1):

```sh
cd examples/playground
yarn install
yarn dev      # builds the WASM package, then starts Vite
yarn build    # production build into dist/
```

The Mac app: see [macos/README.md](macos/README.md). The iOS keyboard: see [ios/README.md](ios/README.md).

To change how something types, change `druti-core` and update the affected entries in
`crates/druti-core/tests/fixtures/` in the same commit (see its
[README](crates/druti-core/tests/fixtures/README.md)).

## License

[MIT](LICENSE)
