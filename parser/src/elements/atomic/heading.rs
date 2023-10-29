use std::rc::Rc;

use strum_macros::*;
use unimarkup_inline::{Inline, ParseInlines};

use crate::elements::blocks::Block;
use crate::elements::Blocks;
use crate::parser::{ElementParser, TokenizeOutput};
use unimarkup_commons::lexer::{
    EndMatcher, Itertools, PrefixMatcher, Symbol, SymbolIterator, SymbolKind,
};

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
        }
    }
}

impl TryFrom<&str> for HeadingLevel {
    type Error = AtomicError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let level = match input {
            "1" | "#" => HeadingLevel::Level1,
            "2" | "##" => HeadingLevel::Level2,
            "3" | "###" => HeadingLevel::Level3,
            "4" | "####" => HeadingLevel::Level4,
            "5" | "#####" => HeadingLevel::Level5,
            "6" | "######" => HeadingLevel::Level6,
            _ => return Err(AtomicError::InvalidHeadingLvl),
        };

        Ok(level)
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
    Content(Vec<&'a Symbol<'a>>),

    /// Marks the end of the heading
    End,
}

impl ElementParser for Heading {
    type Token<'a> = self::HeadingToken<'a>;

    fn tokenize<'i>(input: &mut SymbolIterator<'i>) -> Option<TokenizeOutput<Self::Token<'i>>> {
        let mut heading_start: Vec<SymbolKind> = input
            .peeking_take_while(|symbol| matches!(symbol.kind, SymbolKind::Hash))
            .map(|s| s.kind)
            .collect();

        let level_depth = heading_start.len();
        let level: HeadingLevel = HeadingLevel::try_from(level_depth).ok()?;
        if input.by_ref().nth(level_depth)?.kind != SymbolKind::Whitespace {
            return None;
        }

        heading_start.push(SymbolKind::Whitespace);

        let sub_heading_start: Vec<SymbolKind> = std::iter::repeat(SymbolKind::Hash)
            .take(heading_start.len())
            .chain([SymbolKind::Whitespace])
            .collect();
        let heading_end = move |matcher: &mut dyn EndMatcher| {
            matcher.consumed_is_empty_line()
                || matcher.matches(&[SymbolKind::Eoi])
                || level != HeadingLevel::Level6 && matcher.matches(&sub_heading_start)
        };

        let whitespace_indents: Vec<SymbolKind> = std::iter::repeat(SymbolKind::Whitespace)
            .take(heading_start.len())
            .collect();
        let heading_prefix = move |matcher: &mut dyn PrefixMatcher| {
            matcher.consumed_prefix(&heading_start) || matcher.consumed_prefix(&whitespace_indents)
        };

        let mut content_iter =
            input.nest(Some(Rc::new(heading_prefix)), Some(Rc::new(heading_end)));
        let content_symbols = content_iter.take_to_end();

        // Line prefixes violated => invalid heading syntax
        if !content_iter.end_reached() {
            return None;
        }

        content_iter.update(input);

        let output = TokenizeOutput {
            tokens: vec![
                HeadingToken::Level(level),
                HeadingToken::Content(content_symbols),
                HeadingToken::End,
            ],
        };

        Some(output)
    }

    fn parse(input: Vec<Self::Token<'_>>) -> Option<Blocks> {
        let HeadingToken::Level(level) = input[0] else {
            return None;
        };
        let HeadingToken::Content(ref symbols) = input[1] else {
            return None;
        };
        let inline_start = symbols.get(0)?.start;

        // TODO: Adapt inline lexer to also work with Vec<&'input Symbol>
        let content = symbols
            .iter()
            .map(|&s| *s)
            .collect::<Vec<Symbol<'_>>>()
            .parse_inlines()
            .collect();

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
