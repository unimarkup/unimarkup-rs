//! Contains the [`TokenIterator`], and all related functionality
//! that is used to step through the [`Symbol`]s retrieved from the [`Scanner`](crate::scanner::Scanner).

use std::borrow::BorrowMut;

use crate::lexer::{new::SymbolIterator, Symbol};

use self::{
    base::TokenIteratorBase, extension::TokenIteratorExt, scope_root::TokenIteratorScopedRoot,
};

use super::{Token, TokenKind};

mod cache;
mod matcher;

pub(crate) mod extension;
pub mod implicit;

pub mod base;
pub mod scope_root;

use helper::PeekingNext;
pub use itertools as helper;
pub use matcher::*;

/// The [`TokenIterator`] provides an iterator over [`Symbol`]s.
/// It allows to add matcher functions to notify the iterator,
/// when an end of an element is reached, or what prefixes to strip on a new line.
/// Additionaly, the iterator may be nested to enable transparent iterating for nested elements.
///
/// *Transparent* meaning that the nested iterator does not see [`Symbol`]s consumed by the wrapped (parent) iterator.
/// In other words, wrapped iterators control which [`Symbol`]s will be passed to their nested iterator.
/// Therefore, each nested iterator only sees those [`Symbol`]s that are relevant to its scope.
#[derive(Clone)]
pub struct TokenIterator<'input> {
    /// The [`TokenIteratorKind`] of this iterator.
    parent: TokenIteratorKind<'input>,
    /// The index inside the [`Symbol`]s of the root iterator.
    start_index: usize,
    /// The match index of the iterator inside the [`Symbol`] slice.
    /// Used to keep track of end and prefix matches to consume the matched sequence length.    
    match_index: usize,
    /// The scope this iterator is in, starting at 0 if parent is the root iterator.
    scope: usize,
    /// Flag set to `true` if this iterator pushed a new scope.
    scoped: bool,
    /// Index used to skip end matchings in case subsequent symbols already passed end matching for previous `peeking_next` calls.
    highest_peek_index: usize,
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
    /// Prevents the iterator from returning symbols once no prefix matched.
    prefix_mismatch: bool,
    /// Flag set to `true` to indicate matching context in [`Self::next()`].
    ///
    /// End/Prefix matching in `next()` uses `peeking_next()` to check wether the given function matches or not.
    /// Without this flag, `peeking_next()` would apply end/prefix matching itself,
    /// leading to invalid symbols being passed to matching functions for `next()`.
    next_matching: bool,
    /// Flag set to `true` to indicate matching context in [`Self::peeking_next()`]
    ///
    /// Used to prevent consumed matching while peeking.
    peek_matching: bool,
    /// The previous symbol this iterator returned with `next()` or `consumed_matches()`.
    /// It is only updated if `next()` returns Some`, or `consumed_matches()` matched.
    ///
    /// Symbols matched with prefix matching are skipped, because `Newline` symbol is passed to all nested iterators.
    prev_token: Option<Token<'input>>,
}

impl std::fmt::Debug for TokenIterator<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenIterator")
            .field("parent", &self.parent)
            .field("start_index", &self.start_index)
            .field("match_index", &self.match_index)
            .field("scope", &self.scope)
            .field("scoped", &self.scoped)
            .field("highest_peek_index", &self.highest_peek_index)
            .field("iter_end", &self.iter_end)
            .field("prefix_mismatch", &self.prefix_mismatch)
            .field("next_matching", &self.next_matching)
            .field("peek_matching", &self.peek_matching)
            .field("prev_token", &self.prev_token)
            .finish()
    }
}

