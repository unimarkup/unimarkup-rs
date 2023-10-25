use std::rc::Rc;

use unimarkup_commons::{
    parser::{GroupParser, Parser},
    scanner::{EndMatcher, SymbolKind},
};

use crate::{
    element::{Inline, InlineElement, InlineError},
    new_parser::InlineParser,
};

use super::BOLD_ITALIC_KEYWORD_LIMIT;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bold {
    pub(crate) inner: Vec<Inline>,
}

impl InlineElement for Bold {}

impl Parser<Inline> for Bold {
    fn parse(input: &mut unimarkup_commons::scanner::SymbolIterator) -> Option<Inline> {
        let first_symbol = input.next()?;
        let second_symbol = input.next()?;
        let third_symbol = input.peek()?;

        if first_symbol.kind != SymbolKind::Star
            || second_symbol.kind != SymbolKind::Star
            || third_symbol.kind == SymbolKind::Star
            || third_symbol.kind.is_space()
        {
            return None;
        }

        let mut inner_iter = input.nest_scoped(
            None,
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                !matcher.prev_is_space()
                    // Contiguous keywords are consumed in inline parser
                    && !matcher.matches(BOLD_ITALIC_KEYWORD_LIMIT)
                    && matcher.consumed_matches(&[SymbolKind::Star, SymbolKind::Star])
            })),
        );

        let inline_parser = InlineParser::default();
        let inner = inline_parser.parse(&mut inner_iter);

        inner_iter.update(input);

        Some(Bold { inner }.into())
    }
}

impl From<Bold> for Inline {
    fn from(value: Bold) -> Self {
        Inline::Bold(value)
    }
}

impl TryFrom<Inline> for Bold {
    type Error = InlineError;

    fn try_from(value: Inline) -> Result<Self, Self::Error> {
        match value {
            Inline::Bold(bold) => Ok(bold),
            _ => Err(InlineError::ConversionMismatch),
        }
    }
}

#[cfg(test)]
mod test {
    use unimarkup_commons::scanner::SymbolIterator;

    use crate::element::plain::Plain;

    use super::*;

    #[test]
    fn parse_bold() {
        let symbols = unimarkup_commons::scanner::scan_str("**bold**");
        let mut sym_iter = SymbolIterator::from(&*symbols);

        let inline = Bold::parse(&mut sym_iter).unwrap();

        assert_eq!(
            Bold::try_from(inline).unwrap(),
            Bold {
                inner: vec![Plain {
                    content: "bold".to_string(),
                }
                .into()],
            },
            "Bold not correctly parsed."
        );

        assert_eq!(
            sym_iter.next().unwrap().kind,
            SymbolKind::EOI,
            "Iterator returned symbols after EOI"
        );
    }

    #[test]
    fn parse_bold_without_closing_token() {
        let symbols = unimarkup_commons::scanner::scan_str("**bold");
        let mut sym_iter = SymbolIterator::from(&*symbols);

        let inline = Bold::parse(&mut sym_iter).unwrap();

        assert_eq!(
            Bold::try_from(inline).unwrap(),
            Bold {
                inner: vec![Plain {
                    content: "bold".to_string(),
                }
                .into()],
            },
            "Bold not correctly parsed."
        );

        assert_eq!(sym_iter.next(), None, "Iterator did not reach EOI");
    }
}
