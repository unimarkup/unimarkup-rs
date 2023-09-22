use std::{borrow::BorrowMut, rc::Rc};

use super::{Symbol, SymbolKind};

pub use itertools::*;

#[derive(Clone)]
pub struct SymbolIterator<'input> {
    kind: SymbolIteratorKind<'input>,
    start_index: usize,
    line_prefixes: Vec<Vec<SymbolKind>>,
    end: Option<IteratorEndFn<'input>>,
    iter_end: bool,
}

#[derive(Clone)]
pub struct SymbolIteratorRoot<'input> {
    symbols: &'input [Symbol<'input>],
    curr_index: usize,
    peek_index: usize,
    new_line: bool,
}

impl<'input> From<&'input [Symbol<'input>]> for SymbolIteratorRoot<'input> {
    fn from(value: &'input [Symbol<'input>]) -> Self {
        SymbolIteratorRoot {
            symbols: value,
            curr_index: 0,
            peek_index: 0,
            new_line: false,
        }
    }
}

impl<'input> From<&'input Vec<Symbol<'input>>> for SymbolIteratorRoot<'input> {
    fn from(value: &'input Vec<Symbol<'input>>) -> Self {
        SymbolIteratorRoot {
            symbols: value,
            curr_index: 0,
            peek_index: 0,
            new_line: false,
        }
    }
}

impl<'input> SymbolIteratorRoot<'input> {
    fn remaining_symbols(&self) -> &'input [Symbol<'input>] {
        &self.symbols[self.curr_index..]
    }
}

#[derive(Clone)]
pub enum SymbolIteratorKind<'input> {
    Nested(Box<SymbolIterator<'input>>),
    Root(SymbolIteratorRoot<'input>),
}

pub type IteratorEndFn<'input> = Rc<dyn (Fn(&'input [Symbol<'input>]) -> bool)>;

impl<'input> From<&'input [Symbol<'input>]> for SymbolIterator<'input> {
    fn from(value: &'input [Symbol<'input>]) -> Self {
        SymbolIterator {
            kind: SymbolIteratorKind::Root(SymbolIteratorRoot::from(value)),
            start_index: 0,
            line_prefixes: vec![],
            end: None,
            iter_end: false,
        }
    }
}

impl<'input> From<&'input Vec<Symbol<'input>>> for SymbolIterator<'input> {
    fn from(value: &'input Vec<Symbol<'input>>) -> Self {
        SymbolIterator {
            kind: SymbolIteratorKind::Root(SymbolIteratorRoot::from(value)),
            start_index: 0,
            line_prefixes: vec![],
            end: None,
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
        line_prefix: impl Into<Vec<Vec<SymbolKind>>>,
        end: IteratorEndFn<'input>,
    ) -> Self {
        SymbolIterator {
            kind: SymbolIteratorKind::Root(SymbolIteratorRoot::from(symbols)),
            start_index,
            line_prefixes: line_prefix.into(),
            end: Some(end),
            iter_end: false,
        }
    }

    pub fn len(&self) -> usize {
        match &self.kind {
            SymbolIteratorKind::Nested(parent) => parent.len(),
            SymbolIteratorKind::Root(root) => root.symbols[self.start_index..].len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.kind {
            SymbolIteratorKind::Nested(parent) => parent.is_empty(),
            SymbolIteratorKind::Root(root) => root.symbols[self.start_index..].is_empty(),
        }
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

    pub fn eoi(&self) -> bool {
        self.curr_index() == self.len()
    }

    pub fn remaining_symbols(&self) -> &'input [Symbol<'input>] {
        match &self.kind {
            SymbolIteratorKind::Nested(parent) => parent.remaining_symbols(),
            SymbolIteratorKind::Root(root) => root.remaining_symbols(),
        }
    }

    pub fn new_line(&self) -> bool {
        match &self.kind {
            SymbolIteratorKind::Nested(parent) => parent.new_line(),
            SymbolIteratorKind::Root(root) => root.new_line,
        }
    }

    pub fn peek(&self) -> Option<&'input Symbol<'input>> {
        match &self.kind {
            SymbolIteratorKind::Nested(parent) => parent.peek(),
            SymbolIteratorKind::Root(root) => root.symbols.get(root.curr_index),
        }
    }

    pub fn peek_kind(&self) -> Option<SymbolKind> {
        self.peek().map(|s| s.kind)
    }

    pub fn nest(
        self,
        line_prefix: &[SymbolKind],
        end: Option<IteratorEndFn<'input>>,
    ) -> SymbolIterator<'input> {
        let curr_index = self.curr_index();
        let iter_end = self.iter_end;

        SymbolIterator {
            kind: SymbolIteratorKind::Nested(Box::new(self)),
            start_index: curr_index,
            line_prefixes: vec![line_prefix.to_vec()],
            end,
            iter_end,
        }
    }

    pub fn nest_prefixes(
        &self,
        line_prefixes: impl Into<Vec<Vec<SymbolKind>>>,
        end: Option<IteratorEndFn<'input>>,
    ) -> SymbolIterator<'input> {
        let curr_index = self.curr_index();
        let iter_end = self.iter_end;

        SymbolIterator {
            kind: SymbolIteratorKind::Nested(Box::new(self.clone())),
            start_index: curr_index,
            line_prefixes: line_prefixes.into(),
            end,
            iter_end,
        }
    }

    pub fn skip_to_end(mut self) -> Self {
        while self.next().is_some() {}

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

    pub fn parent(self) -> Option<SymbolIterator<'input>> {
        match self.kind {
            SymbolIteratorKind::Nested(parent) => Some(*parent),
            SymbolIteratorKind::Root(_) => None,
        }
    }
}

impl<'input> Iterator for SymbolIteratorRoot<'input> {
    type Item = &'input Symbol<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.symbols.get(self.curr_index) {
            Some(symbol) => {
                self.curr_index += 1;
                self.peek_index = self.curr_index;
                self.new_line = symbol.kind == SymbolKind::Newline;

                Some(symbol)
            }
            None => None,
        }
    }
}

