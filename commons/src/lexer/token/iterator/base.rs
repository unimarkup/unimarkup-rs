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
#[derive(Clone)]
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
        let first_symbol = self.sym_iter.peeking_next(|_| true)?;
        let first_kind = first_symbol.kind;
        let mut token = Token::from(first_symbol);

        match first_kind {
            SymbolKind::Eoi => token.kind = TokenKind::EOI,
            SymbolKind::Whitespace | SymbolKind::Plain => {
                let seq_len = self.sym_iter.peek_while_count(|s| s.kind == first_kind);

                if seq_len > 0 {
                    let last_symbol = self
                        .sym_iter
                        .peek_nth(seq_len - 1)
                        .expect("Peeked symbols above.");
                    token.offset.extend(last_symbol.offset);
                    token.end = last_symbol.end;
                }
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
                                token.kind = TokenKind::EscapedNewline;
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

            SymbolKind::Newline => {
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
                } else {
                    // No blankline => do not skip whitespaces
                    self.set_peek_index(peek_index);
                }
            }

            _ if first_kind.is_parenthesis() => {
                // TokenKind already set in `From` impl
                // Multiple parenthesis are not combined, because each parenthesis may create a new scope
            }

            // Might be inline formatting token
            _ if first_kind.is_keyword() => {
                let contiguous_keyword_cnt =
                    self.sym_iter.peek_while_count(|s| s.kind == first_kind);

                if contiguous_keyword_cnt > 0 {
                    let last_symbol = self
                        .sym_iter
                        .peek_nth(contiguous_keyword_cnt - 1)
                        .expect("Peeked symbols above");

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
            None
        }
    }
}
