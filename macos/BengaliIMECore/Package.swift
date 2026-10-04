// swift-tools-version:6.0
// The Rust engine for Swift: the UniFFI bindings plus small host helpers that
// need neither AppKit nor UIKit, so they can be unit-tested with `swift test`.
// The macOS input source (../Druti) and the iOS keyboard (../../ios) both use it.
//
// BengaliIMEFFI.xcframework and Sources/BengaliIMECore/Generated/ are build
// products of scripts/build-xcframework.sh (run `make -C macos core` first).
import PackageDescription

let package = Package(
    name: "BengaliIMECore",
    platforms: [.macOS(.v14), .iOS(.v17)],
    products: [
        .library(name: "BengaliIMECore", targets: ["BengaliIMECore"])
    ],
    targets: [
        .binaryTarget(name: "BengaliIMEFFI", path: "BengaliIMEFFI.xcframework"),
        .target(name: "BengaliIMECore", dependencies: ["BengaliIMEFFI"]),
        .testTarget(name: "BengaliIMECoreTests", dependencies: ["BengaliIMECore"]),
    ]
)
