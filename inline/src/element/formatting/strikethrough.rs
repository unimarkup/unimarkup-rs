use std::rc::Rc;

use unimarkup_commons::{
    parser::{GroupParser, Parser},
    scanner::{EndMatcher, SymbolKind},
};

use crate::{
    element::{Inline, InlineElement, InlineError},
    inline_parser,
    new_parser::InlineParser,
    tokenize::{iterator::InlineTokenIterator, token::InlineTokenKind},
};

pub const STRIKETHROUGH_KEYWORD_LIMIT: &[SymbolKind] =
    &[SymbolKind::Tilde, SymbolKind::Tilde, SymbolKind::Tilde];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Strikethrough {
    pub(crate) inner: Vec<Inline>,
}

pub fn parse(input: &mut InlineTokenIterator) -> Option<Inline> {
    let open_token = input.next()?;

    if input.peek_kind()?.is_space() || open_token.kind != InlineTokenKind::Strikethrough {
        return None;
    }

    input.push_format(open_token.kind);

    let inner = inline_parser::InlineParser::default().parse(input);

    // Only consuming token on open/close match, because closing token might be reserved for an outer open format.
    if let Some(close_token) = input.peek() {
        if close_token.kind == open_token.kind {
            input.next()?;
        }
    }

    input.pop_format(open_token.kind);
    Some(Strikethrough { inner }.into())
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

#[cfg(test)]
mod test {
    use unimarkup_commons::scanner::token::iterator::TokenIterator;

    use crate::element::{plain::Plain, spaces::Whitespace};

    use super::*;

    #[test]
    fn parse_strikethrough() {
        let symbols = unimarkup_commons::scanner::scan_str("~~strikethrough~~");
        let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

        let inline = parse(&mut token_iter).unwrap();

        assert_eq!(
            inline,
            Strikethrough {
                inner: vec![Plain {
                    content: "strikethrough".to_string(),
                }
                .into()],
            }
            .into(),
            "Strikethrough not correctly parsed."
        )
    }

    #[test]
    fn parse_strikethrough_invalid_close_but_implicit_end() {
        let symbols = unimarkup_commons::scanner::scan_str("~~strike ~~");
        let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

        let inline = parse(&mut token_iter).unwrap();

        assert_eq!(
            inline,
            Strikethrough {
                inner: vec![
                    Plain {
                        content: "strike".to_string(),
                    }
                    .into(),
                    Whitespace {}.into(),
                    Plain {
                        content: "~~".to_string(),
                    }
                    .into()
                ],
            }
            .into(),
            "Strikethrough with invalid close not correctly parsed."
        )
    }
}
