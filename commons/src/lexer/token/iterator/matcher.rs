//! Contains matcher traits and types used to detect iterator end and strip prefixes.
//! The available matcher traits are implemented for [`SymbolIterator`].

use std::rc::Rc;

use itertools::{Itertools, PeekingNext};

use crate::lexer::token::{Token, TokenKind};

use super::TokenIterator;

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
    fn is_blank_line(&mut self) -> bool;

    /// Wrapper around [`Self::is_empty_line()`] that additionally consumes the matched empty line.
    /// Consuming means the related iterator advances over the matched empty line, but not the end newline.
    /// Not consuming the end newline allows to consume contiguous empty lines.
    ///
    /// **Note:** The iterator is only advanced if an empty line is matched.
    ///
    /// **Note:** The empty line is **not** included in the symbols returned by [`SymbolIterator::take_to_end()`].
    fn consumed_is_blank_line(&mut self) -> bool;

    /// Returns `true` if the given [`Symbol`] sequence matches the upcoming one.
    ///
    /// [`Symbol`]: super::Symbol
    fn matches(&mut self, sequence: &[TokenKind]) -> bool;

    /// Wrapper around [`Self::matches()`] that additionally consumes the matched sequence.
    /// Consuming means the related iterator advances over the matched sequence.
    ///
    /// **Note:** The iterator is only advanced if the sequence is matched.
    ///
    /// **Note:** The matched sequence is **not** included in the symbols returned by [`SymbolIterator::take_to_end()`].
    fn consumed_matches(&mut self, sequence: &[TokenKind]) -> bool;

    /// Returns `true` if the given [`SymbolKind`] is equal to the kind of the previous symbol returned using `next()` or `consumed_matches()`.
    fn matches_prev(&mut self, kind: TokenKind) -> bool;

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
    fn consumed_prefix(&mut self, sequence: &[TokenKind]) -> bool;

    /// Returns `true` if the upcoming [`Symbol`] sequence is an empty line.
    /// Meaning that a line contains no [`Symbol`] or only [`SymbolKind::Whitespace`].
    ///
    /// **Note:** This is also `true` if a parent iterator stripped non-whitespace symbols, and the nested iterator only has whitespace symbols.
    ///
    /// [`Symbol`]: super::Symbol
    fn only_spaces_until_newline(&mut self) -> bool;
}

fn matches_kind(token: &Token<'_>, kind: &TokenKind) -> bool {
    match kind {
        TokenKind::Any => true,
        // TokenKind::PossibleAttributes => todo!(),
        // TokenKind::PossibleDecorator => todo!(),
        TokenKind::Space => match token.kind {
            TokenKind::Whitespace => String::from(token) == " ",
            _ => false,
        },
        _ => token.kind == *kind,
    }
}

impl<'slice, 'input> EndMatcher for TokenIterator<'slice, 'input, '_, '_> {
    fn is_blank_line(&mut self) -> bool {
        matches!(
            self.peek_kind(),
            Some(TokenKind::Blankline) | Some(TokenKind::Eoi)
        )
    }

    fn consumed_is_blank_line(&mut self) -> bool {
        let is_blank_line = self.is_blank_line();

        if is_blank_line {
            self.set_peek_index(self.match_index()); // To consume matched symbols for `peeking_next()`

            if !self.peek_matching {
                self.set_index(self.match_index()); // To consume matched symbols for `next()`
            }
        }

        is_blank_line
    }

    fn matches(&mut self, sequence: &[TokenKind]) -> bool {
        // Note: Multiple matches may be set in the match closure, so we need to ensure that all start at the same index
        let peek_index = self.peek_index();

        for kind in sequence {
            let next_token_opt = self.peeking_next(|s| matches_kind(s, kind));

            if next_token_opt.is_none() {
                self.set_peek_index(peek_index);
                return false;
            }
        }

        self.set_match_index(self.peek_index());
        self.set_peek_index(peek_index);
        true
    }

    fn consumed_matches(&mut self, sequence: &[TokenKind]) -> bool {
        let matched = self.matches(sequence);

        if matched {
            self.set_peek_index(self.match_index()); // To consume matched tokens for `peeking_next()`

            if !self.peek_matching {
                self.set_index(self.match_index()); // To consume matched tokens for `next()`
            }
        }

        matched
    }

    fn matches_prev(&mut self, kind: TokenKind) -> bool {
        self.prev_kind()
            .map_or(false, |self_kind| self_kind == kind)
    }

    fn prev_is_space(&mut self) -> bool {
        // default `true`, because "no prev" means start of input, which is considered as space.
        self.prev_kind().map_or(true, |k| k.is_space())
    }
}

impl<'slice, 'input> PrefixMatcher for TokenIterator<'slice, 'input, '_, '_> {
    fn consumed_prefix(&mut self, sequence: &[TokenKind]) -> bool {
        debug_assert!(
            !sequence.contains(&TokenKind::Newline),
            "Newline token in prefix match is not allowed."
        );
        debug_assert!(
            !sequence.contains(&TokenKind::Blankline),
            "Blankline token in prefix match is not allowed."
        );
        debug_assert!(
            !sequence.contains(&TokenKind::PossibleAttributes),
            "Attirubtes token in prefix match is not allowed."
        );
        debug_assert!(
            !sequence.contains(&TokenKind::PossibleDecorator),
            "Decorator token in prefix match is not allowed."
        );

        self.consumed_matches(sequence)
    }

    fn only_spaces_until_newline(&mut self) -> bool {
        let peek_index = self.peek_index();

        // NOTE: `Newline` at start is already ensured for prefix matches.
        let _whitespaces = self
            .peeking_take_while(|s| matches!(s.kind, TokenKind::Whitespace))
            .count();

        let new_line = self.peeking_next(|s| matches!(s.kind, TokenKind::Newline | TokenKind::Eoi));

        // NOTE: Not consumed, because nested prefix matchers must see the potentially empty line too
        self.set_peek_index(peek_index);
        new_line.is_some()
    }
}
