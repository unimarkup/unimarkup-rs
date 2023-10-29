use std::collections::HashSet;

use unimarkup_commons::scanner::{
    token::{
        implicit::iterator::TokenIteratorImplicitExt,
        iterator::{IteratorEndFn, TokenIterator},
    },
    PeekingNext,
};

use super::token::{InlineToken, InlineTokenKind};

/// The [`TokenIterator`] provides an iterator over [`Token`]s.
/// It allows to add matcher functions to notify the iterator,
/// when an end of an element is reached.
/// Additionaly, the iterator may be nested to enable transparent iterating for nested elements.
///
/// *Transparent* meaning that the nested iterator does not see [`Token`]s consumed by the wrapped (parent) iterator.
/// In other words, wrapped iterators control which [`Token`]s will be passed to their nested iterator.
/// Therefore, each nested iterator only sees those [`Token`]s that are relevant to its scope.
#[derive(Clone)]
pub struct InlineTokenIterator<'input> {
    /// The [`TokenIterator`] of this iterator.
    token_iter: TokenIterator<'input>,
    cached_token: Option<InlineToken<'input>>,
    prev_token: Option<InlineToken<'input>>,
    peeked_cache: bool,
    open_formats: HashSet<InlineTokenKind>,
}

impl<'input> From<TokenIterator<'input>> for InlineTokenIterator<'input> {
    fn from(value: TokenIterator<'input>) -> Self {
        InlineTokenIterator {
            token_iter: value,
            cached_token: None,
            prev_token: None,
            peeked_cache: false,
            open_formats: HashSet::default(),
        }
    }
}

impl<'input> From<InlineTokenIterator<'input>> for TokenIterator<'input> {
    fn from(value: InlineTokenIterator<'input>) -> Self {
        value.token_iter
    }
}

impl<'input> InlineTokenIterator<'input> {
    /// Creates a new [`TokenIterator`] from the given [`TokenIterator`].
    /// This iterator is created without matching functions.
    pub fn new(sym_iter: TokenIterator<'input>) -> Self {
        InlineTokenIterator::from(sym_iter)
    }

    pub fn max_len(&self) -> usize {
        self.token_iter.max_len()
    }

    /// Returns the index this iterator was started from the [`Symbol`] slice of the root iterator.
    pub fn start_index(&self) -> usize {
        self.token_iter.start_index()
    }

    /// Returns the current index this iterator is in the [`Symbol`] slice of the root iterator.
    pub fn index(&self) -> usize {
        self.token_iter.index()
    }

    /// Resets peek to get `peek() == next()`.
    ///
    /// **Note:** Needed to reset peek index after using `peeking_next()`.
    pub fn reset_peek(&mut self) {
        self.peeked_cache = false;
        self.token_iter.reset_peek();
    }

    /// Returns the next [`Token`] without changing the current index.    
    pub fn peek(&mut self) -> Option<InlineToken<'input>> {
        let peek_index = self.token_iter.peek_index();

        let token = self.peeking_next(|_| true);

        self.token_iter.set_peek_index(peek_index); // Note: Resetting index, because peek() must be idempotent

