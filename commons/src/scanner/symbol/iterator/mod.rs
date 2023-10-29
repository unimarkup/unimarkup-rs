//! Contains the [`SymbolIterator`], and all related functionality
//! that is used to step through the [`Symbol`]s retrieved from the [`Scanner`](crate::scanner::Scanner).

use std::borrow::BorrowMut;

use super::{Symbol, SymbolKind};

mod matcher;
mod root;

pub mod new;

pub use itertools::*;
pub use matcher::*;
pub use root::*;

/// The [`SymbolIterator`] provides an iterator over [`Symbol`]s.
/// It allows to add matcher functions to notify the iterator,
/// when an end of an element is reached, or what prefixes to strip on a new line.
/// Additionaly, the iterator may be nested to enable transparent iterating for nested elements.
///
/// *Transparent* meaning that the nested iterator does not see [`Symbol`]s consumed by the wrapped (parent) iterator.
/// In other words, wrapped iterators control which [`Symbol`]s will be passed to their nested iterator.
/// Therefore, each nested iterator only sees those [`Symbol`]s that are relevant to its scope.
#[derive(Clone)]
pub struct SymbolIterator<'input> {
    /// The [`SymbolIteratorKind`] of this iterator.
    parent: SymbolIteratorKind<'input>,
    /// The index inside the [`Symbol`]s of the root iterator.
    start_index: usize,
    /// The scope this iterator is in, starting at 0 if parent is the root iterator.
    scope: usize,
    /// Flag set to `true` if this iterator pushed a new scope.
    scoped: bool,
    /// Index used to skip end matchings in case subsequent symbols already passed end matching for previous `peeking_next` calls.
    highest_peek_index: usize,
    /// Optional matching function that is used to automatically skip matched prefixes after a new line.
    prefix_match: Option<IteratorPrefixFn>,
    /// Optional matching function that is used to indicate the end of this iterator.
    end_match: Option<IteratorEndFn>,
    /// Flag set to `true` if this iterator reached its end.
    ///
    /// Prevents the iterator from jumping over the end sequence.
    iter_end: bool,
    /// Flag set to `true` if prefix mismatch occured.
    ///
    /// Prevents the iterator from returning symbols once no prefix matched.
    prefix_mismatch: bool,
    /// Flag set to `true` to indicate matching context in [`Self::next()`].
    ///
    /// End/Prefix matching in `next()` uses `peeking_next()` to check wether the given function matches or not.
    /// Without this flag, `peeking_next()` would apply end/prefix matching itself,
    /// leading to invalid symbols being passed to matching functions for `next()`.
    next_matching: bool,
    /// Flag set to `true` to indicate matching context in [`Self::peeking_next()`]
    ///
    /// Used to prevent consumed matching while peeking.
    peek_matching: bool,
    /// The previous symbol this iterator returned with `next()` or `consumed_matches()`.
    /// It is only updated if `next()` returns Some`, or `consumed_matches()` matched.
    ///
    /// Symbols matched with prefix matching are skipped, because `Newline` symbol is passed to all nested iterators.
    prev_symbol: Option<Symbol<'input>>,
}

