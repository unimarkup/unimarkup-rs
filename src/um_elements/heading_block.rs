use std::collections::VecDeque;
use std::mem;

use strum_macros::*;

use crate::backend::{BackendError, ParseFromIr, Render};
use crate::frontend::parser_pest::Rule;
use crate::middleend::ContentIrLine;
use crate::um_elements::types::{self, UnimarkupType};
use crate::um_error::UmError;

#[derive(Eq, PartialEq, Debug, strum_macros::Display, EnumString, Clone, Copy)]
#[strum(serialize_all = "kebab-case")]
pub enum HeadingLevel {
    #[strum(serialize = "level-1")]
    Level1 = 1, // start counting from 0
    #[strum(serialize = "level-2")]
    Level2,
    #[strum(serialize = "level-3")]
    Level3,
    #[strum(serialize = "level-4")]
    Level4,
    #[strum(serialize = "level-5")]
    Level5,
    #[strum(serialize = "level-6")]
    Level6,
    Invalid,
}

impl Default for HeadingLevel {
    fn default() -> Self {
        Self::Invalid
    }
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

impl From<&str> for HeadingLevel {
    fn from(input: &str) -> Self {
        match input {
            "1" => HeadingLevel::Level1,
            "2" => HeadingLevel::Level2,
            "3" => HeadingLevel::Level3,
            "4" => HeadingLevel::Level4,
            "5" => HeadingLevel::Level5,
            "6" => HeadingLevel::Level6,
            _ => HeadingLevel::Invalid,
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

impl From<Rule> for HeadingLevel {
    fn from(level_depth: Rule) -> Self {
        match level_depth {
            Rule::heading1 => Self::Level1,
            Rule::heading2 => Self::Level2,
            Rule::heading3 => Self::Level3,
            Rule::heading4 => Self::Level4,
            Rule::heading5 => Self::Level5,
            Rule::heading6 => Self::Level6,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug, Default)]
pub struct HeadingBlock {
    pub id: String,
    pub level: HeadingLevel,
    pub content: String,
    pub attributes: String,
    pub line_nr: usize,
}

impl From<HeadingBlock> for Vec<ContentIrLine> {
    fn from(heading_block: HeadingBlock) -> Self {
        let level = heading_block.level.to_string();

        let mut um_type = UnimarkupType::Heading.to_string();

        um_type.push('_');
        um_type.push_str(&level);

        let line = ContentIrLine::new(
            &heading_block.id,
            heading_block.line_nr,
            um_type,
            &heading_block.content,
            "",
            &heading_block.attributes,
            "",
        );

        vec![line]
    }
}

impl ParseFromIr for HeadingBlock {
    fn parse_from_ir(content_lines: &mut VecDeque<ContentIrLine>) -> Result<Self, UmError> {
        let mut level = HeadingLevel::Invalid;

        if let Some(mut ir_line) = content_lines.pop_front() {
            let heading_pattern = format!("heading{delim}level{delim}", delim = types::DELIMITER);

            if ir_line.um_type.contains(&heading_pattern) {
                let mut split = ir_line.um_type.split(&heading_pattern);

                // first element should be empty
                if let Some("") = split.next() {
                    let level_num = split.next();

                    if let (Some(num_as_str), None) = (level_num, split.next()) {
                        level = HeadingLevel::from(num_as_str);
                    }
                }
            }

            if level == HeadingLevel::Invalid {
                return Err(BackendError::new(format!(
                    "Provided heading level is invalid: {}",
                    ir_line.um_type
                ))
                .into());
            }

            let content = if !ir_line.text.is_empty() {
                mem::take(&mut ir_line.text)
            } else {
                mem::take(&mut ir_line.fallback_text)
            };

            let attributes = if !ir_line.attributes.is_empty() {
                mem::take(&mut ir_line.attributes)
            } else {
                mem::take(&mut ir_line.fallback_attributes)
            };

            let block = HeadingBlock {
                id: mem::take(&mut ir_line.id),
                level,
                content,
                attributes,
                line_nr: ir_line.line_nr,
            };

            return Ok(block);
        }

        Err(BackendError::new("ContentIrLines are empty, could not construct HeadingBlock!").into())
    }
}

impl Render for HeadingBlock {
    fn render_html(&self) -> Result<String, UmError> {
        let mut html = String::default();

        let tag_level = usize::from(self.level).to_string();

        html.push_str("<h");
        html.push_str(&tag_level);
        html.push_str(" id='");
        html.push_str(&self.id);
        html.push_str("'>");

        html.push_str(&self.content);
        html.push_str("</h");
        html.push_str(&tag_level);
        html.push('>');

        Ok(html)
    }
}

#[cfg(test)]
mod heading_tests {
    use std::collections::VecDeque;

    use crate::{
        backend::{ParseFromIr, Render},
        middleend::ContentIrLine,
        um_elements::{heading_block::HeadingLevel, types},
        um_error::UmError,
    };

    use super::HeadingBlock;

    #[test]
    fn render_heading() -> Result<(), UmError> {
        let lowest_level = HeadingLevel::Level1 as usize;
        let highest_level = HeadingLevel::Level6 as usize;

        for level in lowest_level..=highest_level {
            let heading_content = String::from("This is a heading");
            let id = format!("heading-id-{}", level);

            let heading = HeadingBlock {
                id: String::from(&id),
                level: HeadingLevel::from(level),
                content: heading_content,
                attributes: String::default(),
                line_nr: level as usize,
            };

            let html = heading.render_html()?;

            let expected = format!("<h{} id='{}'>This is a heading</h{}>", level, id, level);
            assert_eq!(html, expected);
        }

        Ok(())
    }

    #[test]
    fn parse_from_ir_test() -> Result<(), UmError> {
        let lowest_level = HeadingLevel::Level1 as usize;
        let highest_level = HeadingLevel::Level6 as usize;

        let mut ir_lines: VecDeque<ContentIrLine> = vec![].into();

        for heading_level in lowest_level..=highest_level {
            let ir_line = ContentIrLine::new(
                "some_id",
                42 + heading_level,
                format!(
                    "heading{delim}level{delim}{level}",
                    delim = types::DELIMITER,
                    level = heading_level
                ),
                "This is a heading",
                "",
                "{}",
                "{}",
            );

            ir_lines.push_back(ir_line);
        }

        // parse multiple heading blocks

        let mut iterations = 0;

        while ir_lines.get(0).is_some() {
            // in case something goes wrong
            iterations += 1;
            if iterations > HeadingLevel::Level6 as usize {
                break;
            }

            let block = HeadingBlock::parse_from_ir(&mut ir_lines)?;

            let (id, level, content, attr);

            id = block.id;
            level = block.level;
            content = block.content;
            attr = block.attributes;

            assert_eq!(id, String::from("some_id"));
            assert_eq!(level, HeadingLevel::from(iterations));
            assert_eq!(content, String::from("This is a heading"));
            assert_eq!(attr, String::from("{}"));
        }

        Ok(())
    }

    #[test]
    fn parse_from_ir_bad() {
        let mut ir_lines: VecDeque<ContentIrLine> = vec![].into();

        let bad_ir_line = ContentIrLine::new(
            "some_id",
            42,
            format!("heading{delim}level{delim}0", delim = types::DELIMITER),
            "This is a heading",
            "",
            "{}",
            "{}",
        );

        ir_lines.push_back(bad_ir_line);

        // should panic because error is expected!
        let result = HeadingBlock::parse_from_ir(&mut ir_lines);

        assert!(result.is_err());
        println!("{}", result.err().unwrap());

        let bad_ir_line = ContentIrLine::new(
            "some_id",
            42,
            format!("heading{delim}level{delim}7", delim = types::DELIMITER),
            "This is a heading",
            "",
            "{}",
            "{}",
        );

        ir_lines.push_back(bad_ir_line);

        // should panic because error is expected
        let result = HeadingBlock::parse_from_ir(&mut ir_lines);

        assert!(result.is_err());
        println!("{}", result.err().unwrap());

        let bad_ir_line = ContentIrLine::new(
            "some_id",
            42,
            format!(
                "some{delim}other{delim}type{delim}level{delim}2",
                delim = types::DELIMITER
            ),
            "This is a heading",
            "",
            "{}",
            "{}",
        );

        ir_lines.push_back(bad_ir_line);

        let result = HeadingBlock::parse_from_ir(&mut ir_lines);

        assert!(result.is_err());
        println!("{}", result.err().unwrap());
    }
}
