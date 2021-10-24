use crate::um_elements::heading_block::{HeadingBlock, HeadingLevel};

/// CursorPos struct is used to keep track of the cursor position while parsing input.
/// Additionally it is useful as a way to show better error messages. Using CursorPos it
/// is possible to show exact location in the input, where parsing error is recognized.
#[derive(Copy, Clone)]
pub struct CursorPos {
    /// Index of the line in the given input
    pub line: usize,
    /// index of the symbol in the given line
    pub symbol: usize,
}

pub fn parse_heading(input: &[&str], index: &mut usize) -> HeadingBlock {
    let mut content: String = "".into();

    let mut level = HeadingLevel::Invalid;
    let mut level_counter = 0;
    let mut level_set = false;

    while let Some(word) = input.get(*index) {
        if word.matches('#').count() > 0 && !level_set {
            level_counter += 1;
        } else {
            level_set = true;
            level = HeadingLevel::from(level_counter);
        }

        if level_set {
            // read content
            content.push_str(word);
        }

        *index += 1;
    }

    HeadingBlock {
        level,
        content: content.trim().to_string(),
    }
}
