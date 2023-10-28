use std::rc::Rc;

use unimarkup_commons::{
    parser::{GroupParser, Parser},
    scanner::{
        span::Span,
        token::{iterator::EndMatcher, TokenKind},
        SymbolKind,
    },
};

use crate::{
    inline_parser,
    tokenize::{iterator::InlineTokenIterator, token::InlineTokenKind},
};

use self::hyperlink::Hyperlink;

use super::{Inline, InlineElement, InlineError};

pub mod hyperlink;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextBox {
    pub(crate) inner: Vec<Inline>,
}

pub fn parse(input: &mut InlineTokenIterator) -> Option<Inline> {
    let open_token = input.next()?;

    if open_token.kind != InlineTokenKind::OpenBracket {
        return None;
    }

    let mut scoped_iter: InlineTokenIterator<'_> = input
        .nest_with_scope(Some(Rc::new(|matcher: &mut dyn EndMatcher| {
            matcher.consumed_matches(&[TokenKind::CloseBracket])
        })))
        .into();

    if let Some(box_variant) = parse_box_variant(&mut scoped_iter) {
        scoped_iter.update(input);
        return Some(box_variant);
    }

    // No variant matched => must be regular textbox or hyperlink

    let inner = inline_parser::InlineParser::default().parse(&mut scoped_iter);

    //TODO: get prev token from scoped_iter to get span of closing token, or of implicit close
    let end_reached = scoped_iter.end_reached();
    scoped_iter.update(input);

    // check for `()`
    if end_reached && input.peek_kind() == Some(InlineTokenKind::OpenParenthesis) {
        input.next()?; // Consume open parenthesis
        let mut link_iter: InlineTokenIterator<'_> = input
            .nest_with_scope(Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.consumed_matches(&[TokenKind::CloseParenthesis])
            })))
            .into();

        let link = link_iter.by_ref().take_while(|t| !t.kind.is_space()).fold(
            String::default(),
            |mut combined, token| {
                combined.push_str(token.as_str());
                combined
            },
        );
        let alt_text =
            link_iter
                .take_to_end()
                .iter()
                .fold(String::default(), |mut combined, token| {
                    combined.push_str(token.as_str());
                    combined
                });

        link_iter.update(input);

        return Some(
            Hyperlink {
                inner,
                link,
                alt_text: if alt_text.is_empty() {
                    None
                } else {
                    Some(alt_text)
                },
            }
            .into(),
        );
    }

    Some(TextBox { inner }.into())
}

fn parse_box_variant(input: &mut InlineTokenIterator) -> Option<Inline> {
    //TODO: implement box variants like media insert...

    None
}

impl InlineElement for TextBox {}

impl Parser<Inline> for TextBox {
    fn parse(input: &mut unimarkup_commons::scanner::SymbolIterator) -> Option<Inline> {
        // let first_symbol = input.next()?;

        // if first_symbol.kind != SymbolKind::OpenBracket {
        //     return None;
        // }

        // // New scope to prevent other elements from matching keywords.
        // let mut inner_iter = input.nest_with_scope(
        //     None,
        //     Some(Rc::new(|matcher: &mut dyn EndMatcher| {
        //         matcher.consumed_matches(&[SymbolKind::CloseBracket])
        //     })),
        // );

        // // Note: `base()` only parses Plain, spaces, and escaped symbols
        // let inline_parser = InlineParser::default();
        // let inner = inline_parser.parse(&mut inner_iter);
        // let end_reached = inner_iter.end_reached();

        // inner_iter.update(input);

        // // check for `()`
        // if end_reached {
        //     let mut link_iter = input.nest_with_scope(
        //         None,
        //         Some(Rc::new(|matcher: &mut dyn EndMatcher| {
        //             matcher.consumed_matches(&[SymbolKind::CloseParenthesis])
        //         })),
        //     );

        //     // TODO: Replace Plain with Any kind
        //     let next_kind = link_iter.next().map_or(SymbolKind::Plain, |s| s.kind);
        //     if next_kind == SymbolKind::OpenParenthesis {
        //         let link =
        //             link_iter
        //                 .take_to_end()
        //                 .iter()
        //                 .fold(String::default(), |mut combined, s| {
        //                     combined.push_str(s.as_str());
        //                     combined
        //                 });

        //         link_iter.update(input);

        //         return Some(Hyperlink { inner, link }.into());
        //     }
        // }

        // Some(TextBox { inner }.into())
        todo!()
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

#[cfg(test)]
mod test {
    use unimarkup_commons::scanner::token::iterator::TokenIterator;

    use crate::{
        element::{
            plain::Plain,
            textbox::{hyperlink::Hyperlink, TextBox},
        },
        tokenize::iterator::InlineTokenIterator,
    };

    #[test]
    fn parse_textbox() {
        let symbols = unimarkup_commons::scanner::scan_str("[textbox]");
        let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

        let inline = super::parse(&mut token_iter).unwrap();

        assert_eq!(
            inline,
            TextBox {
                inner: vec![Plain {
                    content: "textbox".to_string(),
                }
                .into()],
            }
            .into(),
            "Textbox not correctly parsed."
        );
    }

    #[test]
    fn parse_hyperlink() {
        let symbols = unimarkup_commons::scanner::scan_str("[](link)");
        let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

        let inline = super::parse(&mut token_iter).unwrap();

        assert_eq!(
            inline,
            Hyperlink {
                inner: vec![],
                link: "link".to_string(),
                alt_text: None,
            }
            .into(),
            "Hyperlink not correctly parsed."
        );
    }
}
