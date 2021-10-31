use std::fmt::Display;

use unicode_segmentation::UnicodeSegmentation;

use super::parser::CursorPos;

pub struct UmSyntaxError {
    pub start_pos: CursorPos,
    pub current_pos: CursorPos,
    pub start_line: String,
    pub current_line: String,
}

impl UmSyntaxError {
    pub fn extract_lines(
        content: &[&str],
        start_pos: &CursorPos,
        current_pos: &CursorPos,
    ) -> (String, String) {
        let start_line = if let Some(line) = content.get(start_pos.line) {
            line
        } else {
            "Invalid line access!"
        };

        let current_line = if let Some(line) = content.get(current_pos.line) {
            line
        } else {
            "Invalid line access!"
        };

        (String::from(start_line), String::from(current_line))
    }
}

impl Display for UmSyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start_line_number = self.start_pos.line;
        let start_symbol = self.start_pos.symbol;

        f.write_fmt(format_args!("\nSyntax Error: "))?;

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

        let curr_line_number = self.current_pos.line;
        let curr_symbol = self.current_pos.symbol;

        f.write_fmt(format_args!("\nCurrent reading point: \n\n"))?;

        let info = format!("{}:{} => ", curr_line_number + 1, curr_symbol + 1);

        f.write_fmt(format_args!("{}{}\n", info, self.current_line))?;

        let skip_length = self
            .current_line
            .graphemes(true)
            .collect::<Vec<&str>>()
            .iter()
            .take(curr_symbol)
            .map(|&word| word.len())
            .sum::<usize>()
            + info.len();

        f.write_fmt(format_args!("{}^\n", " ".repeat(skip_length)))?;

        Ok(())
    }
}
