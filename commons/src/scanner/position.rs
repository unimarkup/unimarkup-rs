use std::ops::{Add, AddAssign, Sub, SubAssign};

use super::span::SpanLen;

/// Indicates position of a symbol in a Unimarkup document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.line += rhs.line;
        self.col_utf8 += rhs.col_utf8;
        self.col_utf16 += rhs.col_utf16;
        self.col_grapheme += rhs.col_grapheme;
    }
}

impl AddAssign<SpanLen> for Position {
    fn add_assign(&mut self, rhs: SpanLen) {
        self.col_utf8 += rhs.len_utf8;
        self.col_utf16 += rhs.len_utf16;
        self.col_grapheme += rhs.len_grapheme;
    }
}

impl AddAssign<Option<SpanLen>> for Position {
    fn add_assign(&mut self, rhs: Option<SpanLen>) {
        if let Some(rhs) = rhs {
            self.col_utf8 += rhs.len_utf8;
            self.col_utf16 += rhs.len_utf16;
            self.col_grapheme += rhs.len_grapheme;
        }
    }
}

impl<T> Add<T> for Position
where
    Position: AddAssign<T>,
{
    type Output = Position;

    fn add(mut self, rhs: T) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.line -= rhs.line;
        self.col_utf8 -= rhs.col_utf8;
        self.col_utf16 -= rhs.col_utf16;
        self.col_grapheme -= rhs.col_grapheme;
    }
}

impl SubAssign<SpanLen> for Position {
    fn sub_assign(&mut self, rhs: SpanLen) {
        self.col_utf8 -= rhs.len_utf8;
        self.col_utf16 -= rhs.len_utf16;
        self.col_grapheme -= rhs.len_grapheme;
    }
}

impl SubAssign<Option<SpanLen>> for Position {
    fn sub_assign(&mut self, rhs: Option<SpanLen>) {
        if let Some(rhs) = rhs {
            self.col_utf8 += rhs.len_utf8;
            self.col_utf16 += rhs.len_utf16;
            self.col_grapheme += rhs.len_grapheme;
        }
    }
}

impl<T> Sub<T> for Position
where
    Position: SubAssign<T>,
{
    type Output = Position;

    fn sub(mut self, rhs: T) -> Self::Output {
        self -= rhs;
        self
    }
}
