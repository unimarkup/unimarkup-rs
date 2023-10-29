use unimarkup_commons::scanner::position::Position;

use crate::tokenize::token::InlineTokenKind;

use self::{
    bold_italic::{Bold, Italic},
    strikethrough::Strikethrough,
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
        InlineTokenKind::Superscript => todo!(),
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
