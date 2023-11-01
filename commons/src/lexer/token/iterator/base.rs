//! Contains the [`TokenIteratorRoot`] that is the root iterator in any [`TokenIterator`](super::TokenIterator).

use std::collections::VecDeque;

use itertools::{Itertools, PeekingNext};

use crate::lexer::{
    new::SymbolIterator,
    token::{Token, TokenKind},
    SymbolKind,
};

use super::extension::TokenIteratorExt;

/// The [`TokenIteratorRoot`] is the root iterator in any [`TokenIterator`](super::TokenIterator).
/// It holds the actual [`Symbol`] slice.
#[derive(Debug, Clone)]
pub struct TokenIteratorBase<'input> {
    /// The [`Symbol`] slice the iterator was created for.
    sym_iter: SymbolIterator<'input>,
    /// The highest scope any nested iterator based on this root iterator is in.
    pub(super) scope: usize,
    prev_token: Option<Token<'input>>,
    prev_peeked: Option<Token<'input>>,
    cache: VecDeque<Token<'input>>,
    index: usize,
    peek_index: usize,
}

impl<'input> TokenIteratorExt<'input> for TokenIteratorBase<'input> {
    /// Returns the symbol that is directly before the current index.
    /// If no previous symbol exists, `None`` is returned.
    fn prev_token(&self) -> Option<&Token<'input>> {
        self.prev_token.as_ref()
    }

    fn max_len(&self) -> usize {
        self.sym_iter.max_len() + self.cache.len()
    }

    /// Returns `true` if no more [`Symbol`]s are available.
    fn is_empty(&self) -> bool {
        self.max_len() == 0
    }

    /// Returns the current index this iterator is in the [`Symbol`] slice of the root iterator.
    fn index(&self) -> usize {
        self.index
    }

    /// Sets the current index of this iterator to the given index.
    fn set_index(&mut self, index: usize) {
        if self.index < index {
            let index_jump = index - self.index;
            let tokens_to_skip = index_jump.saturating_sub(self.cache.len());
            self.cache.drain(0..self.cache.len().min(index_jump));

            if tokens_to_skip > 0 {
                // Make sure to forward index jump to underlying iterator
                self.dropping(tokens_to_skip);
            }

            self.index = index;
            self.peek_index = index;
        }
    }

    /// Returns the index used to peek.
    fn peek_index(&self) -> usize {
        self.peek_index
    }

    /// Sets the peek index of this iterator to the given index.
    fn set_peek_index(&mut self, index: usize) {
        if self.index <= index {
            self.peek_index = index;
        }
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

impl<'input> From<SymbolIterator<'input>> for TokenIteratorBase<'input> {
    fn from(value: SymbolIterator<'input>) -> Self {
        TokenIteratorBase {
            sym_iter: value,
            scope: 0,
            prev_token: None,
            prev_peeked: None,
            cache: VecDeque::default(),
            index: 0,
            peek_index: 0,
        }
    }
}

impl<'input> Iterator for TokenIteratorBase<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sym_iter.is_empty() {
            return None;
        }

        let next = self.cache.pop_front().or_else(|| self.next_from_symbols());
        self.index += 1;
        self.peek_index = self.index;

        if next.is_some() {
            self.prev_token = next;
        }

        next
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.sym_iter.size_hint()
    }
}

impl<'input> PeekingNext for TokenIteratorBase<'input> {
    fn peeking_next<F>(&mut self, accept: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnOnce(&Self::Item) -> bool,
    {
        if self.sym_iter.is_empty() {
            return None;
        }

        debug_assert!(
            self.index <= self.peek_index,
            "Peek index cannot be smaller than main index."
        );
        let rel_cache_index = self.peek_index - self.index;

        let token = self.cache.get(rel_cache_index).copied().or_else(|| {
            let new_token = self.next_from_symbols()?;
            self.cache.push_back(new_token);
            Some(new_token)
        })?;

        if accept(&token) {
            self.prev_peeked = Some(token);
            self.peek_index += 1;
            Some(token)
        } else {
            None
        }
    }
}

impl<'input> TokenIteratorBase<'input> {
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