        token
    }

    /// Returns the [`TokenKind`] of the peeked [`Token`].
    pub fn peek_kind(&mut self) -> Option<InlineTokenKind> {
        self.peek().map(|s| s.kind)
    }

    /// Returns the previous symbol this iterator returned via `next()` or `consumed_matches()`.
    pub fn prev_token(&self) -> Option<&InlineToken<'input>> {
        self.prev_token.as_ref()
    }

    pub fn set_prev_token(&mut self, token: InlineToken<'input>) {
        self.prev_token = Some(token);
    }

    /// Returns the [`TokenKind`] of the previous symbol this iterator returned via `next()` or `consumed_matches()`.
    pub fn prev_kind(&self) -> Option<InlineTokenKind> {
        self.prev_token().map(|s| s.kind)
    }

    pub(crate) fn cache_token(&mut self, token: InlineToken<'input>) {
        self.cached_token = Some(token)
    }

    pub(crate) fn push_format(&mut self, format: InlineTokenKind) {
        self.open_formats.insert(format);
    }

    pub(crate) fn pop_format(&mut self, format: InlineTokenKind) -> bool {
        self.open_formats.remove(&format)
    }

    pub(crate) fn format_closes(&mut self, format: InlineTokenKind) -> bool {
        // previous token is space => close is invalid
        if self.prev_token().map_or(true, |t| t.kind.is_space()) {
            return false;
        }

        self.format_is_open(format)
    }

    pub(crate) fn format_is_open(&self, format: InlineTokenKind) -> bool {
        // check if ambiguous parts are open, because open ambiguous pushes both formats, but not itself
        let ambiguous_open = (format == InlineTokenKind::BoldItalic
            && (self.open_formats.contains(&InlineTokenKind::Italic)
                || self.open_formats.contains(&InlineTokenKind::Bold)))
            || (format == InlineTokenKind::UnderlineSubscript
                && (self.open_formats.contains(&InlineTokenKind::Underline)
                    || self.open_formats.contains(&InlineTokenKind::Subscript)));

        self.open_formats.contains(&format) || ambiguous_open
    }

    /// Nests this iterator, by creating a new iterator that has this iterator set as parent.
    /// Pushes the new iterator to a new scope, and only runs the given matching functions in the new scope.
    ///
    /// **Note:** Any change in this iterator is **not** propagated to the nested iterator.
    /// See [`Self::update()`] on how to synchronize this iterator with the nested one.
    ///
    /// # Arguments
    ///
    /// * `end_match` ... Optional matching function used to indicate the end of the created iterator
    pub fn nest_with_scope(&self, end_match: Option<IteratorEndFn>) -> TokenIterator<'input> {
        self.token_iter.nest_with_scope(None, end_match)
    }

    //TODO: delete function
    /// Merge the parent iterator of the given symbol iterator to take the progress of it.
    pub fn merge(&mut self, token_iter: TokenIterator<'input>) {
        token_iter.update(&mut self.token_iter);
    }

    /// Updates the given parent iterator to take the progress of the nested iterator.
    ///
    /// **Note:** Only updates the parent if `self` is nested.
    pub fn update(self, parent: &mut Self) {
        self.token_iter.update(&mut parent.token_iter);
    }

    /// Tries to skip symbols until one of the end functions signals the end.
    ///
    /// **Note:** This function might not reach the iterator end.
    ///
    /// If no symbols are left, or no given line prefix is matched, the iterator may stop before an end is reached.
    /// Use [`Self::end_reached()`] to check if the end was actually reached.
    pub fn skip_to_end(mut self) -> Self {
        let _last_symbol = self.by_ref().last();

        self
    }

    /// Collects and returns all tokens until one of the end functions signals the end,
    /// or until no line prefix is matched after a new line.
    pub fn take_to_end(&mut self) -> Vec<InlineToken<'input>> {
        let mut tokens = Vec::new();

        for token in self.by_ref() {
            tokens.push(token);
        }

        tokens
    }

    /// Returns `true` if this iterator has reached its end.
    pub fn end_reached(&self) -> bool {
        self.token_iter.end_reached()
    }
}

impl<'input> Iterator for InlineTokenIterator<'input> {
    type Item = InlineToken<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        self.reset_peek();
        self.peeked_cache = false;

        if let Some(token) = self.cached_token {
            self.prev_token = Some(token);
            self.cached_token = None;
            return Some(token);
        }

        if self.end_reached() {
            return None;
        }

        let next = InlineToken::from(&self.token_iter.next()?);

        // Converted token from inner to also get possibly consumed tokens
        self.prev_token = self.token_iter.prev_token().map(InlineToken::from);

        Some(next)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.token_iter.max_len()))
    }
}

impl<'input> PeekingNext for InlineTokenIterator<'input> {
    fn peeking_next<F>(&mut self, _accept: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnOnce(&Self::Item) -> bool,
    {
        if !self.peeked_cache {
            if let Some(token) = self.cached_token {
                self.peeked_cache = true;
                return Some(token);
            }
        }

        if self.end_reached() {
            return None;
        }

        Some(InlineToken::from(&self.token_iter.peeking_next(|_| true)?))
    }
}

impl TokenIteratorImplicitExt for InlineTokenIterator<'_> {
    fn ignore_implicits(&mut self) {
        self.token_iter.ignore_implicits();
    }

    fn allow_implicits(&mut self) {
        self.token_iter.allow_implicits();
    }
}
