//! Contains the [`SymbolIterator`], and all related functionality
//! that is used to step through the [`Symbol`]s retrieved from the [`Scanner`](crate::scanner::Scanner).

use std::borrow::BorrowMut;

use super::{Symbol, SymbolKind};

mod matcher;
mod root;

pub use itertools::*;
pub use matcher::*;
pub use root::*;

/// The [`SymbolIterator`] provides an iterator over [`Symbol`]s.
/// It allows to add matcher functions to notify the iterator,
/// when an end of an element is reached, or what prefixes to strip on a new line.
/// Additionaly, the iterator may be nested to enable transparent iterating for nested elements.
#[derive(Clone)]
pub struct SymbolIterator<'input> {
    /// The [`SymbolIteratorKind`] of this iterator.
    kind: SymbolIteratorKind<'input>,
    /// The index inside the [`Symbol`]s of the root iterator.
    start_index: usize,
    /// Optional matching function that is used to automatically skip matched prefixes after a new line.
    prefix_match: Option<IteratorPrefixFn>,
    /// Optional matching function that is used to indicate the end of this iterator.
    end_match: Option<IteratorEndFn>,
    /// Flag set to `true` if this iterator reached its end.
    iter_end: bool,
}

/// The [`SymbolIteratorKind`] defines the kind of a [`SymbolIterator`].
///
/// **Note:** This enables iterator nesting.
#[derive(Clone)]
pub enum SymbolIteratorKind<'input> {
    /// Defines an iterator as being nested.
    /// The contained iterator is the parent iterator.
    Nested(Box<SymbolIterator<'input>>),
    /// Defines an iterator as being root.
    Root(SymbolIteratorRoot<'input>),
}

impl<'input> SymbolIterator<'input> {
    /// Creates a new [`SymbolIterator`] from the given [`Symbol`] slice.
    /// This iterator is created without matching functions.
    pub fn new(symbols: &'input [Symbol<'input>]) -> Self {
        SymbolIterator::from(symbols)
    }

    /// Creates a new [`SymbolIterator`] from the given [`Symbol`] slice,
    /// and the given matching functions.
    ///
    /// # Arguments
    ///
    /// * `symbols` ... [`Symbol`] slice to iterate over
    /// * `prefix_match` ... Optional matching function used to strip prefix on new lines
    /// * `end_match` ... Optional matching function used to indicate the end of the created iterator
    pub fn with(
        symbols: &'input [Symbol<'input>],
        prefix_match: Option<IteratorPrefixFn>,
        end_match: Option<IteratorEndFn>,
    ) -> Self {
        SymbolIterator {
            kind: SymbolIteratorKind::Root(SymbolIteratorRoot::from(symbols)),
            start_index: 0,
            prefix_match,
            end_match,
            iter_end: false,
        }
    }

    /// Returns the length of the remaining [`Symbol`]s this iterator might return.
    ///
    /// **Note:** This length does not consider parent iterators, or matching functions.
    /// Therefore, the returned number of [`Symbol`]s might differ, but cannot be larger than this length.
    pub fn len(&self) -> usize {
        self.remaining_symbols().unwrap_or(&[]).len()
    }

    /// Returns `true` if no more [`Symbol`]s are available.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the index this iterator was started from the [`Symbol`] slice of the root iterator.
    pub fn start_index(&self) -> usize {
        self.start_index
    }

    /// Returns the current index this iterator is in the [`Symbol`] slice of the root iterator.
    pub fn curr_index(&self) -> usize {
        match &self.kind {
            SymbolIteratorKind::Nested(parent) => parent.curr_index(),
            SymbolIteratorKind::Root(root) => root.curr_index,
        }
    }

    /// Sets the current index of this iterator to the given index.
    pub fn set_curr_index(&mut self, index: usize) {
        if index >= self.start_index {
            match self.kind.borrow_mut() {
                SymbolIteratorKind::Nested(parent) => parent.set_curr_index(index),
                SymbolIteratorKind::Root(root) => {
                    root.curr_index = index;
                    root.peek_index = index;
                }
            }
        }
    }

    /// Returns the index used to peek.
    fn peek_index(&self) -> usize {
        match &self.kind {
            SymbolIteratorKind::Nested(parent) => parent.peek_index(),
            SymbolIteratorKind::Root(root) => root.peek_index,
        }
    }

    /// Sets the peek index of this iterator to the given index.
    pub fn set_peek_index(&mut self, index: usize) {
        if index >= self.curr_index() {
            match self.kind.borrow_mut() {
                SymbolIteratorKind::Nested(parent) => parent.set_peek_index(index),
                SymbolIteratorKind::Root(root) => {
                    root.peek_index = index;
                }
            }
        }
    }

    /// Resets peek to get `peek() == next()`.
    ///
    /// **Note:** Needed to reset peek index after using `peeking_next()`.
    pub fn reset_peek(&mut self) {
        self.set_peek_index(self.curr_index());
    }

    /// Returns the maximal remaining symbols in this iterator.
    ///
    /// **Note:** Similar to `len()`, this does not consider parent iterators and matching functions.
    pub fn remaining_symbols(&self) -> Option<&'input [Symbol<'input>]> {
        match &self.kind {
            SymbolIteratorKind::Nested(parent) => parent.remaining_symbols(),
            SymbolIteratorKind::Root(root) => root.remaining_symbols(),
        }
    }

    /// Returns the next [`Symbol`] without changing the current index.    
    pub fn peek(&mut self) -> Option<&'input Symbol<'input>> {
        let symbol = self.peeking_next(|_| true);
        self.reset_peek(); // Note: Resetting index, because peek() must be idempotent
        symbol
    }

    /// Returns the [`SymbolKind`] of the peeked [`Symbol`].
    pub fn peek_kind(&mut self) -> Option<SymbolKind> {
        self.peek().map(|s| s.kind)
    }

    /// Nests this iterator, by creating a new iterator that has this iterator set as parent.
    ///
    /// **Note:** Any change in this iterator is **not** propagated to the nested iterator.
    /// See [`Self::update()`] on how to synchronize this iterator with the nested one.
    ///
    /// # Arguments
    ///
    /// * `prefix_match` ... Optional matching function used to strip prefix on new lines
    /// * `end_match` ... Optional matching function used to indicate the end of the created iterator
    pub fn nest(
        &self,
        prefix_match: Option<IteratorPrefixFn>,
        end_match: Option<IteratorEndFn>,
    ) -> SymbolIterator<'input> {
        SymbolIterator {
            kind: SymbolIteratorKind::Nested(Box::new(self.clone())),
            start_index: self.curr_index(),
            prefix_match,
            end_match,
            iter_end: self.iter_end,
        }
    }

    /// Updates the given parent iterator to take the progress of the nested iterator.
    ///
    /// **Note:** Only updates the parent if `self` is nested.
    pub fn update(self, parent: &mut Self) {
        if let SymbolIteratorKind::Nested(self_parent) = self.kind {
            *parent = *self_parent;
        }
    }

    /// Tries to skip symbols until one of the end functions signals the end.
    ///
    /// **Note:** This function might not reach the iterator end.
    ///
    /// If no symbols are left, or no given line prefix is matched, the iterator may stop before an end is reached.
    /// Use [`Self::end_reached()`] to check if the end was actually reached.
    pub fn skip_to_end(mut self) -> Self {
        let _last_symbol = self.by_ref().last();

        self
    }

    /// Collects and returns all symbols until one of the end functions signals the end,
    /// or until no line prefix is matched after a new line.
    pub fn take_to_end(&mut self) -> Vec<&'input Symbol<'input>> {
        let mut symbols = Vec::new();

        for symbol in self.by_ref() {
            symbols.push(symbol);
        }

        symbols
    }

    /// Returns `true` if this iterator has reached its end.
    pub fn end_reached(&self) -> bool {
        self.iter_end
    }
}

