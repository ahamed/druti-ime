## MODIFIED Requirements

### Requirement: Platform independence
The engine SHALL have no dependency on any operating system UI framework or I/O. It SHALL build and pass
its tests on Linux and on macOS, and it SHALL build for `wasm32-unknown-unknown`, `aarch64-apple-ios`
and `aarch64-apple-ios-sim`, so the macOS input source, the iOS keyboard extension and the web
playground all use it.

#### Scenario: Linux test run
- **WHEN** the native test suite runs on a Linux machine
- **THEN** it builds and all golden fixtures pass

#### Scenario: WebAssembly build
- **WHEN** the engine is built for `wasm32-unknown-unknown`
- **THEN** it compiles without platform-specific code

#### Scenario: iOS build
- **WHEN** `druti-ffi` is built for `aarch64-apple-ios` and `aarch64-apple-ios-sim`
- **THEN** it compiles without platform-specific code, and the XCFramework gets one arm64 slice for each
