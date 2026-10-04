# Druti for iOS: the app and its keyboard extension.
#
#   make project   build the Rust engine and generate Druti.xcodeproj; open it in Xcode to run
#                  Druti on your iPhone or iPad (see README.md)
#   make app       build for iOS devices without signing (used by CI)
#   make test      run the BengaliIMECore tests on an arm64 iOS Simulator
#   make lint      check Swift formatting and style (swift-format, config in ../.swift-format)
#   make format    rewrite the Swift sources with swift-format
#   make version   print the app's version (MARKETING_VERSION in project.yml)
#   make clean
#
# Needs an Apple Silicon Mac with Xcode, rustup (aarch64-apple-darwin, aarch64-apple-ios,
# aarch64-apple-ios-sim) and XcodeGen. The engine and BengaliIMECore are shared with the macOS
# input source in ../macos.

APP_NAME := Druti
CORE     := ../macos/BengaliIMECore
DERIVED  := build/DerivedData
APP      := $(DERIVED)/Build/Products/Release-iphoneos/$(APP_NAME).app
VERSION  := $(shell sed -n 's/^ *MARKETING_VERSION: *//p' project.yml)
# The first available iPhone simulator. Override with SIMULATOR_ID=<udid>.
SIMULATOR_ID ?= $(shell xcrun simctl list devices available | sed -n 's/^ *iPhone.* (\([0-9A-F-]\{36\}\)).*/\1/p' | head -n 1)

.PHONY: all core project app test lint format version clean

SWIFT_SOURCES := Druti DrutiKeyboard

all: app

# Rust engine → ../macos/BengaliIMECore/BengaliIMEFFI.xcframework (macOS and iOS slices).
core:
	../scripts/build-xcframework.sh

project: core
	xcodegen generate --quiet

app: project
	xcodebuild -project $(APP_NAME).xcodeproj -scheme $(APP_NAME) -configuration Release \
		-derivedDataPath $(DERIVED) -destination 'generic/platform=iOS' \
		CODE_SIGNING_ALLOWED=NO -quiet build

test: core
	@test -n "$(SIMULATOR_ID)" || { echo "No iPhone simulator found: install one in Xcode → Settings → Components"; exit 1; }
	cd $(CORE) && xcodebuild test -scheme BengaliIMECore \
		-destination 'platform=iOS Simulator,id=$(SIMULATOR_ID)' -quiet

lint:
	swift format lint --strict --recursive --parallel $(SWIFT_SOURCES)

format:
	swift format format --in-place --recursive --parallel $(SWIFT_SOURCES)

version:
	@echo $(VERSION)

clean:
	rm -rf build $(APP_NAME).xcodeproj
