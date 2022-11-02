use std::collections::{HashMap, VecDeque};

use logid::capturing::{LogIdTracing, MappedLogId};
use logid::log_id::LogId;
use pest::iterators::{Pair, Pairs};
use pest::Span;
use strum_macros::*;
use unimarkup_inline::{Inline, ParseUnimarkupInlines};
use unimarkup_render::html::Html;
use unimarkup_render::render::Render;

use crate::backend::ParseFromIr;
use crate::elements::log_id::{AtomicErrLogId, GeneralErrLogId};
use crate::elements::types::{self, ElementType};
use crate::elements::{inlines, UnimarkupBlocks};
use crate::frontend::parser::custom_pest_error;
use crate::frontend::parser::{self, Rule, UmParse};
use crate::log_id::CORE_LOG_ID_MAP;
use crate::middleend::{AsIrLines, ContentIrLine};

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

impl From<HeadingLevel> for u8 {
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
#[derive(Debug, Default, Clone)]
pub struct Heading {
    /// Unique identifier for a heading.
    pub id: String,

    /// Heading level.
    pub level: HeadingLevel,

    /// The content of the heading line.
    pub content: Vec<Inline>,

    /// Attributes of the heading.
    pub attributes: String,

    /// Line number, where the heading occurs in
    /// the Unimarkup document.
    pub line_nr: usize,
}

impl Heading {
    /// Parses a single instance of a heading element.
    fn parse_single(pair: &Pair<Rule>) -> Result<Self, MappedLogId> {
        let mut heading_data = pair.clone().into_inner();

        let heading_start = heading_data.next().expect("heading rule has heading_start");

        let heading_content = heading_data
            .next()
            .expect("heading rule has heading_content");

        let attributes = match heading_data.next() {
            Some(attrs_rule) => {
                let attributes: HashMap<&str, &str> = serde_json::from_str(attrs_rule.as_str())
                    .map_err(|err| {
                        (GeneralErrLogId::InvalidAttribute as LogId)
                            .set_event_with(
                                &CORE_LOG_ID_MAP,
                                &custom_pest_error(
                                    "Heading attributes are not valid JSON",
                                    attrs_rule.as_span(),
                                ),
                                file!(),
                                line!(),
                            )
                            .add_info(&format!("Cause: {}", err))
                    })?;

                Some(attributes)
            }
            None => None,
        };

        let level = heading_start.as_str().trim();
        let (line_nr, _) = heading_start.as_span().start_pos().line_col();

        let generated_id = match parser::generate_id(heading_content.as_str()) {
            Some(id) => id.to_lowercase(),
            None => format!("heading-{}-line-{}", level, line_nr),
        };

        let id = match attributes {
            Some(ref attrs) if attrs.get("id").is_some() => attrs.get("id").unwrap().to_string(),
            _ => generated_id,
        };

        Ok(Heading {
            id,
            level: level.into(),
            content: heading_content.as_str().parse_unimarkup_inlines().collect(),
            attributes: serde_json::to_string(&attributes.unwrap_or_default()).unwrap(),
            line_nr,
        })
    }
}

impl UmParse for Heading {
    fn parse(pairs: &mut Pairs<Rule>, span: Span) -> Result<UnimarkupBlocks, MappedLogId>
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
            let mut heading = Self::parse_single(&pair)?;
            // child line number starts with 1
            // which leads to off by 1 error
            // hence minus 1
            heading.line_nr += line_nr - 1;
            headings.push(Box::new(heading));
        }

        Ok(headings)
    }
}

impl AsIrLines<ContentIrLine> for Heading {
    fn as_ir_lines(&self) -> Vec<ContentIrLine> {
        let level = self.level.to_string();

        let mut um_type = ElementType::Heading.to_string();

        um_type.push(types::ELEMENT_TYPE_DELIMITER);
        um_type.push_str(&level);

        let line = ContentIrLine::new(
            &self.id,
            self.line_nr,
            um_type,
            &self
                .content
                .iter()
                .map(|inline| inline.as_string())
                .collect::<String>(),
            "",
            &self.attributes,
            "",
        );

        vec![line]
    }
}

impl AsRef<Self> for Heading {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl ParseFromIr for Heading {
    fn parse_from_ir(content_lines: &mut VecDeque<ContentIrLine>) -> Result<Self, MappedLogId> {
        let mut level = HeadingLevel::Invalid;

        if let Some(ir_line) = content_lines.pop_front() {
            let heading_pattern = format!(
                "heading{delim}level{delim}",
                delim = types::ELEMENT_TYPE_DELIMITER
            );

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
                return Err((AtomicErrLogId::InvalidHeadingLvl as LogId).set_event_with(
                    &CORE_LOG_ID_MAP,
                    &format!("Provided heading level is invalid: {}", ir_line.um_type),
                    file!(),
                    line!(),
                ));
            }

            let content = if !ir_line.text.is_empty() {
                &*ir_line.text
            } else {
                &*ir_line.fallback_text
            }
            .parse_unimarkup_inlines()
            .collect();

            let attributes = if !ir_line.attributes.is_empty() {
                ir_line.attributes
            } else {
                ir_line.fallback_attributes
            };

            let block = Heading {
                id: ir_line.id,
                level,
                content,
                attributes,
                line_nr: ir_line.line_nr,
            };

            return Ok(block);
        }

