/// Indicates position of a symbol in a Unimarkup document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    /// Line the symbol is found at
    pub line: usize,
    /// Column at which the symbol is located in line, when encoded as UTF8
    pub col_utf8: usize,
    /// Column at which the symbol is located in line, when encoded as UTF16
    pub col_utf16: usize,
    /// Column at which the symbol is located in line, when counting graphemes
    pub col_grapheme: usize,
}

// NOTE: text editors start counting from 1. Should we as well?
impl Default for Position {
    fn default() -> Self {
        Self {
            line: 1,
            col_utf8: 1,
            col_utf16: 1,
            col_grapheme: 1,
        }
    }
}

// Note: start inclusive, end exclusive
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Offset {
    pub start: usize,
    pub end: usize,
}
