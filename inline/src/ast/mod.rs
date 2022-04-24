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
  Verbatim(NestedInline),
  /// Representing plain text.
  Plain(FlatInline),
  /// Representing newline in the original content that is treated as normal whitespace.
  PlainNewLine(FlatInline),
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

  /// This function converts an inline element in its verbatim representation.
  /// 
  /// e.g. Verbatim(Bold(Plain(b),EscapedSpace(),Plain(b))) --> `` `**b\ b**` ``
  fn flatten_for_verbatim(self) -> Vec<InlineKind>;
}

impl FlattenInlineKind for Vec<InlineKind> {
  fn flatten(self) -> String {
    let mut s: String = String::new();

    for inline in self {
      s.push_str(&inline.flatten());
    }

    s
  }

  fn flatten_for_verbatim(self) -> Vec<InlineKind> {
    let mut flattened: Vec<InlineKind> = Vec::new();

    for inline in self {
      let mut inner = inline.flatten_for_verbatim();
      if let Some(InlineKind::Plain(last_outer)) = flattened.last_mut() {
        if let Some(InlineKind::Plain(first_inner)) = inner.first() {
          last_outer.content.push_str(&first_inner.content);
          last_outer.span.end = first_inner.span.end;
          flattened.append(&mut inner[1..].into());
        } else {
          flattened.append(&mut inner);
        }
      } else {
        flattened.append(&mut inner);
      }
    }

    flattened
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
        s.push_str(&flat.content.flatten());
        s.push_str(TokenKind::VerbatimClose.as_str());
        s
      },
      InlineKind::Plain(flat)
      | InlineKind::PlainNewLine(flat)
      | InlineKind::EscapedNewLine(flat)
      | InlineKind::EscapedSpace(flat) => {
        flat.content
      },
    }
  }

  fn flatten_for_verbatim(self) -> Vec<InlineKind> {
    match self {
        InlineKind::Bold(nested) => {
          let mut inner = nested.content.flatten_for_verbatim();
          merge_flattend_verbatim(&mut inner, TokenKind::BoldOpen.as_str(), nested.span);
          inner
        },
        InlineKind::Italic(nested) => {
          let mut inner = nested.content.flatten_for_verbatim();
          merge_flattend_verbatim(&mut inner, TokenKind::ItalicOpen.as_str(), nested.span);
          inner
        },
        InlineKind::BoldItalic(nested) => {
          let mut inner = nested.content.flatten_for_verbatim();
          merge_flattend_verbatim(&mut inner, TokenKind::BoldItalicOpen.as_str(), nested.span);
          inner
        },
        _ => {
          vec![self]
        },
    }
  }
}

/// This function merges nested inlines into `Plain` kinds
fn merge_flattend_verbatim(inner: &mut Vec<InlineKind>, outer_token: &str, outer_span: Span) {
  if let Some(first) = inner.first_mut() {
    match first {
      InlineKind::Plain(plain) => {
        plain.content.insert_str(0, outer_token);
        plain.span.start = outer_span.start;
      },
      _ => {
        inner.insert(0,
          InlineKind::Plain(FlatInline{
            content: outer_token.to_string(),
            span: Span { start: outer_span.start,
              end: Position { line: outer_span.start.line,
                column: outer_span.start.column + outer_token.len()
              } 
            },
          })
        );
      }
    }
  }

  if let Some(last) = inner.last_mut() {
    match last {
      InlineKind::Plain(plain) => {
        plain.content.push_str(outer_token);
        plain.span.end = outer_span.end;
      },
      _ => {
        inner.push(
          InlineKind::Plain(FlatInline{
            content: outer_token.to_string(),
            span: Span { 
              start: Position { line: outer_span.end.line,
                column: outer_span.end.column - outer_token.len()
              },
              end: outer_span.end,
            },
          })
        );
      }
    }
  }
}
