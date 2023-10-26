use std::rc::Rc;

use unimarkup_commons::{
    parser::{GroupParser, Parser},
    scanner::{EndMatcher, SymbolKind},
};

use crate::new_parser::InlineParser;

use self::hyperlink::Hyperlink;

use super::{Inline, InlineElement, InlineError};

pub mod hyperlink;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextBox {
    pub(crate) inner: Vec<Inline>,
}

impl InlineElement for TextBox {}

impl Parser<Inline> for TextBox {
    fn parse(input: &mut unimarkup_commons::scanner::SymbolIterator) -> Option<Inline> {
        let first_symbol = input.next()?;

        if first_symbol.kind != SymbolKind::OpenBracket {
            return None;
        }

        // New scope to prevent other elements from matching keywords.
        let mut inner_iter = input.nest_with_scope(
            None,
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.consumed_matches(&[SymbolKind::CloseBracket])
            })),
        );

        // Note: `base()` only parses Plain, spaces, and escaped symbols
        let inline_parser = InlineParser::default();
        let inner = inline_parser.parse(&mut inner_iter);
        let end_reached = inner_iter.end_reached();

        inner_iter.update(input);

        // check for `()`
        if end_reached {
            let mut link_iter = input.nest_with_scope(
                None,
                Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                    matcher.consumed_matches(&[SymbolKind::CloseParenthesis])
                })),
            );

            // TODO: Replace Plain with Any kind
            let next_kind = link_iter.next().map_or(SymbolKind::Plain, |s| s.kind);
            if next_kind == SymbolKind::OpenParenthesis {
                let link =
                    link_iter
                        .take_to_end()
                        .iter()
                        .fold(String::default(), |mut combined, s| {
                            combined.push_str(s.as_str());
                            combined
                        });

                link_iter.update(input);

                return Some(Hyperlink { inner, link }.into());
            }
        }

        Some(TextBox { inner }.into())
    }
}

impl From<TextBox> for Inline {
    fn from(value: TextBox) -> Self {
        Inline::TextBox(value)
    }
}

impl TryFrom<Inline> for TextBox {
    type Error = InlineError;

    fn try_from(value: Inline) -> Result<Self, Self::Error> {
        match value {
            Inline::TextBox(text_box) => Ok(text_box),
            _ => Err(InlineError::ConversionMismatch),
        }
    }
}
