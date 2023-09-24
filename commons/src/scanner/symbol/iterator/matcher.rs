//! Contains matcher traits and types used to detect iterator end and strip prefixes.
//! The available matcher traits are implemented for [`SymbolIterator`].

use std::rc::Rc;

use itertools::{Itertools, PeekingNext};

use crate::scanner::SymbolKind;

use super::SymbolIterator;

/// Function type to notify an iterator if an end was reached.
pub type IteratorEndFn = Rc<dyn (Fn(&mut dyn EndMatcher) -> bool)>;
/// Function type to consume prefix sequences of a new line.
pub type IteratorPrefixFn = Rc<dyn (Fn(&mut dyn PrefixMatcher) -> bool)>;

/// Trait containing functions that are available inside the end matcher function.
pub trait EndMatcher {
    /// Returns `true` if the upcoming [`Symbol`] sequence is an empty line.
    /// Meaning that a line contains no [`Symbol`] or only [`SymbolKind::Whitespace`].
    ///
    /// **Note:** This is also `true` if a parent iterator stripped non-whitespace symbols, and the nested iterator only has whitespace symbols.
    ///
    /// [`Symbol`]: super::Symbol
    fn is_empty_line(&mut self) -> bool;

    /// Wrapper around [`Self::is_empty_line()`] that additionally consumes the matched empty line.
    /// Consuming means the related iterator advances over the matched empty line.
    ///
    /// **Note:** The iterator is only advanced if an empty line is matched.
    fn consumed_is_empty_line(&mut self) -> bool;

    /// Returns `true` if the given [`Symbol`] sequence matches the upcoming one.
    ///
    /// [`Symbol`]: super::Symbol
    fn matches(&mut self, sequence: &[SymbolKind]) -> bool;

    /// Wrapper around [`Self::matches()`] that additionally consumes the matched sequence.
    /// Consuming means the related iterator advances over the matched sequence.
    ///
    /// **Note:** The iterator is only advanced if the sequence is matched.
    fn consumed_matches(&mut self, sequence: &[SymbolKind]) -> bool;
}

/// Trait containing functions that are available inside the prefix matcher function.
pub trait PrefixMatcher {
    /// Consumes and returns `true` if the given [`Symbol`] sequence matches the upcoming one.
    /// Consuming means the related iterator advances over the matched sequence.
    ///
    /// **Note:** The iterator is only advanced if the sequence is matched.
    ///
    /// **Note:** The given sequence must **not** include any [`SymbolKind::Newline`], because matches are only considered per line.
    ///
    /// [`Symbol`]: super::Symbol
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
