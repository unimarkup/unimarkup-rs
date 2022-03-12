use crate::Position;

pub type NestedInline = (Box<InlineKind>, Position);
pub type FlatInline = (String, Position);

pub enum InlineKind {
  Bold(NestedInline),
  Italic(NestedInline),
  Verbatim(FlatInline),
  Plain(FlatInline),
}
