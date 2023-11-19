//! Contains the [`TokenIterator`] and all related functionality
//! that is used to iterate over a [`Token`] slice.

use std::borrow::BorrowMut;

use self::{
    extension::TokenIteratorExt, scope_root::TokenIteratorScopedRoot, slice::TokenSliceIterator,
};

use super::{Token, TokenKind};

mod matcher;
mod slice;

pub(crate) mod extension;

pub mod base;
pub mod scope_root;

pub use itertools::*;
pub use matcher::*;

/// The [`TokenIterator`] provides an iterator over [`Token`]s.
/// It allows to add matcher functions to notify the iterator,
/// when an end of an element is reached, or what prefixes to strip on a new line.
/// Additionaly, the iterator may be nested to enable transparent iterating for nested elements.
///
/// *Transparent* meaning that the outer iterator (created with `nest()`) does not see [`Token`]s consumed by the inner (parent) iterator.
/// In other words, inner iterators control which [`Token`]s will be passed to the outer iterator.
/// Therefore, outer iterators only sees those [`Token`]s that are relevant to their scope.
#[derive(Clone)]
pub struct TokenIterator<'slice, 'input> {
    /// The [`TokenIteratorKind`] of this iterator.
    parent: TokenIteratorKind<'slice, 'input>,
    /// The index inside the [`Token`] slice a parent iterator was at when creating this iterator.
    /// Will be index 0 for iterators created for a new token slice.
    start_index: usize,
    /// The match index of the iterator inside the [`Token`] slice.
    /// Used to keep track of end and prefix matches to consume the matched sequence length.    
    match_index: usize,
    /// The scope this iterator is in, starting at 0 if parent is the root iterator.
    scope: usize,
    /// Flag set to `true` if this iterator pushed a new scope.
    scoped: bool,
    /// Index used to skip end matchings in case subsequent tokens already passed end matching for previous `peeking_next` calls.
    skip_end_until_idx: usize,
    /// Optional matching function that is used to automatically skip matched prefixes after a new line.
    prefix_match: Option<IteratorPrefixFn>,
    /// Optional matching function that is used to indicate the end of this iterator.
    end_match: Option<IteratorEndFn>,
    /// Flag set to `true` if this iterator reached its end.
    ///
    /// Prevents the iterator from jumping over the end sequence.
    iter_end: bool,
    /// Flag set to `true` if prefix mismatch occured.
    ///
    /// Prevents the iterator from returning tokens once no prefix matched.
    prefix_mismatch: bool,
    /// Flag set to `true` to indicate matching context in [`Self::next()`].
    ///
    /// End/Prefix matching in `next()` uses `peeking_next()` to check wether the given function matches or not.
    /// Without this flag, `peeking_next()` would apply end/prefix matching itself,
    /// leading to invalid tokens being passed to matching functions for `next()`.
    matching_in_next: bool,
    /// Flag set to `true` to indicate matching context in [`Self::peeking_next()`]
    ///
    /// Used to prevent consumed matching while peeking.
    matching_in_peek: bool,
    /// Token that was the start of the last prefix match.
    /// Needed to get correct end positions for elements.
    prefix_start: Option<&'slice Token<'input>>,
}

impl std::fmt::Debug for TokenIterator<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenIterator")
            .field("parent", &self.parent)
            .field("start_index", &self.start_index)
            .field("match_index", &self.match_index)
            .field("scope", &self.scope)
            .field("scoped", &self.scoped)
            .field("highest_peek_index", &self.skip_end_until_idx)
            .field("iter_end", &self.iter_end)
            .field("prefix_mismatch", &self.prefix_mismatch)
            .field("matching_in_next", &self.matching_in_next)
            .field("matching_in_peek", &self.matching_in_peek)
            .finish()
    }
}

/// The [`TokenIteratorKind`] defines the kind of a [`TokenIterator`].
#[derive(Debug, Clone)]
pub enum TokenIteratorKind<'slice, 'input> {
    /// Defines an iterator as being nested.
    /// The contained iterator is the parent iterator.
    Nested(Box<TokenIterator<'slice, 'input>>),
    /// Token iterator over a token slice.
    /// It is the first layer on top of the token slice.
    Root(TokenSliceIterator<'slice, 'input>),
    /// Iterator to define a new scope root.
    /// Meaning that the scope for parent iterators remains unchanged.
    ScopedRoot(TokenIteratorScopedRoot<'slice, 'input>),
}

/// An iterator checkpoint to rollback to this iterator state using `rollback()`.
/// The checkpoint does **not** preserve the peek index.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Checkpoint<'slice, 'input> {
    index: usize,
    start_index: usize,
    skip_end_until_idx: usize,
    prefix_start: Option<&'slice Token<'input>>,
}

