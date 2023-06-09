use strum_macros::*;
use unimarkup_inline::{Inline, ParseInlines};
use unimarkup_render::html::Html;
use unimarkup_render::render::Render;

use crate::elements::blocks::Block;
use crate::elements::{inlines, Blocks};
use crate::parser::{ElementParser, TokenizeOutput};
use unimarkup_commons::scanner::{Symbol, SymbolKind};

use super::log_id::AtomicError;

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
    type Error = AtomicError;

    fn try_from(level_depth: usize) -> Result<Self, Self::Error> {
        let level = match level_depth {
            1 => Self::Level1,
            2 => Self::Level2,
            3 => Self::Level3,
            4 => Self::Level4,
            5 => Self::Level5,
            6 => Self::Level6,
            _ => return Err(AtomicError::InvalidHeadingLvl),
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

/// HeadingToken for the [`ElementParser`]
pub enum HeadingToken<'a> {
    /// Level of the heading
    Level(HeadingLevel),

    /// Content of the heading
    Content(&'a [Symbol<'a>]),

    /// Marks the end of the heading
    End,
}

impl ElementParser for Heading {
    type Token<'a> = self::HeadingToken<'a>;

    fn tokenize<'i>(input: &'i [Symbol<'i>]) -> Option<TokenizeOutput<'i, Self::Token<'i>>> {
        let mut level_depth = input
            .iter()
            .take_while(|symbol| matches!(symbol.kind, SymbolKind::Hash))
            .count();

        let level: HeadingLevel = HeadingLevel::try_from(level_depth).ok()?;
        if input.get(level_depth)?.kind != SymbolKind::Whitespace {
            return None;
        }
        level_depth += 1; // +1 space offset

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
            tokens: vec![
                HeadingToken::Level(level),
                HeadingToken::Content(content),
                HeadingToken::End,
            ],
            rest_of_input: rest,
        };

        Some(output)
    }

    fn parse(input: Vec<Self::Token<'_>>) -> Option<Blocks> {
        let HeadingToken::Level(level) = input[0] else {return None};
        let HeadingToken::Content(symbols) = input[1] else {return None};
        let inline_start = symbols.get(0)?.start;

        let content = symbols.parse_inlines().collect();
        let line_nr = inline_start.line;
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
    fn render_html(&self) -> Html {
        let mut html = Html::default();

        let tag_level = u8::from(self.level).to_string();

        html.body.push_str("<h");
        html.body.push_str(&tag_level);
        html.body.push_str(" id='");
        html.body.push_str(&self.id);
        html.body.push_str("'>");

        inlines::push_inlines(&mut html, &self.content);

        html.body.push_str("</h");
        html.body.push_str(&tag_level);
        html.body.push('>');

        html
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use crate::elements::atomic::{Heading, HeadingLevel};
    use unimarkup_commons::scanner::{Scanner, Symbol};
    use unimarkup_inline::ParseInlines;
    use unimarkup_render::render::Render;

    fn scan_str(input: &str) -> Vec<Symbol> {
        let scanner = Scanner::try_new_with_any(icu_testdata::any()).unwrap();
        scanner.scan_str(input)
    }

    #[test]
    fn test__render_html__heading() {
        let lowest_level = HeadingLevel::Level1 as usize;
        let highest_level = HeadingLevel::Level6 as usize;

        for level in lowest_level..=highest_level {
            let heading_content = scan_str("This is a heading").parse_inlines().collect();
            let id = format!("heading-id-{}", level);

            let heading = Heading {
                id: String::from(&id),
                level: HeadingLevel::try_from(level).unwrap(),
                content: heading_content,
                attributes: None,
                line_nr: level,
            };

            let html = heading.render_html();

            let expected = format!("<h{} id='{}'>This is a heading</h{}>", level, id, level);
            assert_eq!(html.body, expected);
        }
    }

    #[test]
    fn test__render_html__heading_with_inline() {
        let lowest_level = HeadingLevel::Level1 as usize;
        let highest_level = HeadingLevel::Level6 as usize;

        for level in lowest_level..=highest_level {
            let heading_content = scan_str("`This` *is _a_* **heading**")
                .parse_inlines()
                .collect();
            let id = format!("heading-id-{}", level);

            let heading = Heading {
                id: String::from(&id),
                level: HeadingLevel::try_from(level).unwrap(),
                content: heading_content,
                attributes: None,
                line_nr: level,
            };

            let html = heading.render_html();

            let expected = format!(
                "<h{} id='{}'><pre><code>This</code></pre> <em>is <sub>a</sub></em> <strong>heading</strong></h{}>",
                level, id, level
            );
            assert_eq!(html.body, expected);
        }
    }
}