impl<'input> PeekingNext for SymbolIteratorRoot<'input> {
    fn peeking_next<F>(&mut self, accept: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnOnce(&Self::Item) -> bool,
    {
        let curr_index = self.curr_index;
        self.curr_index = self.peek_index; // Note: peek_index increases until `next()` is called directly
        let next_item = self.next();

        // revert index to simulate lookahead
        self.curr_index = curr_index;

        match next_item {
            Some(symbol) => {
                if (accept)(&symbol) {
                    next_item
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

impl<'input> Iterator for SymbolIterator<'input> {
    type Item = &'input Symbol<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eoi() || self.end_reached() {
            return None;
        }

        if self.peek_kind()? == SymbolKind::Blankline
            && contains_only_non_whitespace_sequences(&self.line_prefixes)
        {
            return None;
        }

        let symbols = match &self.kind {
            SymbolIteratorKind::Nested(parent) => {
                if parent.end_reached() {
                    self.iter_end = true;
                    return None;
                } else {
                    parent.remaining_symbols()
                }
            }
            SymbolIteratorKind::Root(root) => root.remaining_symbols(),
        };

        if let Some(end_fn) = &self.end {
            if (end_fn)(symbols) {
                self.iter_end = true;
                return None;
            }
        }

        let curr_symbol_opt = match &mut self.kind {
            SymbolIteratorKind::Nested(parent) => parent.next(),
            SymbolIteratorKind::Root(root) => root.next(),
        };

        if self.new_line() && !self.line_prefixes.is_empty() {
            let mut prefix_matched = false;

            for prefix in &self.line_prefixes {
                let curr_prefix_symbolkinds: Vec<_> = self.remaining_symbols()[..prefix.len()]
                    .iter()
                    .map(|s| s.kind)
                    .collect();

                if prefix == &curr_prefix_symbolkinds {
                    prefix_matched = true;
                    // Note: Only update index. Prevents `new_line()` from being changed by possible parent
                    self.set_curr_index(self.curr_index() + prefix.len());
                    break;
                }
            }

            // Note: This mostly indicates a syntax violation, so skipped symbol is ok.
            if !prefix_matched {
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
        match self.kind.borrow_mut() {
            SymbolIteratorKind::Nested(parent) => parent.peeking_next(accept),
            SymbolIteratorKind::Root(root) => root.peeking_next(accept),
        }
    }
}

pub enum SymbolIteratorError {
    /// At least one end-function returned `true`.
    EndReached,
    /// A new line did not start with the expected prefix.
    PrefixMismatch,
    /// Reached end of input.
    Eoi,
}

fn contains_only_non_whitespace_sequences(sequences: &[Vec<SymbolKind>]) -> bool {
    let mut whitespace_sequence_found = false;

    for sequence in sequences {
        whitespace_sequence_found = whitespace_sequence_found || !contains_non_whitespace(sequence);
    }
    whitespace_sequence_found
}

fn contains_non_whitespace(sequence: &[SymbolKind]) -> bool {
    for kind in sequence {
        if !matches!(
            kind,
            SymbolKind::Whitespace | SymbolKind::Newline | SymbolKind::Blankline
        ) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use itertools::{Itertools, PeekingNext};

    use crate::scanner::{Scanner, SymbolKind};

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
            &[],
            Some(Rc::new(|sequence| {
                sequence
                    .get(0)
                    .map(|s| s.kind == SymbolKind::Star)
                    .unwrap_or(false)
            })),
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
            vec![vec![SymbolKind::Star, SymbolKind::Whitespace]],
            Box::new(|_| false),
        );

        let mut inner = iterator.nest(
            &[SymbolKind::Star],
            Some(Box::new(|sequence| {
                sequence
                    .get(0)
                    .map(|s| s.kind == SymbolKind::Star)
                    .unwrap_or(false)
            })),
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
