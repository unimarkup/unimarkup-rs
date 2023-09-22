use std::sync::Arc;

use itertools::PeekingNext;

use super::{Symbol, SymbolKind};

pub use itertools::*;

#[derive(Default, Clone)]
pub struct SymbolIterator<'input, 'end_fn> {
    symbols: &'input [Symbol<'input>],
    curr_index: usize,
    start_index: usize,
    peek_index: usize,
    line_prefixes: Vec<Vec<SymbolKind>>,
    end: Vec<Arc<IteratorEndFn<'input, 'end_fn>>>,
}

pub type IteratorEndFn<'input, 'end_fn> =
    Box<dyn Fn(&'input [Symbol<'input>]) -> bool + Send + Sync + 'end_fn>;

impl<'input, 'end_fn> From<&'input [Symbol<'input>]> for SymbolIterator<'input, 'end_fn> {
    fn from(value: &'input [Symbol<'input>]) -> Self {
        SymbolIterator {
            symbols: value,
            curr_index: 0,
            start_index: 0,
            peek_index: 0,
            line_prefixes: vec![],
            end: vec![],
        }
    }
}

impl<'input, 'end_fn> From<&'input Vec<Symbol<'input>>> for SymbolIterator<'input, 'end_fn> {
    fn from(value: &'input Vec<Symbol<'input>>) -> Self {
        SymbolIterator {
            symbols: value,
            curr_index: 0,
            start_index: 0,
            peek_index: 0,
            line_prefixes: vec![],
            end: vec![],
        }
    }
}

