use itertools::PeekingNext;

use crate::lexer::token::{Token, TokenKind};

#[derive(Debug, Default, Clone)]
pub struct TokenSliceIterator<'slice, 'input> {
    /// The [`Symbol`] slice the iterator was created for.
    tokens: &'slice [Token<'input>],
    /// The current index of the iterator inside the [`Symbol`] slice.
    index: usize,
    /// The peek index of the iterator inside the [`Symbol`] slice.
    peek_index: usize,
    scope: usize,
}

impl<'slice, 'input, T> From<T> for TokenSliceIterator<'slice, 'input>
where
    T: Into<&'slice [Token<'input>]>,
{
    fn from(value: T) -> Self {
        TokenSliceIterator {
            tokens: value.into(),
            index: 0,
            peek_index: 0,
            scope: 0,
        }
    }
}

impl<'slice, 'input> Iterator for TokenSliceIterator<'slice, 'input> {
    type Item = &'slice Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.tokens.get(self.index)?;

        self.index += 1;
        self.peek_index = self.index;

        Some(token)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.max_len()))
    }
}

impl<'slice, 'input> PeekingNext for TokenSliceIterator<'slice, 'input> {
    fn peeking_next<F>(&mut self, accept: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnOnce(&Self::Item) -> bool,
    {
        let token = self.tokens.get(self.peek_index).filter(accept)?;
        self.peek_index += 1;
        Some(token)
    }
}

impl<'slice, 'input> TokenSliceIterator<'slice, 'input> {
    /// Returns the maximum length of the remaining [`Symbol`]s this iterator might return.
    ///
    /// **Note:** This length does not consider parent iterators, or matching functions.
    /// Therefore, the returned number of [`Symbol`]s might differ, but cannot be larger than this length.
    pub fn max_len(&self) -> usize {
        self.tokens.len().saturating_sub(self.index)
    }

    /// Returns `true` if no more [`Symbol`]s are available.
    pub fn is_empty(&self) -> bool {
        self.max_len() == 0
    }

    /// Returns the current index this iterator is in the [`Symbol`] slice of the root iterator.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Sets the current index of this iterator to the given index.
    pub(crate) fn set_index(&mut self, index: usize) {
        self.index = index;
        self.peek_index = index;
    }

    /// Returns the index used to peek.
    pub(crate) fn peek_index(&self) -> usize {
        self.peek_index
    }

    /// Sets the peek index of this iterator to the given index.
    pub(crate) fn set_peek_index(&mut self, index: usize) {
        if self.index() <= index {
            self.peek_index = index;
        }
    }

    pub fn reset_peek(&mut self) {
        self.set_peek_index(self.index());
    }

    /// Returns the next [`Symbol`] without changing the current index.    
    pub fn peek(&mut self) -> Option<&'slice Token<'input>> {
        self.tokens.get(self.peek_index)
    }

    /// Returns the [`SymbolKind`] of the peeked [`Symbol`].
    pub fn peek_kind(&mut self) -> Option<TokenKind> {
        self.peek().map(|s| s.kind)
    }

    pub fn scope(&self) -> usize {
        self.scope
    }

    pub fn set_scope(&mut self, scope: usize) {
        self.scope = scope;
    }

    pub fn prev(&self) -> Option<&'slice Token<'input>> {
        if self.index > 0 {
            self.tokens.get(self.index - 1)
        } else {
            None
        }
    }

    pub fn prev_peeked(&self) -> Option<&'slice Token<'input>> {
        if self.peek_index > 0 {
            self.tokens.get(self.peek_index - 1)
        } else {
            None
        }
    }
}
