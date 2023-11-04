use unimarkup_commons::lexer::{
    token::iterator::{IteratorEndFn, TokenIterator},
    PeekingNext,
};

use crate::element::formatting::{
    ambiguous::is_ambiguous, map_index, OpenFormatMap, NR_OF_UNSCOPED_FORMATS,
};

use super::{kind::InlineTokenKind, InlineToken};

/// The [`TokenIterator`] provides an iterator over [`Token`]s.
/// It allows to add matcher functions to notify the iterator,
/// when an end of an element is reached.
/// Additionaly, the iterator may be nested to enable transparent iterating for nested elements.
///
/// *Transparent* meaning that the nested iterator does not see [`Token`]s consumed by the wrapped (parent) iterator.
/// In other words, wrapped iterators control which [`Token`]s will be passed to their nested iterator.
/// Therefore, each nested iterator only sees those [`Token`]s that are relevant to its scope.
pub(crate) struct InlineTokenIterator<'slice, 'input> {
    /// The [`TokenIterator`] of this iterator.
    token_iter: TokenIterator<'slice, 'input>,
    cached_token: Option<InlineToken<'input>>,
    updated_prev: Option<InlineToken<'input>>,
    peeked_cache: bool,
    open_formats: OpenFormatMap,
}

impl<'slice, 'input> From<TokenIterator<'slice, 'input>> for InlineTokenIterator<'slice, 'input> {
    fn from(value: TokenIterator<'slice, 'input>) -> Self {
        InlineTokenIterator {
            token_iter: value,
            cached_token: None,
            updated_prev: None,
            peeked_cache: false,
            open_formats: [false; NR_OF_UNSCOPED_FORMATS],
        }
    }
}

impl<'slice, 'input> From<InlineTokenIterator<'slice, 'input>> for TokenIterator<'slice, 'input> {
    fn from(value: InlineTokenIterator<'slice, 'input>) -> Self {
        value.token_iter
    }
}

impl<'slice, 'input> InlineTokenIterator<'slice, 'input> {
    pub fn max_len(&self) -> usize {
        self.token_iter.max_len()
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
        let peeked_cache = self.peeked_cache;

        let token = self.peeking_next(|_| true);

        self.token_iter.set_peek_index(peek_index); // Note: Resetting index, because peek() must be idempotent
        self.peeked_cache = peeked_cache;

        token
    }

    /// Returns the [`TokenKind`] of the peeked [`Token`].
    pub fn peek_kind(&mut self) -> Option<InlineTokenKind> {
        self.peek().map(|s| s.kind)
    }

    /// Returns the previous symbol this iterator returned via `next()` or `consumed_matches()`.
    pub fn prev_token(&self) -> Option<InlineToken<'input>> {
        match self.updated_prev {
            Some(updated) => Some(updated),
            None => self.token_iter.prev().map(InlineToken::from),
        }
    }

    pub(crate) fn set_prev_token(&mut self, token: InlineToken<'input>) {
        self.updated_prev = Some(token);
    }

    /// Returns the [`TokenKind`] of the previous symbol this iterator returned via `next()` or `consumed_matches()`.
    pub fn prev_kind(&self) -> Option<InlineTokenKind> {
        self.prev_token().map(|s| s.kind)
    }

    pub(crate) fn cache_token(&mut self, token: InlineToken<'input>) {
        self.cached_token = Some(token)
    }

    pub(crate) fn open_format(&mut self, format: &InlineTokenKind) {
        self.open_formats[map_index(format)] = true;
    }

    pub(crate) fn close_format(&mut self, format: &InlineTokenKind) {
        self.open_formats[map_index(format)] = false;
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
            && (self.open_formats[map_index(&InlineTokenKind::Italic)]
                || self.open_formats[map_index(&InlineTokenKind::Bold)]))
            || (format == InlineTokenKind::UnderlineSubscript
                && (self.open_formats[map_index(&InlineTokenKind::Underline)]
                    || self.open_formats[map_index(&InlineTokenKind::Subscript)]));
        ambiguous_open || (!is_ambiguous(format) && self.open_formats[map_index(&format)])
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
    pub fn nest_with_scope(&mut self, end_match: Option<IteratorEndFn>) -> Self {
        InlineTokenIterator::from(self.token_iter.nest_with_scope(None, end_match))
    }

    pub fn progress(&mut self, child: Self) {
        // Open formats intentionally not updated, because formats are only valid per scope.
        self.updated_prev = child.updated_prev;
        self.cached_token = child.cached_token;
        self.peeked_cache = child.peeked_cache;
        self.token_iter.progress(child.token_iter);
    }

    pub fn skip_to_peek(&mut self) {
        self.token_iter.skip_to_peek();
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

    pub fn prefix_mismatch(&self) -> bool {
        self.token_iter.prefix_mismatch()
    }
}

impl<'slice, 'input> Iterator for InlineTokenIterator<'slice, 'input> {
    type Item = InlineToken<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        self.peeked_cache = true;
        self.updated_prev = None;

        if let Some(token) = self.cached_token {
            self.updated_prev = Some(token);
            self.cached_token = None;
            return Some(token);
        }

        if self.end_reached() {
            return None;
        }

        Some(InlineToken::from(self.token_iter.next()?))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.token_iter.max_len()))
    }
}

impl<'slice, 'input> PeekingNext for InlineTokenIterator<'slice, 'input> {
    fn peeking_next<F>(&mut self, accept: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnOnce(&Self::Item) -> bool,
    {
        if !self.peeked_cache {
            if let Some(token) = self.cached_token {
                if accept(&token) {
                    self.peeked_cache = true;
                    return Some(token);
                } else {
                    return None;
                }
            }
        }

        if self.end_reached() {
            return None;
        }

        let peek_index = self.token_iter.peek_index();
        let token = InlineToken::from(self.token_iter.peeking_next(|_| true)?);
        if accept(&token) {
            Some(token)
        } else {
            self.token_iter.set_peek_index(peek_index);
            None
        }
    }
}
