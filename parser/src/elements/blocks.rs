//! Defines the generic Unimarkup Block that is the base for all block elements.

use super::{
    atomic::{Heading, Paragraph},
    enclosed::Verbatim,
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
