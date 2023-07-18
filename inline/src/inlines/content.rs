use std::collections::VecDeque;

use crate::Inline;

/// Enum representing a reference to the content of an Inline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentRef<'a> {
    /// Reference to the content of any [`Inline`] that contains only String.
    Plain(&'a str),
    /// Reference to the content of any [`Inline`] that can contain other Inlines.
    Nested(&'a VecDeque<Inline>),
}

impl From<ContentRef<'_>> for String {
    fn from(content: ContentRef<'_>) -> Self {
        match content {
            ContentRef::Plain(plain) => plain.to_string(),
            ContentRef::Nested(inner) => inner.iter().map(Inline::as_string).collect(),
        }
    }
}
