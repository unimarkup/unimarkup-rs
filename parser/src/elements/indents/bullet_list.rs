//! Contains the structs and parsers to parse bullet list elements.

use std::rc::Rc;

use unimarkup_commons::lexer::{
    position::Position,
    token::{
        iterator::{EndMatcher, PrefixMatcher},
        Token, TokenKind,
    },
    SymbolKind,
};
use unimarkup_inline::{
    element::{Inline, InlineElement},
    inline_parser,
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
    pub start: Position,
    pub end: Position,
}

impl BlockElement for BulletList {
    fn to_plain_string(&self) -> String {
        todo!()
    }

    fn start(&self) -> unimarkup_commons::lexer::position::Position {
        todo!()
    }

    fn end(&self) -> unimarkup_commons::lexer::position::Position {
        todo!()
    }
}

impl BulletList {
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

    pub start: Position,
    pub end: Position,
}

impl BlockElement for BulletListEntry {
    fn to_plain_string(&self) -> String {
        todo!()
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
                    || matcher.matches(STAR_ENTRY_START)
                    || matcher.matches(MINUS_ENTRY_START)
                    || matcher.matches(PLUS_ENTRY_START)
                    || matcher.matches(STAR_SUB_ENTRY_START)
                    || matcher.matches(MINUS_SUB_ENTRY_START)
                    || matcher.matches(PLUS_SUB_ENTRY_START)
            })),
        );

        let (iter, inline_context, parsed_inlines) = inline_parser::parse_inlines(
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

// impl ElementParser for BulletList {
//     type Token<'a> = self::BulletListEntry;

//     fn tokenize<'i>(
//         input: &mut unimarkup_commons::lexer::SymbolIterator<'i>,
//     ) -> Option<crate::TokenizeOutput<Self::Token<'i>>> {
//         let mut tokens = Vec::new();

//         // `[1..]` to strip newline match for list start
//         while input.matches(&STAR_ENTRY_START[1..])
//             || input.matches(&MINUS_ENTRY_START[1..])
//             || input.matches(&PLUS_ENTRY_START[1..])
//         {
//             match BulletListEntry::tokenize(input) {
//                 Some(entry_tokens) => {
//                     let Block::BulletListEntry(entry) =
//                         BulletListEntry::parse(entry_tokens.tokens)?.pop()?
//                     else {
//                         return None;
//                     };

//                     tokens.push(entry);
//                 }
//                 None => break,
//             }
//         }

//         if tokens.is_empty() {
//             return None;
//         }

//         Some(crate::parser::TokenizeOutput { tokens })
//     }

//     fn parse(input: Vec<Self::Token<'_>>) -> Option<crate::elements::Blocks> {
//         let mut list = Self {
//             id: String::new(),
//             entries: Vec::with_capacity(input.len()),
//         };

//         for entry in input {
//             list.entries.push(entry);
//         }

//         Some(vec![Block::BulletList(list)])
//     }
// }

// impl ElementParser for BulletListEntry {
//     type Token<'a> = self::EntryToken;

//     fn tokenize<'i>(
//         input: &mut unimarkup_commons::lexer::SymbolIterator<'i>,
//     ) -> Option<crate::TokenizeOutput<Self::Token<'i>>> {
//         let entry_keyword = BulletListEntryKeyword::try_from(input.next()?).ok()?;

//         if input.next()?.kind != SymbolKind::Whitespace {
//             return None;
//         }

//         let indent_sequence = &[SymbolKind::Whitespace, SymbolKind::Whitespace];
//         let mut entry_heading_iter = input.nest(
//             Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
//                 matcher.consumed_prefix(indent_sequence)
//             })),
//             Some(Rc::new(|matcher: &mut dyn EndMatcher| {
//                 matcher.consumed_is_empty_line()
//                     || matcher.matches(STAR_ENTRY_START)
//                     || matcher.matches(MINUS_ENTRY_START)
//                     || matcher.matches(PLUS_ENTRY_START)
//                     || matcher.matches(STAR_SUB_ENTRY_START)
//                     || matcher.matches(MINUS_SUB_ENTRY_START)
//                     || matcher.matches(PLUS_SUB_ENTRY_START)
//             })),
//         );

//         let entry_heading_symbols = entry_heading_iter.take_to_end();
//         let entry_heading = entry_heading_symbols
//             .iter()
//             .map(|&s| *s)
//             .collect::<Vec<Symbol<'_>>>()
//             .parse_inlines()
//             .collect();
//         entry_heading_iter.update(input);

//         while input.consumed_is_empty_line() {
//             // skip empty lines
//         }

//         let entry_body = if !input.end_reached()
//             && !input.matches(STAR_ENTRY_START)
//             && !input.matches(MINUS_ENTRY_START)
//             && !input.matches(PLUS_ENTRY_START)
//         {
//             let mut entry_body_iter = input.nest(
//                 Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
//                     matcher.consumed_prefix(indent_sequence) || matcher.empty_line()
//                 })),
//                 None,
//             );
//             let body = MainParser::default().parse(&mut entry_body_iter);
//             entry_body_iter.update(input);
//             body
//         } else {
//             input.next(); // Consume "Newline" symbol of next list entry
//             Vec::new()
//         };

//         Some(crate::TokenizeOutput {
//             tokens: vec![
//                 Self::Token::Id(String::new()),
//                 Self::Token::Keyword(entry_keyword),
//                 Self::Token::Heading(entry_heading),
//                 Self::Token::Body(entry_body),
//                 Self::Token::Attributes(String::new()),
//             ],
//         })
//     }

//     fn parse(mut input: Vec<Self::Token<'_>>) -> Option<crate::elements::Blocks> {
//         let EntryToken::Attributes(attributes) = input.pop()? else {
//             return None;
//         };
//         let EntryToken::Body(body) = input.pop()? else {
//             return None;
//         };
//         let EntryToken::Heading(heading) = input.pop()? else {
//             return None;
//         };
//         let EntryToken::Keyword(keyword) = input.pop()? else {
//             return None;
//         };
//         let EntryToken::Id(id) = input.pop()? else {
//             return None;
//         };

//         Some(vec![Block::BulletListEntry(BulletListEntry {
//             id,
//             keyword,
//             heading,
//             body,
//             attributes,
//         })])
//     }
// }
