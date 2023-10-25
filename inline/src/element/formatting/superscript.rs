use crate::element::Inline;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Superscript {
    inner: Vec<Inline>,
}
