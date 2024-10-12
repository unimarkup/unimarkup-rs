//! Utilities for tracking the positional information about symbols, tokens and other elements in
//! original input.

use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Indicates position of a symbol or token in a Unimarkup document. Counting of both byte and code
/// point offsets starts at zero.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    /// Byte offset into the input where the symbol was found in. Note that input can't be larger
    /// than `2^32 B = 4 GB`
    pub offs: u32,

    // TODO: `len` and `cp_count` can be calculated if we have two consecutive symbols, so maybe we
    // don't need to store them at all times? We would need 4 bytes less per symbol in that case.
    /// Length of the [`Span`] in bytes.
    pub len: u16,
}

impl AddAssign for Span {
    fn add_assign(&mut self, rhs: Self) {
        self.offs += rhs.offs;
        self.len += rhs.len;
    }
}

impl AddAssign<(u32, u16)> for Span {
    fn add_assign(&mut self, (offs, len): (u32, u16)) {
        self.offs += offs;
        self.len += len;
    }
}

impl<T> Add<T> for Span
where
    Span: AddAssign<T>,
{
    type Output = Span;

    fn add(mut self, rhs: T) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign for Span {
    fn sub_assign(&mut self, rhs: Self) {
        self.offs -= rhs.offs;
        self.len -= rhs.len;
    }
}

impl SubAssign<(u32, u16)> for Span {
    fn sub_assign(&mut self, (offs, len): (u32, u16)) {
        self.offs -= offs;
        self.len -= len;
    }
}

impl<T> Sub<T> for Span
where
    Span: SubAssign<T>,
{
    type Output = Span;

    fn sub(mut self, rhs: T) -> Self::Output {
        self -= rhs;
        self
    }
}