impl<'slice, 'input> Checkpoint<'slice, 'input> {
    /// Set the index until end matches should be ignored.
    pub fn skip_end_until(&mut self, index: usize) {
        if self.skip_end_until_idx < index {
            self.skip_end_until_idx = index;
        }
    }
}

impl<'slice, 'input> TokenIterator<'slice, 'input> {
    /// Creates a new [`TokenIterator`] from the given [`Token`] slice,
    /// and the given matching functions.
    ///
    /// # Arguments
    ///
    /// * `tokens` ... [`Token`] slice to iterate over
    /// * `prefix_match` ... Optional matching function used to strip prefix on new lines
    /// * `end_match` ... Optional matching function used to indicate the end of the created iterator
    pub fn with(
        tokens: impl Into<&'slice [Token<'input>]>,
        prefix_match: Option<IteratorPrefixFn>,
        end_match: Option<IteratorEndFn>,
    ) -> Self {
        TokenIterator {
            parent: TokenIteratorKind::Root(TokenSliceIterator::from(tokens)),
            scope: 0,
            scoped: false,
            skip_end_until_idx: 0,
            start_index: 0,
            match_index: 0,
            prefix_match,
            end_match,
            iter_end: false,
            prefix_mismatch: false,
            matching_in_next: false,
            matching_in_peek: false,
            prefix_start: None,
        }
    }

    /// Returns the maximum length of the remaining [`Token`]s this iterator might return.
    ///
    /// **Note:** This length does not consider parent iterators, or matching functions.
    /// Therefore, the returned number of [`Token`]s might differ, but cannot be larger than this length.
    pub fn max_len(&self) -> usize {
        match &self.parent {
            TokenIteratorKind::Nested(parent) => parent.max_len(),
            TokenIteratorKind::Root(root) => root.max_len(),
            TokenIteratorKind::ScopedRoot(scoped_root) => scoped_root.max_len(),
        }
    }

    /// Returns `true` if no more [`Token`]s are available.
    pub fn is_empty(&self) -> bool {
        self.max_len() == 0
    }

    /// Returns the index this iterator was started from the [`Token`] slice of the root iterator.
    pub fn start_index(&self) -> usize {
        self.start_index
    }

    /// Returns the current index this iterator is in the [`Token`] slice of the root iterator.
    pub fn index(&self) -> usize {
        match &self.parent {
            TokenIteratorKind::Nested(parent) => parent.index(),
            TokenIteratorKind::Root(root) => root.index(),
            TokenIteratorKind::ScopedRoot(scoped_root) => scoped_root.index(),
        }
    }

    /// Sets the current index of this iterator to the given index.
    pub fn set_index(&mut self, index: usize) {
        // Because of checkpoint/rollback, it is possible to move index freely around the token slice
        match self.parent.borrow_mut() {
            TokenIteratorKind::Nested(parent) => parent.set_index(index),
            TokenIteratorKind::Root(root) => root.set_index(index),
            TokenIteratorKind::ScopedRoot(scoped_root) => scoped_root.set_index(index),
        }
    }

    /// Returns the index used to peek.
    pub fn peek_index(&self) -> usize {
        match &self.parent {
            TokenIteratorKind::Nested(parent) => parent.peek_index(),
            TokenIteratorKind::Root(root) => root.peek_index(),
            TokenIteratorKind::ScopedRoot(scoped_root) => scoped_root.peek_index(),
        }
    }

    /// Sets the peek index of this iterator to the given index.
    pub fn set_peek_index(&mut self, index: usize) {
        debug_assert!(
            self.index() <= index,
            "Tried to move iterator peek backward."
        );

        // Index may be moved freely by checkpoint/rollback, but peek is still bound to main index
        if index >= self.index() && self.peek_index() != index {
            match self.parent.borrow_mut() {
                TokenIteratorKind::Nested(parent) => parent.set_peek_index(index),
                TokenIteratorKind::Root(root) => root.set_peek_index(index),
                TokenIteratorKind::ScopedRoot(scoped_root) => scoped_root.set_peek_index(index),
            }
        }
    }

    pub(super) fn match_index(&self) -> usize {
        self.match_index
    }

    pub(super) fn set_match_index(&mut self, index: usize) {
        if index >= self.index() {
            self.match_index = index;
        }
    }

    /// Resets peek to get `peek() == next()`.
    ///
    /// **Note:** Needed to reset peek index after using `peeking_next()`.
    pub fn reset_peek(&mut self) {
        self.set_peek_index(self.index());
    }

    /// Returns the next [`Token`] without changing the current index.    
    pub fn peek(&mut self) -> Option<&'slice Token<'input>> {
        let peek_index = self.peek_index();

        let token = self.peeking_next(|_| true);

        self.set_peek_index(peek_index); // Note: Resetting index, because peek() must be idempotent
        token
    }

    /// Returns the [`TokenKind`] of the peeked [`Token`].
    pub fn peek_kind(&mut self) -> Option<TokenKind> {
        self.peek().map(|s| s.kind)
    }

    fn set_curr_scope(&mut self, scope: usize) {
        match self.parent.borrow_mut() {
            TokenIteratorKind::Nested(parent) => parent.set_curr_scope(scope),
            TokenIteratorKind::Root(root) => root.set_scope(scope),
            TokenIteratorKind::ScopedRoot(scoped_root) => scoped_root.set_scope(scope),
        }
    }

    fn root_scope(&self) -> usize {
        match &self.parent {
            TokenIteratorKind::Nested(parent) => parent.root_scope(),
            TokenIteratorKind::Root(root) => root.scope(),
            TokenIteratorKind::ScopedRoot(scoped_root) => scoped_root.scope(),
        }
    }

    /// Returns the previous token this iterator returned via `next()` or `consumed_matches()`.
    pub fn prev(&self) -> Option<&'slice Token<'input>> {
        // Previous token was a newline that caused prefix match to consume tokens.
        if self.prefix_start.is_some() {
            return self.prefix_start;
        }

        match &self.parent {
            TokenIteratorKind::Nested(parent) => parent.prev(),
            TokenIteratorKind::Root(root) => root.prev(),
            TokenIteratorKind::ScopedRoot(scoped_root) => scoped_root.prev(),
        }
    }

    /// Returns the [`TokenKind`] of the previous token this iterator returned via `next()` or `consumed_matches()`.
    pub fn prev_kind(&self) -> Option<TokenKind> {
        self.prev().map(|s| s.kind)
    }

    pub fn skip_to_peek(&mut self) {
        self.set_index(self.peek_index());
    }

    /// Creates a [`Checkpoint`] to `rollback` to at later point if needed.
    pub fn checkpoint(&self) -> Checkpoint<'slice, 'input> {
        Checkpoint {
            index: self.index(),
            start_index: self.start_index,
            skip_end_until_idx: self.skip_end_until_idx,
            prefix_start: self.prefix_start,
        }
    }

    /// Rolls back the iterator to the given checkpoint.
    /// Only works if the checkpoint was created for this iterator.
    ///
    /// Returns `true` if the iterator was rolled back to the checkpoint.
    pub fn rollback(&mut self, checkpoint: Checkpoint<'slice, 'input>) -> bool {
        // Simple check to make sure checkpoint is done on iterator the checkpoint was created from
        // Is not super accurate, but should be sufficient, because most parsers consume at least one token
        // before nesting another iterator. Which leads to a different start index.
        if self.start_index != checkpoint.start_index {
            return false;
        }

        self.set_index(checkpoint.index);
        self.skip_end_until_idx = checkpoint.skip_end_until_idx;
        self.prefix_start = checkpoint.prefix_start;

        // reset iterator flags to ensure moving backwards over the current index works
        self.iter_end = false;
        self.prefix_mismatch = false;

        true
    }

    /// Nests this iterator, by creating a new (outer) iterator that has this iterator set as parent (inner).
    ///
    /// # Arguments
    ///
    /// * `prefix_match` ... Optional matching function used to strip prefix on new lines
    /// * `end_match` ... Optional matching function used to indicate the end of the created iterator
    pub fn nest(
        self,
        prefix_match: Option<IteratorPrefixFn>,
        end_match: Option<IteratorEndFn>,
    ) -> TokenIterator<'slice, 'input> {
        let start_index = self.index();
        let scope = self.scope;
        let iter_end = self.iter_end;
        let prefix_mismatch = self.prefix_mismatch;
        let prefix_start = self.prefix_start;

        TokenIterator {
            parent: TokenIteratorKind::Nested(Box::new(self)),
            start_index,
            match_index: 0,
            scope,
            scoped: false,
            skip_end_until_idx: 0,
            prefix_match,
            end_match,
            iter_end,
            prefix_mismatch,
            matching_in_next: false,
            matching_in_peek: false,
            prefix_start,
        }
    }

    /// Nests this iterator, by creating a new (outer) iterator that has this iterator set as parent (inner).
    /// Pushes the new iterator to a new scope, and only runs the given matching functions in the new scope.
    ///
    /// # Arguments
    ///
    /// * `prefix_match` ... Optional matching function used to strip prefix on new lines
    /// * `end_match` ... Optional matching function used to indicate the end of the created iterator
    pub fn nest_scoped(
        mut self,
        prefix_match: Option<IteratorPrefixFn>,
        end_match: Option<IteratorEndFn>,
    ) -> TokenIterator<'slice, 'input> {
        let start_index = self.index();
        let iter_end = self.iter_end;
        let prefix_mismatch = self.prefix_mismatch;
        let prefix_start = self.prefix_start;

        let scope = self.scope + 1;
        self.set_curr_scope(scope);

        TokenIterator {
            parent: TokenIteratorKind::Nested(Box::new(self)),
            start_index,
            match_index: 0,
            scope,
            scoped: true,
            skip_end_until_idx: 0,
            prefix_match,
            end_match,
            iter_end,
            prefix_mismatch,
            matching_in_next: false,
            matching_in_peek: false,
            prefix_start,
        }
    }

    /// Nests this iterator, by creating a new (outer) iterator that has this iterator set as parent (inner).
    /// Pushes the new iterator to a new scope root, and only runs the given matching functions in the new scope.
    /// Parent (inner) iterators do not get updated to the new scope, so matching functions for scoped parent (inner) iterators are still executed.
    ///
    /// # Arguments
    ///
    /// * `prefix_match` ... Optional matching function used to strip prefix on new lines
    /// * `end_match` ... Optional matching function used to indicate the end of the created iterator
    pub fn new_scope_root(
        self,
        prefix_match: Option<IteratorPrefixFn>,
        end_match: Option<IteratorEndFn>,
    ) -> TokenIterator<'slice, 'input> {
        let start_index = self.index();

        TokenIterator {
            parent: TokenIteratorKind::ScopedRoot(TokenIteratorScopedRoot::from(self)),
            scope: 0,
            scoped: false,
            skip_end_until_idx: 0,
            start_index,
            match_index: 0,
            prefix_match,
            end_match,
            iter_end: false,
            prefix_mismatch: false,
            matching_in_next: false,
            matching_in_peek: false,
            prefix_start: None,
        }
    }

    /// Returns `true` if this iterator has inner iterators.
    pub fn is_nested(&self) -> bool {
        matches!(
            self.parent,
            TokenIteratorKind::Nested(_) | TokenIteratorKind::ScopedRoot(_)
        )
    }

    /// Returns `true` if this iterator created a new scope.
    pub fn is_scoped(&self) -> bool {
        self.scoped
    }

    /// Returns the parent (inner) iterator if this iterator is nested.
    /// If this iterator has no inner iterator, this iterator is returned unchanged.
    pub fn into_inner(self) -> Self {
        match self.parent {
            TokenIteratorKind::Nested(mut parent) => {
                parent.set_curr_scope(parent.scope);
                *parent
            }
            TokenIteratorKind::Root(_) => {
                debug_assert!(false, "Tried to unfold non-nested iterator.");
                self
            }
            TokenIteratorKind::ScopedRoot(scoped_root) => *scoped_root.token_iter,
        }
    }

    /// Tries to skip tokens until one of the end functions signals the end.
    ///
    /// **Note:** This function might not reach the iterator end.
    ///
    /// If no tokens are left, or no given line prefix is matched, the iterator may stop before an end is reached.
    /// Use [`Self::end_reached()`] to check if the end was actually reached.
    pub fn skip_to_end(mut self) -> Self {
        let _last_token = self.by_ref().last();

        self
    }

    pub fn peek_nth(&mut self, n: usize) -> Option<&'slice Token<'input>> {
        std::iter::repeat_with(|| self.peeking_next(|_| true))
            .nth(n)
            .flatten()
    }

    /// Collects and returns all tokens until one of the end functions signals the end,
    /// or until no line prefix is matched after a new line.
    pub fn take_to_end(&mut self) -> Vec<&'slice Token<'input>> {
        self.by_ref().collect()
    }

    /// Returns `true` if this iterator has reached its end.
    pub fn end_reached(&self) -> bool {
        self.iter_end
    }

    /// Returns `true` if no prefix match matched the last prefix sequence.
    pub fn prefix_mismatch(&self) -> bool {
        self.prefix_mismatch
    }
}

