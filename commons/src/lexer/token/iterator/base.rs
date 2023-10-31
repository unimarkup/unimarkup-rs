//! Contains the [`TokenIteratorRoot`] that is the root iterator in any [`TokenIterator`](super::TokenIterator).

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
}

impl<'input> TokenIteratorExt<'input> for TokenIteratorBase<'input> {
    /// Returns the symbol that is directly before the current index.
    /// If no previous symbol exists, `None`` is returned.
    fn prev_token(&self) -> Option<&Token<'input>> {
        self.prev_token.as_ref()
    }

    fn max_len(&self) -> usize {
        self.sym_iter.max_len()
    }

    /// Returns `true` if no more [`Symbol`]s are available.
    fn is_empty(&self) -> bool {
        self.max_len() == 0
    }

    /// Returns the current index this iterator is in the [`Symbol`] slice of the root iterator.
    fn index(&self) -> usize {
        self.sym_iter.index()
    }

    /// Sets the current index of this iterator to the given index.
    fn set_index(&mut self, index: usize) {
        self.sym_iter.set_index(index);
    }

    /// Returns the index used to peek.
    fn peek_index(&self) -> usize {
        self.sym_iter.peek_index()
    }

    /// Sets the peek index of this iterator to the given index.
    fn set_peek_index(&mut self, index: usize) {
        self.sym_iter.set_peek_index(index);
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
        }
    }
}

impl<'input> Iterator for TokenIteratorBase<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        self.sym_iter.reset_peek();

        let next = self.peeking_next(|_| true);
        self.sym_iter.set_index(self.sym_iter.peek_index());

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
        let peek_index = self.peek_index();
        let first_symbol = self.sym_iter.peeking_next(|_| true)?;
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
                    token.offset.extend(last_symbol.offset);
                    token.end = last_symbol.end;
                }
            }
            SymbolKind::Whitespace => {
                // Multiple whitespace cannot be consumed, because most prefix matching is done per single space
                // Kind is already set in From impl above.
            }
            SymbolKind::Backslash => {
                let escaped_symbol_opt = self.sym_iter.peeking_next(|_| true);

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
                        return None;
                    }
                }
            }

            SymbolKind::Newline => match self.sym_iter.peek_kind() {
                // Skip over Newline before EOI
                Some(SymbolKind::Eoi) => {
                    return match self.peeking_next(accept) {
                        Some(mut eoi) => {
                            eoi.start = token.start;
                            Some(eoi)
                        }
                        None => None,
                    };
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
                    token.kind = TokenKind::from((first_kind, contiguous_keyword_cnt + 1)); // +1 because first symbol is same keyword
                    token.offset.extend(last_symbol.offset);
                    token.end = last_symbol.end;
                }
            }
            _ => {
                token.kind = TokenKind::Plain;
            }
        }

        if accept(&token) {
            Some(token)
        } else {
            // reset peek to also reset peek of sym iterator, because sym peeking_next was without condition.
            self.set_peek_index(peek_index);
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

    fn make_blankline(&mut self, mut token: Token<'input>) -> Option<Token<'input>> {
        let peek_index = self.peek_index();
        let _whitespaces = self
            .sym_iter
            .peeking_take_while(|s| s.kind == SymbolKind::Whitespace)
            .count();

        let symbol_opt = self.sym_iter.peek();

        if symbol_opt.map_or(false, |s| {
            s.kind == SymbolKind::Newline || s.kind == SymbolKind::Eoi
        }) {
            let symbol = symbol_opt.expect("Checked above to be some symbol.");
            token.offset.extend(symbol.offset);
            token.end = symbol.end;
            token.kind = TokenKind::Blankline;
            Some(token)
        } else {
            // No blankline => do not skip whitespaces
            self.set_peek_index(peek_index);
            None
        }
    }
}
