//! Defines the UnimarkupBlockKind a Unimarkup document consists of

use crate::backend::Render;
use crate::elements::HeadingBlock;
use crate::elements::ParagraphBlock;
use crate::elements::VerbatimBlock;

/// Enum of supported Unimarkup block elements
#[derive(Debug, Clone)]
pub enum UnimarkupBlockKind {
    /// Represents the heading block
    Heading(HeadingBlock),
    /// Represents the paragraph block
    Paragraph(ParagraphBlock),
    /// Represents the verbatim block
    Verbatim(VerbatimBlock),
}

impl Render for UnimarkupBlockKind {
    fn render_html(&self) -> Result<String, crate::backend::error::BackendError> {
        match self {
            UnimarkupBlockKind::Heading(heading) => heading.render_html(),
            UnimarkupBlockKind::Paragraph(paragraph) => paragraph.render_html(),
            UnimarkupBlockKind::Verbatim(verbatim) => verbatim.render_html(),
        }
    }
}
