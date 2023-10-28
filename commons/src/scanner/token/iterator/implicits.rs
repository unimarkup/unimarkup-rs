//! Contains the [`TokenIteratorRoot`] that is the root iterator in any [`TokenIterator`](super::TokenIterator).

use itertools::PeekingNext;

use crate::scanner::{
    new::SymbolIterator,
    token::{Token, TokenKind},
};

use super::{base::TokenIteratorBase, extension::TokenIteratorExt};

/// The [`TokenIteratorRoot`] is the root iterator in any [`TokenIterator`](super::TokenIterator).
/// It holds the actual [`Symbol`] slice.
#[derive(Clone)]
pub struct TokenIteratorImplicits<'input> {
    /// The [`Symbol`] slice the iterator was created for.
    base_iter: TokenIteratorBase<'input>,
    prev_token: Option<Token<'input>>,
    prev_peeked_token: Option<Token<'input>>,
    allow_implicits: bool,
    /// Flag to mark if arrow substitutions are allowed,
    /// without ensuring that it is followed by another arrow, terminal punctuation, or space.
    ///
    /// Helps to prevent repeated lookaheads.
    allow_arrow: bool,
    /// Flag to mark if arrow substitutions are allowed,
    /// without ensuring that it is followed by another emoji, terminal punctuation, or space.
    ///
    /// Helps to prevent repeated lookaheads.
    allow_emoji: bool,
}

pub trait TokenIteratorImplicitExt {
    fn ignore_implicits(&mut self);
    fn allow_implicits(&mut self);
}

impl<'input> TokenIteratorImplicitExt for TokenIteratorImplicits<'input> {
    fn ignore_implicits(&mut self) {
        self.allow_implicits = false;
    }

    fn allow_implicits(&mut self) {
        self.allow_implicits = true;
    }
}

impl<'input> TokenIteratorExt<'input> for TokenIteratorImplicits<'input> {
    /// Returns the symbol that is directly before the current index.
    /// If no previous symbol exists, `None`` is returned.
    fn prev_token(&self) -> Option<&Token<'input>> {
        self.prev_token.as_ref()
    }

    fn max_len(&self) -> usize {
        self.base_iter.max_len()
    }

    /// Returns `true` if no more [`Symbol`]s are available.
    fn is_empty(&self) -> bool {
        self.max_len() == 0
    }

    /// Returns the current index this iterator is in the [`Symbol`] slice of the root iterator.
    fn index(&self) -> usize {
        self.base_iter.index()
    }

    /// Sets the current index of this iterator to the given index.
    fn set_index(&mut self, index: usize) {
        self.base_iter.set_index(index);
    }

    /// Returns the index used to peek.
    fn peek_index(&self) -> usize {
        self.base_iter.peek_index()
    }

    /// Sets the peek index of this iterator to the given index.
    fn set_peek_index(&mut self, index: usize) {
        self.base_iter.set_peek_index(index);
    }

    fn reset_peek(&mut self) {
        self.set_peek_index(self.index());
    }

    fn scope(&self) -> usize {
        self.base_iter.scope()
    }

    fn set_scope(&mut self, scope: usize) {
        self.base_iter.set_scope(scope);
    }
}

impl<'input> From<SymbolIterator<'input>> for TokenIteratorImplicits<'input> {
    fn from(value: SymbolIterator<'input>) -> Self {
        TokenIteratorImplicits {
            base_iter: TokenIteratorBase::from(value),
            prev_token: None,
            prev_peeked_token: None,
            allow_implicits: true,
            allow_arrow: false,
            allow_emoji: false,
        }
    }
}

impl<'input> Iterator for TokenIteratorImplicits<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        self.base_iter.reset_peek();

        let next = self.peeking_next(|_| true);
        self.base_iter.set_index(self.base_iter.peek_index());

        if next.is_some() {
            self.prev_token = next;
        }

        next
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base_iter.size_hint()
    }
}

impl<'input> PeekingNext for TokenIteratorImplicits<'input> {
    fn peeking_next<F>(&mut self, accept: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnOnce(&Self::Item) -> bool,
    {
        let mut token = self.base_iter.peeking_next(|_| true)?;
        let kind = token.kind;

        if self.allow_implicits {
            if kind == TokenKind::Punctuation || kind.is_space() {
                // reached end of arrow/emoji sequence
                // => next arrow/emoji must ensure it is followed by another arrow/emoji, terminal punctuation, or space.
                self.allow_arrow = false;
                self.allow_emoji = false;
            }

            let mut implicit_iter = self.clone();
            if let Some(implicit_token) = crate::scanner::token::get_implicit(&mut implicit_iter) {
                token = implicit_token;
                *self = implicit_iter;
            }
        } else {
            self.allow_arrow = false;
            self.allow_emoji = false;
        }

        if accept(&token) {
            self.prev_peeked_token = Some(token);
            Some(token)
        } else {
            None
        }
    }
}
