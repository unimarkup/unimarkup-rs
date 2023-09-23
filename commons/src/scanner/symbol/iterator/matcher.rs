use std::rc::Rc;

use itertools::{Itertools, PeekingNext};

use crate::scanner::SymbolKind;

use super::SymbolIterator;

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
        // Note: Multiple matches may be set in the match closure, so we need to ensure that all start at the same index
        self.reset_peek();

        let next = self
            .peeking_next(|s| matches!(s.kind, SymbolKind::Blankline | SymbolKind::Newline))
            .map(|s| s.kind);

        let is_empty_line = if Some(SymbolKind::Newline) == next {
            let _whitespaces = self
                .peeking_take_while(|s| s.kind == SymbolKind::Whitespace)
                .count();

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
        // Note: Multiple matches may be set in the match closure, so we need to ensure that all start at the same index
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
