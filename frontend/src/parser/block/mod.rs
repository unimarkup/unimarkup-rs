use crate::span::Span;

pub mod bulletlist;
pub mod heading;
pub mod paragraph;
pub mod verbatim;

/// Generic enum for all Unimarkup block elements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    /// Represents one blankline.
    /// Needed in contexts where newlines must be kept.
    Blankline(Span),
    /// Represents the heading block
    Heading(heading::Heading),
    /// Represents the paragraph block
    Paragraph(paragraph::Paragraph),
    /// Represents the verbatim block
    Verbatim(verbatim::Verbatim),
    /// Represents the bullet list block
    BulletList(bulletlist::BulletList),
}
