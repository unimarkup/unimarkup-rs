//! Contains the [`SymbolIteratorRoot`] that is the root iterator in any [`SymbolIterator`](super::SymbolIterator).

use itertools::PeekingNext;

use crate::scanner::Symbol;

/// The [`SymbolIteratorRoot`] is the root iterator in any [`SymbolIterator`](super::SymbolIterator).
/// It holds the actual [`Symbol`] slice.
#[derive(Clone)]
pub struct SymbolIteratorRoot<'input> {
    /// The [`Symbol`] slice the iterator was created for.
    symbols: &'input [Symbol<'input>],
    /// The current index of the iterator inside the [`Symbol`] slice.
    pub(super) curr_index: usize,
    /// The peek index of the iterator inside the [`Symbol`] slice.
    pub(super) peek_index: usize,
}

impl<'input> SymbolIteratorRoot<'input> {
    /// Returns the remaining symbols in this iterator, or `None` if there are no symbols left.
    pub(super) fn remaining_symbols(&self) -> Option<&'input [Symbol<'input>]> {
        self.symbols.get(self.curr_index..)
    }
}

impl<'input> From<&'input [Symbol<'input>]> for SymbolIteratorRoot<'input> {
    fn from(value: &'input [Symbol<'input>]) -> Self {
        SymbolIteratorRoot {
            symbols: value,
            curr_index: 0,
            peek_index: 0,
        }
    }
}

impl<'input> From<&'input Vec<Symbol<'input>>> for SymbolIteratorRoot<'input> {
    fn from(value: &'input Vec<Symbol<'input>>) -> Self {
        SymbolIteratorRoot {
            symbols: value,
            curr_index: 0,
            peek_index: 0,
        }
    }
}

impl<'input> Iterator for SymbolIteratorRoot<'input> {
    type Item = &'input Symbol<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        let symbol = self.symbols.get(self.curr_index)?;

        self.curr_index += 1;
        self.peek_index = self.curr_index;

        Some(symbol)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.remaining_symbols().map(<[_]>::len).unwrap_or(0);
        (len, Some(len))
    }
}

impl<'input> PeekingNext for SymbolIteratorRoot<'input> {
    fn peeking_next<F>(&mut self, accept: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnOnce(&Self::Item) -> bool,
    {
        let symbol = self.symbols.get(self.peek_index).filter(accept)?;
        self.peek_index += 1;
        Some(symbol)
    }
}
