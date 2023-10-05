//! Defines the generic Unimarkup Block that is the base for all block elements.

use super::{
    atomic::{Heading, Paragraph},
    enclosed::Verbatim,
    indents::{BulletList, BulletListEntry},
};

/// Generic enum for all Unimarkup block elements.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    /// Represents the heading block
    Heading(Heading),
    /// Represents the paragraph block
    Paragraph(Paragraph),
    /// Represents the verbatim block
    Verbatim(Verbatim),
    /// Represents the bullet list block
    BulletList(BulletList),
    /// Represents the bullet list entry block
    BulletListEntry(BulletListEntry),
}

impl Block {
    /// Returns the variant of [`Block`] as string.
    ///
    /// # Example:
    ///
    /// ```
    /// # use unimarkup_parser::elements::{blocks::Block, atomic::Paragraph};
    /// # fn get_paragraph() -> Paragraph {
    /// #     Paragraph {
    /// #         id: String::new(),
    /// #         content: vec![],
    /// #         attributes: None,
    /// #         line_nr: 0,
    /// #     }
    /// # }
    /// let block = Block::Paragraph(get_paragraph());
    ///
    /// assert_eq!(block.variant_str(), "Paragraph");
    /// ```
    pub fn variant_str(&self) -> &'static str {
        match self {
            Block::Heading(_) => "Heading",
            Block::Paragraph(_) => "Paragraph",
            Block::Verbatim(_) => "Verbatim",
            Block::BulletList(_) => "BulletList",
            Block::BulletListEntry(_) => "BulletListEntry",
        }
    }
}

/// Generate implementation of From<Block> trait for Block for a unimarkup block struct
///
/// ## Usage
///
/// ```ignore
/// impl_from!(Heading from Heading);
///
/// // expands to
///
/// impl From<Heading> for Block {
///     fn from(block: block) -> Self {
///         Self::Heading
///     }
/// }
/// ```
macro_rules! impl_from {
  ($($variant:ident from $struct:ty),*) => {
      $(
          impl From<$struct> for Block {
              fn from(block: $struct) -> Self {
                  Self::$variant(block)
              }
          }
      )*
  };
}

impl_from!(Heading from Heading);
impl_from!(Verbatim from Verbatim);
impl_from!(Paragraph from Paragraph);
impl_from!(BulletList from BulletList);
impl_from!(BulletListEntry from BulletListEntry);
