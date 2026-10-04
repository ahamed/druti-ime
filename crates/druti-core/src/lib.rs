//! Phonetic roman-to-Bengali transliteration engine.
//!
//! See `README.md` for the API overview. This crate is the only
//! implementation of Druti's algorithm: the macOS input method uses it through
//! `druti-ffi`, and the web playground through `druti-wasm`. Its behaviour is
//! pinned by the golden fixtures in `tests/fixtures/`.

mod autocorrect;
mod composer;
pub mod data;
mod engine;
mod letters;
mod rules;
mod transpile;
mod vowel_attach;

pub use composer::{Composer, Update};
pub use engine::{Action, Config, Engine};
pub use transpile::{transpile_roman_document, transpile_roman_document_with_config};
pub use vowel_attach::{ends_with_consonant_and_chandrabindu, ends_with_kar_taking_consonant};
