//! Utilities for tracking the positional information about symbols, tokens and other elements in
//! original input.

use std::ops::{Add, AddAssign, Sub, SubAssign};

use super::span::SpanLen;

/// Indicates position of a symbol in a Unimarkup document. Both line and column
/// counting starts from 1.
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

/// Symbol offset in the original input.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Offset {
    /// Start offset of a symbol, inclusive. This is the same as the end offset
    /// of the previous symbol.
    pub start: usize,
    /// End offset of a symbol, exclusive. This is the same as the start offset
    /// of the next symbol.
    pub end: usize,
}

impl Offset {
    pub fn extend(&mut self, other: Offset) {
        self.end = self.end.max(other.end)
    }
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            line,
            col_grapheme: column,
            col_utf8: column,
            col_utf16: column,
        }
    }
}

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