        Err((AtomicErrLogId::InvalidHeadingLvl as LogId).set_event_with(
            &CORE_LOG_ID_MAP,
            "ContentIrLines are empty, could not construct Heading!",
            file!(),
            line!(),
        ))
    }
}

impl Render for Heading {
    fn render_html(&self) -> Result<Html, MappedLogId> {
        let mut html = Html::default();

        let tag_level = u8::from(self.level).to_string();

        html.body.push_str("<h");
        html.body.push_str(&tag_level);
        html.body.push_str(" id='");
        html.body.push_str(&self.id);
        html.body.push_str("'>");

        inlines::push_inlines(&mut html, &self.content)?;

        html.body.push_str("</h");
        html.body.push_str(&tag_level);
        html.body.push('>');

        Ok(html)
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use unimarkup_inline::ParseUnimarkupInlines;
    use unimarkup_render::render::Render;

    use crate::{
        backend::ParseFromIr,
        elements::{atomic::HeadingLevel, types},
        middleend::ContentIrLine,
    };

    use super::Heading;

    #[test]
    fn test__render_html__heading() {
        let lowest_level = HeadingLevel::Level1 as usize;
        let highest_level = HeadingLevel::Level6 as usize;

        for level in lowest_level..=highest_level {
            let heading_content = "This is a heading".parse_unimarkup_inlines().collect();
            let id = format!("heading-id-{}", level);

            let heading = Heading {
                id: String::from(&id),
                level: HeadingLevel::from(level),
                content: heading_content,
                attributes: String::default(),
                line_nr: level as usize,
            };

            let html = heading.render_html().unwrap();

            let expected = format!("<h{} id='{}'>This is a heading</h{}>", level, id, level);
            assert_eq!(html.body, expected);
        }
    }

    #[test]
    fn test__render_html__heading_with_inline() {
        let lowest_level = HeadingLevel::Level1 as usize;
        let highest_level = HeadingLevel::Level6 as usize;

        for level in lowest_level..=highest_level {
            let heading_content = "`This` *is _a_* **heading**"
                .parse_unimarkup_inlines()
                .collect();
            let id = format!("heading-id-{}", level);

            let heading = Heading {
                id: String::from(&id),
                level: HeadingLevel::from(level),
                content: heading_content,
                attributes: String::default(),
                line_nr: level as usize,
            };

            let html = heading.render_html().unwrap();

            let expected = format!(
                "<h{} id='{}'><pre><code>This</code></pre> <em>is <sub>a</sub></em> <strong>heading</strong></h{}>",
                level, id, level
            );
            assert_eq!(html.body, expected);
        }
    }

    #[test]
    fn test__parse_from_ir__valid_heading() {
        let lowest_level = HeadingLevel::Level1 as usize;
        let highest_level = HeadingLevel::Level6 as usize;

        let mut ir_lines: VecDeque<ContentIrLine> = vec![].into();

        for heading_level in lowest_level..=highest_level {
            let ir_line = ContentIrLine::new(
                "some_id",
                42 + heading_level,
                format!(
                    "heading{delim}level{delim}{level}",
                    delim = types::ELEMENT_TYPE_DELIMITER,
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

            let block = Heading::parse_from_ir(&mut ir_lines).unwrap();

            let (id, level, content, attr);

            id = block.id;
            level = block.level;
            content = block.content;
            attr = block.attributes;

            assert_eq!(id, String::from("some_id"));
            assert_eq!(level, HeadingLevel::from(iterations));
            assert_eq!(
                content
                    .iter()
                    .map(|inline| inline.as_string())
                    .collect::<String>(),
                String::from("This is a heading")
            );
            assert_eq!(attr, String::from("{}"));
        }
    }

    #[test]
    fn test__parse_from_ir__invalid_heading() {
        let mut ir_lines: VecDeque<ContentIrLine> = vec![].into();

        let bad_ir_line = ContentIrLine::new(
            "some_id",
            42,
            format!(
                "heading{delim}level{delim}0",
                delim = types::ELEMENT_TYPE_DELIMITER
            ),
            "This is a heading",
            "",
            "{}",
            "{}",
        );

        ir_lines.push_back(bad_ir_line);

        // should panic because error is expected!
        let result = Heading::parse_from_ir(&mut ir_lines);

        assert!(result.is_err());

        let bad_ir_line = ContentIrLine::new(
            "some_id",
            42,
            format!(
                "heading{delim}level{delim}7",
                delim = types::ELEMENT_TYPE_DELIMITER
            ),
            "This is a heading",
            "",
            "{}",
            "{}",
        );

        ir_lines.push_back(bad_ir_line);

        // should panic because error is expected
        let result = Heading::parse_from_ir(&mut ir_lines);

        assert!(result.is_err());

        let bad_ir_line = ContentIrLine::new(
            "some_id",
            42,
            format!(
                "some{delim}other{delim}type{delim}level{delim}2",
                delim = types::ELEMENT_TYPE_DELIMITER
            ),
            "This is a heading",
            "",
            "{}",
            "{}",
        );

        ir_lines.push_back(bad_ir_line);

        let result = Heading::parse_from_ir(&mut ir_lines);

        assert!(result.is_err());
    }
}
