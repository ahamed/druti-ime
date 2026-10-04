# CLAUDE.md

Druti is a phonetic roman-to-Bengali typing engine. Rust is the only implementation of the
algorithm (`crates/druti-core`). Swift (the macOS input method in `macos/`, the iOS keyboard in
`ios/`) and TypeScript (the playground in `examples/playground/`) are thin hosts over it. The Apple
hosts run on Apple silicon only (arm64): no Intel Macs, no Rosetta, no x86_64 simulator. See `README.md` for the layout and
`openspec/` for specs and design history.

## Commands

| What | Command |
|---|---|
| Rust: format, lint, test, docs | `cargo fmt --all`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all`, `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --workspace` |
| Rust: WASM lint | `cargo clippy -p druti-wasm --target wasm32-unknown-unknown -- -D warnings` |
| Rust: MSRV | `cargo +1.91 check --workspace --all-targets --all-features` |
| Swift: format, lint | `make -C macos format`, `make -C macos lint` |
| Swift: test (builds the Rust core first) | `make -C macos test` (macOS), `make -C ios test` (iOS Simulator) |
| Swift: build the apps | `make -C macos app`, `make -C ios app` |
| Swift: format, lint (iOS) | `make -C ios format`, `make -C ios lint` |

CI (`.github/workflows/ci.yml`) runs all of these. Run the relevant ones before calling a change
done; a change is not finished while any of them fails or warns.

## Rules that span both languages

- **Behaviour lives in Rust.** Bengali logic goes in `druti-core`, where it is tested on any OS.
  Swift and TypeScript only route keys, read and write host text, and show UI. If a host needs a
  decision, expose it from Rust (as `key_reads_document` is) rather than re-implementing it.
- **Behaviour is pinned by golden fixtures** in `crates/druti-core/tests/fixtures/`. A change that
  alters typing output updates the affected fixtures in the same commit; a refactor changes none.
- **Lengths across the FFI and WASM boundaries are UTF-16 code units** (`NSRange`, JavaScript
  strings). Name them as such (`chars_back`, `last_letter_len_utf16`, `_utf16`), never as "characters".
- **The binding surfaces mirror each other.** `druti-ffi` (UniFFI, Swift) and `druti-wasm`
  (wasm-bindgen, JavaScript) expose the same types and functions with the same docs. Change both
  together; each has a test that it matches `druti_core`.
- **Nukta letters and other normalization-sensitive text are written as `\u{...}` escapes** in
  code (ড় U+09DC, ঢ় U+09DD, য় U+09DF), with the glyph in a comment. Editors and tools may
  otherwise decompose them.
- **Comments explain why, and name the spec or design decision** they come from ("design D9",
  "macos-input-source spec"). Short, plain sentences. Don't narrate what the code already says.
- **Never swallow a failure silently.** Every `catch_unwind`, `try?`, or ignored `Result` has a
  comment saying why it is safe (see `druti-ffi`'s `guarded`: the key passes through to the app).

## Rust

Follows the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/), rustfmt, and
clippy `pedantic`. The workspace, not each crate, owns the policy.

### Toolchain and manifests

- Edition **2024**, resolver 3, `rust-version = "1.91"` (the MSRV; CI checks it). Raise the MSRV
  only on purpose, in its own commit, with the reason in the `Cargo.toml` comment.
- Shared dependencies are declared once in `[workspace.dependencies]`; crates use
  `dep.workspace = true`. Binding generators (`uniffi`, `wasm-bindgen`) and Unicode tables are
  pinned with `=` because their output ships in the app; upgrading one is a deliberate commit.
- Every crate has `[lints] workspace = true`. Lint levels live in the root `Cargo.toml`
  (`[workspace.lints]`); `clippy.toml` holds clippy configuration only.
- `Cargo.lock` is committed. Don't add a dependency for something the standard library does.

### Formatting and lints

- `cargo fmt` with default rustfmt settings (100 columns, 2024 style edition). No `rustfmt.toml`.
- Clippy `pedantic` is on, minus `must_use_candidate`, `missing_errors_doc` and
  `module_name_repetitions`. CI denies all warnings.
- To silence a lint, use `#[expect(lint, reason = "...")]` on the narrowest item, never a bare
  `#[allow]` (`allow_attributes_without_reason` is on). `expect` fails once the lint no longer
  fires, so stale suppressions get removed.
- No `dbg!`, `todo!`, `unimplemented!` in committed code.
- No `unsafe` in `druti-core`. The binding crates contain only what their macros generate.

### API and documentation

- Every public item has a doc comment (`missing_docs` is on). The first line is one sentence
  saying what it is or does; later paragraphs give the contract (units, ordering, when it
  panics). Link items with ``[`Name`]``; rustdoc runs with `-D warnings`, so broken links fail.
- Crate and module docs (`//!`) say what the module is for and how it fits the others.
- Keep the public surface small: modules are private and re-exported from `lib.rs`; helpers are
  `pub(crate)` or private. `data` is public because the fixture tests compare its tables.
- Types derive what they can: `Debug` always (`missing_debug_implementations` is on), then
  `Clone`, `Copy`, `PartialEq`, `Eq`, `Default` where they make sense.
- Constructors: `new()` for the obvious default, `with_*` for variants, `Default` alongside a
  no-argument `new`. Conversions between core and binding types use `From`.
- Take `&str` / `&[T]` / `Option<&str>` rather than owned values unless the function stores them.
  Binding crates are the exception (UniFFI and wasm-bindgen pass owned values): suppress
  `needless_pass_by_value` there with `#[expect]` and a reason.