impl<'slice, 'input, T> From<T> for TokenIterator<'slice, 'input>
where
    T: Into<&'slice [Token<'input>]>,
{
    fn from(value: T) -> Self {
        TokenIterator {
            parent: TokenIteratorKind::Root(TokenSliceIterator::from(value)),
            start_index: 0,
            match_index: 0,
            scope: 0,
            scoped: false,
            skip_end_until_idx: 0,
            prefix_match: None,
            end_match: None,
            iter_end: false,
            prefix_mismatch: false,
            matching_in_next: false,
            matching_in_peek: false,
            prefix_start: None,
        }
    }
}

impl<'slice, 'input> Iterator for TokenIterator<'slice, 'input> {
    type Item = &'slice Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        self.prefix_start = None;

        if self.prefix_mismatch || self.end_reached() {
            return None;
        }
        self.reset_peek();

        let in_scope = !self.scoped || self.scope == self.root_scope();
        let allow_end_matching = in_scope && (self.skip_end_until_idx <= self.index());

        if allow_end_matching {
            if let Some(end_fn) = self.end_match.clone() {
                self.matching_in_next = true;

                if (end_fn)(self) {
                    self.iter_end = true;
                    self.matching_in_next = false;
                    return None;
                }
            }
        }

        let curr_token_opt = match &mut self.parent {
            TokenIteratorKind::Nested(parent) => parent.next(),
            TokenIteratorKind::Root(root) => root.next(),
            TokenIteratorKind::ScopedRoot(scoped_root) => scoped_root.next(),
        };

