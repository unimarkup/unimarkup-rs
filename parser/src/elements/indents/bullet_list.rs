//! Contains the structs and parsers to parse bullet list elements.

use std::rc::Rc;

use unimarkup_commons::lexer::{
    position::Position,
    token::{
        iterator::{EndMatcher, PrefixMatcher},
        Token, TokenKind,
    },
    Itertools, SymbolKind,
};
use unimarkup_inline::{
    element::{Inline, InlineElement},
    parser,
};

use crate::{
    elements::{blocks::Block, BlockElement},
    BlockParser,
};

/// Structure of a Unimarkup bullet list element.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BulletList {
    /// The list entries of this bullet list.
    pub entries: Vec<BulletListEntry>,
    /// The start of this bullet list in the original content.
    pub start: Position,
    /// The end of this bullet list in theoriginal content.
    pub end: Position,
}

impl BlockElement for BulletList {
    fn to_plain_string(&self) -> String {
        let mut s = String::default();

        for entry in &self.entries {
            s.push_str(&entry.to_plain_string())
        }

        s
    }

    fn start(&self) -> unimarkup_commons::lexer::position::Position {
        self.start
    }

    fn end(&self) -> unimarkup_commons::lexer::position::Position {
        self.end
    }
}

impl BulletList {
    /// Tries to create a bullet list from the current position of the given [`BlockParser`].
    ///
    /// Returns the block parser, and the optional bullet list.
    pub(crate) fn parse<'s, 'i>(
        mut parser: BlockParser<'s, 'i>,
    ) -> (BlockParser<'s, 'i>, Option<Block>) {
        let mut entries = Vec::new();

        // `[1..]` to strip newline match for list start
        while parser.iter.matches(&STAR_ENTRY_START[1..])
            || parser.iter.matches(&MINUS_ENTRY_START[1..])
            || parser.iter.matches(&PLUS_ENTRY_START[1..])
        {
            let checkpoint = parser.iter.checkpoint();
            let (updated_parser, list_entry_opt) = BulletListEntry::parse(parser);
            parser = updated_parser;

            match list_entry_opt {
                Some(list_entry) => {
                    entries.push(list_entry);
                }
                None => {
                    // Reverts last tried entry parsing
                    parser.iter.rollback(checkpoint);
                    break;
                }
            }
        }

        if entries.is_empty() {
            return (parser, None);
        }

        let start = entries
            .first()
            .expect("Ensured above that entries exist.")
            .start();
        let end = entries
            .last()
            .expect("Ensured above that entries exist.")
            .end();

        (
            parser,
            Some(Block::BulletList(BulletList {
                entries,
                start,
                end,
            })),
        )
    }
}

/// Structure of a Unimarkup bullet list entry.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BulletListEntry {
    /// The [`BulletListEntryKeyword`] used to create this entry.
    pub keyword: BulletListEntryKeyword,
    /// The entry heading content of this entry.
    pub heading: Vec<Inline>,
    /// The body of this entry.
    pub body: Vec<Block>,
    /// The start of this entry in the original content.
    pub start: Position,
    /// The end of this entry in the original content.
    pub end: Position,
}

impl BlockElement for BulletListEntry {
    fn to_plain_string(&self) -> String {
        let head_body_separator = match self.body.first() {
            Some(Block::BulletList(_)) | None => "",
            _ => SymbolKind::Newline.as_str(),
        };

        let plain_body = if self.body.is_empty() {
            String::default()
        } else {
            self.body.to_plain_string().lines().join("\n  ")
        }; // Two space indentation after newline

        format!(
            "{} {}\n{}{}",
            self.keyword.as_str(),
            self.heading.to_plain_string(),
            head_body_separator,
            plain_body
        )
    }

    fn start(&self) -> unimarkup_commons::lexer::position::Position {
        self.start
    }

    fn end(&self) -> unimarkup_commons::lexer::position::Position {
        self.end
    }
}

/// Enum representing the keyword used to create a [`BulletListEntry`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BulletListEntryKeyword {
    /// Minus keyword: `-`
    Minus,
    /// Plus keyword: `+`
    Plus,
    /// Star keyword: `*`
    Star,
}

impl BulletListEntryKeyword {
    /// String representation of the [`BulletListEntryKeyword`].
    pub fn as_str(&self) -> &str {
        match self {
            BulletListEntryKeyword::Minus => SymbolKind::Minus.as_str(),
            BulletListEntryKeyword::Plus => SymbolKind::Plus.as_str(),
            BulletListEntryKeyword::Star => SymbolKind::Star.as_str(),
        }
    }
}

impl<'slice, 'input> TryFrom<&'slice Token<'input>> for BulletListEntryKeyword {
    type Error = ConversionError;

    fn try_from(value: &'slice Token<'input>) -> Result<Self, Self::Error> {
        value.kind.try_into()
    }
}

