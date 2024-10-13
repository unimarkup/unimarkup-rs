#![allow(dead_code)]
pub mod token;
pub mod token_kind;

use ribbon::{Enroll, Ribbon, Tape};
use token::Token;
use token_kind::TokenKind;

use crate::scanner::SymbolStream;
use crate::symbol::{Symbol, SymbolKind};

/// Lexes the indentation token. Indentation is defined as some number of spaces at the beginning
/// of a line.
fn indentation_or_blankline<'input>(
    start_sym: &Symbol<'input>,
    sym_stream: &mut Tape<SymbolStream<'input>>,
) -> Token<'input> {
    // at least one space token is seen by the caller
    let mut indent = 1;
    let mut span = start_sym.span;

    // make sure we have all spaces
    sym_stream.expand_while(|s| s.kind == SymbolKind::Space);

    while let Some(sym) = sym_stream.pop_front() {
        if sym.kind == SymbolKind::Space {
            indent += 1;
            span.len += sym.span.len;
        }
    }

    sym_stream.expand();

    match sym_stream.peek_front() {
        Some(sym) if sym.kind == SymbolKind::Newline => {
            // indentation means there was a newline, then spaces
            // and now again a newline, meaning we found a line that's blank
            span.len += sym.len();
            Token {
                input: start_sym.input,
                kind: TokenKind::Blankline,
                span,
            }
        }
        _ => Token {
            input: start_sym.input,
            kind: TokenKind::Indentation(indent),
            span,
        },
    }
}

fn identifier<'input>(
    start_sym: &Symbol<'input>,
    sym_stream: &mut Tape<SymbolStream<'input>>,
) -> Token<'input> {
    let mut pos_info = start_sym.span;

    sym_stream.expand_while(|s| s.kind == SymbolKind::Plain);

    while let Some(sym) = sym_stream.pop_front() {
        pos_info.len += sym.span.len;
    }

    Token {
        input: start_sym.input,
        kind: TokenKind::Plain,
        span: pos_info,
    }
}

fn punctuation<'input>(
    start_sym: &Symbol<'input>,
    sym_stream: &mut Tape<SymbolStream<'input>>,
) -> Token<'input> {
    let mut pos_info = start_sym.span;

    sym_stream.expand_while(|s| s.kind == SymbolKind::TerminalPunctuation);

    while let Some(sym) = sym_stream.pop_front() {
        // TODO: how do we handle multiple punctuation symbols? Should it be one symbol?
        //       e.g.: This sentence ends with three dots...
        //                                               ^^^ - should this be one token?
        pos_info.len += sym.span.len;
    }

    Token {
        input: start_sym.input,
        kind: TokenKind::TerminalPunctuation,
        span: pos_info,
    }
}

fn whitespace<'input>(
    start_sym: &Symbol<'input>,
    sym_stream: &mut Tape<SymbolStream<'input>>,
) -> Token<'input> {
    let mut span = start_sym.span;

    sym_stream.expand_while(|s| s.kind == SymbolKind::Whitespace);

    while let Some(sym) = sym_stream.pop_front() {
        span.len += sym.len();
    }

    Token {
        input: start_sym.input,
        kind: TokenKind::Whitespace,
        span,
    }
}

fn plain<'input>(
    start_sym: &Symbol<'input>,
    sym_stream: &mut Tape<SymbolStream<'input>>,
) -> Token<'input> {
    let mut span = start_sym.span;

    sym_stream.expand_while(|s| s.kind == start_sym.kind);

    while let Some(sym) = sym_stream.pop_front() {
        span.len += sym.len();
    }

    Token {
        input: start_sym.input,
        kind: TokenKind::Plain,
        span,
    }
}

fn repeated<'input>(
    start_sym: &Symbol<'input>,
    sym_stream: &mut Tape<SymbolStream<'input>>,
) -> Token<'input> {
    let mut span = start_sym.span;

    sym_stream.expand_while(|symbol| symbol.kind == start_sym.kind);

    while let Some(sym) = sym_stream.pop_front() {
        span.len += sym.len();
    }

    Token {
        input: start_sym.input,
        kind: TokenKind::from((start_sym.kind, span.len)),
        span,
    }
}

