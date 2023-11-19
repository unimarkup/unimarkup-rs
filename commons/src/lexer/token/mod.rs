mod kind;
use itertools::Itertools;
pub use kind::*;

use super::{
    position::{Offset, Position},
    symbol::{iterator::SymbolIterator, Symbol, SymbolKind},
};

pub mod implicit;
pub mod iterator;

/// Token lexed from grapheme [`Symbol`]s of the given input.
///
/// # Lifetimes
///
/// * `'input` - lifetime of input the [`Token`] was lexed from.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token<'input> {
    pub input: &'input str,
    pub offset: Offset,
    pub kind: TokenKind,
    pub start: Position,
    pub end: Position,
}

impl std::fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Token")
            .field("input", &self.input)
            .field(
                "output",
                &self.input[self.offset.start..self.offset.end].to_string(),
            )
            .field("offset", &self.offset)
            .field("kind", &self.kind)
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

impl<'s, 't> From<&Symbol<'s>> for Token<'t>
where
    's: 't,
{
    fn from(value: &Symbol<'s>) -> Self {
        Token {
            input: value.input,
            offset: value.offset,
            kind: TokenKind::from(value.kind),
            start: value.start,
            end: value.end,
        }
    }
}

impl From<&Token<'_>> for String {
    fn from(value: &Token<'_>) -> Self {
        match value.kind {
            TokenKind::Plain | TokenKind::TerminalPunctuation | TokenKind::Whitespace => {
                value.input[value.offset.start..value.offset.end].to_string()
            }
            TokenKind::EscapedPlain | TokenKind::EscapedWhitespace => {
                let escaped_str = &value.input
                    [(value.offset.start + SymbolKind::Backslash.as_str().len())..value.offset.end];
                let mut s = String::with_capacity(escaped_str.len());
                s.push_str(escaped_str);
                s
            }
            _ => String::from(value.kind),
        }
    }
}

impl From<Token<'_>> for String {
    fn from(value: Token<'_>) -> Self {
        String::from(&value)
    }
}

impl<'input> Token<'input> {
    /// Returns the underlying content the given slice spans.
    ///
    /// **Note:** The tokens must have the same input.
    pub fn flatten(tokens: &[Self]) -> Option<&str> {
        let (first, last) = (tokens.first()?, tokens.last()?);

        debug_assert!(std::ptr::eq(first.input, last.input));

        let input = first.input;

        let start = first.offset.start;
        let end = last.offset.end;

        Some(&input[start..end])
    }
}

/// Creates a vector of [`Token`]s from the given string.
pub fn lex_str<'input>(s: &'input str) -> Vec<Token<'input>> {
    let symbols: Vec<Symbol<'input>> = super::scan_str(s);
    let mut sym_iter: SymbolIterator<'_, 'input> = SymbolIterator::from(&*symbols);
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(symbol) = sym_iter.next() {
        tokens.push(next_token(&mut sym_iter, *symbol))
    }

    tokens
}

/// Gets the next [`Token`] from the underlying symbols.
fn next_token<'input>(
    sym_iter: &mut SymbolIterator<'_, 'input>,
    first_symbol: Symbol<'input>,
) -> Token<'input> {
    let first_kind = first_symbol.kind;
    let mut token = Token {
        input: first_symbol.input,
        offset: first_symbol.offset,
        kind: TokenKind::from(first_symbol.kind),
        start: first_symbol.start,
        end: first_symbol.end,
    };

    match first_kind {
        SymbolKind::Eoi => token.kind = TokenKind::Eoi,
        SymbolKind::Plain => {
            // Consume contiguous plain symbols
            if let Some(last_symbol) = sym_iter.peeking_take_while(|s| s.kind == first_kind).last()
            {
                // Consume peeked symbols without iterating over them again
                sym_iter.set_index(sym_iter.peek_index());
                token.offset.extend(last_symbol.offset);
                token.end = last_symbol.end;
            }
        }
        SymbolKind::Whitespace => {
            // Multiple whitespace cannot be consumed, because most prefix matching is done per single space
            // Kind is already set in From impl above.
        }
        SymbolKind::Backslash => {
            let escaped_symbol_opt = sym_iter.next();

            match escaped_symbol_opt {
                Some(escaped_symbol) => {
                    match escaped_symbol.kind {
                        SymbolKind::Whitespace => {
                            token.kind = TokenKind::EscapedWhitespace;
                        }
                        SymbolKind::Newline | SymbolKind::Eoi => {
                            // Only escape non-blanklines, to get correct block-end matching
                            match make_blankline(sym_iter, token) {
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
                // No Symbol after backslash => not possible, because last is always EOI
                None => {
                    unreachable!("No symbol after backslash! Missing EOI symbol at end.");
                }
            }
        }

        SymbolKind::Newline => match sym_iter.peek() {
            // Skip over Newline before EOI
            Some(symbol) if symbol.kind == SymbolKind::Eoi => {
                sym_iter.set_index(sym_iter.peek_index() + 1); // consume Newline in sym iter
                token.kind = TokenKind::Eoi;
                token.end = symbol.end;
            }
            _ => match make_blankline(sym_iter, token) {
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
            let contgiuous_keywords = sym_iter.peeking_take_while(|s| {
                let accept = s.kind == first_kind;
                if accept {
                    contiguous_keyword_cnt += 1;
                }
                accept
            });

            if let Some(last_symbol) = contgiuous_keywords.last() {
                // Consume peeked symbols without iterating over them again
                sym_iter.set_index(sym_iter.peek_index());
                token.kind = TokenKind::from((first_kind, contiguous_keyword_cnt + 1)); // +1 because first symbol is same keyword
                token.offset.extend(last_symbol.offset);
                token.end = last_symbol.end;
            }
        }
        _ => {
            token.kind = TokenKind::Plain;
        }
    }

    token
}

/// Converts a [`TokenKind::Newline`] into a [`TokenKind::Blankline`] if there are only whitespaces until the next [`TokenKind::Newline`].
fn make_blankline<'input>(
    sym_iter: &mut SymbolIterator<'_, 'input>,
    mut token: Token<'input>,
) -> Option<Token<'input>> {
    let _whitespaces = sym_iter
        .peeking_take_while(|s| s.kind == SymbolKind::Whitespace)
        .count();

    let symbol_opt = sym_iter.peek(); // Do not consume last newline/EOI, to allow chaining contiguous blanklines

    if symbol_opt.map_or(false, |s| {
        s.kind == SymbolKind::Newline || s.kind == SymbolKind::Eoi
    }) {
        // Consume peeking_next symbols without iterating over them again
        sym_iter.set_index(sym_iter.peek_index());

        let symbol = symbol_opt.expect("Checked above to be some symbol.");
        token.offset.extend(symbol.offset);
        token.end = symbol.start; // Start position, because last newline/EOI is not consumed
        token.kind = TokenKind::Blankline;
        Some(token)
    } else {
        // No blankline => do not skip whitespaces
        None
    }
}
