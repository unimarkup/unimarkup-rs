use crate::span::Span;

/// Structure of a Unimarkup paragraph element.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Paragraph {
    /// The content of the paragraph.
    pub content: Vec<String>,

    /// The span this element occupies in the Unimarkup input.
    pub span: Span,
}
