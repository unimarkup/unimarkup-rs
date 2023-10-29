use crate::element::Inline;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hyperlink {
    pub(crate) inner: Vec<Inline>,
    pub(crate) link: String,
    pub(crate) alt_text: Option<String>,
}

impl From<Hyperlink> for Inline {
    fn from(value: Hyperlink) -> Self {
        Inline::Hyperlink(value)
    }
}
