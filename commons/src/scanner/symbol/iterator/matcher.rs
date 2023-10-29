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
    /// Consuming means the related iterator advances over the matched empty line, but not the end newline.
    /// Not consuming the end newline allows to consume contiguous empty lines.
    ///
    /// **Note:** The iterator is only advanced if an empty line is matched.
    ///
    /// **Note:** The empty line is **not** included in the symbols returned by [`SymbolIterator::take_to_end()`].
    fn consumed_is_empty_line(&mut self) -> bool;

    /// Returns `true` if the given [`Symbol`] sequence matches the upcoming one.
    ///
    /// [`Symbol`]: super::Symbol
    fn matches(&mut self, sequence: &[SymbolKind]) -> bool;

    /// Wrapper around [`Self::matches()`] that additionally consumes the matched sequence.
    /// Consuming means the related iterator advances over the matched sequence.
    ///
    /// **Note:** The iterator is only advanced if the sequence is matched.
    ///
    /// **Note:** The matched sequence is **not** included in the symbols returned by [`SymbolIterator::take_to_end()`].
    fn consumed_matches(&mut self, sequence: &[SymbolKind]) -> bool;

    /// Returns `true` if the given [`SymbolKind`] is equal to the kind of the previous symbol returned using `next()` or `consumed_matches()`.
    fn matches_prev(&mut self, kind: SymbolKind) -> bool;

    /// Returns `true` if the previous symbol returned using `next()` or `consumed_matches()` is either
    /// whitespace, newline, EOI, or no previous symbol exists.
    fn prev_is_space(&mut self) -> bool;
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
    /// **Note:** The matched sequence is **not** included in the symbols returned by [`SymbolIterator::take_to_end()`].
    ///
    /// [`Symbol`]: super::Symbol
    fn consumed_prefix(&mut self, sequence: &[SymbolKind]) -> bool;

    /// Returns `true` if the upcoming [`Symbol`] sequence is an empty line.
    /// Meaning that a line contains no [`Symbol`] or only [`SymbolKind::Whitespace`].
    ///
    /// **Note:** This is also `true` if a parent iterator stripped non-whitespace symbols, and the nested iterator only has whitespace symbols.
    ///
    /// [`Symbol`]: super::Symbol
    fn empty_line(&mut self) -> bool;
}

impl<'input> EndMatcher for SymbolIterator<'input> {
    fn is_empty_line(&mut self) -> bool {
        let peek_index = self.peek_index();

        let next = self
            .peeking_next(|s| matches!(s.kind, SymbolKind::Newline | SymbolKind::Eoi))
            .map(|s| s.kind);

        let is_empty_line = if Some(SymbolKind::Newline) == next {
            let _whitespaces = self
                .peeking_take_while(|s| s.kind == SymbolKind::Whitespace)
                .count();

            let new_line =
                self.peeking_next(|s| matches!(s.kind, SymbolKind::Newline | SymbolKind::Eoi));

            if Some(SymbolKind::Newline) == new_line.map(|s| s.kind) {
                self.set_peek_index(self.peek_index() - 1); // Do not consume next newline to enable consumption of contiguous empty lines
            }

            new_line.is_some()
        } else {
            next.is_some()
        };

        self.set_match_index(self.peek_index());
        self.set_peek_index(peek_index);

        is_empty_line
    }

    fn consumed_is_empty_line(&mut self) -> bool {
        let is_empty_line = self.is_empty_line();

        if is_empty_line {
            self.set_peek_index(self.match_index()); // To consume matched symbols for `peeking_next()`

            if !self.peek_matching {
                self.set_index(self.match_index()); // To consume matched symbols for `next()`

                // This could set the wrong symbol for prefix matches,
                // but the previous symbol for prefix matches gets overwritten in `next()` anyways.
                self.prev_symbol = self.prev_root_symbol().copied();
            }
        }

        is_empty_line
    }

    fn matches(&mut self, sequence: &[SymbolKind]) -> bool {
        // Note: Multiple matches may be set in the match closure, so we need to ensure that all start at the same index
        let peek_index = self.peek_index();

        for kind in sequence {
            if self.peeking_next(|s| s.kind == *kind).is_none() {
                self.set_peek_index(peek_index);
                return false;
            }
        }

        self.set_match_index(self.peek_index());
        self.set_peek_index(peek_index);
        true
    }

    fn consumed_matches(&mut self, sequence: &[SymbolKind]) -> bool {
        let matched = self.matches(sequence);

        if matched {
            self.set_peek_index(self.match_index()); // To consume matched symbols for `peeking_next()`

            if !self.peek_matching {
                self.set_index(self.match_index()); // To consume matched symbols for `next()`

                // This could set the wrong symbol for prefix matches,
                // but the previous symbol for prefix matches gets overwritten in `next()` anyways.
                self.prev_symbol = self.prev_root_symbol().copied();
            }
        }

        matched
    }

    fn matches_prev(&mut self, kind: SymbolKind) -> bool {
        self.prev_kind()
            .map_or(false, |self_kind| self_kind == kind)
    }

    fn prev_is_space(&mut self) -> bool {
        // default `true`, because "no prev" means start of input, which is considered as space.
        self.prev_kind().map_or(true, |k| k.is_space())
    }
}

impl<'input> PrefixMatcher for SymbolIterator<'input> {
    fn consumed_prefix(&mut self, sequence: &[SymbolKind]) -> bool {
        debug_assert!(
            !sequence.contains(&SymbolKind::Newline),
            "Newline symbol in prefix match is not allowed."
        );

        self.consumed_matches(sequence)
    }

    fn empty_line(&mut self) -> bool {
        let peek_index = self.peek_index();

        // NOTE: `Newline` at start is already ensured for prefix matches.
        let _whitespaces = self
            .peeking_take_while(|s| s.kind == SymbolKind::Whitespace)
            .count();

        let new_line =
            self.peeking_next(|s| matches!(s.kind, SymbolKind::Newline | SymbolKind::Eoi));

        self.set_peek_index(peek_index);
        new_line.is_some()
    }
}