/// The [`SymbolIteratorKind`] defines the kind of a [`SymbolIterator`].
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
            parent: SymbolIteratorKind::Root(SymbolIteratorRoot::from(symbols)),
            scope: 0,
            scoped: false,
            highest_peek_index: 0,
            start_index: 0,
            prefix_match,
            end_match,
            iter_end: false,
            prefix_mismatch: false,
            next_matching: false,
            peek_matching: false,
            prev_symbol: None,
        }
    }

    /// Creates a new scoped [`SymbolIterator`] from the given [`Symbol`] slice,
    /// and the given matching functions.
    ///
    /// # Arguments
    ///
    /// * `symbols` ... [`Symbol`] slice to iterate over
    /// * `prefix_match` ... Optional matching function used to strip prefix on new lines
    /// * `end_match` ... Optional matching function used to indicate the end of the created iterator
    pub fn scoped(
        symbols: &'input [Symbol<'input>],
        prefix_match: Option<IteratorPrefixFn>,
        end_match: Option<IteratorEndFn>,
    ) -> Self {
        SymbolIterator {
            parent: SymbolIteratorKind::Root(SymbolIteratorRoot::from(symbols)),
            scope: 0,
            scoped: true,
            highest_peek_index: 0,
            start_index: 0,
            prefix_match,
            end_match,
            iter_end: false,
            prefix_mismatch: false,
            next_matching: false,
            peek_matching: false,
            prev_symbol: None,
        }
    }

    /// Returns the maximum length of the remaining [`Symbol`]s this iterator might return.
    ///
    /// **Note:** This length does not consider parent iterators, or matching functions.
    /// Therefore, the returned number of [`Symbol`]s might differ, but cannot be larger than this length.
    pub fn max_len(&self) -> usize {
        self.max_remaining_symbols().unwrap_or(&[]).len()
    }

    /// Returns `true` if no more [`Symbol`]s are available.
    pub fn is_empty(&self) -> bool {
        self.max_remaining_symbols().unwrap_or(&[]).is_empty()
    }

    /// Returns the index this iterator was started from the [`Symbol`] slice of the root iterator.
    pub fn start_index(&self) -> usize {
        self.start_index
    }

    /// Returns the current index this iterator is in the [`Symbol`] slice of the root iterator.
    pub fn index(&self) -> usize {
        match &self.parent {
            SymbolIteratorKind::Nested(parent) => parent.index(),
            SymbolIteratorKind::Root(root) => root.index,
        }
    }

    /// Sets the current index of this iterator to the given index.
    pub fn set_index(&mut self, index: usize) {
        if index >= self.start_index {
            match self.parent.borrow_mut() {
                SymbolIteratorKind::Nested(parent) => parent.set_index(index),
                SymbolIteratorKind::Root(root) => {
                    root.index = index;
                    root.peek_index = index;
                }
            }
        }
    }

    /// Returns the index used to peek.
    pub fn peek_index(&self) -> usize {
        match &self.parent {
            SymbolIteratorKind::Nested(parent) => parent.peek_index(),
            SymbolIteratorKind::Root(root) => root.peek_index,
        }
    }

    /// Sets the peek index of this iterator to the given index.
    pub fn set_peek_index(&mut self, index: usize) {
        if index >= self.index() {
            match self.parent.borrow_mut() {
                SymbolIteratorKind::Nested(parent) => parent.set_peek_index(index),
                SymbolIteratorKind::Root(root) => {
                    root.peek_index = index;
                }
            }
        }
    }

    pub fn match_index(&self) -> usize {
        match &self.parent {
            SymbolIteratorKind::Nested(parent) => parent.match_index(),
            SymbolIteratorKind::Root(root) => root.match_index,
        }
    }

    pub fn set_match_index(&mut self, index: usize) {
        if index >= self.index() {
            match self.parent.borrow_mut() {
                SymbolIteratorKind::Nested(parent) => parent.set_match_index(index),
                SymbolIteratorKind::Root(root) => {
                    root.match_index = index;
                }
            }
        }
    }

    /// Resets peek to get `peek() == next()`.
    ///
    /// **Note:** Needed to reset peek index after using `peeking_next()`.
    pub fn reset_peek(&mut self) {
        self.set_peek_index(self.index());
    }

    /// Returns the maximal remaining symbols in this iterator.
    ///
    /// **Note:** This slice does not consider parent iterators, or matching functions.
    /// Therefore, the returned [`Symbol`] slice might differ from the symbols returned by calling [`Self::next()`],
    /// but [`Self::next()`] cannot return more symbols than those inside the returned slice.
    pub fn max_remaining_symbols(&self) -> Option<&'input [Symbol<'input>]> {
        match &self.parent {
            SymbolIteratorKind::Nested(parent) => parent.max_remaining_symbols(),
            SymbolIteratorKind::Root(root) => root.remaining_symbols(),
        }
    }

    /// Returns the next [`Symbol`] without changing the current index.    
    pub fn peek(&mut self) -> Option<&'input Symbol<'input>> {
        let peek_index = self.peek_index();

        let symbol = self.peeking_next(|_| true);

        self.set_peek_index(peek_index); // Note: Resetting index, because peek() must be idempotent

        symbol
    }

    /// Returns the [`SymbolKind`] of the peeked [`Symbol`].
    pub fn peek_kind(&mut self) -> Option<SymbolKind> {
        self.peek().map(|s| s.kind)
    }

    fn push_scope(&mut self, scope: usize) {
        match self.parent.borrow_mut() {
            SymbolIteratorKind::Nested(parent) => parent.push_scope(scope),
            SymbolIteratorKind::Root(root) => root.scope = scope,
        }
    }

    fn root_scope(&self) -> usize {
        match &self.parent {
            SymbolIteratorKind::Nested(parent) => parent.root_scope(),
            SymbolIteratorKind::Root(root) => root.scope,
        }
    }

    /// Returns the previous symbol this iterator returned via `next()` or `consumed_matches()`.
    pub fn prev_symbol(&self) -> Option<&Symbol<'input>> {
        self.prev_symbol.as_ref()
    }

    /// Returns the [`SymbolKind`] of the previous symbol this iterator returned via `next()` or `consumed_matches()`.
    pub fn prev_kind(&self) -> Option<SymbolKind> {
        self.prev_symbol.map(|s| s.kind)
    }

    fn prev_root_symbol(&self) -> Option<&Symbol<'input>> {
        match &self.parent {
            SymbolIteratorKind::Nested(parent) => parent.prev_root_symbol(),
            SymbolIteratorKind::Root(root) => root.prev(),
        }
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
            parent: SymbolIteratorKind::Nested(Box::new(self.clone())),
            start_index: self.index(),
            scope: self.scope,
            scoped: false,
            highest_peek_index: self.index(),
            prefix_match,
            end_match,
            iter_end: self.iter_end,
            prefix_mismatch: self.prefix_mismatch,
            next_matching: self.next_matching,
            peek_matching: self.peek_matching,
            prev_symbol: None,
        }
    }

    /// Nests this iterator, by creating a new iterator that has this iterator set as parent.
    /// Pushes the new iterator to a new scope, and only runs the given matching functions in the new scope.
    ///
    /// **Note:** Any change in this iterator is **not** propagated to the nested iterator.
    /// See [`Self::update()`] on how to synchronize this iterator with the nested one.
    ///
    /// # Arguments
    ///
    /// * `prefix_match` ... Optional matching function used to strip prefix on new lines
    /// * `end_match` ... Optional matching function used to indicate the end of the created iterator
    pub fn nest_with_scope(
        &self,
        prefix_match: Option<IteratorPrefixFn>,
        end_match: Option<IteratorEndFn>,
    ) -> SymbolIterator<'input> {
        let scope = self.scope + 1;
        let mut parent = self.clone();
        parent.push_scope(scope);

        SymbolIterator {
            parent: SymbolIteratorKind::Nested(Box::new(parent)),
            start_index: self.index(),
            scope,
            scoped: true,
            highest_peek_index: self.index(),
            prefix_match,
            end_match,
            iter_end: self.iter_end,
            prefix_mismatch: self.prefix_mismatch,
            next_matching: self.next_matching,
            peek_matching: self.peek_matching,
            prev_symbol: None,
        }
    }

    /// Nests this iterator, by creating a new iterator that has this iterator set as parent.
    /// Matching functions of the new iterator are only run in the scope of this iterator.
    ///
    /// **Note:** Any change in this iterator is **not** propagated to the nested iterator.
    /// See [`Self::update()`] on how to synchronize this iterator with the nested one.
    ///
    /// # Arguments
    ///
    /// * `prefix_match` ... Optional matching function used to strip prefix on new lines
    /// * `end_match` ... Optional matching function used to indicate the end of the created iterator
    pub fn nest_scoped(
        &self,
        prefix_match: Option<IteratorPrefixFn>,
        end_match: Option<IteratorEndFn>,
    ) -> SymbolIterator<'input> {
        SymbolIterator {
            parent: SymbolIteratorKind::Nested(Box::new(self.clone())),
            start_index: self.index(),
            scope: self.scope,
            scoped: true,
            highest_peek_index: self.index(),
            prefix_match,
            end_match,
            iter_end: self.iter_end,
            prefix_mismatch: self.prefix_mismatch,
            next_matching: self.next_matching,
            peek_matching: self.peek_matching,
            prev_symbol: None,
        }
    }

    /// Updates the given parent iterator to take the progress of the nested iterator.
    ///
    /// **Note:** Only updates the parent if `self` is nested.
    pub fn update(self, parent: &mut Self) {
        if let SymbolIteratorKind::Nested(mut self_parent) = self.parent {
            // Make sure it actually is the parent.
            // It is not possible to check more precisely, because other indices are expected to be different due to `clone()`.
            debug_assert_eq!(
                self_parent.start_index, parent.start_index,
                "Updated iterator is not the actual parent of this iterator."
            );
            self_parent.push_scope(self_parent.scope);

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

    pub fn peek_nth(&mut self, n: usize) -> Option<&'input Symbol<'input>> {
        let mut symbol = self.peeking_next(|_| true);

        for _ in 0..n {
            symbol = self.peeking_next(|_| true);
            symbol?;
        }

        symbol
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

impl<'input, T> From<T> for SymbolIterator<'input>
where
    T: Into<&'input [Symbol<'input>]>,
{
    fn from(value: T) -> Self {
        SymbolIterator {
            parent: SymbolIteratorKind::Root(SymbolIteratorRoot::from(value)),
            start_index: 0,
            scope: 0,
            scoped: false,
            highest_peek_index: 0,
            prefix_match: None,
            end_match: None,
            iter_end: false,
            prefix_mismatch: false,
            next_matching: false,
            peek_matching: false,
            prev_symbol: None,
        }
    }
}

impl<'input> Iterator for SymbolIterator<'input> {
    type Item = &'input Symbol<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.prefix_mismatch || self.end_reached() {
            return None;
        }

        let in_scope = !self.scoped || self.scope == self.root_scope();
        let allow_end_matching = in_scope && (self.highest_peek_index <= self.index());

        if allow_end_matching {
            if let Some(end_fn) = self.end_match.clone() {
                self.next_matching = true;

                if (end_fn)(self) {
                    self.iter_end = true;
                    self.next_matching = false;
                    return None;
                }
            }
        }

        let curr_symbol_opt = match &mut self.parent {
            SymbolIteratorKind::Nested(parent) => parent.next(),
            SymbolIteratorKind::Root(root) => root.next(),
        };

        // Prefix matching after `peeking_next()` to skip prefix symbols, but pass `Newline` to nested iterators.
        if in_scope && curr_symbol_opt?.kind == SymbolKind::Newline && self.prefix_match.is_some() {
            let prefix_match = self
                .prefix_match
                .clone()
                .expect("Prefix match checked above to be some.");
            self.next_matching = true;

            // Note: This mostly indicates a syntax violation, so skipped symbol is ok.
            if !prefix_match(self) {
                self.prefix_mismatch = true;
                self.next_matching = false;
                return None;
            }
        }

        if curr_symbol_opt.is_some() {
            self.prev_symbol = curr_symbol_opt.copied();
        }

        self.next_matching = false;
        curr_symbol_opt
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.max_len()))
    }
}