        // Prefix matching after `next()` to skip prefix tokens, but pass (escaped) `Newline` to nested iterators.
        if matches!(
            curr_token_opt.map(|t| t.kind),
            Some(TokenKind::Newline) | Some(TokenKind::EscapedNewline | TokenKind::Blankline)
        ) {
            // To store the newline token at the start of a possible prefix match.
            // Needed for `prev()`, because prefix match implicitly consumes tokens,
            // but these tokens should not be visible to child iterators.
            // Every newline is stored, because parent iterators may have prefix matches.
            self.prefix_start = curr_token_opt;

            if in_scope && self.prefix_match.is_some() {
                let prefix_match = self
                    .prefix_match
                    .clone()
                    .expect("Prefix match checked above to be some.");
                self.matching_in_next = true;

                // Note: This mostly indicates a syntax violation, so skipped token is ok.
                if !prefix_match(self) {
                    self.prefix_mismatch = true;
                    self.matching_in_next = false;
                    return None;
                }
            }
        }

        self.matching_in_next = false;
        curr_token_opt
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.max_len()))
    }
}

impl<'slice, 'input> PeekingNext for TokenIterator<'slice, 'input> {
    fn peeking_next<F>(&mut self, accept: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnOnce(&Self::Item) -> bool,
    {
        if self.prefix_mismatch || self.end_reached() {
            return None;
        }

        // Note: Only end matching can be optimized, because once an end is reached, subsequent calls return None,
        // which might not be the case for prefix matching.
        let in_scope = !self.scoped || self.scope == self.root_scope();
        let allow_end_matching = in_scope && (self.skip_end_until_idx <= self.peek_index());

        if allow_end_matching && !self.matching_in_next && !self.matching_in_peek {
            if let Some(end_fn) = self.end_match.clone() {
                let peek_index = self.peek_index();
                self.matching_in_peek = true;

                let end_matched = (end_fn)(self);

                self.matching_in_peek = false;
                self.set_peek_index(peek_index);

                if end_matched {
                    return None;
                }

                self.skip_end_until_idx = self.skip_end_until_idx.max(self.peek_index());
            }
        }

        let peeked_token_opt = match &mut self.parent {
            TokenIteratorKind::Nested(parent) => parent.peeking_next(accept),
            TokenIteratorKind::Root(root) => root.peeking_next(accept),
            TokenIteratorKind::ScopedRoot(scoped_root) => scoped_root.peeking_next(accept),
        };

        // Prefix matching after `peeking_next()` to skip prefix tokens, but pass `Newline` to nested iterators.
        if in_scope
            && !self.matching_in_next
            && !self.matching_in_peek
            && matches!(
                peeked_token_opt?.kind,
                TokenKind::Newline | TokenKind::EscapedNewline | TokenKind::Blankline
            )
            && self.prefix_match.is_some()
        {
            let prefix_match = self
                .prefix_match
                .clone()
                .expect("Prefix match checked above to be some.");
            let peek_index = self.peek_index();
            self.matching_in_peek = true;

            let prefix_matched = prefix_match(self);

            self.matching_in_peek = false;

            if !prefix_matched {
                self.set_peek_index(peek_index);
                return None;
            }
        }

        peeked_token_opt
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use itertools::Itertools;

    use crate::lexer::token::lex_str;

    use super::*;

    #[test]
    fn peek_while_index() {
        let tokens = lex_str("##++ ");

        let mut iterator = TokenIterator::from(&*tokens);
        let token_cnt = iterator
            .peeking_take_while(|token| token.kind.is_keyword())
            .count();

        let hash_token = iterator.next().unwrap();
        let plus_token = iterator.next().unwrap();

        let whitespace_token = iterator.next().unwrap();
        let curr_index = iterator.index();

        assert_eq!(
            token_cnt, 2,
            "The two keyword tokens were not correctly detected."
        );
        assert_eq!(
            hash_token.kind,
            TokenKind::Hash(2),
            "Hash tokens in input not correctly detected."
        );
        assert_eq!(
            plus_token.kind,
            TokenKind::Plus(2),
            "Plus tokens in input not correctly detected."
        );
        assert_eq!(curr_index, 3, "Current index was not updated correctly.");
        assert_eq!(
            whitespace_token.kind,
            TokenKind::Whitespace,
            "Whitespace after keywords was not detected."
        );
        assert!(
            iterator.next().unwrap().kind == TokenKind::Eoi,
            "Input end reached, but EOI was not returned."
        );
    }

    #[test]
    fn peek_next() {
        let tokens = lex_str("#*");

        let mut iterator = TokenIterator::from(&*tokens);

        let peeked_token = iterator.peeking_next(|_| true);
        let next_token = iterator.next();
        let next_peeked_token = iterator.peeking_next(|_| true);
        let curr_index = iterator.index();

        assert_eq!(curr_index, 1, "Current index was not updated correctly.");
        assert_eq!(
            peeked_token.map(|s| s.kind),
            Some(TokenKind::Hash(1)),
            "peek_next() did not return hash token."
        );
        assert_eq!(
            next_token.map(|s| s.kind),
            Some(TokenKind::Hash(1)),
            "next() did not return hash token."
        );
        assert_eq!(
            next_peeked_token.map(|s| s.kind),
            Some(TokenKind::Star(1)),
            "Star token not peeked next."
        );
        assert_eq!(
            iterator.next().map(|s| s.kind),
            Some(TokenKind::Star(1)),
            "Star token not returned."
        );
    }

    #[test]
    fn reach_end() {
        let tokens = lex_str("text*");

        let base_iter = TokenIterator::from(&*tokens);
        let mut iterator = base_iter.nest(
            None,
            Some(Rc::new(|matcher| matcher.matches(&[TokenKind::Star(1)]))),
        );

        let taken_kinds = iterator
            .take_to_end()
            .iter()
            .map(|s| s.kind)
            .collect::<Vec<_>>();

        assert!(iterator.end_reached(), "Iterator end was not reached.");
        assert_eq!(
            taken_kinds,
            vec![TokenKind::Plain],
            "Tokens till end was reached are incorrect."
        );
    }

    #[test]
    fn reach_consumed_end() {
        let tokens = lex_str("text*");

        let base_iter = TokenIterator::from(&*tokens);
        let mut iterator = base_iter.nest(
            None,
            Some(Rc::new(|matcher| {
                matcher.consumed_matches(&[TokenKind::Star(1)])
            })),
        );

        let taken_symkinds = iterator
            .take_to_end()
            .iter()
            .map(|s| s.kind)
            .collect::<Vec<_>>();

        assert!(iterator.end_reached(), "Iterator end was not reached.");
        assert!(
            iterator.next().is_none(),
            "Iterator returns token after end."
        );
        assert_eq!(
            String::from(iterator.prev().unwrap().kind),
            "*",
            "Previous token was not the matched one."
        );
        assert_eq!(
            taken_symkinds,
            vec![TokenKind::Plain],
            "Tokens till end was reached are incorrect."
        );
    }

    #[test]
    fn with_nested_and_parent_prefix() {
        let tokens = lex_str("a\n* *b");

        let iterator = TokenIterator::with(
            &*tokens,
            Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
                matcher.consumed_prefix(&[TokenKind::Star(1), TokenKind::Whitespace])
            })),
            None,
        );

