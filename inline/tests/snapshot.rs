use std::ops::{Deref, DerefMut};

use unimarkup_commons::{
    scanner::{position::Position, span::Span},
    test_runner::as_snapshot::AsSnapshot,
};

/// Wrapper type for implementing the `AsSnapshot` trait.
/// Integration `tests` is treated as an extra crate, so we can't implement
/// trait for types where neither are defined in this (`tests`) crate.
pub struct Snapshot<T>(pub T);

impl<T> Deref for Snapshot<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Snapshot<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsSnapshot for Snapshot<&Span> {
    fn as_snapshot(&self) -> String {
        let start = Snapshot(&self.start).as_snapshot();
        let end = Snapshot(&self.end).as_snapshot();
        format!(" @ ({})->({})", start, end)
    }
}

impl AsSnapshot for Snapshot<&Position> {
    fn as_snapshot(&self) -> String {
        format!("{}:{}", self.line, self.col_grapheme)
    }
}

impl<T> Snapshot<T> {
    pub const NEWLINE_SYBMOL: &'static str = "\u{23CE}";
    pub const BLANK_SYMBOL: &'static str = "\u{2422}";

    pub fn snap(inner: T) -> String
    where
        Snapshot<T>: AsSnapshot,
    {
        AsSnapshot::as_snapshot(&Snapshot(inner))
    }
}
