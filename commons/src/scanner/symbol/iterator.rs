use std::sync::Arc;

use super::{Symbol, SymbolKind};

#[derive(Default, Clone)]
pub struct SymbolIterator<'input, 'end_fn> {
    symbols: &'input [Symbol<'input>],
    curr_index: usize,
    start_index: usize,
    line_prefixes: Vec<Vec<SymbolKind>>,
    end: Vec<Arc<dyn IteratorEndFn<'input> + 'end_fn>>,
}

pub trait IteratorEndFn<'input>: Fn(&'input [Symbol<'input>]) -> bool + Send + Sync {}

impl<'input, 'end_fn> SymbolIterator<'input, 'end_fn> {
    pub fn new(symbols: &'input [Symbol<'input>], start_index: usize) -> Self {
        SymbolIterator {
            symbols,
            curr_index: start_index,
            start_index,
            line_prefixes: vec![],
            end: vec![],
        }
    }

    pub fn with(
        symbols: &'input [Symbol<'input>],
        start_index: usize,
        line_prefix: impl Into<Vec<Vec<SymbolKind>>>,
        end: impl IteratorEndFn<'input> + 'end_fn,
    ) -> Self {
        SymbolIterator {
            symbols,
            curr_index: start_index,
            start_index,
            line_prefixes: line_prefix.into(),
            end: vec![Arc::new(end)],
        }
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
        }
    }

    pub fn eoi(&self) -> bool {
        self.curr_index == self.symbols.len()
    }

    pub fn nest<'inner_end>(
        &self,
        line_prefix: &[SymbolKind],
        end: Option<impl IteratorEndFn<'input> + 'inner_end>,
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
            line_prefixes: nested_prefixes,
            end: merged_end,
        }
    }

    pub fn next(&mut self) -> Result<&Symbol<'input>, SymbolIteratorError> {
        if self.eoi() {
            return Err(SymbolIteratorError::Eoi);
        }

        let mut curr_symbolkind = match self.symbols.get(self.curr_index) {
            Some(curr_symbol) => curr_symbol.kind,
            None => return Err(SymbolIteratorError::Eoi),
        };

        if curr_symbolkind == SymbolKind::Newline && !self.line_prefixes.is_empty() {
            let curr_prefix_symbolkinds: Vec<_> = self.symbols[self.curr_index + 1..]
                .iter()
                .map(|s| s.kind)
                .collect();

            let mut prefix_matched = false;

            for prefix in &self.line_prefixes {
                if prefix == &curr_prefix_symbolkinds {
                    prefix_matched = true;
                    self.curr_index += prefix.len();
                    curr_symbolkind = match self.symbols.get(self.curr_index) {
                        Some(curr_symbol) => curr_symbol.kind,
                        None => return Err(SymbolIteratorError::Eoi),
                    };
                    break;
                }
            }

            if !prefix_matched {
                return Err(SymbolIteratorError::PrefixMismatch);
            }
        } else if curr_symbolkind == SymbolKind::Blankline
            && contains_only_non_whitespace_sequences(&self.line_prefixes)
        {
            return Err(SymbolIteratorError::PrefixMismatch);
        }

        for f in &self.end {
            if f(&self.symbols[self.curr_index..]) {
                return Err(SymbolIteratorError::EndReached);
            }
        }

        let symbol_opt = self.symbols.get(self.curr_index);
        self.curr_index += 1;

        symbol_opt.ok_or(SymbolIteratorError::Eoi)
    }

    pub fn skip_to_end(mut self) -> Self {
        let mut end_reached = false;

        while !end_reached || !self.eoi() {
            for f in &self.end {
                if f(&self.symbols[self.curr_index..]) {
                    end_reached = true;
                }
            }

            if !end_reached {
                self.curr_index += 1;
            }
        }

        self
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
