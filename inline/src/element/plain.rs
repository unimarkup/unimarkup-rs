use super::Inline;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plain {
    pub(crate) content: String,
}

impl From<Plain> for Inline {
    fn from(value: Plain) -> Self {
        Inline::Plain(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EscapedPlain {
    pub(crate) content: String,
}

impl From<EscapedPlain> for Inline {
    fn from(value: EscapedPlain) -> Self {
        Inline::EscapedPlain(value)
    }
}
