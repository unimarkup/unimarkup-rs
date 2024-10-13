use crate::span::Span;

/// Enum of possible heading levels for unimarkup headings
#[derive(Eq, PartialEq, Debug, strum_macros::Display, strum_macros::EnumString, Clone, Copy)]
#[strum(serialize_all = "kebab-case")]
pub enum HeadingLevel {
    /// Heading level 1, corresponds to `# ` in Unimarkup.
    #[strum(serialize = "level-1")]
    Level1 = 1, // start counting from 0

    /// Heading level 2, corresponds to `## ` in Unimarkup.
    #[strum(serialize = "level-2")]
    Level2,

    /// Heading level 3, corresponds to `### ` in Unimarkup.
    #[strum(serialize = "level-3")]
    Level3,

    /// Heading level 4, corresponds to `#### ` in Unimarkup.
    #[strum(serialize = "level-4")]
    Level4,

    /// Heading level 5, corresponds to `##### ` in Unimarkup.
    #[strum(serialize = "level-5")]
    Level5,

    /// Heading level 6, corresponds to `###### ` in Unimarkup.
    #[strum(serialize = "level-6")]
    Level6,
}

impl TryFrom<u32> for HeadingLevel {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let level = match value {
            1 => HeadingLevel::Level1,
            2 => HeadingLevel::Level2,
            3 => HeadingLevel::Level3,
            4 => HeadingLevel::Level4,
            5 => HeadingLevel::Level5,
            6 => HeadingLevel::Level6,
            other => return Err(format!("Invalid heading level: {other}")),
        };

        Ok(level)
    }
}

impl From<HeadingLevel> for u8 {
    fn from(value: HeadingLevel) -> Self {
        value as u8
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    /// Unique identifier for a heading.
    pub id: String,

    /// Heading level.
    pub level: HeadingLevel,

    /// The content of the heading line.
    pub content: Vec<String>,

    /// Attributes of the heading.
    pub attributes: Option<String>,

    /// The span this element occupies in the Unimarkup input.
    pub span: Span,
}
