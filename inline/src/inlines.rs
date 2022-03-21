use crate::Span;

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

pub trait UnfoldInlineKind {
  fn to_string(self) -> String;
} 

impl UnfoldInlineKind for Vec<InlineKind> {
  fn to_string(self) -> String {
    let mut s: String = String::new();

    for inline in self {
      s.push_str(&inline.to_string());
    }

    s
  }
}

impl InlineKind {
  pub fn to_string(self) -> String {
    match self {
      InlineKind::Bold(nested) => {
        let mut s = String::from("**");
        s.push_str(&nested.content.to_string());
        s.push_str("**");
        s
      },
      InlineKind::Italic(nested) => {
        let mut s = String::from("*");
        s.push_str(&nested.content.to_string());
        s.push_str("*");
        s
      },
      InlineKind::BoldItalic(nested) => {
        let mut s = String::from("***");
        s.push_str(&nested.content.to_string());
        s.push_str("***");
        s
      },
      InlineKind::Verbatim(flat) => {
        let mut s = String::from("`");
        s.push_str(&flat.content.to_string());
        s.push_str("`");
        s
      },
      InlineKind::Plain(flat) => {
        flat.content
      },
      InlineKind::EscapedNewLine(flat)
      | InlineKind::EscapedSpace(flat) => {
        let mut s = String::from("\\");
        s.push_str(&flat.content.to_string());
        s
      },
    }
  }
}
