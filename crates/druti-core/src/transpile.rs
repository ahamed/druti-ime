//! Whole-document conversion: every character is typed as a keystroke.

use crate::autocorrect;
use crate::data::ENTER_KEY;
use crate::engine::{Action, Config, Engine};

/// Converts a whole roman document. With `preserve_line_breaks`, `\n` is typed
/// as the Enter key and a newline is inserted at each paragraph break;
/// otherwise `\n` passes through as a plain character.
pub fn transpile_roman_document(document: &str, preserve_line_breaks: bool) -> String {
    transpile_roman_document_with_config(document, preserve_line_breaks, Config::default())
}

/// [`transpile_roman_document`] with output toggles (the macOS "Convert
/// selection" command applies the current input menu settings). With
/// [`Config::autocorrect`], every word of the result found in the Autocorrect
/// list is corrected, the last one included (autocorrect spec, "Autocorrect
/// in bulk conversion").
pub fn transpile_roman_document_with_config(
    document: &str,
    preserve_line_breaks: bool,
    config: Config,
) -> String {
    let mut engine = Engine::with_config(config);
    let mut break_positions: Vec<usize> = Vec::new();
    let mut key_buffer = [0u8; 4];

    for ch in document.chars() {
        let key: &str = if preserve_line_breaks && ch == '\n' {
            ENTER_KEY
        } else {
            ch.encode_utf8(&mut key_buffer)
        };
        let actions = engine.process(key, None);
        for action in actions {
            if action == Action::SplitBlock {
                break_positions.push(engine.output_len_utf16());
            }
        }
    }

    break_positions.sort_unstable();
    let mut result = engine.output.clone();
    for (offset, position) in break_positions.into_iter().enumerate() {
        result.insert(position + offset, u16::from(b'\n'));
    }
    let result = String::from_utf16_lossy(&result);
    if config.autocorrect {
        autocorrect::correct_words(&result)
    } else {
        result
    }
}
