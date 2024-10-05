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

    /// Code point offset into the input where the symbol was found in. Note that input can't have
    /// more than 2^32 code points, which limits us to inputs up to 4 GB large.
    pub cp_offs: u32,

    // TODO: `len` and `cp_count` can be calculated if we have two consecutive symbols, so maybe we
    // don't need to store them at all times? We would need 4 bytes less per symbol in that case.
    /// Length of the [`Span`] in bytes.
    pub len: u16,

    /// Number of code points this [`Span`] spreads over.
    pub cp_count: u16,
}

impl AddAssign for Span {
    fn add_assign(&mut self, rhs: Self) {
        self.offs += rhs.offs;
        self.cp_offs += rhs.cp_offs;
        self.len += rhs.len;
        self.cp_count += rhs.cp_count;
    }
}

impl AddAssign<(u32, u16)> for Span {
    fn add_assign(&mut self, (offs, len): (u32, u16)) {
        self.offs += offs;
        self.cp_offs += offs;
        self.len += len;
        self.cp_count += len;
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
        self.cp_offs -= rhs.cp_offs;
        self.len -= rhs.len;
        self.cp_count -= rhs.cp_count;
    }
}

impl SubAssign<(u32, u16)> for Span {
    fn sub_assign(&mut self, (offs, len): (u32, u16)) {
        self.offs -= offs;
        self.cp_offs -= offs;
        self.len -= len;
        self.cp_count -= len;
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
