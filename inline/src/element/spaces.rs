use super::Inline;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Whitespace {}

impl From<Whitespace> for Inline {
    fn from(value: Whitespace) -> Self {
        Inline::Whitespace(value)
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
