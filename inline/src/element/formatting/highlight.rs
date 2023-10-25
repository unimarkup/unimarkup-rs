use crate::element::Inline;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Highlight {
    inner: Vec<Inline>,
}
