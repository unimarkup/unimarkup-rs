use std::fmt;

use unicode_segmentation::UnicodeSegmentation;

type CursorPos = (usize, usize);

/// SyntaxError is the [`UmError::Syntax`] variant of [`UmError`].
///
/// [`UmError::Syntax`]: crate::um_error::UmError::Syntax
/// [`UmError`]: crate::um_error::UmError
#[derive(Debug)]
pub struct SyntaxError {
    /// Start position in the Unimarkup document, where [`SyntaxError`] occured.
    pub start_pos: CursorPos,
    /// Position in the Unimarkup document, where [`SyntaxError`] occured.
    pub current_pos: CursorPos,
    /// Start line in the Unimarkup document, where [`SyntaxError`] occured.
    pub start_line: String,
    /// The line in the Unimarkup document, where [`SyntaxError`] occured.
    pub current_line: String,
    /// Custom message to provide additional information about the error.
    pub message: String,
}

impl SyntaxError {
    fn extract_lines(
        content: &[&str],
        start_pos: &CursorPos,
        current_pos: &CursorPos,
    ) -> (String, String) {
        let start_line = if let Some(line) = content.get(start_pos.0) {
            line
        } else {
            "Invalid line access!"
        };

        let current_line = if let Some(line) = content.get(current_pos.0) {
            line
        } else {
            "Invalid line access!"
        };

        (String::from(start_line), String::from(current_line))
    }

    /// Constructs a new [`SyntaxError`]
    ///
    /// # Arguments
    ///
    /// * `content` - Lines of &str from the Unimarkup document
    /// * `start_pos` - Start position in the Unimarkup document, where [`SyntaxError`] occured
    /// * `current_pos` - Position in the Unimarkup document, where [`SyntaxError`] occured
    /// * `message` - Custom message to provide additional information about the error
    pub fn new(
        content: &[&str],
        start_pos: &CursorPos,
        current_pos: &CursorPos,
        message: impl Into<String>,
    ) -> Self {
        let (start_line, current_line) = Self::extract_lines(content, start_pos, current_pos);

        SyntaxError {
            start_pos: *start_pos,
            current_pos: *current_pos,
            start_line,
            current_line,
            message: message.into(),
        }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (start_line_number, start_symbol) = self.start_pos;

        let prefix = "Syntax Error: ";

        f.write_fmt(format_args!("\n{}", prefix))?;
        for (count, line) in self.message.lines().enumerate() {
            let prefix_len = if count > 0 { prefix.len() } else { 0 };

            f.write_fmt(format_args!("{}{}\n", " ".repeat(prefix_len), line))?;
        }

        f.write_fmt(format_args!("\n"))?;

        f.write_fmt(format_args!("Begin reading point: \n\n"))?;

        let info = format!("{}:{} => ", start_line_number + 1, start_symbol + 1);

        f.write_fmt(format_args!("{}{}\n", info, self.start_line))?;

        let skip_length = self
            .start_line
            .graphemes(true)
            .collect::<Vec<&str>>()
            .iter()
            .take(start_symbol)
            .map(|&word| word.len())
            .sum::<usize>()
            + info.len();

        f.write_fmt(format_args!("{}^", " ".repeat(skip_length)))?;

        let (curr_line_number, curr_symbol) = self.current_pos;

        f.write_fmt(format_args!("\nError occured at: \n\n"))?;

        let info = format!("{}:{} => ", curr_line_number + 1, curr_symbol + 1);

        f.write_fmt(format_args!("{}{}\n", info, self.current_line))?;

        let skip_length = self
            .current_line
            .graphemes(true)
            .take(curr_symbol)
            .map(|grapheme| grapheme.len())
            .sum::<usize>()
            + info.len();

        f.write_fmt(format_args!("{}^\n", " ".repeat(skip_length)))?;

        Ok(())
    }
}
