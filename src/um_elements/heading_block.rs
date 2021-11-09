use strum_macros::*;
use unicode_segmentation::UnicodeSegmentation;

use crate::frontend::parser::count_symbol_until;
use crate::frontend::{parser::CursorPos, syntax_error::UmSyntaxError};
use crate::middleend::ir::ParseForIr;
use crate::middleend::ir_block::IrBlock;
use crate::middleend::ir_content::ContentIrLine;
use crate::um_elements::types::UnimarkupType;

#[derive(Eq, PartialEq, Debug, strum_macros::Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum HeadingLevel {
    #[strum(serialize = "level_1")]
    Level1 = 1, // start counting from 0
    #[strum(serialize = "level_2")]
    Level2,
    #[strum(serialize = "level_3")]
    Level3,
    #[strum(serialize = "level_4")]
    Level4,
    #[strum(serialize = "level_5")]
    Level5,
    #[strum(serialize = "level_6")]
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
            _ => 7,
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
    pub id: String,
    pub level: HeadingLevel,
    pub content: String,
    pub attributes: String,
}

impl ParseForIr for HeadingBlock {
    fn parse_for_ir(
        content: &[&str],
        cursor_pos: &CursorPos,
    ) -> Result<(IrBlock, CursorPos), UmSyntaxError> {
        let mut curr_pos = *cursor_pos;
        let start_line_nr = curr_pos.line;

        let mut heading_block = HeadingBlock {
            id: "".to_string(),
            level: HeadingLevel::Invalid,
            content: "".to_string(),
            attributes: "".to_string(),
        };

        while let Some(&line) = content.get(curr_pos.line) {
            if line.trim().is_empty() {
                if heading_block.level == HeadingLevel::Invalid {
                    return Err(UmSyntaxError::generate_error(
                        content,
                        cursor_pos,
                        &curr_pos,
                        "Invalid heading syntax. \n".to_owned()
                            + "Headings are defined as 1 to 6 '#' symbols, \n"
                            + "followed by whitespace and Heading content.",
                    ));
                } else {
                    break;
                }
            }

            let mut heading_count = 0;

            if heading_block.level == HeadingLevel::Invalid {
                let count_res = count_symbol_until(line, "#", char::is_whitespace);

                match count_res {
                    Ok(count) => heading_count = count,
                    Err((count, message)) => {
                        curr_pos.symbol = count;

                        return Err(UmSyntaxError::generate_error(
                            content, cursor_pos, &curr_pos, message,
                        ));
                    }
                }

                if heading_count > HeadingLevel::Level6 as usize {
                    // to many hashtags, when heading expected

                    // index starts from 0, HeadingLevel from 1
                    curr_pos.symbol = (HeadingLevel::Invalid as usize) - 1;

                    return Err(UmSyntaxError::generate_error(
                        content,
                        cursor_pos,
                        &curr_pos,
                        "Invalid number of '#' symbols.",
                    ));
                }

                heading_block.level = HeadingLevel::from(heading_count);
            }

            heading_block.content.push_str(
                line.graphemes(true)
                    .skip(heading_count)
                    .collect::<String>()
                    .trim(),
            );

            curr_pos.line += 1;
        }

        let mut ir_block = IrBlock::new();
        ir_block.append_content_lines(&mut heading_block.generate_ir_lines(start_line_nr));

        Ok((ir_block, curr_pos))
    }

    fn generate_ir_lines(&self, line_nr: usize) -> Vec<ContentIrLine> {
        let level = self.level.to_string();

        let mut um_type = UnimarkupType::Heading.to_string();

        um_type.push('_');
        um_type.push_str(&level);

        let line = ContentIrLine::new(
            &self.id,
            um_type,
            line_nr,
            &self.content,
            "",
            &self.attributes,
            "",
        );

        vec![line]
    }
}
