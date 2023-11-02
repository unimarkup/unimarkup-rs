use std::rc::Rc;

use unimarkup_commons::{
    lexer::{
        position::Position,
        token::{iterator::EndMatcher, TokenKind},
    },
    parsing::InlineContext,
};

use crate::tokenize::{iterator::InlineTokenIterator, kind::InlineTokenKind};

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

pub(crate) fn parse(
    input: &mut InlineTokenIterator,
    context: &mut InlineContext,
) -> Option<Inline> {
    let open_token = input.next()?;

    debug_assert_eq!(
        open_token.kind,
        InlineTokenKind::OpenBracket,
        "Called TextBox parser on open kind '{:?}'.",
        open_token.kind
    );

    let mut scoped_iter = input.nest_with_scope(Some(Rc::new(|matcher: &mut dyn EndMatcher| {
        matcher.consumed_matches(&[TokenKind::CloseBracket])
    })));

    if let Some(box_variant) = parse_box_variant(&mut scoped_iter) {
        input.progress(scoped_iter);
        return Some(box_variant);
    }

    // No variant matched => must be regular textbox or hyperlink

    let inner = crate::inline_parser::parse(&mut scoped_iter, context);

    let prev_token = if inner.is_empty() {
        open_token
    } else {
        scoped_iter
            .prev_token()
            .expect("Inlines in textbox => previous token must exist.")
    };
    let end_reached = scoped_iter.end_reached();
    input.progress(scoped_iter);

    // check for `()`
    if end_reached && input.peek_kind() == Some(InlineTokenKind::OpenParenthesis) {
        input
            .next()
            .expect("Peeked before, so `next` must return Some."); // Consume open parenthesis
        let mut link_iter: InlineTokenIterator =
            input.nest_with_scope(Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.consumed_matches(&[TokenKind::CloseParenthesis])
            })));

        let link = link_iter.by_ref().take_while(|t| !t.kind.is_space()).fold(
            String::default(),
            |mut combined, token| {
                combined.push_str(token.as_str());
                combined
            },
        );
        let link_text =
            link_iter
                .take_to_end()
                .iter()
                .fold(String::default(), |mut combined, token| {
                    combined.push_str(token.as_str());
                    combined
                });

        let link_close_token = if link_text.is_empty() && !link_iter.end_reached() {
            prev_token
        } else {
            link_iter.prev_token().expect(
                "Link text has content or closing parenthesis found => previous token must exist.",
            )
        };

        input.progress(link_iter);

        return Some(
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
        );
    }

    Some(
        TextBox {
            inner,
            attributes: None,
            start: open_token.start,
            end: crate::element::helper::implicit_end_using_prev(&prev_token),
        }
        .into(),
    )
}

fn parse_box_variant(_input: &mut InlineTokenIterator) -> Option<Inline> {
    //TODO: implement box variants like media insert...

    None
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
