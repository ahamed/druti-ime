# Spec Delta

## ADDED Requirements

### Requirement: Autocorrect toggle
The input menu SHALL offer a checkable Autocorrect item, on until the user turns it off, stored like
the other toggles and kept across upgrades. Convert selection SHALL apply it like the other toggles.

#### Scenario: Turning on Autocorrect
- **WHEN** the author checks Autocorrect and types `amra` then space
- **THEN** `আমরা ` is inserted, and the setting is still on after logging out and back in

#### Scenario: On after an upgrade
- **WHEN** Druti 1.x is upgraded to a version with Autocorrect
- **THEN** Autocorrect is on and the other toggles are unchanged

#### Scenario: Turning it off is remembered
- **WHEN** the author unchecks Autocorrect and later installs a newer version
- **THEN** Autocorrect stays off
