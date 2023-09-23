use std::{borrow::BorrowMut, rc::Rc};

use super::{Symbol, SymbolKind};

pub use itertools::*;

#[derive(Clone)]
pub struct SymbolIterator<'input> {
    kind: SymbolIteratorKind<'input>,
    start_index: usize,
    prefix_match: Option<IteratorPrefixFn>,
    end_match: Option<IteratorEndFn>,
    iter_end: bool,
}

#[derive(Clone)]
pub struct SymbolIteratorRoot<'input> {
    symbols: &'input [Symbol<'input>],
    curr_index: usize,
    peek_index: usize,
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

impl<'input> SymbolIteratorRoot<'input> {
    fn remaining_symbols(&self) -> Option<&'input [Symbol<'input>]> {
        self.symbols.get(self.curr_index..)
    }
}

#[derive(Clone)]
pub enum SymbolIteratorKind<'input> {
    Nested(Box<SymbolIterator<'input>>),
    Root(SymbolIteratorRoot<'input>),
}

pub type IteratorEndFn = Rc<dyn (Fn(&mut dyn EndMatcher) -> bool)>;
pub type IteratorPrefixFn = Rc<dyn (Fn(&mut dyn PrefixMatcher) -> bool)>;

pub trait EndMatcher {
    fn is_empty_line(&mut self) -> bool;
    fn consumed_is_empty_line(&mut self) -> bool;
    fn matches(&mut self, sequence: &[SymbolKind]) -> bool;
    fn consumed_matches(&mut self, sequence: &[SymbolKind]) -> bool;
}

pub trait PrefixMatcher {
    fn consumed_prefix(&mut self, sequence: &[SymbolKind]) -> bool;
}

impl<'input> EndMatcher for SymbolIterator<'input> {
    fn is_empty_line(&mut self) -> bool {
        self.reset_peek();

        let next = self
            .peeking_next(|s| matches!(s.kind, SymbolKind::Blankline | SymbolKind::Newline))
            .map(|s| s.kind);

        let is_empty_line = if Some(SymbolKind::Newline) == next {
            let _whitespaces = self
                .peeking_take_while(|s| s.kind == SymbolKind::Whitespace)
                .count();
            // self.set_peek_index(self.peek_index().saturating_sub(1)); // Note: To compensate last "peeking_next()" in "peeking_take_while()"

            let new_line = self
                .peeking_next(|s| matches!(s.kind, SymbolKind::Blankline | SymbolKind::Newline));
            new_line.is_some()
        } else {
            Some(SymbolKind::Blankline) == next
        };

        is_empty_line
    }

    fn consumed_is_empty_line(&mut self) -> bool {
        let is_empty_line = self.is_empty_line();

        if is_empty_line {
            self.set_curr_index(self.peek_index()); // To consume peeked symbols
        }

        is_empty_line
    }

    fn matches(&mut self, sequence: &[SymbolKind]) -> bool {
        self.reset_peek();

        for kind in sequence {
            if self.peeking_next(|s| s.kind == *kind).is_none() {
                return false;
            }
        }

        true
    }

    fn consumed_matches(&mut self, sequence: &[SymbolKind]) -> bool {
        let matched = self.matches(sequence);

        if matched {
            self.set_curr_index(self.peek_index()); // To consume peeked symbols
        }

        matched
    }
}

impl<'input> PrefixMatcher for SymbolIterator<'input> {
    fn consumed_prefix(&mut self, sequence: &[SymbolKind]) -> bool {
        #[cfg(debug_assertions)]
        assert!(
            !sequence.contains(&SymbolKind::Newline),
            "Newline symbol in prefix match is not allowed."
        );

        self.consumed_matches(sequence)
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

impl<'input> SymbolIterator<'input> {
    pub fn new(symbols: &'input [Symbol<'input>], start_index: usize) -> Self {
        let mut iter = SymbolIterator::from(symbols);
        iter.start_index = start_index;
        iter
    }

    pub fn with(
        symbols: &'input [Symbol<'input>],
        start_index: usize,
        prefix_match: Option<IteratorPrefixFn>,
        end: Option<IteratorEndFn>,
    ) -> Self {
        SymbolIterator {
            kind: SymbolIteratorKind::Root(SymbolIteratorRoot::from(symbols)),
            start_index,
            prefix_match,
            end_match: end,
            iter_end: false,
        }
    }

    pub fn len(&self) -> usize {
        self.remaining_symbols().unwrap_or(&[]).len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn start_index(&self) -> usize {
        self.start_index
    }

    pub fn curr_index(&self) -> usize {
        match &self.kind {
            SymbolIteratorKind::Nested(parent) => parent.curr_index(),
            SymbolIteratorKind::Root(root) => root.curr_index,
        }
    }

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

    fn peek_index(&self) -> usize {
        match &self.kind {
            SymbolIteratorKind::Nested(parent) => parent.peek_index(),
            SymbolIteratorKind::Root(root) => root.peek_index,
        }
    }

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

    pub fn reset_peek(&mut self) {
        self.set_peek_index(self.curr_index());
    }

    pub fn remaining_symbols(&self) -> Option<&'input [Symbol<'input>]> {
        match &self.kind {
            SymbolIteratorKind::Nested(parent) => parent.remaining_symbols(),
            SymbolIteratorKind::Root(root) => root.remaining_symbols(),
        }
    }

    pub fn peek(&mut self) -> Option<&'input Symbol<'input>> {
        let symbol = self.peeking_next(|_| true);
        self.reset_peek(); // Note: Resetting index, because peek() must be idempotent
        symbol
    }

    pub fn peek_kind(&mut self) -> Option<SymbolKind> {
        self.peek().map(|s| s.kind)
    }

    pub fn nest(
        &self,
        prefix_match: Option<IteratorPrefixFn>,
        end: Option<IteratorEndFn>,
    ) -> SymbolIterator<'input> {
        SymbolIterator {
            kind: SymbolIteratorKind::Nested(Box::new(self.clone())),
            start_index: self.curr_index(),
            prefix_match,
            end_match: end,
            iter_end: self.iter_end,
        }
    }

    pub fn update(self, parent: &mut Self) {
        if let SymbolIteratorKind::Nested(self_parent) = self.kind {
            *parent = *self_parent;
        }
    }

    /// Tries to skip symbols until one of the end functions signals the end.
    ///
    /// **Note:** This function might not reach the iterator end.
    /// If no symbols are left, or no given line prefix is matched, the iterator may stop before an end is reached.
    /// Use `end_reached()` to check if the end was actually reached.
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

    pub fn end_reached(&self) -> bool {
        self.iter_end
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
            0,
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
