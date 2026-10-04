# Spec Delta

## ADDED Requirements

### Requirement: Autocorrect in the playground
The WASM settings object SHALL include `autocorrect`, false by default, with the same meaning as in
`druti-core`. The playground SHALL show an Autocorrect toggle next to the other settings, applied to
both typing and conversion.

#### Scenario: Toggle in the playground
- **WHEN** Autocorrect is switched on in the playground and `amra ` is typed
- **THEN** the text area shows `আমরা `
