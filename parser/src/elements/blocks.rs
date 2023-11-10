//! Defines the generic Unimarkup Block that is the base for all block elements.

use unimarkup_commons::lexer::{position::Position, span::Span, token::TokenKind};

use super::{
    atomic::{Heading, Paragraph},
    enclosed::VerbatimBlock,
    indents::{BulletList, BulletListEntry},
    BlockElement,
};

/// Generic enum for all Unimarkup block elements.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    /// Represents one blankline.
    /// Needed in contexts where newlines must be kept.
    Blankline(Span),
    /// Represents the heading block
    Heading(Heading),
    /// Represents the paragraph block
    Paragraph(Paragraph),
    /// Represents the verbatim block
    VerbatimBlock(VerbatimBlock),
    /// Represents the bullet list block
    BulletList(BulletList),
    /// Represents the bullet list entry block
    BulletListEntry(BulletListEntry),
}

impl Block {
    /// Returns the variant of [`Block`] as string.
    /// e.g. "Paragraph" for [`Block::Paragraph`]
    pub fn variant_str(&self) -> &'static str {
        match self {
            Block::Blankline(_) => "Blankline",
            Block::Heading(_) => "Heading",
            Block::Paragraph(_) => "Paragraph",
            Block::VerbatimBlock(_) => "VerbatimBlock",
            Block::BulletList(_) => "BulletList",
            Block::BulletListEntry(_) => "BulletListEntry",
        }
    }
}

impl BlockElement for Block {
    fn to_plain_string(&self) -> String {
        match self {
            Block::Blankline(_) => String::from(TokenKind::Blankline),
            Block::Heading(block) => block.to_plain_string(),
            Block::Paragraph(block) => block.to_plain_string(),
            Block::VerbatimBlock(block) => block.to_plain_string(),
            Block::BulletList(block) => block.to_plain_string(),
            Block::BulletListEntry(block) => block.to_plain_string(),
        }
    }

    fn start(&self) -> unimarkup_commons::lexer::position::Position {
        match self {
            Block::Blankline(span) => span.start,
            Block::Heading(block) => block.start(),
            Block::Paragraph(block) => block.start(),
            Block::VerbatimBlock(block) => block.start(),
            Block::BulletList(block) => block.start(),
            Block::BulletListEntry(block) => block.start(),
        }
    }

    fn end(&self) -> unimarkup_commons::lexer::position::Position {
        match self {
            Block::Blankline(span) => span.end,
            Block::Heading(block) => block.end(),
            Block::Paragraph(block) => block.end(),
            Block::VerbatimBlock(block) => block.end(),
            Block::BulletList(block) => block.end(),
            Block::BulletListEntry(block) => block.end(),
        }
    }
}

impl BlockElement for Vec<Block> {
    fn to_plain_string(&self) -> String {
        self.iter().fold(String::default(), |mut combined, block| {
            combined.push_str(&block.to_plain_string());
            combined
        })
    }

    fn start(&self) -> unimarkup_commons::lexer::position::Position {
        match self.first() {
            Some(first) => first.start(),
            None => Position::default(),
        }
    }

    fn end(&self) -> unimarkup_commons::lexer::position::Position {
        match self.last() {
            Some(last) => last.end(),
            None => Position::default(),
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
impl_from!(VerbatimBlock from VerbatimBlock);
impl_from!(Paragraph from Paragraph);
impl_from!(BulletList from BulletList);
impl_from!(BulletListEntry from BulletListEntry);