        let mut inner = iterator.nest(
            Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
                matcher.consumed_prefix(&[TokenKind::Star(1)])
            })),
            None,
        );

        let sym_kinds = inner
            .take_to_end()
            .iter()
            .map(|s| s.kind)
            .collect::<Vec<_>>();

        assert_eq!(
            sym_kinds,
            vec![
                TokenKind::Plain,   // a
                TokenKind::Newline, // \\n
                TokenKind::Plain,   // b
                TokenKind::Eoi
            ],
            "Prefix tokens not correctly skipped"
        );
    }

    #[test]
    fn nested_peek() {
        let tokens = lex_str("a\n* *b");

        let iterator = TokenIterator::with(
            &*tokens,
            Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
                matcher.consumed_prefix(&[TokenKind::Star(1), TokenKind::Whitespace])
            })),
            None,
        );

        let mut inner = iterator.nest(
            Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
                matcher.consumed_prefix(&[TokenKind::Star(1)])
            })),
            None,
        );

        let token_1 = inner.peeking_next(|_| true);
        assert_eq!(
            "a",
            String::from(token_1.unwrap()),
            "Peeking next token did not return 'a'."
        );
        let token_2 = inner.peeking_next(|_| true);
        assert_eq!(
            "\n",
            String::from(token_2.unwrap()),
            "Peeking next token did not return newline."
        );
        let token_3 = inner.peeking_next(|_| true);
        assert_eq!(
            "b",
            String::from(token_3.unwrap()),
            "Peeking next token did not return 'b'."
        );
    }

    #[test]
    fn outer_end_match_takes_precedence() {
        let tokens = lex_str("e+f-");

        let mut iterator = TokenIterator::with(
            &*tokens,
            None,
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.matches(&[TokenKind::Plus(1)])
            })),
        );

        let mut inner = iterator.nest(
            None,
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.matches(&[TokenKind::Minus(1)])
            })),
        );

        assert_eq!(
            "e",
            String::from(inner.peeking_next(|_| true).unwrap()),
            "First peeked token is not 'e'."
        );
        assert!(
            inner.peeking_next(|_| true).is_none(),
            "Outer end did not take precedence with `peeking_next()`."
        );
        assert!(
            !inner.end_reached(),
            "Peeking end wrongfully set 'end_reached()'."
        );
        assert!(
            inner.peeking_next(|_| true).is_none(),
            "Successive peek over outer end returned token."
        );

        inner.reset_peek();

        assert_eq!(
            "e",
            String::from(inner.next().unwrap()),
            "First token is not 'e'."
        );
        assert!(
            inner.next().is_none(),
            "Outer end did not take precedence with `next()`."
        );
        assert!(
            !inner.end_reached(),
            "Reaching end set for inner, eventhough only outer reached end."
        );
        assert!(
            inner.next().is_none(),
            "Successive `next()` over outer end returned token."
        );

        // inner.update(&mut iterator);
        iterator = inner.into_inner();

        assert!(
            iterator.end_reached(),
            "`end_reached()` not set for outer iterator."
        );
        assert!(
            iterator.next().is_none(),
            "Successive `next()` over outer end returned token."
        );
    }

    #[test]
    fn peek_and_next_return_same_tokens() {
        let tokens = lex_str("a\n* *b+-");

        let iterator = TokenIterator::with(
            &*tokens,
            Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
                matcher.consumed_prefix(&[TokenKind::Star(1), TokenKind::Whitespace])
            })),
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.matches(&[TokenKind::Plus(1)])
            })),
        );

        let mut inner = iterator.nest(
            Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
                matcher.consumed_prefix(&[TokenKind::Star(1)])
            })),
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.matches(&[TokenKind::Minus(1)])
            })),
        );

        let peeked_tokens = inner.peeking_take_while(|_| true).collect::<Vec<_>>();
        inner.reset_peek();
        let next_tokens = inner.take_to_end();

        assert_eq!(
            peeked_tokens, next_tokens,
            "Peeked (left) and next (right) token differ."
        );
    }

    #[test]
    fn scoping() {
        let tokens = lex_str("[o [i] o]");

        let mut iterator = TokenIterator::with(&*tokens, None, None);

        iterator = iterator.dropping(1); // To skip first open bracket

        // Nest like this, because TokenIterator does not provide a way to initialize with scoped = true
        // which is intentional, because the lowest iterator layer should not be scoped
        let mut scoped_iterator = iterator.nest_scoped(
            None,
            Some(Rc::new(|matcher| {
                matcher.consumed_matches(&[TokenKind::CloseBracket])
            })),
        );

        let mut taken_outer = scoped_iterator
            .by_ref()
            // Note: This will skip the open bracket for both iterators, but this is ok for this test
            .take_while(|s| s.kind != TokenKind::OpenBracket)
            .collect::<Vec<_>>();

        let mut inner_iter = scoped_iterator.nest_scoped(
            None,
            Some(Rc::new(|matcher| {
                matcher.consumed_matches(&[TokenKind::CloseBracket])
            })),
        );

        let taken_inner = inner_iter.take_to_end();
        assert!(
            inner_iter.end_reached(),
            "Inner iterator end was not reached."
        );

        scoped_iterator = inner_iter.into_inner();

        let mut end = scoped_iterator.take_to_end();
        taken_outer.append(&mut end);

        assert!(
            scoped_iterator.end_reached(),
            "Iterator end was not reached."
        );
        assert_eq!(
            taken_inner
                .iter()
                .copied()
                .map(String::from)
                .collect::<Vec<_>>(),
            vec!["i"],
            "Inner tokens are incorrect."
        );
        assert_eq!(
            taken_outer
                .iter()
                .copied()
                .map(String::from)
                .collect::<Vec<_>>(),
            vec!["o", " ", " ", "o"],
            "Outer tokens are incorrect."
        );
    }

    #[test]
    fn prefix_mismatch_returns_none_forever() {
        let tokens = lex_str("a\n  b\nc");

        let mut iterator = TokenIterator::with(
            &*tokens,
            Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
                matcher.consumed_prefix(&[TokenKind::Space, TokenKind::Space])
            })),
            None,
        );

        let sym_kinds = iterator
            .take_to_end()
            .iter()
            .map(|s| s.kind)
            .collect::<Vec<_>>();

        assert_eq!(
            sym_kinds,
            vec![TokenKind::Plain, TokenKind::Newline, TokenKind::Plain,],
            "Iterator did not stop on prefix mismatch"
        );
        assert!(
            iterator.next().is_none(),
            "Prefix mismatch not returning `None`."
        );
        assert!(
            iterator.next().is_none(),
            "Prefix mismatch not returning `None`."
        );
    }

    #[test]
    fn match_any_token() {
        let tokens = lex_str("a* -\n:");

        let mut iterator = TokenIterator::with(
            &*tokens,
            None,
            // Matches "\n:"
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.consumed_matches(&[TokenKind::Any, TokenKind::Colon(1)])
            })),
        );

        // Matches "a*"
        let mut inner = iterator.nest(
            None,
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.consumed_matches(&[TokenKind::Any, TokenKind::Star(1)])
            })),
        );
        inner.take_to_end();
        assert!(
            inner.end_reached(),
            "First inner iterator did not reach end."
        );

        iterator = inner.into_inner();

        // Matches " -"
        let mut inner = iterator.nest(
            None,
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.consumed_matches(&[TokenKind::Any, TokenKind::Minus(1)])
            })),
        );
        inner.take_to_end();
        assert!(
            inner.end_reached(),
            "Second inner iterator did not reach end."
        );

        iterator = inner.into_inner();
        iterator.take_to_end();

        assert!(iterator.end_reached(), "Main iterator did not reach end.");
    }

    #[test]
    fn newline_before_eoi_skipped() {
        let tokens = lex_str("a\nb\n");

        let mut iterator = TokenIterator::with(&*tokens, None, None);

        assert_eq!(
            String::from(iterator.next().unwrap()),
            "a",
            "`next()` returned wrong token."
        );
        assert_eq!(
            iterator.prev_kind().unwrap(),
            TokenKind::Plain,
            "Previous TokenKind not correctly stored."
        );
        assert_eq!(
            String::from(iterator.next().unwrap()),
            "\n",
            "`next()` returned wrong token."
        );
        assert_eq!(
            iterator.prev_kind().unwrap(),
            TokenKind::Newline,
            "Previous TokenKind not correctly stored."
        );
        assert_eq!(
            String::from(iterator.next().unwrap()),
            "b",
            "`next()` returned wrong token."
        );
        assert_eq!(
            iterator.prev_kind().unwrap(),
            TokenKind::Plain,
            "Previous TokenKind not correctly stored."
        );

        assert_eq!(
            iterator.next().unwrap().kind,
            TokenKind::Eoi,
            "`next()` did not skip the Newline before EOI."
        );
    }

    #[test]
    fn prev_kind() {
        let tokens = lex_str("a *\nb");

        let mut iterator = TokenIterator::with(&*tokens, None, None);

        assert_eq!(
            String::from(iterator.next().unwrap()),
            "a",
            "`next()` returned wrong token."
        );
        assert_eq!(
            iterator.prev_kind().unwrap(),
            TokenKind::Plain,
            "Previous TokenKind not correctly stored."
        );

        assert_eq!(
            String::from(iterator.next().unwrap()),
            " ",
            "`next()` returned wrong token."
        );
        assert_eq!(
            iterator.prev_kind().unwrap(),
            TokenKind::Whitespace,
            "Previous TokenKind not correctly stored."
        );

        assert_eq!(
            String::from(iterator.next().unwrap()),
            "*",
            "`next()` returned wrong token."
        );
        assert_eq!(
            iterator.prev_kind().unwrap(),
            TokenKind::Star(1),
            "Previous TokenKind not correctly stored."
        );

        assert_eq!(
            String::from(iterator.next().unwrap()),
            "\n",
            "`next()` returned wrong token."
        );
        assert_eq!(
            iterator.prev_kind().unwrap(),
            TokenKind::Newline,
            "Previous TokenKind not correctly stored."
        );
    }

    #[test]
    fn prev_token_from_end_match() {
        let tokens = lex_str("a*+b");

        let mut iterator = TokenIterator::with(
            &*tokens,
            None,
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.consumed_matches(&[TokenKind::Star(1), TokenKind::Plus(1)])
            })),
        );

        let content = iterator
            .take_to_end()
            .iter()
            .fold(String::new(), |mut combined, s| {
                combined.push_str(&String::from(*s));
                combined
            });

        assert_eq!(content, "a", "End match returned wrong content.");
        assert_eq!(
            iterator.prev().unwrap().kind,
            TokenKind::Plus(1),
            "Previous token not correctly updated from end match."
        );
    }

    #[test]
    fn prev_token_from_prefix_match() {
        let tokens = lex_str("\n*+b");

        let mut iterator = TokenIterator::with(
            &*tokens,
            Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
                matcher.consumed_prefix(&[TokenKind::Star(1), TokenKind::Plus(1)])
            })),
            None,
        );

        assert_eq!(
            iterator.next().unwrap().kind,
            TokenKind::Newline,
            "`next()` returned wrong token."
        );
        // Previous token is not set for prefix token, because `Newline` token gets passed to nested iterators for their prefix match
        assert_eq!(
            iterator.prev().unwrap().kind,
            TokenKind::Newline,
            "Previous token not correctly updated from prefix match."
        );
        assert_eq!(
            String::from(iterator.next().unwrap()),
            "b",
            "`next()` returned wrong token."
        );
    }
}