impl TryFrom<TokenKind> for BulletListEntryKeyword {
    type Error = ConversionError;

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        match value {
            TokenKind::Minus(1) => Ok(BulletListEntryKeyword::Minus),
            TokenKind::Plus(1) => Ok(BulletListEntryKeyword::Plus),
            TokenKind::Star(1) => Ok(BulletListEntryKeyword::Star),
            _ => Err(ConversionError::CannotConvertSymbol),
        }
    }
}

/// Enum representing possible conversion errors
/// that may occur when converting [`SymbolKind`] to [`BulletListEntryKeyword`].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConversionError {
    /// Error denoting that the given [`SymbolKind`] could not be converted to a [`BulletListEntryKeyword`].
    CannotConvertSymbol,
}

// Consts below help with matching to prevent dynamic allocations.

const STAR_ENTRY_START: &[TokenKind] = &[TokenKind::Newline, TokenKind::Star(1), TokenKind::Space];
const MINUS_ENTRY_START: &[TokenKind] =
    &[TokenKind::Newline, TokenKind::Minus(1), TokenKind::Space];
const PLUS_ENTRY_START: &[TokenKind] = &[TokenKind::Newline, TokenKind::Plus(1), TokenKind::Space];

const STAR_SUB_ENTRY_START: &[TokenKind] = &[
    TokenKind::Newline,
    TokenKind::Space,
    TokenKind::Space,
    TokenKind::Star(1),
    TokenKind::Space,
];
const MINUS_SUB_ENTRY_START: &[TokenKind] = &[
    TokenKind::Newline,
    TokenKind::Space,
    TokenKind::Space,
    TokenKind::Minus(1),
    TokenKind::Space,
];
const PLUS_SUB_ENTRY_START: &[TokenKind] = &[
    TokenKind::Newline,
    TokenKind::Space,
    TokenKind::Space,
    TokenKind::Plus(1),
    TokenKind::Space,
];

impl BulletListEntry {
    /// Tries to create a bullet list entry from the current position of the given [`BlockParser`].
    ///
    /// Returns the block parser, and the optional bullet list entry.
    pub(crate) fn parse<'s, 'i>(
        mut parser: BlockParser<'s, 'i>,
    ) -> (BlockParser<'s, 'i>, Option<BulletListEntry>) {
        // It is ensured by the BulletList parser, that the entry start is valid
        // => we can consume the start tokens without checking
        let start_token = parser
            .iter
            .next()
            .expect("Correct list entry start ensured in bullet list parser.");
        let entry_keyword = BulletListEntryKeyword::try_from(start_token)
            .expect("Correct list entry start ensured in bullet list parser.");

        parser.iter.next(); // Consume space after keyword

        let indent_sequence = &[TokenKind::Space, TokenKind::Space];
        let mut entry_heading_parser = parser.nest(
            Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
                matcher.consumed_prefix(indent_sequence)
            })),
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.consumed_is_blank_line()
                    || matcher.outer_end()
                    || matcher.matches(STAR_ENTRY_START)
                    || matcher.matches(MINUS_ENTRY_START)
                    || matcher.matches(PLUS_ENTRY_START)
                    || matcher.matches(STAR_SUB_ENTRY_START)
                    || matcher.matches(MINUS_SUB_ENTRY_START)
                    || matcher.matches(PLUS_SUB_ENTRY_START)
            })),
        );

        let (iter, inline_context, parsed_inlines) = parser::parse_inlines(
            entry_heading_parser.iter,
            (&entry_heading_parser.context).into(),
            None,
            None,
        );
        entry_heading_parser.iter = iter;
        entry_heading_parser.context.update_from(inline_context);
        let entry_heading = parsed_inlines.to_inlines();

        parser = entry_heading_parser.unfold();

        // List entries without content are invalid
        if entry_heading.is_empty() {
            return (parser, None);
        }

        while parser.iter.consumed_is_blank_line() {
            // skip empty lines
            //TODO: add blanklines in case newlines should be kept
        }

        if !parser.iter.end_reached()
            && !parser.iter.matches(STAR_ENTRY_START)
            && !parser.iter.matches(MINUS_ENTRY_START)
            && !parser.iter.matches(PLUS_ENTRY_START)
        {
            let entry_body_parser = parser.nest(
                Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
                    matcher.consumed_prefix(indent_sequence) || matcher.only_spaces_until_newline()
                })),
                None,
            );
            let (updated_parser, blocks) = BlockParser::parse(entry_body_parser);
            //TODO: checkpoint and rollback in block parser if prefix match failed
            parser = updated_parser.unfold();

            if !blocks.is_empty() {
                let end = blocks.last().expect("At least one block must exist.").end();

                return (
                    parser,
                    Some(BulletListEntry {
                        keyword: entry_keyword,
                        heading: entry_heading,
                        body: blocks,
                        start: start_token.start,
                        end,
                    }),
                );
            }
        } else {
            parser.iter.next(); // Consume "Newline" symbol of next list entry
        };

        let end = entry_heading
            .last()
            .expect("Ensured above that entry heading has elements.")
            .end();

        (
            parser,
            Some(BulletListEntry {
                keyword: entry_keyword,
                heading: entry_heading,
                body: Vec::new(),
                start: start_token.start,
                end,
            }),
        )
    }
}
