use super::{Inline, InlineError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Whitespace {}

impl From<Whitespace> for Inline {
    fn from(value: Whitespace) -> Self {
        Inline::Whitespace(value)
    }
}

impl TryFrom<Inline> for Whitespace {
    type Error = InlineError;

    fn try_from(value: Inline) -> Result<Self, Self::Error> {
        match value {
            Inline::Whitespace(whitespace) => Ok(whitespace),
            _ => Err(InlineError::ConversionMismatch),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EscapedWhitespace {
    pub(crate) space: String,
}

impl From<EscapedWhitespace> for Inline {
    fn from(value: EscapedWhitespace) -> Self {
        Inline::EscapedWhitespace(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Newline {}

impl From<Newline> for Inline {
    fn from(value: Newline) -> Self {
        Inline::Newline(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EscapedNewline {}

impl From<EscapedNewline> for Inline {
    fn from(value: EscapedNewline) -> Self {
        Inline::EscapedNewline(value)
    }
}
