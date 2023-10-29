use unimarkup_commons::scanner::position::Position;

use crate::{
    inline_parser,
    tokenize::{iterator::InlineTokenIterator, token::InlineTokenKind},
};

use self::{
    bold_italic::{Bold, Italic},
    strikethrough::Strikethrough,
    superscript::Superscript,
};

use super::Inline;

pub mod ambiguous;
pub mod bold_italic;
pub mod highlight;
pub mod math;
pub mod overline;
pub mod quote;
pub mod strikethrough;
pub mod superscript;
pub mod underline_subscript;
pub mod verbatim;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Underline {
    pub(crate) inner: Vec<Inline>,
}

impl From<Underline> for Inline {
    fn from(value: Underline) -> Self {
        Inline::Underline(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subscript {
    pub(crate) inner: Vec<Inline>,
}

impl From<Subscript> for Inline {
    fn from(value: Subscript) -> Self {
        Inline::Subscript(value)
    }
}

pub(crate) fn to_formatting(
    kind: InlineTokenKind,
    inner: Vec<Inline>,
    attributes: Option<Vec<Inline>>,
    start: Position,
    end: Position,
    implicit_end: bool,
) -> Inline {
    match kind {
        InlineTokenKind::Bold => Bold { inner }.into(),
        InlineTokenKind::Italic => Italic { inner }.into(),

        InlineTokenKind::Underline => Underline { inner }.into(),
        InlineTokenKind::Subscript => Subscript { inner }.into(),
        InlineTokenKind::Superscript => Superscript { inner }.into(),
        InlineTokenKind::Overline => todo!(),
        InlineTokenKind::Strikethrough => Strikethrough { inner }.into(),
        InlineTokenKind::Highlight => todo!(),
        InlineTokenKind::Verbatim => todo!(),
        InlineTokenKind::Quote => todo!(),
        InlineTokenKind::Math => todo!(),

        InlineTokenKind::NamedSubstitution => todo!(),

        _ => panic!(
            "Tried to create inline format from non-format kind '{:?}'",
            kind
        ),
    }
}

pub fn parse_distinct_format(input: &mut InlineTokenIterator) -> Option<Inline> {
    let open_token = input.next()?;

    // No need to check for correct opening format, because parser is only assigned for valid opening tokens.
    if input.peek_kind()?.is_space() {
        return None;
    }

    input.push_format(open_token.kind);

    let inner = inline_parser::InlineParser::default().parse(input);

    let attributes = None;
    let mut implicit_end = true;

    // Only consuming token on open/close match, because closing token might be reserved for an outer open format.
    let end = if let Some(close_token) = input.peek() {
        if close_token.kind == open_token.kind {
            input.next()?;
            implicit_end = false;

            //TODO: check for optional attributes here
            close_token.end
        } else {
            close_token.start
        }
    } else {
        input
            .prev_token()
            .expect("Previous token must exist here, because format was opened.")
            .end
    };

    input.pop_format(open_token.kind);
    Some(to_formatting(
        open_token.kind,
        inner,
        attributes,
        open_token.start,
        end,
        implicit_end,
    ))
}