impl<'input> From<&'input [Symbol<'input>]> for SymbolIterator<'input> {
    fn from(value: &'input [Symbol<'input>]) -> Self {
        SymbolIterator {
            kind: SymbolIteratorKind::Root(SymbolIteratorRoot::from(value)),
            start_index: 0,
            prefix_match: None,
            end_match: None,
            iter_end: false,
        }
    }
}

impl<'input> From<&'input Vec<Symbol<'input>>> for SymbolIterator<'input> {
    fn from(value: &'input Vec<Symbol<'input>>) -> Self {
        SymbolIterator {
            kind: SymbolIteratorKind::Root(SymbolIteratorRoot::from(value)),
            start_index: 0,
            prefix_match: None,
            end_match: None,
            iter_end: false,
        }
    }
}

impl<'input> Iterator for SymbolIterator<'input> {
    type Item = &'input Symbol<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end_reached() {
            return None;
        }

        if let Some(end_fn) = self.end_match.clone() {
            if (end_fn)(self) {
                self.iter_end = true;
                return None;
            }
        }

        let curr_symbol_opt = match &mut self.kind {
            SymbolIteratorKind::Nested(parent) => parent.next(),
            SymbolIteratorKind::Root(root) => root.next(),
        };

        if curr_symbol_opt?.kind == SymbolKind::Newline && self.prefix_match.is_some() {
            let prefix_match = self
                .prefix_match
                .clone()
                .expect("Prefix match checked above to be some.");

            // Note: This mostly indicates a syntax violation, so skipped symbol is ok.
            if !prefix_match(self) {
                return None;
            }
        }

        curr_symbol_opt
    }
}