/// The [`TokenIteratorKind`] defines the kind of a [`SymbolIterator`].
#[derive(Debug, Clone)]
pub enum TokenIteratorKind<'input> {
    /// Defines an iterator as being nested.
    /// The contained iterator is the parent iterator.
    Nested(Box<TokenIterator<'input>>),
    /// Iterator that resolves implicit substitutions.
    /// It is the first layer above the conversion from symbols to tokens.
    Root(Box<TokenIteratorBase<'input>>),
    /// Iterator to define a new scope root.
    /// Meaning that the scope for parent iterators remains unchanged.
    ScopedRoot(Box<TokenIteratorScopedRoot<'input>>),
}

impl<'input> TokenIterator<'input> {
    /// Creates a new [`SymbolIterator`] from the given [`Symbol`] slice.
    /// This iterator is created without matching functions.
    pub fn new(sym_iter: SymbolIterator<'input>) -> Self {
        TokenIterator::from(sym_iter)
    }

    /// Creates a new [`SymbolIterator`] from the given [`Symbol`] slice,
    /// and the given matching functions.
    ///
    /// # Arguments
    ///
    /// * `symbols` ... [`Symbol`] slice to iterate over
    /// * `prefix_match` ... Optional matching function used to strip prefix on new lines
    /// * `end_match` ... Optional matching function used to indicate the end of the created iterator
    pub fn with(
        sym_iter: SymbolIterator<'input>,
        prefix_match: Option<IteratorPrefixFn>,
        end_match: Option<IteratorEndFn>,
    ) -> Self {
        TokenIterator {
            parent: TokenIteratorKind::Root(Box::new(TokenIteratorBase::from(sym_iter))),
            scope: 0,
            scoped: false,
            highest_peek_index: 0,
            start_index: 0,
            match_index: 0,
            prefix_match,
            end_match,
            iter_end: false,
            prefix_mismatch: false,
            next_matching: false,
            peek_matching: false,
            prev_token: None,
        }
    }

    pub fn with_scoped_root(token_iter: TokenIterator<'input>) -> Self {
        TokenIterator {
            parent: TokenIteratorKind::ScopedRoot(Box::new(TokenIteratorScopedRoot::from(
                token_iter,
            ))),
            scope: 0,
            scoped: false,
            highest_peek_index: 0,
            start_index: 0,
            match_index: 0,
            prefix_match: None,
            end_match: None,
            iter_end: false,
            prefix_mismatch: false,
            next_matching: false,
            peek_matching: false,
            prev_token: None,
        }
    }

    /// Returns the maximum length of the remaining [`Symbol`]s this iterator might return.
    ///
    /// **Note:** This length does not consider parent iterators, or matching functions.
    /// Therefore, the returned number of [`Symbol`]s might differ, but cannot be larger than this length.
    pub fn max_len(&self) -> usize {
        match &self.parent {
            TokenIteratorKind::Nested(parent) => parent.max_len(),
            TokenIteratorKind::Root(root) => root.max_len(),
            TokenIteratorKind::ScopedRoot(scoped_root) => scoped_root.max_len(),
        }
    }

    /// Returns `true` if no more [`Symbol`]s are available.
    pub fn is_empty(&self) -> bool {
        self.max_len() == 0
    }

    /// Returns the index this iterator was started from the [`Symbol`] slice of the root iterator.
    pub fn start_index(&self) -> usize {
        self.start_index
    }

    /// Returns the current index this iterator is in the [`Symbol`] slice of the root iterator.
    pub(super) fn index(&self) -> usize {
        match &self.parent {
            TokenIteratorKind::Nested(parent) => parent.index(),
            TokenIteratorKind::Root(root) => root.index(),
            TokenIteratorKind::ScopedRoot(scoped_root) => scoped_root.index(),
        }
    }

    /// Sets the current index of this iterator to the given index.
    pub(super) fn set_index(&mut self, index: usize) {
        if index >= self.start_index {
            match self.parent.borrow_mut() {
                TokenIteratorKind::Nested(parent) => parent.set_index(index),
                TokenIteratorKind::Root(root) => root.set_index(index),
                TokenIteratorKind::ScopedRoot(scoped_root) => scoped_root.set_index(index),
            }
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
        if index >= self.index() {
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

    /// Returns the next [`Symbol`] without changing the current index.    
    pub fn peek(&mut self) -> Option<Token<'input>> {
        let peek_index = self.peek_index();

        let token = self.peeking_next(|_| true);

        self.set_peek_index(peek_index); // Note: Resetting index, because peek() must be idempotent

        token
    }

    /// Returns the [`SymbolKind`] of the peeked [`Symbol`].
    pub fn peek_kind(&mut self) -> Option<TokenKind> {
        self.peek().map(|s| s.kind)
    }

    fn push_scope(&mut self, scope: usize) {
        match self.parent.borrow_mut() {
            TokenIteratorKind::Nested(parent) => parent.push_scope(scope),
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

    /// Returns the previous symbol this iterator returned via `next()` or `consumed_matches()`.
    pub fn prev_token(&self) -> Option<&Token<'input>> {
        self.prev_token.as_ref()
    }

    /// Returns the [`SymbolKind`] of the previous symbol this iterator returned via `next()` or `consumed_matches()`.
    pub fn prev_kind(&self) -> Option<TokenKind> {
        self.prev_token.map(|s| s.kind)
    }

    pub(crate) fn prev_peeked(&self) -> Option<&Token<'input>> {
        match &self.parent {
            TokenIteratorKind::Nested(parent) => parent.prev_peeked(),
            TokenIteratorKind::Root(root) => root.prev_peeked(),
            TokenIteratorKind::ScopedRoot(scoped_root) => scoped_root.prev_peeked(),
        }
    }

    /// Nests this iterator, by creating a new iterator that has this iterator set as parent.
    ///
    /// **Note:** Any change in this iterator is **not** propagated to the nested iterator.
    /// See [`Self::update()`] on how to synchronize this iterator with the nested one.
    ///
    /// # Arguments
    ///
    /// * `prefix_match` ... Optional matching function used to strip prefix on new lines
    /// * `end_match` ... Optional matching function used to indicate the end of the created iterator
    pub fn nest(
        &self,
        prefix_match: Option<IteratorPrefixFn>,
        end_match: Option<IteratorEndFn>,
    ) -> TokenIterator<'input> {
        TokenIterator {
            parent: TokenIteratorKind::Nested(Box::new(self.clone())),
            start_index: self.index(),
            match_index: self.match_index,
            scope: self.scope,
            scoped: false,
            highest_peek_index: self.index(),
            prefix_match,
            end_match,
            iter_end: self.iter_end,
            prefix_mismatch: self.prefix_mismatch,
            next_matching: self.next_matching,
            peek_matching: self.peek_matching,
            prev_token: None,
        }
    }

    /// Nests this iterator, by creating a new iterator that has this iterator set as parent.
    /// Pushes the new iterator to a new scope, and only runs the given matching functions in the new scope.
    ///
    /// **Note:** Any change in this iterator is **not** propagated to the nested iterator.
    /// See [`Self::update()`] on how to synchronize this iterator with the nested one.
    ///
    /// # Arguments
    ///
    /// * `prefix_match` ... Optional matching function used to strip prefix on new lines
    /// * `end_match` ... Optional matching function used to indicate the end of the created iterator
    pub fn nest_with_scope(
        &self,
        prefix_match: Option<IteratorPrefixFn>,
        end_match: Option<IteratorEndFn>,
    ) -> TokenIterator<'input> {
        let scope = self.scope + 1;
        let mut parent = self.clone();
        parent.push_scope(scope);

        TokenIterator {
            parent: TokenIteratorKind::Nested(Box::new(parent)),
            start_index: self.index(),
            match_index: self.match_index,
            scope,
            scoped: true,
            highest_peek_index: self.index(),
            prefix_match,
            end_match,
            iter_end: self.iter_end,
            prefix_mismatch: self.prefix_mismatch,
            next_matching: self.next_matching,
            peek_matching: self.peek_matching,
            prev_token: None,
        }
    }

    /// Updates the given parent iterator to take the progress of the nested iterator.
    ///
    /// **Note:** Only updates the parent if `self` is nested.
    pub fn update(self, parent: &mut Self) {
        if let TokenIteratorKind::Nested(mut self_parent) = self.parent {
            // Make sure it actually is the parent.
            // It is not possible to check more precisely, because other indices are expected to be different due to `clone()`.
            debug_assert_eq!(
                self_parent.start_index, parent.start_index,
                "Updated iterator is not the actual parent of this iterator."
            );
            self_parent.push_scope(self_parent.scope);
            // Take the previous token from the nested iterator to get consumed tokens.
            let prev = self.prev_token;

            *parent = *self_parent;

            parent.prev_token = prev;
        }
    }

    /// Tries to skip symbols until one of the end functions signals the end.
    ///
    /// **Note:** This function might not reach the iterator end.
    ///
    /// If no symbols are left, or no given line prefix is matched, the iterator may stop before an end is reached.
    /// Use [`Self::end_reached()`] to check if the end was actually reached.
    pub fn skip_to_end(mut self) -> Self {
        let _last_token = self.by_ref().last();

        self
    }

    pub fn peek_nth(&mut self, n: usize) -> Option<Token<'input>> {
        let mut token = self.peeking_next(|_| true);

        for _ in 0..n {
            token = self.peeking_next(|_| true);
            token?;
        }

        token
    }

    /// Collects and returns all symbols until one of the end functions signals the end,
    /// or until no line prefix is matched after a new line.
    pub fn take_to_end(&mut self) -> Vec<Token<'input>> {
        let mut tokens = Vec::new();

        for symbol in self.by_ref() {
            tokens.push(symbol);
        }

        tokens
    }

    /// Returns `true` if this iterator has reached its end.
    pub fn end_reached(&self) -> bool {
        self.iter_end
    }
}

