use std::rc::Rc;

use unimarkup_commons::{
    parser::{GroupParser, Parser},
    scanner::{EndMatcher, SymbolKind},
};

use crate::{
    element::{Inline, InlineElement, InlineError},
    new_parser::InlineParser,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Italic {
    pub(crate) inner: Vec<Inline>,
}

impl InlineElement for Italic {}

impl Parser<Inline> for Italic {
    fn parse(input: &mut unimarkup_commons::scanner::SymbolIterator) -> Option<Inline> {
        let first_symbol = input.next()?;
        let second_symbol = input.peek()?;

        if first_symbol.kind != SymbolKind::Star
            || second_symbol.kind == SymbolKind::Star
            || second_symbol.kind.is_space()
        {
            return None;
        }

        let mut inner_iter = input.nest_scoped(
            None,
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                !matcher.prev_is_space()
                    && !matcher.matches(&[SymbolKind::Star, SymbolKind::Star])
                    && matcher.consumed_matches(&[SymbolKind::Star])
            })),
        );

        let inline_parser = InlineParser::default();
        let inner = inline_parser.parse(&mut inner_iter);

        inner_iter.update(input);

        Some(Italic { inner }.into())
    }
}

impl From<Italic> for Inline {
    fn from(value: Italic) -> Self {
        Inline::Italic(value)
    }
}

impl TryFrom<Inline> for Italic {
    type Error = InlineError;

    fn try_from(value: Inline) -> Result<Self, Self::Error> {
        match value {
            Inline::Italic(italic) => Ok(italic),
            _ => Err(InlineError::ConversionMismatch),
        }
    }
}