### Correctness

- No `as` casts between integer types that can truncate or change sign. Use `From`,
  `try_from` (with a handled or `expect`ed error), or `saturating_*` arithmetic. `u32::try_from(x)
  .unwrap_or(u32::MAX)` is the pattern for lengths crossing the FFI.
- No `unwrap()` in library code. Use `?`, `let ... else`, or `expect("why this cannot fail")`.
  `debug_assert!` documents invariants that the fixtures prove.
- Library code does not panic on host input. The FFI layer still catches panics and resets, so a
  bug can never stop typing, but that is a safety net, not error handling.
- Prefer iterators and slice methods (`ends_with`, `split_last`) over index arithmetic. Slice by
  UTF-16 units only where a host range requires it, and cut at code point boundaries.

### Tests

- Behaviour tests are data: add a case to the JSON fixtures in `tests/fixtures/` (see its
  README). Write Rust unit tests for things fixtures can't express, such as binding round trips.
- Test names are sentences about behaviour (`config_changes_apply_from_the_next_key`).
- `unwrap`/`expect` and `panic!` are fine in tests; failure messages name the case and the step.

## Swift

Follows the [Swift API Design Guidelines](https://www.swift.org/documentation/api-design-guidelines/),
Swift 6 strict concurrency, and Apple's `swift-format` (bundled with Xcode 16 and later).

### Toolchain and project

- **Swift 6 language mode** everywhere: `swift-tools-version:6.0` in `Package.swift` and
  `SWIFT_VERSION: "6.0"` in `project.yml`. The build has zero warnings; treat a new one as an
  error.
- Deployment targets macOS 14 and iOS 17; arm64 only. Check availability before using newer
  APIs (for example `Synchronization.Mutex` needs macOS 15 and iOS 18).
- `macos/project.yml` and `ios/project.yml` (XcodeGen) are each project's source of truth; the `.xcodeproj` is generated and
  not committed. Build through the Makefile.
- `BengaliIMECore` (in `macos/`, shared by both apps) holds the generated UniFFI bindings plus
  logic free of AppKit and UIKit, so it is unit tested with `swift test` and on the iOS Simulator.
  Put anything testable there (the iOS `KeyboardSession` and `KeyboardState` are), not in an app
  target.
- Never edit `Sources/BengaliIMECore/Generated/`; change the Rust side and run `make -C macos core`.

### Formatting and style

- `swift-format` with the repo's root `.swift-format`: 4-space indent, 100 columns, ordered imports,
  no force unwraps (`!`) or `try!`, early exits with `guard`, documented public declarations.
  `make -C macos format` / `make -C ios format` rewrite; the matching `lint` targets (run in CI
  with `--strict`) check.
- Suppress a rule only with `// swift-format-ignore: RuleName` on the line above, plus a comment
  saying why.
- Implicitly unwrapped parameters (`Any!`, `NSEvent!`) appear only where an `override` of an
  Objective-C API requires them. Unwrap them with `guard let` or pass them straight through;
  never use them as if they were non-optional.

### Naming and API design

- Names read as English phrases at the call site: `ClientText.beforeCaret(of: client)`,
  `Installer.install(from: url)`. Argument labels for prepositions; no redundant type names.
- `UpperCamelCase` types and protocols, `lowerCamelCase` everything else, including acronyms
  (`bundleID`, `inputModeID`, `utf16`).
- Namespaces of static functions are caseless `enum`s (`DocumentText`, `KeyRouting`, `Settings`).
- Types are `final class` unless designed for subclassing; prefer `struct` and `enum` for values.
- Access control is as tight as possible: `private` by default, `internal` for the module,
  `public` only for what the app imports from `BengaliIMECore`.
- Every public declaration has a `///` doc comment starting with a one-sentence summary.

### Concurrency

- Public value types declare `Sendable` explicitly (`KeyPress`, `KeyAction`); the compiler does
  not infer it across modules.
- UI code (AppKit alerts, `NSApplication`) is `@MainActor`: whole types where possible
  (`InstallerUI`), single methods where the type is a nonisolated Objective-C subclass
  (`InputController.uninstall`). InputMethodKit carries no actor annotations but calls on the
  main thread.
- No mutable global or static state. Shared state has one owner: `Settings` reads UserDefaults
  on every access instead of caching; the Rust `Composer` guards its own state with a mutex.
- No `@unchecked Sendable`, `nonisolated(unsafe)` or `@preconcurrency` in hand-written code
  without a comment proving why it is safe.

### Errors and logging

- Throw error types conforming to `LocalizedError` with a message a user can act on
  (`Installer.Failure`). Use `try?` only where failure is expected and harmless, and say so.
- Log with `os.Logger` (`Log.input`). Interpolated values are private by default; mark only
  non-personal values `privacy: .public`. Never log typed text.
- Command-line tools (`Tools/*.swift`) exit through one `fail(_:)` helper that writes to stderr.

### Tests

- New tests use **Swift Testing** (`import Testing`, `@Suite`, `@Test`, `#expect`), not XCTest.
- Parameterize over inputs with `@Test(arguments:)` instead of loops inside one test.
- Test names describe behaviour (`copiesElsewhereBecomeTheInstaller`). Tests must be independent:
  each creates its own `Composer`.
- CI runs the `BengaliIMECore` tests on macOS and on the iOS Simulator; don't depend on the
  platform. Code that touches the iOS text field goes behind `TextDocument`, so a fake can test it.
