use crate::element::Inline;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Overline {
    inner: Vec<Inline>,
}
