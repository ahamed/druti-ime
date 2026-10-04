#!/usr/bin/env bash
# Builds the Rust engine for Apple Silicon Macs, iPhones and iPads, and the arm64 iOS Simulator,
# and packages it for Swift. Apple silicon only: there are no x86_64 slices (Intel Macs, Rosetta,
# the Intel simulator); see design D1 of the apple-silicon-and-ios-keyboard change.
#
# Outputs (build products, git-ignored):
#   macos/BengaliIMECore/BengaliIMEFFI.xcframework
#       one arm64 static library per platform (macos-arm64, ios-arm64, ios-arm64-simulator)
#       + C header + module map (the `bengali_ime_ffiFFI` module)
#   macos/BengaliIMECore/Sources/BengaliIMECore/Generated/bengali_ime_ffi.swift
#       UniFFI Swift bindings (`Composer`, `Update`, `Config`, ...), the same for every platform
#
# Needs a Mac with Xcode and rustup
# (`rustup target add aarch64-apple-darwin aarch64-apple-ios aarch64-apple-ios-sim`).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PACKAGE="$ROOT/macos/BengaliIMECore"
TARGETS=(aarch64-apple-darwin aarch64-apple-ios aarch64-apple-ios-sim)
# The deployment targets match macos/project.yml and ios/project.yml.
export MACOSX_DEPLOYMENT_TARGET="${MACOSX_DEPLOYMENT_TARGET:-14.0}"
export IPHONEOS_DEPLOYMENT_TARGET="${IPHONEOS_DEPLOYMENT_TARGET:-17.0}"

cd "$ROOT"

for TARGET in "${TARGETS[@]}"; do
  echo "==> Building druti-ffi for $TARGET"
  cargo build --release --package druti-ffi --target "$TARGET"
done

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

echo "==> Generating Swift bindings"
# Library mode reads the interface from the compiled library's metadata without running it, and
# the interface is the same for every target.
cargo run --quiet --release --package druti-ffi --features cli --bin uniffi-bindgen -- \
  generate "$ROOT/target/aarch64-apple-darwin/release/libbengali_ime_ffi.dylib" --language swift --no-format --out-dir "$WORK/bindings"

mkdir -p "$WORK/headers"
cp "$WORK/bindings/bengali_ime_ffiFFI.h" "$WORK/headers/"
# Swift finds a C module through a file named exactly module.modulemap.
cp "$WORK/bindings/bengali_ime_ffiFFI.modulemap" "$WORK/headers/module.modulemap"

echo "==> Creating BengaliIMEFFI.xcframework"
LIBRARIES=()
for TARGET in "${TARGETS[@]}"; do
  LIBRARIES+=(-library "$ROOT/target/$TARGET/release/libbengali_ime_ffi.a" -headers "$WORK/headers")
done
rm -rf "$PACKAGE/BengaliIMEFFI.xcframework"
xcodebuild -create-xcframework "${LIBRARIES[@]}" -output "$PACKAGE/BengaliIMEFFI.xcframework"

mkdir -p "$PACKAGE/Sources/BengaliIMECore/Generated"
cp "$WORK/bindings/bengali_ime_ffi.swift" "$PACKAGE/Sources/BengaliIMECore/Generated/"

for SLICE in macos-arm64 ios-arm64 ios-arm64-simulator; do
  lipo -info "$PACKAGE/BengaliIMEFFI.xcframework/$SLICE/libbengali_ime_ffi.a"
done
echo "==> Done"
