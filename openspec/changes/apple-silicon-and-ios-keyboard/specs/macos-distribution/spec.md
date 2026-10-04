## ADDED Requirements

### Requirement: Apple Silicon build for macOS 14 or later
The released `Druti.app` SHALL contain native code for Apple silicon (arm64) only, and SHALL declare
macOS 14 as its minimum system version. CI SHALL check that the app's executable and the engine
library are arm64 only. CI SHALL NOT build or test for x86_64 or under Rosetta.

#### Scenario: Inspecting a release build
- **WHEN** `lipo -archs` is run on the released app's executable
- **THEN** it prints `arm64` and nothing else

#### Scenario: Opening on an Intel Mac
- **WHEN** someone opens the released `Druti.app` on an Intel Mac
- **THEN** macOS refuses to open it, and the README, the DMG's Read Me and the release notes have said that Druti needs a Mac with Apple silicon

## MODIFIED Requirements

### Requirement: Tag-triggered draft release
Pushing a `macos-vX.Y.Z` tag SHALL build the Apple silicon app and the DMG on a macOS CI runner from
that tagged commit, and SHALL create a draft GitHub Release for that tag. The release SHALL have these
assets attached: the DMG, a zip of `Druti.app` named `Druti-X.Y.Z.app.zip` that unpacks to the same
signed bundle as the one in the DMG, and a SHA-256 checksum file for each. The workflow SHALL NOT
publish the release; the maintainer publishes it after testing the downloads. The release notes SHALL
include the "Open Anyway" step, say that the DMG is the recommended download and the zip is for users
who prefer it, link to the web playground, and say that Druti needs a Mac with Apple silicon.

#### Scenario: Releasing 1.0.0
- **WHEN** the maintainer pushes the tag `macos-v1.0.0`
- **THEN** a draft release "Druti 1.0.0" appears with `Druti-1.0.0.dmg`, `Druti-1.0.0.dmg.sha256`, `Druti-1.0.0.app.zip` and `Druti-1.0.0.app.zip.sha256`, visible only to maintainers

#### Scenario: Verifying a download
- **WHEN** a user runs `shasum -a 256` on the downloaded DMG or zip
- **THEN** the output matches the matching published `.sha256` file

#### Scenario: Installing from the zip
- **WHEN** a user downloads `Druti-1.0.0.app.zip` through a browser, unzips it and opens `Druti.app`, approving it with "Open Anyway"
- **THEN** the installer mode runs exactly as it does from the DMG and Druti ends up in `~/Library/Input Methods`

## REMOVED Requirements

### Requirement: Universal build for macOS 14 or later
**Reason**: Druti supports Apple silicon only (design D1). The Intel slice was never tested on Intel
hardware, and its Rosetta tests slowed every CI run.
**Migration**: Intel Mac users keep the Universal releases already published on GitHub Releases.
Replaced by "Apple Silicon build for macOS 14 or later".
