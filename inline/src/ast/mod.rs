use crate::tokenizer::{Position, TokenKind};

pub(crate) mod collect;
mod substitutions;

/// Represents an AST of Unimarkup inline elements
pub type Inline = Vec<InlineKind>;


#[derive(Debug, Default, Clone, PartialEq, Copy)]
pub struct Span {
  pub start: Position,
  pub end: Position,
}

#[derive(Debug, PartialEq)]
pub struct NestedInline {
  pub content: Vec<InlineKind>, 
  pub span: Span
}

#[derive(Debug, PartialEq)]
pub struct FlatInline {
  pub content: String,
  pub span: Span,
}

#[derive(Debug, PartialEq)]
pub enum InlineKind {
  Bold(NestedInline),
  Italic(NestedInline),
  BoldItalic(NestedInline),
  Verbatim(FlatInline),
  Plain(FlatInline),
  EscapedNewLine(FlatInline),
  EscapedSpace(FlatInline),
}

pub trait FlattenInlineKind {
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

