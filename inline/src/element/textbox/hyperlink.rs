use crate::element::{Inline, InlineError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hyperlink {
    pub(crate) inner: Vec<Inline>,
    pub(crate) link: String,
    pub(crate) alt_text: Option<String>,
}

impl From<Hyperlink> for Inline {
    fn from(value: Hyperlink) -> Self {
        Inline::Hyperlink(value)
    }
}

impl TryFrom<Inline> for Hyperlink {
    type Error = InlineError;

    fn try_from(value: Inline) -> Result<Self, Self::Error> {
        match value {
            Inline::Hyperlink(hyperlink) => Ok(hyperlink),
            _ => Err(InlineError::ConversionMismatch),
        }
    }
}
