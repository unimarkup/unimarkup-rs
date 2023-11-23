use itertools::PeekingNext;

use crate::lexer::token::Token;

use super::{extension::TokenIteratorExt, TokenIterator};

#[derive(Debug, Clone)]
pub struct TokenIteratorScopedRoot<'slice, 'input> {
    /// The underlying [`TokenIterator`].
    pub(super) token_iter: Box<TokenIterator<'slice, 'input>>,
    scope: usize,
}

impl<'slice, 'input> TokenIteratorExt<'slice, 'input, &'slice Token<'input>>
    for TokenIteratorScopedRoot<'slice, 'input>
{
    /// Returns the token that is directly before the current index.
    /// If no previous token exists, `None`` is returned.
    fn prev(&self) -> Option<&'slice Token<'input>> {
        self.token_iter.prev()
    }

    fn max_len(&self) -> usize {
        self.token_iter.max_len()
    }

    /// Returns `true` if no more [`Token`]s are available.
    fn is_empty(&self) -> bool {
        self.max_len() == 0
    }

    /// Returns the current index this iterator is in the [`Token`] slice of the root iterator.
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

impl<'slice, 'input> From<TokenIterator<'slice, 'input>>
    for TokenIteratorScopedRoot<'slice, 'input>
{
    fn from(value: TokenIterator<'slice, 'input>) -> Self {
        TokenIteratorScopedRoot {
            token_iter: Box::new(value),
            scope: 0,
        }
    }
}

impl<'slice, 'input> Iterator for TokenIteratorScopedRoot<'slice, 'input> {
    type Item = &'slice Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.token_iter.size_hint()
    }
}

impl<'slice, 'input> PeekingNext for TokenIteratorScopedRoot<'slice, 'input> {
    fn peeking_next<F>(&mut self, accept: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnOnce(&Self::Item) -> bool,
    {
        self.token_iter.peeking_next(accept)
    }
}
