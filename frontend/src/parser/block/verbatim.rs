use crate::span::Span;

/// Structure of a Unimarkup verbatim block element.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Verbatim {
    /// The content of the verbatim block.
    pub content: String,
    /// The language used to highlight the content.
    pub data_lang: Option<String>,
    /// Attributes of the verbatim block.
    // TODO: make attributes data structure
    pub attributes: Option<String>,
    /// Marks that this verbatim block was implicitly closed.
    pub implicit_closed: bool,
    /// The number of backticks this verbatim block was created with.
    pub tick_len: usize,
    /// The span this element occupies in the Unimarkup input.
    pub span: Span,
}
