#[derive(Eq, PartialEq, Debug)]
pub enum HeadingLevel {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Invalid,
}

impl From<usize> for HeadingLevel {
    fn from(level_depth: usize) -> Self {
        match level_depth {
            1 => Self::Level1,
            2 => Self::Level2,
            3 => Self::Level3,
            4 => Self::Level4,
            5 => Self::Level5,
            6 => Self::Level6,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug)]
pub struct HeadingBlock {
    pub level: HeadingLevel,
    pub content: String,
}