fn escaped<'input>(
    start_sym: &Symbol<'input>,
    sym_stream: &mut Tape<SymbolStream<'input>>,
) -> Token<'input> {
    let mut span = start_sym.span;

    sym_stream.expand();

    let sym = sym_stream.pop_front().expect("Unexpected EOI after '\\'");
    span.len += sym.len();

    Token {
        input: start_sym.input,
        kind: TokenKind::Plain,
        span,
    }
}

fn whitespace_or_blankline<'input>(
    start_sym: &Symbol<'input>,
    sym_stream: &mut Tape<SymbolStream<'input>>,
) -> Token<'input> {
    let mut span = start_sym.span;

    sym_stream.expand_while(|s| s.kind == start_sym.kind);

    while let Some(sym) = sym_stream.pop_front() {
        span.len += sym.len();
    }

    sym_stream.expand();

    match sym_stream.peek_front() {
        Some(sym) if sym.kind == SymbolKind::Newline => {
            span.len += sym.len();
            Token {
                input: start_sym.input,
                kind: TokenKind::Blankline,
                span,
            }
        }

        _ => Token {
            input: start_sym.input,
            kind: TokenKind::Whitespace,
            span,
        },
    }
}

pub struct TokenStream<'input> {
    input: &'input str,
    sym_stream: Tape<SymbolStream<'input>>,
    last_newline_offs: u32,
}

impl<'input> TokenStream<'input> {
    pub fn tokenize(input: &'input str) -> Self {
        let sym_stream = SymbolStream::scan_str(input).tape();

        Self {
            input,
            sym_stream,
            last_newline_offs: 0,
        }
    }
}

impl<'input> Iterator for TokenStream<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let sym = self.sym_stream.next()?;

            match sym.kind {
                SymbolKind::Space => {
                    if self.is_start_of_line(&sym) {
                        return Some(indentation_or_blankline(&sym, &mut self.sym_stream));
                    } else {
                        return Some(Token {
                            input: self.input,
                            kind: TokenKind::Whitespace,
                            span: sym.span,
                        });
                    }
                }

                SymbolKind::Newline => {
                    let input = self.input;
                    let span = sym.span;

                    let kind = if self.is_start_of_line(&sym) {
                        TokenKind::Blankline
                    } else {
                        TokenKind::Newline
                    };

                    self.last_newline_offs = sym.span.offs;
                    return Some(Token { input, kind, span });
                }

                SymbolKind::Backslash => {
                    self.sym_stream.expand();

                    if matches!(
                        self.sym_stream.peek_front(),
                        Some(Symbol {
                            kind: SymbolKind::Newline,
                            ..
                        })
                    ) {
                        // skip the newline!
                        self.sym_stream.pop_front();
                    } else {
                        return Some(escaped(&sym, &mut self.sym_stream));
                    }
                }

                SymbolKind::TerminalPunctuation => {
                    return Some(punctuation(&sym, &mut self.sym_stream))
                }

                SymbolKind::Whitespace => {
                    if self.is_start_of_line(&sym) {
                        return Some(whitespace_or_blankline(&sym, &mut self.sym_stream));
                    } else {
                        return Some(whitespace(&sym, &mut self.sym_stream));
                    }
                }

                SymbolKind::Eoi => return None,

                SymbolKind::Hash => {
                    return Some(repeated(&sym, &mut self.sym_stream));
                }

                SymbolKind::Star
                | SymbolKind::Tick
                | SymbolKind::Tilde
                | SymbolKind::Underline
                | SymbolKind::Caret
                | SymbolKind::Quote
                | SymbolKind::Dollar
                | SymbolKind::Colon
                | SymbolKind::Pipe
                | SymbolKind::Plus
                | SymbolKind::Dot
                | SymbolKind::Ampersand
                | SymbolKind::Comma
                | SymbolKind::OpenParenthesis
                | SymbolKind::CloseParenthesis
                | SymbolKind::OpenBracket
                | SymbolKind::CloseBracket
                | SymbolKind::OpenBrace
                | SymbolKind::CloseBrace => return Some(repeated(&sym, &mut self.sym_stream)),

                SymbolKind::Plain => return Some(plain(&sym, &mut self.sym_stream)),

                _other => {
                    return Some(identifier(&sym, &mut self.sym_stream));
                }
            }
        }
    }
}

