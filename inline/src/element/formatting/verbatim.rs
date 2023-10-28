use std::rc::Rc;

use unimarkup_commons::{
    parser::{GroupParser, Parser},
    scanner::{
        token::{implicit::iterator::TokenIteratorImplicitExt, iterator::EndMatcher, TokenKind},
        SymbolKind,
    },
};

use crate::{
    element::{Inline, InlineElement, InlineError},
    inline_parser,
    new_parser::InlineParser,
    tokenize::{iterator::InlineTokenIterator, token::InlineTokenKind},
};

pub const VERBATIM_KEYWORD_LIMIT: &[SymbolKind] = &[SymbolKind::Tick, SymbolKind::Tick];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verbatim {
    pub(crate) inner: Vec<Inline>,
}

pub fn parse(input: &mut InlineTokenIterator) -> Option<Inline> {
    let open_token = input.next()?;

    if input.peek_kind()?.is_space() || open_token.kind != InlineTokenKind::Verbatim {
        return None;
    }

    let mut scoped_iter: InlineTokenIterator<'_> = input
        .nest_with_scope(Some(Rc::new(|matcher: &mut dyn EndMatcher| {
            !matcher.prev_is_space() && matcher.consumed_matches(&[TokenKind::Tick(1)])
        })))
        .into();
    scoped_iter.ignore_implicits();

    let inner = inline_parser::parse_with_macros_only(&mut scoped_iter);

    //TODO: get prev token from scoped_iter to get span of closing tick, or of implicit close

    scoped_iter.update(input);

    Some(Verbatim { inner }.into())
}

impl InlineElement for Verbatim {}

impl Parser<Inline> for Verbatim {
    fn parse(input: &mut unimarkup_commons::scanner::SymbolIterator) -> Option<Inline> {
        todo!()
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
    use unimarkup_commons::scanner::{token::iterator::TokenIterator, SymbolIterator};

    use crate::element::plain::{EscapedPlain, Plain};

    use super::*;

    #[test]
    fn parse_new_verbatim() {
        let symbols = unimarkup_commons::scanner::scan_str("`verbatim`");
        let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

        let inline = parse(&mut token_iter).unwrap();

        assert_eq!(
            inline,
            Verbatim {
                inner: vec![Plain {
                    content: "verbatim".to_string(),
                }
                .into()],
            }
            .into(),
            "Verbatim not correctly parsed."
        )
    }

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