impl<'input> From<SymbolIterator<'input>> for TokenIterator<'input> {
    fn from(value: SymbolIterator<'input>) -> Self {
        TokenIterator {
            parent: TokenIteratorKind::Root(Box::new(TokenIteratorBase::from(value))),
            start_index: 0,
            match_index: 0,
            scope: 0,
            scoped: false,
            highest_peek_index: 0,
            prefix_match: None,
            end_match: None,
            iter_end: false,
            prefix_mismatch: false,
            next_matching: false,
            peek_matching: false,
            prev_token: None,
        }
    }
}

impl<'input, T> From<T> for TokenIterator<'input>
where
    T: Into<&'input [Symbol<'input>]>,
{
    fn from(value: T) -> Self {
        TokenIterator::from(SymbolIterator::from(value))
    }
}

impl<'input> Iterator for TokenIterator<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.prefix_mismatch || self.end_reached() {
            return None;
        }
        self.reset_peek();

        let in_scope = !self.scoped || self.scope == self.root_scope();
        let allow_end_matching = in_scope && (self.highest_peek_index <= self.index());

        if allow_end_matching {
            if let Some(end_fn) = self.end_match.clone() {
                self.next_matching = true;

                if (end_fn)(self) {
                    self.iter_end = true;
                    self.next_matching = false;
                    return None;
                }
            }
        }

        let curr_token_opt = match &mut self.parent {
            TokenIteratorKind::Nested(parent) => parent.next(),
            TokenIteratorKind::Root(root) => root.next(),
            TokenIteratorKind::ScopedRoot(scoped_root) => scoped_root.next(),
        };

        let kind = curr_token_opt?.kind;
        // Prefix matching after `peeking_next()` to skip prefix symbols, but pass (escaped) `Newline` to nested iterators.
        if in_scope
            && matches!(kind, TokenKind::Newline | TokenKind::EscapedNewline)
            && self.prefix_match.is_some()
        {
            let prefix_match = self
                .prefix_match
                .clone()
                .expect("Prefix match checked above to be some.");
            self.next_matching = true;

            // Note: This mostly indicates a syntax violation, so skipped symbol is ok.
            if !prefix_match(self) {
                self.prefix_mismatch = true;
                self.next_matching = false;
                return None;
            }
        }

        if curr_token_opt.is_some() {
            self.prev_token = curr_token_opt;
        }

        self.next_matching = false;
        curr_token_opt
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.max_len()))
    }
}

