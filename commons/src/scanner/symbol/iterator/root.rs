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
    pub(super) index: usize,
    /// The peek index of the iterator inside the [`Symbol`] slice.
    pub(super) peek_index: usize,
    /// The match index of the iterator inside the [`Symbol`] slice.
    /// Used to keep track of end and prefix matches to consume the matched sequence length.    
    pub(super) match_index: usize,
}

impl<'input> SymbolIteratorRoot<'input> {
    /// Returns the remaining symbols in this iterator, or `None` if there are no symbols left.
    pub(super) fn remaining_symbols(&self) -> Option<&'input [Symbol<'input>]> {
        self.symbols.get(self.index..)
    }
}

impl<'input, T> From<T> for SymbolIteratorRoot<'input>
where
    T: Into<&'input [Symbol<'input>]>,
{
    fn from(value: T) -> Self {
        SymbolIteratorRoot {
            symbols: value.into(),
            index: 0,
            peek_index: 0,
            match_index: 0,
        }
    }
}

impl<'input> Iterator for SymbolIteratorRoot<'input> {
    type Item = &'input Symbol<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        let symbol = self.symbols.get(self.index)?;

        self.index += 1;
        self.peek_index = self.index;

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
