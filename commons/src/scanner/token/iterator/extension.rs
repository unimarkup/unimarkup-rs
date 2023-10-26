use itertools::PeekingNext;

use crate::scanner::token::Token;

pub(crate) trait TokenIteratorExt<'input>:
    Iterator<Item = Token<'input>> + PeekingNext + Clone
{
    fn prev(&self) -> Option<&Token<'input>>;

    fn max_len(&self) -> usize;

    /// Returns `true` if no more [`Symbol`]s are available.
    fn is_empty(&self) -> bool;

    /// Returns the current index this iterator is in the [`Symbol`] slice of the root iterator.
    fn index(&self) -> usize;

    /// Sets the current index of this iterator to the given index.
    fn set_index(&mut self, index: usize);

    /// Returns the index used to peek.
    fn peek_index(&self) -> usize;

    /// Sets the peek index of this iterator to the given index.
    fn set_peek_index(&mut self, index: usize);

    fn reset_peek(&mut self);

    fn scope(&self) -> usize;

    fn set_scope(&mut self, scope: usize);
}
