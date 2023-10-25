use std::rc::Rc;

use unimarkup_commons::{
    parser::{GroupParser, Parser},
    scanner::{EndMatcher, SymbolKind},
};

use crate::{
    element::{Inline, InlineElement, InlineError},
    new_parser::InlineParser,
};

pub const VERBATIM_KEYWORD_LIMIT: &[SymbolKind] = &[SymbolKind::Tick, SymbolKind::Tick];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verbatim {
    pub(crate) inner: Vec<Inline>,
}

impl InlineElement for Verbatim {}

impl Parser<Inline> for Verbatim {
    fn parse(input: &mut unimarkup_commons::scanner::SymbolIterator) -> Option<Inline> {
        // New scope to prevent other elements from matching keywords.
        // Needed in case `peek()` would already return a keyword
        let mut start_iter = input.nest_with_scope(None, None);

        let first_symbol = start_iter.next()?;
        let second_symbol = start_iter.peek()?;

        if first_symbol.kind != SymbolKind::Tick
            || second_symbol.kind == SymbolKind::Tick
            || second_symbol.kind.is_space()
        {
            return None;
        }

        start_iter.update(input);

        // New scope to prevent other elements from matching keywords.
        let mut inner_iter = input.nest_with_scope(
            None,
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                !matcher.prev_is_space()
                    && !matcher.matches(VERBATIM_KEYWORD_LIMIT)
                    && matcher.consumed_matches(&[SymbolKind::Tick])
            })),
        );

        // Note: `base()` only parses Plain, spaces, and escaped symbols
        let inline_parser = InlineParser::base();
        let inner = inline_parser.parse(&mut inner_iter);

        inner_iter.update(input);

        Some(Verbatim { inner }.into())
    }
}

impl From<Verbatim> for Inline {
    fn from(value: Verbatim) -> Self {
        Inline::Verbatim(value)
    }
}

impl TryFrom<Inline> for Verbatim {
    type Error = InlineError;

    fn try_from(value: Inline) -> Result<Self, Self::Error> {
        match value {
            Inline::Verbatim(verbatim) => Ok(verbatim),
            _ => Err(InlineError::ConversionMismatch),
        }
    }
}

#[cfg(test)]
mod test {
    use unimarkup_commons::scanner::SymbolIterator;

    use crate::element::plain::{EscapedPlain, Plain};

    use super::*;

    #[test]
    fn parse_verbatim() {
        let symbols = unimarkup_commons::scanner::scan_str("`verbatim`");
        let mut sym_iter = SymbolIterator::from(&*symbols);

        let inline = Verbatim::parse(&mut sym_iter).unwrap();

        assert_eq!(
            Verbatim::try_from(inline).unwrap(),
            Verbatim {
                inner: vec![Plain {
                    content: "verbatim".to_string(),
                }
                .into()],
            },
            "Verbatim not correctly parsed."
        );

        assert_eq!(
            sym_iter.next().unwrap().kind,
            SymbolKind::EOI,
            "Iterator returned symbols after EOI"
        );
    }

    #[test]
    fn parse_verbatim_with_contiguous_ticks_as_plain() {
        let symbols = unimarkup_commons::scanner::scan_str("`verb``atim`");
        let mut sym_iter = SymbolIterator::from(&*symbols);

        let inline = Verbatim::parse(&mut sym_iter).unwrap();

        assert_eq!(
            Verbatim::try_from(inline).unwrap(),
            Verbatim {
                inner: vec![Plain {
                    content: "verb``atim".to_string(),
                }
                .into()],
            },
            "Verbatim with contiguous ticks not correctly parsed."
        );

        assert_eq!(
            sym_iter.next().unwrap().kind,
            SymbolKind::EOI,
            "Iterator returned symbols after EOI"
        );
    }

    #[test]
    fn parse_verbatim_with_escaped_tick() {
        let symbols = unimarkup_commons::scanner::scan_str("`ver\\`batim`");
        let mut sym_iter = SymbolIterator::from(&*symbols);

        let inline = Verbatim::parse(&mut sym_iter).unwrap();

        assert_eq!(
            Verbatim::try_from(inline).unwrap(),
            Verbatim {
                inner: vec![
                    Plain {
                        content: "ver".to_string(),
                    }
                    .into(),
                    EscapedPlain {
                        content: "`".to_string(),
                    }
                    .into(),
                    Plain {
                        content: "batim".to_string(),
                    }
                    .into()
                ],
            },
            "Verbatim not correctly parsed with escaped tick inside."
        );

        assert_eq!(
            sym_iter.next().unwrap().kind,
            SymbolKind::EOI,
            "Iterator returned symbols after EOI"
        );
    }
}
