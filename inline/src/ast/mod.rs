//! This module provides types and functionality to create a Unimarkup inline AST out of a given list of tokens.

use crate::tokenizer::{Position, TokenKind};

pub(crate) mod collect;
mod substitutions;

/// Represents an AST of Unimarkup inline elements
pub type Inline = Vec<InlineKind>;

/// Convenient function to convert a string into plain inline.
pub fn flat_inline(s: &str) -> Inline {
  vec![InlineKind::Plain(FlatInline{ content: s.to_string(), span: Span::default() })]
}

/// Struct to set the span of an inline element in a given input.
/// 
/// Note: If the inline element only consists of one grapheme, start and end point to the same position.
#[derive(Debug, Default, Clone, PartialEq, Copy)]
pub struct Span {
  /// The start position of an inline element.
  pub start: Position,
  /// The end position of an inline element.
  pub end: Position,
}

/// Struct representing inline elements that allow nesting.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NestedInline {
  pub content: Vec<InlineKind>, 
  pub span: Span
}

/// Struct representing inline elements that do not allow nesting.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FlatInline {
  pub content: String,
  pub span: Span,
}

/// Enum representing all supported Unimarkup inline elements.
#[derive(Debug, Clone, PartialEq)]
pub enum InlineKind {
  /// Representing the bold inline element.
  Bold(NestedInline),
  /// Representing the italic inline element.
  Italic(NestedInline),
  /// Representing the combined bold and italic inline element.
  BoldItalic(NestedInline),
  /// Representing the verbatim inline element.
  Verbatim(FlatInline),
  /// Representing plain text.
  Plain(FlatInline),
  /// Representing excplicit newlines.
  EscapedNewLine(FlatInline),
  /// Representing explicit spaces.
  EscapedSpace(FlatInline),
}

/// Trait to flatten inline elements.

pub trait FlattenInlineKind {
  /// This function converts an inline element back into its original plain representation.
  /// 
  /// e.g. `Bold(Plain(text))` --> `**text**`
  fn flatten(self) -> String;
}

impl FlattenInlineKind for Vec<InlineKind> {
  fn flatten(self) -> String {
    let mut s: String = String::new();

    for inline in self {
      s.push_str(&inline.flatten());
    }

    s
  }
}

impl FlattenInlineKind for InlineKind {
  fn flatten(self) -> String {
    match self {
      InlineKind::Bold(nested) => {
        let mut s = String::from(TokenKind::BoldOpen.as_str());
        s.push_str(&nested.content.flatten());
        s.push_str(TokenKind::BoldClose.as_str());
        s
      },
      InlineKind::Italic(nested) => {
        let mut s = String::from(TokenKind::ItalicOpen.as_str());
        s.push_str(&nested.content.flatten());
        s.push_str(TokenKind::ItalicClose.as_str());
        s
      },
      InlineKind::BoldItalic(nested) => {
        let mut s = String::from(TokenKind::BoldItalicOpen.as_str());
        s.push_str(&nested.content.flatten());
        s.push_str(TokenKind::BoldItalicClose.as_str());
        s
      },
      InlineKind::Verbatim(flat) => {
        let mut s = String::from(TokenKind::VerbatimOpen.as_str());
        s.push_str(&flat.content);
        s.push_str(TokenKind::VerbatimClose.as_str());
        s
      },
      InlineKind::Plain(flat) => {
        flat.content
      },
      InlineKind::EscapedNewLine(flat)
      | InlineKind::EscapedSpace(flat) => {
        let mut s = String::from("\\");
        s.push_str(&flat.content);
        s
      },
    }
  }
}

