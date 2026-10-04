# Druti

Phonetic **roman-to-Bengali** typing: you type roman letters (Avro-style) and get Bengali Unicode
text. This is transliteration, not English-to-Bengali translation: `ami banglay gan gai` →
`আমি বাংলায় গান গাই`.

**[Try it in your browser →](https://ahamed.github.io/druti-ime/)** The playground runs the same
engine as the Mac app, compiled to WebAssembly.

## Download for macOS

**Druti** is a Mac input source that types Bengali in any app. It needs macOS 14 or later, on Apple
Silicon or Intel.

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

Consonants typed together join with a hasant (`prothom` → `প্রথম`), with two exceptions you can hear,
and `o` always keeps them apart (`korote` → `করতে`):

- **A single `r` never joins the next consonant:** `korte` → `করতে`, `dorkar` → `দরকার`. Type
  **`rr` for reph**: the second `r` shows the hasant at once (`korr` → `কর্`), and the next
  consonant completes it (`korrta` → `কর্তা`). A vowel after `rr` cancels the reph (`korra` →
  `করা`), except `i`, which makes ঋ (`rriN` → `ঋণ`, `krriShi` → `কৃষি`).
- **Nothing joins after a breathy letter** (খ ঘ ছ ঝ ঠ ঢ থ ধ ফ ভ), হ, ড়, ঢ় or য়: `dekhte` → `দেখতে`,
  `jayga` → `জায়গা`. The ফলা keys `r`, `l`, `m`, `n`, `N`, `w` and `y` still join (`cihno` →
  `চিহ্ন`, `dhwoni` → `ধ্বনি`); `b` doesn't (`dekhbe` → `দেখবে`).
- **`y`** after a consonant is য-ফলা. After a single `r` it stays visible as `র‍্য` (`poryonto` →
  `পর‍্যন্ত`); reph over য is typed `rry` (`porryonto` → `পর্যন্ত`). **`z`** is the letter য
  (`porzonto` → `পরযন্ত`).

Three options can be turned off in the Mac input menu and in the playground: Bengali digits,
দাঁড়ি for `.`, and smart quotes. A fourth, Autocorrect, is off until you turn it on.

### Autocorrect

Many words have a vowel that is written but not said: আমরা is said "amra", একটা "ekTa". Typed
that way, the engine joins the consonants (`amra` → `আম্রা`), and you type the `o` to keep them
apart (`amora` → `আমরা`). With **Autocorrect** on, you can leave it out in common words:

- When a word ends (a space, punctuation, a digit or Enter), Druti looks it up in a fixed list of
  about 1,300 common words and, if it's there, removes the extra hasant: `amra` + space → `আমরা `,
  `amra ekTa jinis dekhte cai` → `আমরা একটা জিনিস দেখতে চাই`.
- It only ever removes a hasant, so the result is exactly what typing the `o` gives. It never adds a
  conjunct or a reph, and it leaves real words alone: `aste` stays `আস্তে`.
- **Backspace** right after the correction undoes it (`আম্রা` again), and that word is left as you
  typed it. Enter commits the corrected word at once.
- The engine itself doesn't change: with Autocorrect off, or for any word not in the list, the same
  keys give the same text as always. The list changes only with a new version of Druti.

Convert selection and the playground's converter apply it too when it's on.

## Repository

| Path                                          | What it is                                                                                                                                           |
| --------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`crates/druti-core`](crates/druti-core/)     | The engine, in Rust: keystroke rules, the pending/commit composer, bulk conversion. Its behaviour is pinned by golden fixtures in `tests/fixtures/`. |
| [`crates/druti-ffi`](crates/druti-ffi/)       | UniFFI bindings for Swift, used by the Mac app.                                                                                                      |
| [`crates/druti-wasm`](crates/druti-wasm/)     | wasm-bindgen bindings for JavaScript, used by the playground.                                                                                        |
| [`examples/playground`](examples/playground/) | The web playground (Vite + TypeScript), deployed to GitHub Pages from `main`.                                                                        |
| [`macos`](macos/)                             | The Druti input source (Swift, AppKit, InputMethodKit).                                                                                              |
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

The Mac app: see [macos/README.md](macos/README.md).

To change how something types, change `druti-core` and update the affected entries in
`crates/druti-core/tests/fixtures/` in the same commit (see its
[README](crates/druti-core/tests/fixtures/README.md)).

## License

[MIT](LICENSE), except the Autocorrect word list.

The Autocorrect word list is derived from the Dakshina dataset (Google Research, CC BY-SA 4.0) and
from FrequencyWords by Hermit Dave (built from OpenSubtitles, CC BY-SA 4.0). It is licensed
[CC BY-SA 4.0](crates/druti-core/data/LICENSE-DATA). The rest of Druti is MIT.