impl<'input> PeekingNext for SymbolIterator<'input> {
    fn peeking_next<F>(&mut self, accept: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnOnce(&Self::Item) -> bool,
    {
        if self.prefix_mismatch || self.end_reached() {
            return None;
        }

        // Note: Only end matching can be optimized, because once an end is reached, subsequent calls return None,
        // which might not be the case for prefix matching.
        let in_scope = !self.scoped || self.scope == self.root_scope();
        let allow_end_matching = in_scope && (self.highest_peek_index <= self.peek_index());

        if allow_end_matching && !self.next_matching && !self.peek_matching {
            if let Some(end_fn) = self.end_match.clone() {
                let peek_index = self.peek_index();
                self.peek_matching = true;

                let end_matched = (end_fn)(self);

                self.peek_matching = false;
                self.set_peek_index(peek_index);

                if end_matched {
                    return None;
                }

                self.highest_peek_index = self.highest_peek_index.max(self.peek_index());
            }
        }

        let peeked_symbol_opt = match &mut self.parent {
            SymbolIteratorKind::Nested(parent) => parent.peeking_next(accept),
            SymbolIteratorKind::Root(root) => root.peeking_next(accept),
        };

        // Prefix matching after `peeking_next()` to skip prefix symbols, but pass `Newline` to nested iterators.
        if in_scope
            && !self.next_matching
            && !self.peek_matching
            && peeked_symbol_opt?.kind == SymbolKind::Newline
            && self.prefix_match.is_some()
        {
            let prefix_match = self
                .prefix_match
                .clone()
                .expect("Prefix match checked above to be some.");
            let peek_index = self.peek_index();
            self.peek_matching = true;

            let prefix_matched = prefix_match(self);

            self.peek_matching = false;

            if !prefix_matched {
                self.set_peek_index(peek_index);
                return None;
            }
        }

        peeked_symbol_opt
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use itertools::{Itertools, PeekingNext};

    use crate::scanner::{PrefixMatcher, SymbolKind};

    use super::{EndMatcher, SymbolIterator};

    #[test]
    fn peek_while_index() {
        let symbols = crate::scanner::scan_str("## ");

        let mut iterator = SymbolIterator::from(&*symbols);
        let hash_cnt = iterator
            .peeking_take_while(|symbol| symbol.kind == SymbolKind::Hash)
            .count();

        let next_symbol = iterator.nth(hash_cnt);
        let curr_index = iterator.index();

        assert_eq!(hash_cnt, 2, "Hash symbols in input not correctly detected.");
        assert_eq!(curr_index, 3, "Current index was not updated correctly.");
        assert_eq!(
            next_symbol.map(|s| s.kind),
            Some(SymbolKind::Whitespace),
            "Whitespace after hash symbols was not detected."
        );
        assert!(
            iterator.next().unwrap().kind == SymbolKind::Eoi,
            "Input end reached, but new symbol was returned."
        );
    }

    #[test]
    fn peek_next() {
        let symbols = crate::scanner::scan_str("#*");

        let mut iterator = SymbolIterator::from(&*symbols);

        let peeked_symbol = iterator.peeking_next(|_| true);
        let next_symbol = iterator.next();
        let next_peeked_symbol = iterator.peeking_next(|_| true);
        let curr_index = iterator.index();

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
        let symbols = crate::scanner::scan_str("text*");

        let mut iterator = SymbolIterator::from(&*symbols).nest(
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
    fn reach_consumed_end() {
        let symbols = crate::scanner::scan_str("text*");

        let mut iterator = SymbolIterator::from(&*symbols).nest(
            None,
            Some(Rc::new(|matcher| {
                matcher.consumed_matches(&[SymbolKind::Star])
            })),
        );

        let taken_symkinds = iterator
            .take_to_end()
            .iter()
            .map(|s| s.kind)
            .collect::<Vec<_>>();

        assert!(iterator.end_reached(), "Iterator end was not reached.");
        assert!(
            iterator.next().is_none(),
            "Iterator returns symbol after end."
        );
        assert_eq!(
            iterator.prev_symbol().unwrap().as_str(),
            "*",
            "Previous symbol was not the matched one."
        );
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
        let symbols = crate::scanner::scan_str("a\n* *b");

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
            vec![
                SymbolKind::Plain,
                SymbolKind::Newline,
                SymbolKind::Plain,
                SymbolKind::Eoi
            ],
            "Prefix symbols not correctly skipped"
        );
    }

    #[test]
    fn nested_peek() {
        let symbols = crate::scanner::scan_str("a\n* *b");

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

        let sym_1 = inner.peeking_next(|_| true);
        assert_eq!(
            "a",
            sym_1.unwrap().as_str(),
            "Peeking next symbol did not return 'a'."
        );
        let sym_2 = inner.peeking_next(|_| true);
        assert_eq!(
            "\n",
            sym_2.unwrap().as_str(),
            "Peeking next symbol did not return newline."
        );
        let sym_3 = inner.peeking_next(|_| true);
        assert_eq!(
            "b",
            sym_3.unwrap().as_str(),
            "Peeking next symbol did not return 'b'."
        );
    }

    #[test]
    fn outer_end_match_takes_precedence() {
        let symbols = crate::scanner::scan_str("e+f-");

        let mut iterator = SymbolIterator::with(
            &symbols,
            None,
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.matches(&[SymbolKind::Plus])
            })),
        );

        let mut inner = iterator.nest(
            None,
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.matches(&[SymbolKind::Minus])
            })),
        );

        assert_eq!(
            "e",
            inner.peeking_next(|_| true).unwrap().as_str(),
            "First peeked symbol is not 'e'."
        );
        assert!(
            inner.peeking_next(|_| true).is_none(),
            "Outer end did not take precedence with `peeking_next()`."
        );
        assert!(
            !inner.end_reached(),
            "Peeking end wrongfully set 'end_reached()'."
        );
        assert!(
            inner.peeking_next(|_| true).is_none(),
            "Successive peek over outer end returned symbol."
        );

        inner.reset_peek();

        assert_eq!(
            "e",
            inner.next().unwrap().as_str(),
            "First symbol is not 'e'."
        );
        assert!(
            inner.next().is_none(),
            "Outer end did not take precedence with `next()`."
        );
        assert!(
            !inner.end_reached(),
            "Reaching end set for inner, eventhough only outer reached end."
        );
        assert!(
            inner.next().is_none(),
            "Successive `next()` over outer end returned symbol."
        );

        inner.update(&mut iterator);

        assert!(
            iterator.end_reached(),
            "`end_reached()` not set for outer iterator."
        );
        assert!(
            iterator.next().is_none(),
            "Successive `next()` over outer end returned symbol."
        );
    }

    #[test]
    fn peek_and_next_return_same_symbols() {
        let symbols = crate::scanner::scan_str("a\n* *b+-");

        let iterator = SymbolIterator::with(
            &symbols,
            Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
                matcher.consumed_prefix(&[SymbolKind::Star, SymbolKind::Whitespace])
            })),
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.matches(&[SymbolKind::Plus])
            })),
        );

        let mut inner = iterator.nest(
            Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
                matcher.consumed_prefix(&[SymbolKind::Star])
            })),
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.matches(&[SymbolKind::Minus])
            })),
        );

        let peeked_symbols = inner.peeking_take_while(|_| true).collect::<Vec<_>>();
        inner.reset_peek();
        let next_symbols = inner.take_to_end();

        assert_eq!(
            peeked_symbols, next_symbols,
            "Peeked (left) and next (right) symbols differ."
        );
    }

    #[test]
    fn scoping() {
        let symbols = crate::scanner::scan_str("[o [i] o]");

        let mut iterator = SymbolIterator::scoped(
            &symbols,
            None,
            Some(Rc::new(|matcher| {
                matcher.consumed_matches(&[SymbolKind::CloseBracket])
            })),
        );

        iterator = iterator.dropping(1); // To skip first open bracket
        let mut taken_outer = iterator
            .by_ref()
            // Note: This will skip the open bracket for both iterators, but this is ok for this test
            .take_while(|s| s.kind != SymbolKind::OpenBracket)
            .collect::<Vec<_>>();

        let mut inner_iter = iterator.nest_with_scope(
            None,
            Some(Rc::new(|matcher| {
                matcher.consumed_matches(&[SymbolKind::CloseBracket])
            })),
        );

        let taken_inner = inner_iter.take_to_end();
        assert!(
            inner_iter.end_reached(),
            "Inner iterator end was not reached."
        );

        inner_iter.update(&mut iterator);

        taken_outer.extend(iterator.take_to_end().iter());

        assert!(iterator.end_reached(), "Iterator end was not reached.");
        assert_eq!(
            taken_inner.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            vec!["i"],
            "Inner symbols are incorrect."
        );
        assert_eq!(
            taken_outer.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            vec!["o", " ", " ", "o"],
            "Outer symbols are incorrect."
        );
    }

    #[test]
    fn prefix_mismatch_returns_none_forever() {
        let symbols = crate::scanner::scan_str("a\n  b\nc");

        let mut iterator = SymbolIterator::with(
            &symbols,
            Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
                matcher.consumed_prefix(&[SymbolKind::Whitespace, SymbolKind::Whitespace])
            })),
            None,
        );

        let sym_kinds = iterator
            .take_to_end()
            .iter()
            .map(|s| s.kind)
            .collect::<Vec<_>>();

        assert_eq!(
            sym_kinds,
            vec![SymbolKind::Plain, SymbolKind::Newline, SymbolKind::Plain,],
            "Iterator did not stop on prefix mismatch"
        );
        assert!(
            iterator.next().is_none(),
            "Prefix mismatch not returning `None`."
        );
        assert!(
            iterator.next().is_none(),
            "Prefix mismatch not returning `None`."
        );
    }

    #[test]
    fn prev_kind() {
        let symbols = crate::scanner::scan_str("a *\n");

        let mut iterator = SymbolIterator::with(&symbols, None, None);

        assert_eq!(
            iterator.next().unwrap().as_str(),
            "a",
            "`next()` returned wrong symbol."
        );
        assert_eq!(
            iterator.prev_kind().unwrap(),
            SymbolKind::Plain,
            "Previous SymbolKind not correctly stored."
        );

        assert_eq!(
            iterator.next().unwrap().as_str(),
            " ",
            "`next()` returned wrong symbol."
        );
        assert_eq!(
            iterator.prev_kind().unwrap(),
            SymbolKind::Whitespace,
            "Previous SymbolKind not correctly stored."
        );

        assert_eq!(
            iterator.next().unwrap().as_str(),
            "*",
            "`next()` returned wrong symbol."
        );
        assert_eq!(
            iterator.prev_kind().unwrap(),
            SymbolKind::Star,
            "Previous SymbolKind not correctly stored."
        );

        assert_eq!(
            iterator.next().unwrap().as_str(),
            "\n",
            "`next()` returned wrong symbol."
        );
        assert_eq!(
            iterator.prev_kind().unwrap(),
            SymbolKind::Newline,
            "Previous SymbolKind not correctly stored."
        );
    }

    #[test]
    fn prev_symbol_from_end_match() {
        let symbols = crate::scanner::scan_str("a*+b");

        let mut iterator = SymbolIterator::with(
            &symbols,
            None,
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.consumed_matches(&[SymbolKind::Star, SymbolKind::Plus])
            })),
        );

        let content = iterator
            .take_to_end()
            .iter()
            .fold(String::new(), |mut combined, s| {
                combined.push_str(s.as_str());
                combined
            });

        assert_eq!(content, "a", "End match returned wrong content.");
        assert_eq!(
            iterator.prev_symbol().unwrap().as_str(),
            "+",
            "Previous symbol not correctly updated from end match."
        );
    }

    #[test]
    fn prev_symbol_from_prefix_match() {
        let symbols = crate::scanner::scan_str("\n*+b");

        let mut iterator = SymbolIterator::with(
            &symbols,
            Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
                matcher.consumed_prefix(&[SymbolKind::Star, SymbolKind::Plus])
            })),
            None,
        );

        assert_eq!(
            iterator.next().unwrap().as_str(),
            "\n",
            "`next()` returned wrong symbol."
        );
        // Previous symbol is not set for prefix symbols, because `Newline` symbol gets passed to nested iterators for their prefix match
        assert_eq!(
            iterator.prev_symbol().unwrap().as_str(),
            "\n",
            "Previous symbol not correctly updated from prefix match."
        );
        assert_eq!(
            iterator.next().unwrap().as_str(),
            "b",
            "`next()` returned wrong symbol."
        );
    }
}
