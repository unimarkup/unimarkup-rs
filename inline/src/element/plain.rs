use unimarkup_commons::parser::Parser;

use super::{Inline, InlineElement, InlineError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plain {
    pub(crate) content: String,
}

impl InlineElement for Plain {}

impl Parser<Inline> for Plain {
    fn parse(input: &mut unimarkup_commons::scanner::SymbolIterator) -> Option<Inline> {
        let symbol = input.next()?;

        Some(
            Plain {
                content: symbol.as_str().to_string(),
            }
            .into(),
        )
    }
}

impl From<Plain> for Inline {
    fn from(value: Plain) -> Self {
        Inline::Plain(value)
    }
}

impl TryFrom<Inline> for Plain {
    type Error = InlineError;

    fn try_from(value: Inline) -> Result<Self, Self::Error> {
        match value {
            Inline::Plain(plain) => Ok(plain),
            _ => Err(InlineError::ConversionMismatch),
        }
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

impl TryFrom<Inline> for EscapedPlain {
    type Error = InlineError;

    fn try_from(value: Inline) -> Result<Self, Self::Error> {
        match value {
            Inline::EscapedPlain(escaped_plain) => Ok(escaped_plain),
            _ => Err(InlineError::ConversionMismatch),
        }
    }
}
