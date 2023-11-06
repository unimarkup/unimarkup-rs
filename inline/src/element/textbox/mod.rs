use std::rc::Rc;

use unimarkup_commons::lexer::{
    position::Position,
    token::{iterator::EndMatcher, TokenKind},
    PeekingNext,
};

use crate::{inline_parser::InlineParser, tokenize::kind::InlineTokenKind};

use self::hyperlink::Hyperlink;

use super::{Inline, InlineElement};

pub mod hyperlink;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextBox {
    inner: Vec<Inline>,
    attributes: Option<Vec<Inline>>,
    start: Position,
    end: Position,
}

impl TextBox {
    pub fn new(
        inner: Vec<Inline>,
        attributes: Option<Vec<Inline>>,
        start: Position,
        end: Position,
    ) -> Self {
        Self {
            inner,
            attributes,
            start,
            end,
        }
    }

    pub fn inner(&self) -> &Vec<Inline> {
        &self.inner
    }

    pub fn attributes(&self) -> Option<&Vec<Inline>> {
        self.attributes.as_ref()
    }
}

pub(crate) fn parse<'slice, 'input>(
    mut parser: InlineParser<'slice, 'input>,
) -> (InlineParser<'slice, 'input>, Option<Inline>) {
    let open_token_opt = parser.iter.peeking_next(|_| true);
    if open_token_opt.is_none() {
        return (parser, None);
    }

    let open_token = open_token_opt.expect("Checked above to be not None.");

    debug_assert_eq!(
        open_token.kind,
        InlineTokenKind::OpenBracket,
        "Called TextBox parser on open kind '{:?}'.",
        open_token.kind
    );

    let mut scoped_parser = parser.nest_scoped(Some(Rc::new(|matcher: &mut dyn EndMatcher| {
        matcher.consumed_matches(&[TokenKind::CloseBracket])
    })));

    let checkpoint = scoped_parser.iter.checkpoint();
    let (updated_parser, box_variant_opt) = parse_box_variant(scoped_parser);
    scoped_parser = updated_parser;

    match box_variant_opt {
        Some(box_variant) => {
            return (scoped_parser.unfold(), Some(box_variant));
        }
        None => {
            scoped_parser.iter.rollback(checkpoint);
        }
    }

    // No variant matched => must be regular textbox or hyperlink

    let (updated_parser, inner) = InlineParser::parse(scoped_parser);
    scoped_parser = updated_parser;

    let prev_token = if inner.is_empty() {
        open_token
    } else {
        scoped_parser
            .iter
            .prev_token()
            .expect("Inlines in textbox => previous token must exist.")
    };
    let end_reached = scoped_parser.iter.end_reached();
    parser = scoped_parser.unfold();

    // check for `()`
    if end_reached && parser.iter.peek_kind() == Some(InlineTokenKind::OpenParenthesis) {
        parser
            .iter
            .next()
            .expect("Peeked before, so `next` must return Some."); // Consume open parenthesis
        let mut link_parser = parser.nest_scoped(Some(Rc::new(|matcher: &mut dyn EndMatcher| {
            matcher.consumed_matches(&[TokenKind::CloseParenthesis])
        })));

        let link = link_parser
            .iter
            .by_ref()
            .take_while(|t| !t.kind.is_space())
            .fold(String::default(), |mut combined, token| {
                combined.push_str(token.as_str());
                combined
            });
        let link_text =
            link_parser
                .iter
                .take_to_end()
                .iter()
                .fold(String::default(), |mut combined, token| {
                    combined.push_str(token.as_str());
                    combined
                });

        let link_close_token = if link_text.is_empty() && !link_parser.iter.end_reached() {
            prev_token
        } else {
            link_parser.iter.prev_token().expect(
                "Link text has content or closing parenthesis found => previous token must exist.",
            )
        };

        parser = link_parser.unfold();

        return (
            parser,
            Some(
                Hyperlink::new(
                    inner,
                    link,
                    if link_text.is_empty() {
                        None
                    } else {
                        Some(link_text)
                    },
                    None,
                    open_token.start,
                    crate::element::helper::implicit_end_using_prev(&link_close_token),
                )
                .into(),
            ),
        );
    }

    (
        parser,
        Some(
            TextBox {
                inner,
                attributes: None,
                start: open_token.start,
                end: crate::element::helper::implicit_end_using_prev(&prev_token),
            }
            .into(),
        ),
    )
}

fn parse_box_variant<'slice, 'input>(
    parser: InlineParser<'slice, 'input>,
) -> (InlineParser<'slice, 'input>, Option<Inline>) {
    //TODO: implement box variants like media insert...

    (parser, None)
}

impl From<TextBox> for Inline {
    fn from(value: TextBox) -> Self {
        Inline::TextBox(value)
    }
}

impl InlineElement for TextBox {
    fn to_plain_string(&self) -> String {
        format!("[{}]", self.inner.to_plain_string())
    }

    fn start(&self) -> Position {
        self.start
    }

    fn end(&self) -> Position {
        self.end
    }
}

// #[cfg(test)]
// mod test {
//     use unimarkup_commons::lexer::token::iterator::TokenIterator;

//     use crate::{
//         element::{
//             plain::Plain,
//             textbox::{hyperlink::Hyperlink, TextBox},
//         },
//         tokenize::iterator::InlineTokenIterator,
//     };

//     #[test]
//     fn parse_textbox() {
//         let symbols = unimarkup_commons::lexer::scan_str("[textbox]");
//         let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

//         let inline = super::parse(&mut token_iter).unwrap();

//         assert_eq!(
//             inline,
//             TextBox {
//                 inner: vec![Plain {
//                     content: "textbox".to_string(),
//                 }
//                 .into()],
//             }
//             .into(),
//             "Textbox not correctly parsed."
//         );
//     }

//     #[test]
//     fn parse_hyperlink() {
//         let symbols = unimarkup_commons::lexer::scan_str("[](link)");
//         let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

//         let inline = super::parse(&mut token_iter).unwrap();

//         assert_eq!(
//             inline,
//             Hyperlink {
//                 inner: vec![],
//                 link: "link".to_string(),
//                 alt_text: None,
//             }
//             .into(),
//             "Hyperlink not correctly parsed."
//         );
//     }
// }
