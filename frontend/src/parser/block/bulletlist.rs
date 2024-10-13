use crate::span::Span;

/// Enum representing the keyword used to create a [`BulletListEntry`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BulletListEntryKeyword {
    /// Minus keyword: `-`
    Minus,
    /// Plus keyword: `+`
    Plus,
    /// Star keyword: `*`
    Star,
}

/// Structure of a Unimarkup bullet list entry.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BulletListEntry {
    /// The [`BulletListEntryKeyword`] used to create this entry.
    pub keyword: BulletListEntryKeyword,
    /// The entry heading content of this entry.
    pub heading: Vec<String>,
    /// The body of this entry.
    pub body: Vec<super::Block>,
    /// The span this element occupies in the Unimarkup input.
    pub span: Span,
}

/// Structure of a Unimarkup bullet list element.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BulletList {
    /// The list entries of this bullet list.
    pub entries: Vec<BulletListEntry>,
    /// The span this element occupies in the Unimarkup input.
    pub span: Span,
}
