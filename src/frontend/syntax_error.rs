use std::fmt::Display;

use unicode_segmentation::UnicodeSegmentation;

use super::parser::CursorPos;

pub struct UmSyntaxError<'a> {
    pub start_pos: CursorPos,
    pub current_pos: CursorPos,
    pub lines: &'a [&'a str],
}

impl Display for UmSyntaxError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start_line = self.start_pos.line;
        let start_symbol = self.start_pos.symbol;

        f.write_fmt(format_args!("\nSyntax Error: "))?;

        if let Some(&line) = self.lines.get(start_line) {
            f.write_fmt(format_args!("Begin reading point: \n\n"))?;

            let info = format!("{}:{} => ", start_line + 1, start_symbol + 1);

            f.write_fmt(format_args!("{}{}\n", info, line))?;

            let skip_length = line
                .split_word_bounds()
                .collect::<Vec<&str>>()
                .iter()
                .take(start_symbol)
                .map(|&word| word.len())
                .sum::<usize>()
                + info.len();

            f.write_fmt(format_args!("{}^", " ".repeat(skip_length)))?;
        }

        let curr_line = self.current_pos.line;
        let curr_symbol = self.current_pos.symbol;

        if let Some(&line) = self.lines.get(curr_line) {
            f.write_fmt(format_args!("\nCurrent reading point: \n\n"))?;

            let info = format!("{}:{} => ", curr_line + 1, curr_symbol + 1);

            f.write_fmt(format_args!("{}{}\n", info, line))?;

            let skip_length = line
                .split_word_bounds()
                .collect::<Vec<&str>>()
                .iter()
                .take(curr_symbol)
                .map(|&word| word.len())
                .sum::<usize>()
                + info.len();

            f.write_fmt(format_args!("{}^\n", " ".repeat(skip_length)))?;
        }

        Ok(())
    }
}
