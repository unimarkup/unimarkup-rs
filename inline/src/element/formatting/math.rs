use crate::element::Inline;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Math {
    inner: Vec<Inline>,
}
