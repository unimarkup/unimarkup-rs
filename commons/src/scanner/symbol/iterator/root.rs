use itertools::PeekingNext;

use crate::scanner::Symbol;

#[derive(Clone)]
pub struct SymbolIteratorRoot<'input> {
    symbols: &'input [Symbol<'input>],
    pub(super) curr_index: usize,
    pub(super) peek_index: usize,
}

impl<'input> SymbolIteratorRoot<'input> {
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
}

impl<'input> PeekingNext for SymbolIteratorRoot<'input> {
    fn peeking_next<F>(&mut self, accept: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnOnce(&Self::Item) -> bool,
    {
        let symbol = self.symbols.get(self.peek_index)?;

        if !(accept)(&symbol) {
            return None;
        }

        self.peek_index += 1;

        Some(symbol)
    }
}
