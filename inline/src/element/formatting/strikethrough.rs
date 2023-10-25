use std::rc::Rc;

use unimarkup_commons::{
    parser::{GroupParser, Parser},
    scanner::{EndMatcher, SymbolKind},
};

use crate::{
    element::{Inline, InlineElement, InlineError},
    new_parser::InlineParser,
};

pub const STRIKETHROUGH_KEYWORD_LIMIT: &[SymbolKind] =
    &[SymbolKind::Tilde, SymbolKind::Tilde, SymbolKind::Tilde];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Strikethrough {
    pub(crate) inner: Vec<Inline>,
}

impl InlineElement for Strikethrough {}

impl Parser<Inline> for Strikethrough {
    fn parse(input: &mut unimarkup_commons::scanner::SymbolIterator) -> Option<Inline> {
        let first_symbol = input.next()?;
        let second_symbol = input.next()?;
        let third_symbol = input.peek()?;

        if first_symbol.kind != SymbolKind::Tilde
            || second_symbol.kind != SymbolKind::Tilde
            || third_symbol.kind == SymbolKind::Tilde
            || third_symbol.kind.is_space()
        {
            return None;
        }

        let mut inner_iter = input.nest_scoped(
            None,
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                !matcher.prev_is_space()
                    // Contiguous keywords are consumed in inline parser
                    && !matcher.matches(STRIKETHROUGH_KEYWORD_LIMIT)
                    && matcher.consumed_matches(&[SymbolKind::Tilde, SymbolKind::Tilde])
            })),
        );

        let inline_parser = InlineParser::default();
        let inner = inline_parser.parse(&mut inner_iter);

        inner_iter.update(input);

        Some(Strikethrough { inner }.into())
    }
}

impl From<Strikethrough> for Inline {
    fn from(value: Strikethrough) -> Self {
        Inline::Strikethrough(value)
    }
}

impl TryFrom<Inline> for Strikethrough {
    type Error = InlineError;

    fn try_from(value: Inline) -> Result<Self, Self::Error> {
        match value {
            Inline::Strikethrough(strikethrough) => Ok(strikethrough),
            _ => Err(InlineError::ConversionMismatch),
        }
    }
}
