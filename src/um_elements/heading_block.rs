use unicode_segmentation::UnicodeSegmentation;

use crate::frontend::{
    ir::{IrBlock, IrLine, ParseForIr},
    parser::CursorPos,
    syntax_error::UmSyntaxError,
};

#[derive(Eq, PartialEq, Debug)]
pub enum HeadingLevel {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Invalid,
}

impl From<HeadingLevel> for usize {
    fn from(level: HeadingLevel) -> Self {
        match level {
            HeadingLevel::Level1 => 1,
            HeadingLevel::Level2 => 2,
            HeadingLevel::Level3 => 3,
            HeadingLevel::Level4 => 4,
            HeadingLevel::Level5 => 5,
            HeadingLevel::Level6 => 6,
            HeadingLevel::Invalid => 7,
        }
    }
}

impl From<usize> for HeadingLevel {
    fn from(level_depth: usize) -> Self {
        match level_depth {
            1 => Self::Level1,
            2 => Self::Level2,
            3 => Self::Level3,
            4 => Self::Level4,
            5 => Self::Level5,
            6 => Self::Level6,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug)]
pub struct HeadingBlock {
    pub level: HeadingLevel,
    pub content: String,
}

impl ParseForIr for HeadingBlock {
    fn parse_for_ir<'a>(
        content: &'a [&str],
        cursor_pos: &CursorPos,
    ) -> Result<(IrBlock, CursorPos), UmSyntaxError<'a>> {
        let mut curr_pos = *cursor_pos;

        let mut heading_block = HeadingBlock {
            level: HeadingLevel::Invalid,
            content: "".to_string(),
        };

        while let Some(&line) = content.get(curr_pos.line) {
            if line.trim().is_empty() {
                if heading_block.level == HeadingLevel::Invalid {
                    return Err(UmSyntaxError {
                        start_pos: *cursor_pos,
                        current_pos: curr_pos,
                        lines: content,
                    });
                } else {
                    break;
                }
            }

            let mut heading_count = 0;

            if heading_block.level == HeadingLevel::Invalid {
                let heading_symbols = line.split_word_bounds().take(7);
                heading_count = heading_symbols
                    .take_while(|&symbol| symbol == "#" && symbol != " ")
                    .count();

                if heading_count > 6 {
                    // to many hashtags, when heading expected
                    curr_pos.symbol = 6;

                    return Err(UmSyntaxError {
                        start_pos: *cursor_pos,
                        current_pos: curr_pos,
                        lines: content,
                    });
                }

                heading_block.level = HeadingLevel::from(heading_count);
            }

            heading_block.content.push_str(
                line.split_word_bounds()
                    .skip(heading_count)
                    .collect::<String>()
                    .trim(),
            );

            curr_pos.line += 1;
        }

        let ir_block = IrBlock {
            lines: heading_block.generate_ir_lines(),
        };

        Ok((ir_block, curr_pos))
    }

    fn generate_ir_lines(&self) -> Vec<IrLine> {
        todo!()
    }
}
