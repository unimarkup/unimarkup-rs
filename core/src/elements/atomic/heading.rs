use std::collections::HashMap;

use logid::capturing::{LogIdTracing, MappedLogId};
use logid::log_id::LogId;
use pest::iterators::{Pair, Pairs};
use pest::Span;
use strum_macros::*;
use unimarkup_inline::{Inline, ParseUnimarkupInlines};
use unimarkup_render::html::Html;
use unimarkup_render::render::Render;

use crate::elements::blocks::Block;
use crate::elements::log_id::GeneralErrLogId;
use crate::elements::{inlines, Blocks};
use crate::frontend::parser::custom_pest_error;
use crate::frontend::parser::{self, Rule, UmParse};
use crate::log_id::CORE_LOG_ID_MAP;
use crate::parser::symbol::{Symbol, SymbolKind};
use crate::parser::{ElementParser, TokenizeOutput};

use super::log_id::AtomicErrLogId;

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

impl TryFrom<usize> for HeadingLevel {
    type Error = MappedLogId;

    fn try_from(level_depth: usize) -> Result<Self, Self::Error> {
        let level = match level_depth {
            1 => Self::Level1,
            2 => Self::Level2,
            3 => Self::Level3,
            4 => Self::Level4,
            5 => Self::Level5,
            6 => Self::Level6,
            _ => {
                return Err((AtomicErrLogId::InvalidHeadingLvl as LogId).set_event_with(
                    &CORE_LOG_ID_MAP,
                    &format!("Invalid heading level: {level_depth}"),
                    file!(),
                    line!(),
                ))
            }
        };

        Ok(level)
    }
}

/// Structure of a Unimarkup heading element.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Heading {
    /// Unique identifier for a heading.
    pub id: String,

    /// Heading level.
    pub level: HeadingLevel,

    /// The content of the heading line.
    pub content: Vec<Inline>,

    /// Attributes of the heading.
    pub attributes: Option<String>,

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
            attributes: serde_json::to_string(&attributes.unwrap_or_default()).ok(),
            line_nr,
        })
    }
}

impl UmParse for Heading {
    fn parse(pairs: &mut Pairs<Rule>, span: Span) -> Result<Blocks, MappedLogId>
    where
        Self: Sized,
    {
        let heading_pairs = pairs
            .next()
            .expect("At least one pair available")
            .into_inner();

        let mut headings: Blocks = Vec::new();

        let (line_nr, _column_nr) = span.start_pos().line_col();

        for pair in heading_pairs {
            let mut heading = Self::parse_single(&pair)?;
            // child line number starts with 1
            // which leads to off by 1 error
            // hence minus 1
            heading.line_nr += line_nr - 1;
            headings.push(heading.into());
        }

        Ok(headings)
    }
}

pub enum Token<'a> {
    Level(HeadingLevel),
    Content(&'a [Symbol<'a>]),
    End,
}

impl ElementParser for Heading {
    type Token<'a> = self::Token<'a>;

    fn tokenize<'i>(input: &'i [Symbol<'i>]) -> Option<TokenizeOutput<'i, Self::Token<'i>>> {
        let level_depth = input
            .iter()
            .take_while(|symbol| matches!(symbol.kind, SymbolKind::Hash))
            .count();

        let level = HeadingLevel::try_from(level_depth).ok()?;
        let content_symbols = input
            .iter()
            .skip(level_depth)
            .take_while(|symbol| !matches!(symbol.kind, SymbolKind::Blankline | SymbolKind::EOI))
            .count();

        let content_start = level_depth;
        let content_end = content_start + content_symbols;

        let content = &input[content_start..content_end];
        let rest = &input[content_end..];

        let output = TokenizeOutput {
            tokens: vec![Token::Level(level), Token::Content(content), Token::End],
            rest_of_input: rest,
        };

        Some(output)
    }

    fn parse(input: Vec<Self::Token<'_>>) -> Option<Blocks> {
        let Token::Level(level) = input[0] else {return None};
        let Token::Content(symbols) = input[1] else {return None};

        let content = Symbol::flatten(symbols).parse_unimarkup_inlines().collect();
        let line_nr = symbols.get(0)?.start.line;

        // TODO: introduce data structure for Id of block.
        // Right now we use generate_id function, that is not optimal. We can do better by
        // encapsulating Id into a separate data structure and implement Render trait for it etc.
        let block = Self {
            id: String::default(),
            level,
            content,
            attributes: None,
            line_nr,
        };

        Some(vec![Block::Heading(block)])
    }
}

impl AsRef<Self> for Heading {
    fn as_ref(&self) -> &Self {
        self
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
    use crate::elements::atomic::{Heading, HeadingLevel};
    use unimarkup_inline::ParseUnimarkupInlines;
    use unimarkup_render::render::Render;

    #[test]
    fn test__render_html__heading() {
        let lowest_level = HeadingLevel::Level1 as usize;
        let highest_level = HeadingLevel::Level6 as usize;

        for level in lowest_level..=highest_level {
            let heading_content = "This is a heading".parse_unimarkup_inlines().collect();
            let id = format!("heading-id-{}", level);

            let heading = Heading {
                id: String::from(&id),
                level: HeadingLevel::try_from(level).unwrap(),
                content: heading_content,
                attributes: None,
                line_nr: level,
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
                level: HeadingLevel::try_from(level).unwrap(),
                content: heading_content,
                attributes: None,
                line_nr: level,
            };

            let html = heading.render_html().unwrap();

            let expected = format!(
                "<h{} id='{}'><pre><code>This</code></pre> <em>is <sub>a</sub></em> <strong>heading</strong></h{}>",
                level, id, level
            );
            assert_eq!(html.body, expected);
        }
    }
}
