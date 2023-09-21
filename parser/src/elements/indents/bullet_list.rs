use unimarkup_commons::scanner::{Symbol, SymbolKind};
use unimarkup_inline::Inline;

use crate::{elements::blocks::Block, ElementParser};

/// Structure of a Unimarkup bullet list element.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BulletList {
    /// Unique identifier for a bullet list.
    pub id: String,

    pub entries: Vec<BulletListEntry>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BulletListEntry {
    pub id: String,

    pub keyword: BulletListEntryKeyword,

    pub heading: Inline,

    pub body: Vec<Block>,

    pub attributes: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BulletListEntryKeyword {
    Minus,
    Plus,
    Star,
}

impl<'a> TryFrom<Symbol<'a>> for BulletListEntryKeyword {
    type Error = ConversionError;

    fn try_from(value: Symbol<'a>) -> Result<Self, Self::Error> {
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

pub(crate) enum Token<'a> {
    NewEntry { keyword: BulletListEntryKeyword },
    EntryHeading(&'a [Symbol<'a>]),
    EntryBody(&'a [Symbol<'a>]),
}

impl ElementParser for BulletList {
    type Token<'a> = self::Token<'a>;

    fn tokenize<'i>(input: &'i [Symbol<'i>]) -> Option<crate::TokenizeOutput<'i, Self::Token<'i>>> {
        if input.len() == 0 {
            return None;
        }

        let first_entry_key: BulletListEntryKeyword = input[0].try_into().ok()?;
    }

    fn parse(input: Vec<Self::Token<'_>>) -> Option<crate::elements::Blocks> {
        todo!()
    }
}

pub enum ConversionError {
    CannotConvertSymbol,
}
