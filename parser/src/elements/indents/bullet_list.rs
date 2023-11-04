//! Contains the structs and parsers to parse bullet list elements.

use std::rc::Rc;

use unimarkup_commons::lexer::{EndMatcher, PrefixMatcher, Symbol, SymbolKind};
use unimarkup_inline::element::Inline;

use crate::elements::blocks::Block;

/// Structure of a Unimarkup bullet list element.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BulletList {
    /// Unique identifier for a bullet list.
    pub id: String,
    /// The list entries of this bullet list.
    pub entries: Vec<BulletListEntry>,
}

/// Structure of a Unimarkup bullet list entry.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BulletListEntry {
    /// The ID of this entry.
    pub id: String,
    /// The [`BulletListEntryKeyword`] used to create this entry.
    pub keyword: BulletListEntryKeyword,
    /// The entry heading content of this entry.
    pub heading: Vec<Inline>,
    /// The body of this entry.
    pub body: Vec<Block>,
    /// The attributes set for this entry.
    pub attributes: String,
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

impl<'a> TryFrom<&'a Symbol<'a>> for BulletListEntryKeyword {
    type Error = ConversionError;

    fn try_from(value: &'a Symbol<'a>) -> Result<Self, Self::Error> {
        value.kind.try_into()
    }
}

impl TryFrom<SymbolKind> for BulletListEntryKeyword {
    type Error = ConversionError;

    fn try_from(value: SymbolKind) -> Result<Self, Self::Error> {
        match value {
            SymbolKind::Minus => Ok(BulletListEntryKeyword::Minus),
            SymbolKind::Plus => Ok(BulletListEntryKeyword::Plus),
            SymbolKind::Star => Ok(BulletListEntryKeyword::Star),
            _ => Err(ConversionError::CannotConvertSymbol),
        }
    }
}

/// Enum representing possible conversion errors
/// that may occur when converting [`SymbolKind`] to [`BulletListEntryKeyword`].
pub enum ConversionError {
    /// Error denoting that the given [`SymbolKind`] could not be converted to a [`BulletListEntryKeyword`].
    CannotConvertSymbol,
}

pub(crate) enum EntryToken {
    Id(String),
    Keyword(BulletListEntryKeyword),
    Heading(Vec<Inline>),
    Body(Vec<Block>),
    Attributes(String),
}

const STAR_ENTRY_START: &[SymbolKind] = &[
    SymbolKind::Newline,
    SymbolKind::Star,
    SymbolKind::Whitespace,
];
const MINUS_ENTRY_START: &[SymbolKind] = &[
    SymbolKind::Newline,
    SymbolKind::Minus,
    SymbolKind::Whitespace,
];
const PLUS_ENTRY_START: &[SymbolKind] = &[
    SymbolKind::Newline,
    SymbolKind::Plus,
    SymbolKind::Whitespace,
];

const STAR_SUB_ENTRY_START: &[SymbolKind] = &[
    SymbolKind::Newline,
    SymbolKind::Whitespace,
    SymbolKind::Whitespace,
    SymbolKind::Star,
    SymbolKind::Whitespace,
];
const MINUS_SUB_ENTRY_START: &[SymbolKind] = &[
    SymbolKind::Newline,
    SymbolKind::Whitespace,
    SymbolKind::Whitespace,
    SymbolKind::Minus,
    SymbolKind::Whitespace,
];
const PLUS_SUB_ENTRY_START: &[SymbolKind] = &[
    SymbolKind::Newline,
    SymbolKind::Whitespace,
    SymbolKind::Whitespace,
    SymbolKind::Plus,
    SymbolKind::Whitespace,
];

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