impl<'input> PeekingNext for TokenIterator<'input> {
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
        let allow_end_matching = in_scope && (self.highest_peek_index <= self.peek_index());

        if allow_end_matching && !self.next_matching && !self.peek_matching {
            if let Some(end_fn) = self.end_match.clone() {
                let peek_index = self.peek_index();
                self.peek_matching = true;

                let end_matched = (end_fn)(self);

                self.peek_matching = false;
                self.set_peek_index(peek_index);

                if end_matched {
                    return None;
                }

                self.highest_peek_index = self.highest_peek_index.max(self.peek_index());
            }
        }

        let peeked_token_opt = match &mut self.parent {
            TokenIteratorKind::Nested(parent) => parent.peeking_next(accept),
            TokenIteratorKind::Root(root) => root.peeking_next(accept),
            TokenIteratorKind::ScopedRoot(scoped_root) => scoped_root.peeking_next(accept),
        };

        // Prefix matching after `peeking_next()` to skip prefix symbols, but pass `Newline` to nested iterators.
        if in_scope
            && !self.next_matching
            && !self.peek_matching
            && peeked_token_opt?.kind == TokenKind::Newline
            && self.prefix_match.is_some()
        {
            let prefix_match = self
                .prefix_match
                .clone()
                .expect("Prefix match checked above to be some.");
            let peek_index = self.peek_index();
            self.peek_matching = true;

            let prefix_matched = prefix_match(self);

            self.peek_matching = false;

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

    use super::*;

    #[test]
    fn peek_while_index() {
        let symbols = crate::lexer::scan_str("##++ ");

        let mut iterator = TokenIterator::from(&*symbols);
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
            "Hash symbols in input not correctly detected."
        );
        assert_eq!(
            plus_token.kind,
            TokenKind::Plus(2),
            "Plus symbols in input not correctly detected."
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
        let symbols = crate::lexer::scan_str("#*");

        let mut iterator = TokenIterator::from(&*symbols);

        let peeked_symbol = iterator.peeking_next(|_| true);
        let next_symbol = iterator.next();
        let next_peeked_symbol = iterator.peeking_next(|_| true);
        let curr_index = iterator.index();

        assert_eq!(curr_index, 1, "Current index was not updated correctly.");
        assert_eq!(
            peeked_symbol.map(|s| s.kind),
            Some(TokenKind::Hash(1)),
            "peek_next() did not return hash symbol."
        );
        assert_eq!(
            next_symbol.map(|s| s.kind),
            Some(TokenKind::Hash(1)),
            "next() did not return hash symbol."
        );
        assert_eq!(
            next_peeked_symbol.map(|s| s.kind),
            Some(TokenKind::Star(1)),
            "Star symbol not peeked next."
        );
        assert_eq!(
            iterator.next().map(|s| s.kind),
            Some(TokenKind::Star(1)),
            "Star symbol not returned."
        );
    }

    #[test]
    fn reach_end() {
        let symbols = crate::lexer::scan_str("text*");

        let mut iterator = TokenIterator::from(&*symbols).nest(
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
        let symbols = crate::lexer::scan_str("text*");

        let mut iterator = TokenIterator::from(&*symbols).nest(
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
            String::from(iterator.prev_token().unwrap().kind),
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
        let symbols = crate::lexer::scan_str("a\n* *b");

        let iterator = TokenIterator::with(
            (&*symbols).into(),
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
            "Prefix symbols not correctly skipped"
        );
    }

    #[test]
    fn nested_peek() {
        let symbols = crate::lexer::scan_str("a\n* *b");

        let iterator = TokenIterator::with(
            (&*symbols).into(),
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
        let symbols = crate::lexer::scan_str("e+f-");

        let mut iterator = TokenIterator::with(
            (&*symbols).into(),
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

        inner.update(&mut iterator);

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
        let symbols = crate::lexer::scan_str("a\n* *b+-");

        let iterator = TokenIterator::with(
            (&*symbols).into(),
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
        let symbols = crate::lexer::scan_str("[o [i] o]");

        let mut iterator = TokenIterator::with((&*symbols).into(), None, None);

        iterator = iterator.dropping(1); // To skip first open bracket

        // Nest like this, because TokenIterator does not provide a way to initialize with scoped = true
        // which is intentional, because the lowest iterator layer should not be scoped
        iterator = iterator.nest_with_scope(
            None,
            Some(Rc::new(|matcher| {
                matcher.consumed_matches(&[TokenKind::CloseBracket])
            })),
        );

        let mut taken_outer = iterator
            .by_ref()
            // Note: This will skip the open bracket for both iterators, but this is ok for this test
            .take_while(|s| s.kind != TokenKind::OpenBracket)
            .collect::<Vec<_>>();

        let mut inner_iter = iterator.nest_with_scope(
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

        inner_iter.update(&mut iterator);

        taken_outer.extend(iterator.take_to_end().iter());

        assert!(iterator.end_reached(), "Iterator end was not reached.");
        assert_eq!(
            taken_inner.iter().map(String::from).collect::<Vec<_>>(),
            vec!["i"],
            "Inner symbols are incorrect."
        );
        assert_eq!(
            taken_outer.iter().map(String::from).collect::<Vec<_>>(),
            vec!["o", " ", " ", "o"],
            "Outer symbols are incorrect."
        );
    }

    #[test]
    fn prefix_mismatch_returns_none_forever() {
        let symbols = crate::lexer::scan_str("a\n  b\nc");

        let mut iterator = TokenIterator::with(
            (&*symbols).into(),
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
    fn match_any_symbol() {
        let symbols = crate::lexer::scan_str("a* -\n:");

        let mut iterator = TokenIterator::with(
            (&*symbols).into(),
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

        inner.update(&mut iterator);

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

        inner.update(&mut iterator);

        iterator.take_to_end();

        assert!(iterator.end_reached(), "Main iterator did not reach end.");
    }

    #[test]
    fn newline_before_eoi_skipped() {
        let symbols = crate::lexer::scan_str("a\nb\n");

        let mut iterator = TokenIterator::with((&*symbols).into(), None, None);

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
        let symbols = crate::lexer::scan_str("a *\nb");

        let mut iterator = TokenIterator::with((&*symbols).into(), None, None);

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
        let symbols = crate::lexer::scan_str("a*+b");

        let mut iterator = TokenIterator::with(
            (&*symbols).into(),
            None,
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.consumed_matches(&[TokenKind::Star(1), TokenKind::Plus(1)])
            })),
        );

        let content = iterator
            .take_to_end()
            .iter()
            .fold(String::new(), |mut combined, s| {
                combined.push_str(&String::from(s));
                combined
            });

        assert_eq!(content, "a", "End match returned wrong content.");
        assert_eq!(
            iterator.prev_token().unwrap().kind,
            TokenKind::Plus(1),
            "Previous token not correctly updated from end match."
        );
    }

    #[test]
    fn prev_token_from_prefix_match() {
        let symbols = crate::lexer::scan_str("\n*+b");

        let mut iterator = TokenIterator::with(
            (&*symbols).into(),
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
            iterator.prev_token().unwrap().kind,
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
