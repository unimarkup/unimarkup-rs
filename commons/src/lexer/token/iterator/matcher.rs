//! Contains matcher traits and types used to detect iterator end and strip prefixes.
//! The available matcher traits are implemented for [`TokenIterator`].

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
    /// Returns `true` if the upcoming [`Token`] sequence is a blank line.
    ///
    /// **Note:** This is also `true` if a parent (inner) iterator stripped non-whitespace tokens,
    /// and the nested (outer) iterator only has whitespace tokens.
    fn is_blank_line(&mut self) -> bool;

    /// Wrapper around [`Self::is_blank_line()`] that additionally consumes the matched blank line.
    /// Consuming means the related iterator advances over the matched blank line, but not the end newline.
    /// Not consuming the end newline allows to consume contiguous blank lines.
    ///
    /// **Note:** The iterator is only advanced if a blank line is matched.
    ///
    /// **Note:** The blank line is **not** included in the tokens returned by [`TokenIterator::take_to_end()`].
    fn consumed_is_blank_line(&mut self) -> bool;

    /// Returns `true` if the given [`Token`] sequence matches the upcoming one.
    fn matches(&mut self, sequence: &[TokenKind]) -> bool;

    /// Wrapper around [`Self::matches()`] that additionally consumes the matched sequence.
    /// Consuming means the related iterator advances over the matched sequence.
    ///
    /// **Note:** The iterator is only advanced if the sequence is matched.
    ///
    /// **Note:** The matched sequence is **not** included in the tokens returned by [`TokenIterator::take_to_end()`].
    fn consumed_matches(&mut self, sequence: &[TokenKind]) -> bool;

    /// Returns `true` if the given [`TokenKind`] is equal to the kind of the previous token returned using `next()` or `consumed_matches()`.
    fn matches_prev(&mut self, kind: TokenKind) -> bool;

    /// Returns `true` if the previous token returned using `next()` or `consumed_matches()` is either
    /// whitespace, newline, EOI, or no previous token exists.
    fn prev_is_space(&mut self) -> bool;

    /// Returns `true` if any parent (inner) iterator ended.
    /// Meaning this iterator returns `None` before reaching `EOI`.
    fn outer_end(&mut self) -> bool;
}

/// Trait containing functions that are available inside the prefix matcher function.
pub trait PrefixMatcher {
    /// Consumes and returns `true` if the given [`Token`] sequence matches the upcoming one.
    /// Consuming means the related iterator advances over the matched sequence.
    ///
    /// **Note:** The iterator is only advanced if the sequence is matched.
    ///
    /// **Note:** The given sequence must **not** include any [`TokenKind::Newline`] or [`TokenKind::Blankline`], because matches are only considered per line.
    ///
    /// **Note:** The matched sequence is **not** included in the tokens returned by [`TokenIterator::take_to_end()`].
    fn consumed_prefix(&mut self, sequence: &[TokenKind]) -> bool;

    /// Returns `true` if the upcoming [`Token`] sequence only has whitespace tokens left until a new line is encountered.
    /// Meaning that a line contains no [`Token`] or only [`TokenKind::Whitespace`].
    ///
    /// **Note:** This is also `true` if a parent (inner) iterator stripped non-whitespace tokens, and the nested (outer) iterator only has whitespace tokens.
    fn only_spaces_until_newline(&mut self) -> bool;
}

fn matches_kind(token: &Token<'_>, kind: &TokenKind) -> bool {
    match kind {
        // EnclosedBlockEnd is validated in `matches()`
        TokenKind::Any | TokenKind::EnclosedBlockEnd => true,
        TokenKind::Space => match token.kind {
            TokenKind::Whitespace => String::from(token) == " ",
            _ => false,
        },
        _ => token.kind == *kind,
    }
}

impl<'slice, 'input> EndMatcher for TokenIterator<'slice, 'input> {
    fn is_blank_line(&mut self) -> bool {
        let peek_index = self.peek_index();

        // peeking_next to move peek index forward
        let is_blankline = if matches!(
            self.peeking_next(|_| true).map(|t| t.kind),
            Some(TokenKind::Blankline) | Some(TokenKind::Eoi)
        ) {
            self.set_match_index(self.peek_index());

            true
        } else {
            false
        };

        self.set_peek_index(peek_index);
        is_blankline
    }

    fn consumed_is_blank_line(&mut self) -> bool {
        let is_blank_line = self.is_blank_line();

        if is_blank_line {
            self.set_peek_index(self.match_index()); // To consume matched tokens for `peeking_next()`

            if !self.matching_in_peek {
                self.set_index(self.match_index()); // To consume matched tokens for `next()`
            }
        }

        is_blank_line
    }

    fn matches(&mut self, sequence: &[TokenKind]) -> bool {
        // Note: Multiple matches may be set in the match closure, so we need to ensure that all start at the same index
        let peek_index = self.peek_index();

        for kind in sequence {
            let next_token_opt = self.peeking_next(|s| matches_kind(s, kind));

            let matched = match next_token_opt {
                Some(token) => {
                    kind != &TokenKind::EnclosedBlockEnd
                        || matches!(token.kind, TokenKind::Blankline | TokenKind::Eoi)
                }
                None => kind == &TokenKind::EnclosedBlockEnd || kind == &TokenKind::Any,
            };

            if !matched {
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

            if !self.matching_in_peek {
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

    fn outer_end(&mut self) -> bool {
        self.peek().is_none()
    }
}

impl<'slice, 'input> PrefixMatcher for TokenIterator<'slice, 'input> {
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

        let new_line = self.peeking_next(|s| {
            matches!(
                s.kind,
                TokenKind::Blankline | TokenKind::Newline | TokenKind::Eoi
            )
        });

        // NOTE: Not consumed, because nested prefix matchers must see the potentially empty line too
        self.set_peek_index(peek_index);
        new_line.is_some()
    }
}