impl<'input> PeekingNext for SymbolIterator<'input> {
    fn peeking_next<F>(&mut self, accept: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnOnce(&Self::Item) -> bool,
    {
        // Note: Not possible to restrict peek to return only symbols `next()` would return,
        // because `peeking_next()` is needed in End- and PrefixMatcher.
        // Using the same logic as in `next()` would result in endless loop inside `peeking_next()` => StackOverflow

        match &mut self.kind {
            SymbolIteratorKind::Nested(parent) => parent.peeking_next(accept),
            SymbolIteratorKind::Root(root) => root.peeking_next(accept),
        }
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use itertools::{Itertools, PeekingNext};

    use crate::scanner::{PrefixMatcher, Scanner, SymbolKind};

    use super::SymbolIterator;

    #[test]
    fn peek_while_index() {
        let symbols = Scanner::try_new()
            .expect("Must be valid provider.")
            .scan_str("## ");

        let mut iterator = SymbolIterator::from(&symbols);
        let hash_cnt = iterator
            .peeking_take_while(|symbol| symbol.kind == SymbolKind::Hash)
            .count();

        let next_symbol = iterator.nth(hash_cnt);
        let curr_index = iterator.curr_index();

        assert_eq!(hash_cnt, 2, "Hash symbols in input not correctly detected.");
        assert_eq!(curr_index, 3, "Current index was not updated correctly.");
        assert_eq!(
            next_symbol.map(|s| s.kind),
            Some(SymbolKind::Whitespace),
            "Whitespace after hash symbols was not detected."
        );
        assert!(
            iterator.next().is_none(),
            "Input end reached, but new symbol was returned."
        );
    }

    #[test]
    fn peek_next() {
        let symbols = Scanner::try_new()
            .expect("Must be valid provider.")
            .scan_str("#*");

        let mut iterator = SymbolIterator::from(&symbols);

        let peeked_symbol = iterator.peeking_next(|_| true);
        let next_symbol = iterator.next();
        let next_peeked_symbol = iterator.peeking_next(|_| true);
        let curr_index = iterator.curr_index();

        assert_eq!(curr_index, 1, "Current index was not updated correctly.");
        assert_eq!(
            peeked_symbol.map(|s| s.kind),
            Some(SymbolKind::Hash),
            "peek_next() did not return hash symbol."
        );
        assert_eq!(
            next_symbol.map(|s| s.kind),
            Some(SymbolKind::Hash),
            "next() did not return hash symbol."
        );
        assert_eq!(
            next_peeked_symbol.map(|s| s.kind),
            Some(SymbolKind::Star),
            "Star symbol not peeked next."
        );
        assert_eq!(
            iterator.next().map(|s| s.kind),
            Some(SymbolKind::Star),
            "Star symbol not returned."
        );
    }

    #[test]
    fn reach_end() {
        let symbols = Scanner::try_new()
            .expect("Must be valid provider.")
            .scan_str("text*");

        let mut iterator = SymbolIterator::from(&symbols).nest(
            None,
            Some(Rc::new(|matcher| matcher.matches(&[SymbolKind::Star]))),
        );

        let taken_symkinds = iterator
            .take_to_end()
            .iter()
            .map(|s| s.kind)
            .collect::<Vec<_>>();

        assert!(iterator.end_reached(), "Iterator end was not reached.");
        assert_eq!(
            taken_symkinds,
            vec![
                SymbolKind::Plain,
                SymbolKind::Plain,
                SymbolKind::Plain,
                SymbolKind::Plain
            ],
            "Symbols till end was reached are incorrect."
        );
    }

    #[test]
    fn with_nested_and_parent_prefix() {
        let symbols = Scanner::try_new()
            .expect("Must be valid provider.")
            .scan_str("a\n* *b");

        let iterator = SymbolIterator::with(
            &symbols,
            Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
                matcher.consumed_prefix(&[SymbolKind::Star, SymbolKind::Whitespace])
            })),
            None,
        );

        let mut inner = iterator.nest(
            Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
                matcher.consumed_prefix(&[SymbolKind::Star])
            })),
            None,
        );

        let sym_kinds = inner
            .take_to_end()
            .iter()
            .map(|s| s.kind)
            .collect::<Vec<_>>();

        assert_eq!(
            sym_kinds,
            vec![SymbolKind::Plain, SymbolKind::Newline, SymbolKind::Plain],
            "Prefix symbols not correctly skipped"
        );
    }
}
