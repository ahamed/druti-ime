# Spec Delta

## ADDED Requirements

### Requirement: Autocorrect toggle
The input menu SHALL offer a checkable Autocorrect item, off on a new install, stored like the other
toggles and kept across upgrades. Convert selection SHALL apply it like the other toggles.

#### Scenario: Turning on Autocorrect
- **WHEN** the author checks Autocorrect and types `amra` then space
- **THEN** `আমরা ` is inserted, and the setting is still on after logging out and back in

#### Scenario: Upgrade keeps it off
- **WHEN** Druti 1.x is upgraded to a version with Autocorrect
- **THEN** Autocorrect is off and the other toggles are unchanged