impl<'input, 'end_fn> SymbolIterator<'input, 'end_fn> {
    pub fn new(symbols: &'input [Symbol<'input>], start_index: usize) -> Self {
        SymbolIterator {
            symbols,
            curr_index: start_index,
            start_index,
            peek_index: start_index,
            line_prefixes: vec![],
            end: vec![],
        }
    }

    pub fn with(
        symbols: &'input [Symbol<'input>],
        start_index: usize,
        line_prefix: impl Into<Vec<Vec<SymbolKind>>>,
        end: IteratorEndFn<'input, 'end_fn>,
    ) -> Self {
        SymbolIterator {
            symbols,
            curr_index: start_index,
            start_index,
            peek_index: start_index,
            line_prefixes: line_prefix.into(),
            end: vec![Arc::new(end)],
        }
    }

    pub fn len(&self) -> usize {
        self.symbols[self.start_index..].len()
    }

    pub fn is_empty(&self) -> bool {
        self.symbols[self.start_index..].is_empty()
    }

    pub fn start_index(&self) -> usize {
        self.start_index
    }

    pub fn curr_index(&self) -> usize {
        self.curr_index
    }

    pub fn set_curr_index(&mut self, index: usize) {
        if index >= self.start_index {
            self.curr_index = index;
            self.peek_index = self.curr_index;
        }
    }

    pub fn eoi(&self) -> bool {
        self.curr_index == self.symbols.len()
    }

    pub fn remaining_symbols(&self) -> &'input [Symbol<'input>] {
        &self.symbols[self.curr_index..]
    }

    pub fn peek(&self) -> Option<&'input Symbol<'input>> {
        self.symbols.get(self.curr_index)
    }

    pub fn peek_kind(&self) -> Option<SymbolKind> {
        self.symbols.get(self.curr_index).map(|s| s.kind)
    }

    pub fn nest<'inner_end>(
        &self,
        line_prefix: &[SymbolKind],
        end: Option<IteratorEndFn<'input, 'inner_end>>,
    ) -> SymbolIterator<'input, 'inner_end>
    where
        'end_fn: 'inner_end,
    {
        let mut nested_prefixes = self.line_prefixes.clone();
        if nested_prefixes.is_empty() {
            nested_prefixes.push(vec![]);
        }

        if !line_prefix.contains(&SymbolKind::Blankline) {
            nested_prefixes
                .iter_mut()
                .for_each(|p| p.extend_from_slice(line_prefix));
        }

        let mut outer_end = self.end.clone();
        let merged_end = match end {
            Some(inner_end) => {
                outer_end.push(Arc::new(inner_end));
                outer_end
            }
            None => outer_end,
        };

        SymbolIterator {
            symbols: self.symbols,
            curr_index: self.curr_index,
            start_index: self.curr_index,
            peek_index: self.curr_index,
            line_prefixes: nested_prefixes,
            end: merged_end,
        }
    }

    pub fn nest_prefixes<'inner_end>(
        &self,
        line_prefixes: &[Vec<SymbolKind>],
        end: Option<IteratorEndFn<'input, 'inner_end>>,
    ) -> SymbolIterator<'input, 'inner_end>
    where
        'end_fn: 'inner_end,
    {
        let prefixes = if self.line_prefixes.is_empty() {
            let mut nested_prefixes = self.line_prefixes.clone();
            nested_prefixes.extend_from_slice(line_prefixes);
            nested_prefixes
        } else {
            // create cartesian prefix
            self.line_prefixes
                .iter()
                .flat_map(|outer_prefixes| {
                    line_prefixes.iter().map(|inner_prefixes| {
                        let mut prefix = outer_prefixes.clone();

                        if !inner_prefixes.contains(&SymbolKind::Blankline) {
                            prefix.extend(inner_prefixes);
                        }

                        prefix
                    })
                })
                .collect()
        };

        let mut outer_end = self.end.clone();
        let merged_end = match end {
            Some(inner_end) => {
                outer_end.push(Arc::new(inner_end));
                outer_end
            }
            None => outer_end,
        };

        SymbolIterator {
            symbols: self.symbols,
            curr_index: self.curr_index,
            start_index: self.curr_index,
            peek_index: self.curr_index,
            line_prefixes: prefixes,
            end: merged_end,
        }
    }

    // #[allow(clippy::should_implement_trait)]
    // pub fn next(&mut self) -> Result<&Symbol<'input>, SymbolIteratorError> {
    //     if self.eoi() {
    //         return Err(SymbolIteratorError::Eoi);
    //     }

    //     let mut curr_symbolkind = match self.symbols.get(self.curr_index) {
    //         Some(curr_symbol) => curr_symbol.kind,
    //         None => return Err(SymbolIteratorError::Eoi),
    //     };

    //     if curr_symbolkind == SymbolKind::Newline && !self.line_prefixes.is_empty() {
    //         let curr_prefix_symbolkinds: Vec<_> = self.symbols[self.curr_index + 1..]
    //             .iter()
    //             .map(|s| s.kind)
    //             .collect();

    //         let mut prefix_matched = false;

    //         for prefix in &self.line_prefixes {
    //             if prefix == &curr_prefix_symbolkinds {
    //                 prefix_matched = true;
    //                 self.curr_index += prefix.len();
    //                 curr_symbolkind = match self.symbols.get(self.curr_index) {
    //                     Some(curr_symbol) => curr_symbol.kind,
    //                     None => return Err(SymbolIteratorError::Eoi),
    //                 };
    //                 break;
    //             }
    //         }

    //         if !prefix_matched {
    //             return Err(SymbolIteratorError::PrefixMismatch);
    //         }
    //     } else if curr_symbolkind == SymbolKind::Blankline
    //         && contains_only_non_whitespace_sequences(&self.line_prefixes)
    //     {
    //         return Err(SymbolIteratorError::PrefixMismatch);
    //     }

    //     for f in &self.end {
    //         if f(&self.symbols[self.curr_index..]) {
    //             return Err(SymbolIteratorError::EndReached);
    //         }
    //     }

    //     let symbol_opt = self.symbols.get(self.curr_index);
    //     self.curr_index += 1;

    //     symbol_opt.ok_or(SymbolIteratorError::Eoi)
    // }

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
        for f in &self.end {
            if f(&self.symbols[self.curr_index..]) {
                return true;
            }
        }

        false
    }
}

impl<'input, 'end_fn> Iterator for SymbolIterator<'input, 'end_fn> {
    type Item = &'input Symbol<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eoi() {
            return None;
        }

        let curr_symbol_opt = self.symbols.get(self.curr_index);
        let curr_symbolkind = match curr_symbol_opt {
            Some(curr_symbol) => curr_symbol.kind,
            None => return None,
        };

        if curr_symbolkind == SymbolKind::Newline && !self.line_prefixes.is_empty() {
            let mut prefix_matched = false;

            for prefix in &self.line_prefixes {
                let curr_prefix_symbolkinds: Vec<_> = self.symbols
                    [self.curr_index + 1..self.curr_index + prefix.len()]
                    .iter()
                    .map(|s| s.kind)
                    .collect();

                if prefix == &curr_prefix_symbolkinds {
                    prefix_matched = true;
                    self.curr_index += prefix.len();
                    self.peek_index = self.curr_index;
                    break;
                }
            }

            if !prefix_matched {
                return None;
            }
        } else if curr_symbolkind == SymbolKind::Blankline
            && contains_only_non_whitespace_sequences(&self.line_prefixes)
        {
            return None;
        }

        if self.end_reached() {
            return None;
        }

        self.curr_index += 1;
        self.peek_index = self.curr_index;
        curr_symbol_opt
    }
}

impl<'input, 'end_fn> PeekingNext for SymbolIterator<'input, 'end_fn> {
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
            Some(Box::new(|sequence| {
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
}
