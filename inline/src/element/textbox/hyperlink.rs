//! Contains the [`Hyperlink`] element.

use unimarkup_commons::lexer::position::Position;

use crate::element::{Inline, InlineElement};

/// Represents a Unimarkup hyperlink.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hyperlink {
    inner: Vec<Inline>,
    link: String,
    link_text: Option<String>,
    attributes: Option<Vec<Inline>>,
    start: Position,
    end: Position,
}

impl Hyperlink {
    pub fn new(
        inner: Vec<Inline>,
        link: String,
        link_text: Option<String>,
        attributes: Option<Vec<Inline>>,
        start: Position,
        end: Position,
    ) -> Self {
        Self {
            inner,
            link,
            link_text,
            attributes,
            start,
            end,
        }
    }

    /// Returns the content inside the brackets of the hyperlink.
    pub fn inner(&self) -> &Vec<Inline> {
        &self.inner
    }

    /// Returns the link of this hyperlink.
    pub fn link(&self) -> &str {
        &self.link
    }

    /// Returns the optional link text of this hyperlink.
    pub fn link_text(&self) -> Option<&str> {
        self.link_text.as_deref()
    }

    pub fn attributes(&self) -> Option<&Vec<Inline>> {
        self.attributes.as_ref()
    }
}

impl From<Hyperlink> for Inline {
    fn from(value: Hyperlink) -> Self {
        Inline::Hyperlink(value)
    }
}

impl InlineElement for Hyperlink {
    fn to_plain_string(&self) -> String {
        format!("[{}]({})", self.inner.to_plain_string(), self.link)
    }

    fn start(&self) -> Position {
        self.start
    }

    fn end(&self) -> Position {
        self.end
    }
}