impl TokenStream<'_> {
    fn is_start_of_line(&self, sym: &Symbol<'_>) -> bool {
        if self.last_newline_offs == 0 {
            sym.span.offs == 0
        } else {
            sym.span.offs.saturating_sub(self.last_newline_offs) == 1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::token_kind::TokenKind;

    use super::TokenStream;

    #[test]
    fn indentation() {
        let input = "    hello";
        let tokens: Vec<_> = super::TokenStream::tokenize(input).collect();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.first().unwrap().kind, TokenKind::Indentation(4));

        let second = tokens.get(1).unwrap();
        assert_eq!(second.kind, TokenKind::Plain);
        assert_eq!(second.as_input_str(), "hello");
    }

    #[test]
    fn multi_line_indent() {
        let input = "    hello\n      there";
        let tokens: Vec<_> = dbg!(super::TokenStream::tokenize(input).collect());

        assert_eq!(tokens.len(), 5);

        let first = tokens.first().unwrap();
        assert_eq!(first.kind, TokenKind::Indentation(4));

        let second = tokens.get(1).unwrap();
        assert_eq!(second.kind, TokenKind::Plain);
        assert_eq!(second.as_input_str(), "hello");

        let third = tokens.get(2).unwrap();

        assert_eq!(third.kind, TokenKind::Newline);

        let fourth = tokens.get(3).unwrap();
        assert_eq!(fourth.kind, TokenKind::Indentation(6));

        let fifth = tokens.get(4).unwrap();
        assert_eq!(fifth.kind, TokenKind::Plain);
        assert_eq!(fifth.as_input_str(), "there");
    }

    #[test]
    fn lf_newline() {
        let input = "hello\nthere";

        let tokens: Vec<_> = dbg!(TokenStream::tokenize(input).collect());

        assert_eq!(tokens.len(), 3);

        let first = tokens.first().unwrap();
        assert_eq!(first.kind, TokenKind::Plain);

        assert_eq!(first.as_input_str(), "hello");

        let second = tokens.get(1).unwrap();
        assert_eq!(second.kind, TokenKind::Newline);

        let third = tokens.get(2).unwrap();
        assert_eq!(third.kind, TokenKind::Plain);

        assert_eq!(third.as_input_str(), "there");
    }

    #[test]
    fn cr_newline() {
        let input = "hello\rthere";

        let tokens: Vec<_> = dbg!(TokenStream::tokenize(input).collect());

        assert_eq!(tokens.len(), 3);

        let first = tokens.first().unwrap();
        assert!(first.kind == TokenKind::Plain);

        assert_eq!(first.as_input_str(), "hello");

        let second = tokens.get(1).unwrap();
        assert!(second.kind == TokenKind::Newline);

        let third = tokens.get(2).unwrap();
        assert!(third.kind == TokenKind::Plain);

        assert_eq!(third.as_input_str(), "there");
    }

    #[test]
    fn cr_lf_newline() {
        let input = "hello\r\nthere";

        let tokens: Vec<_> = dbg!(TokenStream::tokenize(input).collect());

        assert_eq!(tokens.len(), 3);

        let first = tokens.first().unwrap();
        assert!(first.kind == TokenKind::Plain);

        assert_eq!(first.as_input_str(), "hello");

        let second = tokens.get(1).unwrap();
        assert!(dbg!(second.kind) == TokenKind::Newline);

        let third = tokens.get(2).unwrap();
        assert!(third.kind == TokenKind::Plain);

        assert_eq!(third.as_input_str(), "there");
    }

    #[test]
    fn headline() {
        let input = "## Hello there";

        let tokens: Vec<_> = dbg!(TokenStream::tokenize(input).collect());

        assert_eq!(tokens.len(), 5);

        let first = &tokens[0];
        assert_eq!(first.kind, TokenKind::Hash(2));
        assert_eq!(first.as_input_str(), "##");

        let second = &tokens[1];
        assert_eq!(second.kind, TokenKind::Whitespace);
        assert_eq!(second.as_input_str(), " ");

        let third = &tokens[2];
        assert_eq!(third.kind, TokenKind::Plain);
        assert_eq!(third.as_input_str(), "Hello");

        let fourth = &tokens[3];
        assert_eq!(fourth.kind, TokenKind::Whitespace);
        assert_eq!(fourth.as_input_str(), " ");

        let fifth = &tokens[4];
        assert_eq!(fifth.kind, TokenKind::Plain);
        assert_eq!(fifth.as_input_str(), "there");
    }
}
