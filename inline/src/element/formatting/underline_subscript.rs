use crate::element::Inline;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Underline {
    inner: Vec<Inline>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subscript {
    inner: Vec<Inline>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnderlineSubscript {
    inner: Vec<Inline>,
}
