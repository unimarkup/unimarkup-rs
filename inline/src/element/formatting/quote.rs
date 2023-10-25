use crate::element::Inline;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quote {
    inner: Vec<Inline>,
}
