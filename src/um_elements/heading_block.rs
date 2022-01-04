use std::collections::VecDeque;

use pest::iterators::{Pair, Pairs};
use pest::Span;
use strum_macros::*;

use crate::backend::{BackendError, ParseFromIr, Render};
use crate::frontend::parser::{self, Rule, UmParse};
use crate::frontend::UnimarkupBlocks;
use crate::middleend::{AsIrLines, ContentIrLine};
use crate::um_elements::types::{self, UnimarkupType};
use crate::um_error::UmError;

/// Enum of possible heading levels for unimarkup headings
#[derive(Eq, PartialEq, Debug, strum_macros::Display, EnumString, Clone, Copy)]
#[strum(serialize_all = "kebab-case")]
pub enum HeadingLevel {
    /// Heading level 1, corresponds to `# ` in Unimarkup.
    #[strum(serialize = "level-1")]
    Level1 = 1, // start counting from 0

    /// Heading level 2, corresponds to `## ` in Unimarkup.
    #[strum(serialize = "level-2")]
    Level2,

    /// Heading level 3, corresponds to `### ` in Unimarkup.
    #[strum(serialize = "level-3")]
    Level3,

    /// Heading level 4, corresponds to `#### ` in Unimarkup.
    #[strum(serialize = "level-4")]
    Level4,

    /// Heading level 5, corresponds to `##### ` in Unimarkup.
    #[strum(serialize = "level-5")]
    Level5,

    /// Heading level 6, corresponds to `###### ` in Unimarkup.
    #[strum(serialize = "level-6")]
    Level6,

    /// Invalid level to denote an invalid heading syntax
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
            "1" | "#" => HeadingLevel::Level1,
            "2" | "##" => HeadingLevel::Level2,
            "3" | "###" => HeadingLevel::Level3,
            "4" | "####" => HeadingLevel::Level4,
            "5" | "#####" => HeadingLevel::Level5,
            "6" | "######" => HeadingLevel::Level6,
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

/// Structure of a Unimarkup heading element.
#[derive(Debug, Default)]
pub struct HeadingBlock {
    /// Unique identifier for a heading.
    pub id: String,

    /// Heading level.
    pub level: HeadingLevel,

    /// The content of the heading line.
    pub content: String,

    /// Attributes of the heading.
    pub attributes: String,

    /// Line number, where the heading occurs in
    /// the Unimarkup document.
    pub line_nr: usize,
}

impl HeadingBlock {
    /// Parses a single instance of a heading element.
    fn parse_single(pair: &Pair<Rule>) -> Self {
        let mut heading_data = pair.clone().into_inner();

        // heading_start
        let heading_start = heading_data.next().expect("heading rule has heading_start");

        let heading_content = heading_data
            .next()
            .expect("heading rule has heading_content");

        let level = heading_start.as_str().trim().into();
        let (line_nr, _) = heading_start.as_span().start_pos().line_col();

        let id = parser::generate_id(heading_content.as_str());

        HeadingBlock {
            id,
            level,
            content: heading_content.as_str().trim().into(),
            attributes: "{}".into(),
            line_nr,
        }
    }
}

impl UmParse for HeadingBlock {
    fn parse(pairs: &mut Pairs<Rule>, span: Span) -> Result<UnimarkupBlocks, UmError>
    where
        Self: Sized,
    {
        let heading_pairs = pairs
            .next()
            .expect("At least one pair available")
            .into_inner();

        let mut headings: UnimarkupBlocks = Vec::new();

        let (line_nr, _column_nr) = span.start_pos().line_col();

        for pair in heading_pairs {
            let mut heading = Self::parse_single(&pair);
            // child line number starts with 1
            // which leads to off by 1 error
            // hence minus 1
            heading.line_nr += line_nr - 1;
            headings.push(Box::new(heading));
        }

        Ok(headings)
    }
}

impl AsIrLines for HeadingBlock {
    fn as_ir_lines(&self) -> Vec<ContentIrLine> {
        let level = self.level.to_string();

        let mut um_type = UnimarkupType::Heading.to_string();

        um_type.push(types::DELIMITER);
        um_type.push_str(&level);

        let line = ContentIrLine::new(
            &self.id,
            self.line_nr,
            um_type,
            &self.content,
            "",
            &self.attributes,
            "",
        );

        vec![line]
    }
}

impl AsRef<Self> for HeadingBlock {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl ParseFromIr for HeadingBlock {
    fn parse_from_ir(content_lines: &mut VecDeque<ContentIrLine>) -> Result<Self, UmError> {
        let mut level = HeadingLevel::Invalid;

        if let Some(ir_line) = content_lines.pop_front() {
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
                ir_line.text
            } else {
                ir_line.fallback_text
            };

            let attributes = if !ir_line.attributes.is_empty() {
                ir_line.attributes
            } else {
                ir_line.fallback_attributes
            };

            let block = HeadingBlock {
                id: ir_line.id,
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
