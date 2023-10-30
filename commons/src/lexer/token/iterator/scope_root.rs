use itertools::PeekingNext;

use crate::lexer::token::{implicit::iterator::TokenIteratorImplicitExt, Token};

use super::{extension::TokenIteratorExt, TokenIterator};

#[derive(Clone)]
pub struct TokenIteratorScopedRoot<'input> {
    /// The [`Symbol`] slice the iterator was created for.
    token_iter: TokenIterator<'input>,
    scope: usize,
}

impl<'input> TokenIteratorScopedRoot<'input> {
    pub(crate) fn prev_peeked(&self) -> Option<&Token<'input>> {
        self.token_iter.prev_peeked()
    }
}

impl TokenIteratorImplicitExt for TokenIteratorScopedRoot<'_> {
    fn ignore_implicits(&mut self) {
        self.token_iter.ignore_implicits();
    }

    fn allow_implicits(&mut self) {
        self.token_iter.allow_implicits();
    }

    fn implicits_allowed(&self) -> bool {
        self.token_iter.implicits_allowed()
    }
}

impl<'input> TokenIteratorExt<'input> for TokenIteratorScopedRoot<'input> {
    /// Returns the symbol that is directly before the current index.
    /// If no previous symbol exists, `None`` is returned.
    fn prev_token(&self) -> Option<&Token<'input>> {
        self.token_iter.prev_token()
    }

    fn max_len(&self) -> usize {
        self.token_iter.max_len()
    }

    /// Returns `true` if no more [`Symbol`]s are available.
    fn is_empty(&self) -> bool {
        self.max_len() == 0
    }

    /// Returns the current index this iterator is in the [`Symbol`] slice of the root iterator.
    fn index(&self) -> usize {
        self.token_iter.index()
    }

    /// Sets the current index of this iterator to the given index.
    fn set_index(&mut self, index: usize) {
        self.token_iter.set_index(index);
    }

    /// Returns the index used to peek.
    fn peek_index(&self) -> usize {
        self.token_iter.peek_index()
    }

    /// Sets the peek index of this iterator to the given index.
    fn set_peek_index(&mut self, index: usize) {
        self.token_iter.set_peek_index(index);
    }

    fn reset_peek(&mut self) {
        self.set_peek_index(self.index());
    }

    fn scope(&self) -> usize {
        self.scope
    }

    fn set_scope(&mut self, scope: usize) {
        self.scope = scope;
    }
}

impl<'input> From<TokenIterator<'input>> for TokenIteratorScopedRoot<'input> {
    fn from(value: TokenIterator<'input>) -> Self {
        TokenIteratorScopedRoot {
            token_iter: value,
            scope: 0,
        }
    }
}

impl<'input> Iterator for TokenIteratorScopedRoot<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.token_iter.size_hint()
    }
}

impl<'input> PeekingNext for TokenIteratorScopedRoot<'input> {
    fn peeking_next<F>(&mut self, accept: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnOnce(&Self::Item) -> bool,
    {
        self.token_iter.peeking_next(accept)
    }
}
