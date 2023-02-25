//! Defines the UnimarkupBlockKind a Unimarkup document consists of

use crate::backend::Render;
use crate::elements::HeadingBlock;
use crate::elements::ParagraphBlock;
use crate::elements::VerbatimBlock;

/// Generate implementation of From<Block> trait for UnimarkupBlockKind for a unimarkup block struct
///
/// ## Usage
///
/// ```ignore
/// impl_from!(Heading from HeadingBlock);
///
/// // expands to
///
/// impl From<HeadingBlock> for UnimarkupBlockKind {
///     fn from(block: block) -> Self {
///         Self::Heading
///     }
/// }
/// ```
macro_rules! impl_from {
    ($($variant:ident from $struct:ty),*) => {
        $(
            impl From<$struct> for UnimarkupBlock {
                fn from(block: $struct) -> Self {
                    Self::$variant(block)
                }
            }
        )*
    };
}

/// Enum of supported Unimarkup block elements
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnimarkupBlock {
    /// Represents the heading block
    Heading(HeadingBlock),
    /// Represents the paragraph block
    Paragraph(ParagraphBlock),
    /// Represents the verbatim block
    Verbatim(VerbatimBlock),
}

impl_from!(Heading from HeadingBlock);
impl_from!(Verbatim from VerbatimBlock);
impl_from!(Paragraph from ParagraphBlock);

impl Render for UnimarkupBlock {
    fn render_html(&self) -> Result<String, crate::backend::error::BackendError> {
        match self {
            UnimarkupBlock::Heading(heading) => heading.render_html(),
            UnimarkupBlock::Paragraph(paragraph) => paragraph.render_html(),
            UnimarkupBlock::Verbatim(verbatim) => verbatim.render_html(),
        }
    }
}
