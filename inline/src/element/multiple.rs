use super::Inline;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Multiple {
    pub(crate) inner: Vec<Inline>,
}