    pub fn prev_peeked(&self) -> Option<&Token<'input>> {
        self.prev_peeked.as_ref()
    }

    fn make_blankline(&mut self, mut token: Token<'input>) -> Option<Token<'input>> {
        let _whitespaces = self
            .sym_iter
            .peeking_take_while(|s| s.kind == SymbolKind::Whitespace)
            .count();

        let symbol_opt = self.sym_iter.peek();

        if symbol_opt.map_or(false, |s| {
            s.kind == SymbolKind::Newline || s.kind == SymbolKind::Eoi
        }) {
            // Consume peeked symbols without iterating over them again
            self.sym_iter.set_index(self.sym_iter.peek_index());

            let symbol = symbol_opt.expect("Checked above to be some symbol.");
            token.offset.extend(symbol.offset);
            token.end = symbol.end;
            token.kind = TokenKind::Blankline;
            Some(token)
        } else {
            // No blankline => do not skip whitespaces
            None
        }
    }

    fn next_from_symbols(&mut self) -> Option<Token<'input>> {
        let first_symbol = self.sym_iter.next()?;
        let first_kind = first_symbol.kind;
        let mut token = Token::from(first_symbol);

        match first_kind {
            SymbolKind::Eoi => token.kind = TokenKind::Eoi,
            SymbolKind::Plain => {
                // Consume contiguous plain symbols
                if let Some(last_symbol) = self
                    .sym_iter
                    .peeking_take_while(|s| s.kind == first_kind)
                    .last()
                {
                    // Consume peeked symbols without iterating over them again
                    self.sym_iter.set_index(self.sym_iter.peek_index());
                    token.offset.extend(last_symbol.offset);
                    token.end = last_symbol.end;
                }
            }
            SymbolKind::Whitespace => {
                // Multiple whitespace cannot be consumed, because most prefix matching is done per single space
                // Kind is already set in From impl above.
            }
            SymbolKind::Backslash => {
                let escaped_symbol_opt = self.sym_iter.next();

                match escaped_symbol_opt {
                    Some(escaped_symbol) => {
                        match escaped_symbol.kind {
                            SymbolKind::Whitespace => {
                                token.kind = TokenKind::EscapedWhitespace;
                            }
                            SymbolKind::Newline | SymbolKind::Eoi => {
                                // Only escape non-blanklines, to get correct block-end matching
                                match self.make_blankline(token) {
                                    Some(blankline) => {
                                        token = blankline;
                                    }
                                    None => {
                                        token.kind = TokenKind::EscapedNewline;
                                    }
                                }
                            }
                            _ => {
                                token.kind = TokenKind::EscapedPlain;
                            }
                        };
                        token.offset.extend(escaped_symbol.offset);
                        token.end = escaped_symbol.end;
                    }
                    // No Symbol after backslash => ignore backslash at end of input
                    None => {
                        #[cfg(debug_assertions)]
                        panic!("No symbol after backslash! Missing EOI symbol at end.");

                        #[cfg(not(debug_assertions))]
                        return None;
                    }
                }
            }

            SymbolKind::Newline => match self.sym_iter.peek() {
                // Skip over Newline before EOI
                Some(symbol) if symbol.kind == SymbolKind::Eoi => {
                    self.sym_iter.set_index(self.sym_iter.peek_index() + 1); // consume Newline in sym iter
                    token.kind = TokenKind::Eoi;
                    token.end = symbol.end;
                }
                _ => match self.make_blankline(token) {
                    Some(blankline) => {
                        token = blankline;
                    }
                    None => {
                        token.kind = TokenKind::Newline;
                    }
                },
            },

            SymbolKind::TerminalPunctuation => {
                token.kind = TokenKind::TerminalPunctuation;
            }

            _ if first_kind.is_parenthesis() => {
                // TokenKind already set in `From` impl
                // Multiple parenthesis are not combined, because each parenthesis may create a new scope
            }

            // Might be inline formatting token
            _ if first_kind.is_keyword() => {
                let mut contiguous_keyword_cnt = 0;
                let contgiuous_keywords = self.sym_iter.peeking_take_while(|s| {
                    let accept = s.kind == first_kind;
                    if accept {
                        contiguous_keyword_cnt += 1;
                    }
                    accept
                });

                if let Some(last_symbol) = contgiuous_keywords.last() {
                    // Consume peeked symbols without iterating over them again
                    self.sym_iter.set_index(self.sym_iter.peek_index());
                    token.kind = TokenKind::from((first_kind, contiguous_keyword_cnt + 1)); // +1 because first symbol is same keyword
                    token.offset.extend(last_symbol.offset);
                    token.end = last_symbol.end;
                }
            }
            _ => {
                token.kind = TokenKind::Plain;
            }
        }

        Some(token)
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use crate::lexer::{
        new::SymbolIterator,
        token::{iterator::extension::TokenIteratorExt, TokenKind},
    };

    use super::TokenIteratorBase;

    #[test]
    fn peek_while_cached() {
        let symbols = crate::lexer::scan_str("*+ # - ~");
        let mut base_iter = TokenIteratorBase::from(SymbolIterator::from(&*symbols));

        let peeked_cnt = base_iter
            .peeking_take_while(|t| t.kind != TokenKind::Tilde(1))
            .count();

        assert_eq!(
            base_iter.cache.len(),
            peeked_cnt + 1, // +1 because peeked token for condition is also cached
            "Iterator did not cache tokens."
        );

        let tokens = base_iter
            .take_while(|t| t.kind != TokenKind::Tilde(1))
            .map(|t| t.kind)
            .collect_vec();

        assert_eq!(
            tokens.len(),
            peeked_cnt,
            "Peek and take while did not return the same number of tokens."
        );

        assert_eq!(
            tokens,
            vec![
                TokenKind::Star(1),
                TokenKind::Plus(1),
                TokenKind::Whitespace,
                TokenKind::Hash(1),
                TokenKind::Whitespace,
                TokenKind::Minus(1),
                TokenKind::Whitespace
            ],
            "Take while returned wrong tokens."
        )
    }

    #[test]
    fn cached_tokens_spanning_multiple_symbols() {
        let symbols = crate::lexer::scan_str("**bold** plain");
        let mut cached_iter = TokenIteratorBase::from(SymbolIterator::from(&*symbols));

        let peeked_cnt = cached_iter.peeking_take_while(|_| true).count();

        assert_eq!(
            cached_iter.cache.len(),
            cached_iter.peek_index(),
            "Iterator did not cache tokens."
        );

        let tokens = cached_iter
            .take_while(|t| t.kind != TokenKind::Tilde(1))
            .map(|t| t.kind)
            .collect_vec();

        assert_eq!(
            tokens.len(),
            peeked_cnt,
            "Peek and take while did not return the same number of tokens."
        );

        assert_eq!(
            tokens,
            vec![
                TokenKind::Star(2),
                TokenKind::Plain,
                TokenKind::Star(2),
                TokenKind::Whitespace,
                TokenKind::Plain,
                TokenKind::Eoi,
            ],
            "Take while returned wrong tokens."
        )
    }

    #[test]
    fn cached_tokens_peeked() {
        let symbols = crate::lexer::scan_str("**bold** plain");
        let mut base_iter = TokenIteratorBase::from(SymbolIterator::from(&*symbols));

        let first_peeked_tokens = base_iter
            .peeking_take_while(|_| true)
            .map(|t| t.kind)
            .collect_vec();
        let peek_index = base_iter.peek_index();

        base_iter.reset_peek();

        assert_eq!(
            base_iter.cache.len(),
            peek_index,
            "Iterator removed cached tokens on peek reset."
        );

        let second_peeked_tokens = base_iter
            .peeking_take_while(|_| true)
            .map(|t| t.kind)
            .collect_vec();

        assert_eq!(
            first_peeked_tokens, second_peeked_tokens,
            "Second peek while did not return same tokens as first."
        )
    }
}
