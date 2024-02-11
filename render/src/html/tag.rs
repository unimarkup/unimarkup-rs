//! Defines the [`HtmlTag`] enum that is used to represent supported HTML tags.

use unimarkup_parser::elements::atomic::HeadingLevel;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum HtmlTag {
    /// This tag may be used to create an element without HTML tags.
    #[default]
    PlainContent,
    Html,
    Head,
    Body,
    P,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Code,
    Pre,
    Strong,
    Em,
    Span,
    Sub,
    Sup,
    Mark,
    Q,
    Br,
    Ul,
    Li,
    A,
    Script,
}

impl HtmlTag {
    pub fn as_str(&self) -> &'static str {
        match *self {
            HtmlTag::PlainContent => "",
            HtmlTag::Html => "html",
            HtmlTag::Head => "head",
            HtmlTag::Body => "body",
            HtmlTag::P => "p",
            HtmlTag::H1 => "h1",
            HtmlTag::H2 => "h2",
            HtmlTag::H3 => "h3",
            HtmlTag::H4 => "h4",
            HtmlTag::H5 => "h5",
            HtmlTag::H6 => "h6",
            HtmlTag::Code => "code",
            HtmlTag::Pre => "pre",
            HtmlTag::Strong => "strong",
            HtmlTag::Em => "em",
            HtmlTag::Span => "span",
            HtmlTag::Sub => "sub",
            HtmlTag::Sup => "sup",
            HtmlTag::Mark => "mark",
            HtmlTag::Q => "q",
            HtmlTag::Br => "br",
            HtmlTag::Ul => "ul",
            HtmlTag::Li => "li",
            HtmlTag::A => "a",
            HtmlTag::Script => "script",
        }
    }
}

impl From<HeadingLevel> for HtmlTag {
    fn from(value: HeadingLevel) -> Self {
        match value {
            HeadingLevel::Level1 => HtmlTag::H1,
            HeadingLevel::Level2 => HtmlTag::H2,
            HeadingLevel::Level3 => HtmlTag::H3,
            HeadingLevel::Level4 => HtmlTag::H4,
            HeadingLevel::Level5 => HtmlTag::H5,
            HeadingLevel::Level6 => HtmlTag::H6,
        }
    }
}

impl std::fmt::Display for HtmlTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
