#![allow(dead_code)]
mod token;
mod token_kind;

use ribbon::{Enroll, Ribbon, Tape};
use token::Token;
use token_kind::TokenKind;

use crate::scanner::SymbolStream;
use crate::symbol::{Symbol, SymbolKind};

/// Lexes the indentation token. Indentation is defined as some number of spaces at the beginning
/// of a line.
fn indentation<'input>(
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

    Token {
        input: start_sym.input,
        kind: TokenKind::Indentation(indent),
        span,
    }
}

fn dot<'input>(
    _start_sym: &Symbol<'input>,
    _sym_stream: &mut Tape<SymbolStream<'input>>,
) -> Token<'input> {
    todo!()
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

pub struct TokenStream<'input> {
    input: &'input str,
    sym_stream: Tape<SymbolStream<'input>>,
    is_newline: bool,
}

impl<'input> TokenStream<'input> {
    pub fn tokenize(input: &'input str) -> Self {
        let sym_stream = SymbolStream::scan_str(input).tape();

        Self {
            input,
            sym_stream,
            is_newline: true,
        }
    }
}

impl<'input> Iterator for TokenStream<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let sym = self.sym_stream.next()?;

            let mut was_newline = false;

            match sym.kind {
                SymbolKind::Space => {
                    if self.is_newline {
                        return Some(indentation(&sym, &mut self.sym_stream));
                    } else {
                        return Some(Token {
                            input: self.input,
                            kind: TokenKind::Whitespace,
                            span: sym.span,
                        });
                    }
                }

                SymbolKind::Dot => {
                    return Some(dot(&sym, &mut self.sym_stream));
                }

                SymbolKind::Newline => {
                    was_newline = true;
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
                        self.is_newline = false;
                        self.sym_stream.pop_front();
                    }
                }

                SymbolKind::TerminalPunctuation => {
                    return Some(punctuation(&sym, &mut self.sym_stream))
                }

                _other => {
                    return Some(identifier(&sym, &mut self.sym_stream));
                } /*
                              SymbolKind::Plain => todo!(),
                              SymbolKind::Whitespace => todo!(),
                              SymbolKind::Newline => todo!(),
                              SymbolKind::Eoi => todo!(),
                              SymbolKind::Backslash => {
                                  if let Some(next) = sym_stream.next() {
                                      match next.kind {
                                          SymbolKind::Newline => {
                                              todo!("create an escaped newline token");
                                          }
                                          SymbolKind::Whitespace | SymbolKind::Space => {
                                              todo!("create an escaped whitespace");
                                          }
                                          _ => {
                                              todo!("handle more cases");
                                          }
                                      }
                                  } else {
                                      todo!("Unexpected EOI?");
                                  }
                              }
                              SymbolKind::Hash => todo!(),
                              SymbolKind::Star => todo!(),
                              SymbolKind::Minus => todo!(),
                              SymbolKind::Plus => todo!(),
                              SymbolKind::Underline => todo!(),
                              SymbolKind::Caret => todo!(),
                              SymbolKind::Tick => todo!(),
                              SymbolKind::Overline => todo!(),
                              SymbolKind::Pipe => todo!(),
                              SymbolKind::Tilde => todo!(),
                              SymbolKind::Quote => todo!(),
                              SymbolKind::Dollar => todo!(),
                              SymbolKind::Colon => todo!(),
                              SymbolKind::Dot => todo!(),
                              SymbolKind::Ampersand => todo!(),
                              SymbolKind::Comma => todo!(),
                              SymbolKind::OpenParenthesis => todo!(),
                              SymbolKind::CloseParenthesis => todo!(),
                              SymbolKind::OpenBracket => todo!(),
                              SymbolKind::CloseBracket => todo!(),
                              SymbolKind::OpenBrace => todo!(),
                              SymbolKind::CloseBrace => todo!(),
                  */
            }

            self.is_newline = was_newline;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{token::Token, token_kind::TokenKind};

    use super::TokenStream;

    #[test]
    fn indentation() {
        let input = "    hello";
        let tokens: Vec<_> = super::TokenStream::tokenize(input).collect();

        assert_eq!(tokens.len(), 2);
        assert!(matches!(
            tokens.first(),
            Some(&Token {
                kind: TokenKind::Indentation(4),
                ..
            })
        ));

        assert!(matches!(
            tokens.get(1),
            Some(&Token {
                kind: TokenKind::Plain,
                ..
            })
        ));

        let second = tokens.get(1).unwrap();

        assert_eq!(second.as_input_str(), "hello");
    }

    #[test]
    fn multi_line_indent() {
        let input = "    hello\n      there";
        let tokens: Vec<_> = super::TokenStream::tokenize(input).collect();

        assert_eq!(tokens.len(), 4);

        assert!(matches!(
            tokens.first(),
            Some(&Token {
                kind: TokenKind::Indentation(4),
                ..
            })
        ));

        let second = tokens.get(1).unwrap();

        assert!(matches!(
            second,
            &Token {
                kind: TokenKind::Plain,
                ..
            }
        ));

        assert!(matches!(second.as_input_str(), "hello"));

        let third = tokens.get(2).unwrap();
        assert!(matches!(
            third,
            &Token {
                kind: TokenKind::Indentation(6),
                ..
            }
        ));

        let fourth = tokens.get(3).unwrap();
        assert!(matches!(
            fourth,
            &Token {
                kind: TokenKind::Plain,
                ..
            }
        ));

        assert!(matches!(fourth.as_input_str(), "there"));
    }

    #[test]
    fn lf_newline() {
        let input = "hello\nthere";

        let tokens: Vec<_> = dbg!(TokenStream::tokenize(input).collect());

        assert_eq!(tokens.len(), 2);

        let first = tokens.first().unwrap();
        assert!(matches!(
            first,
            &Token {
                kind: TokenKind::Plain,
                ..
            }
        ));

        // newline is not present
        assert_eq!(first.as_input_str(), "hello");

        let second = tokens.get(1).unwrap();
        assert!(matches!(
            second,
            &Token {
                kind: TokenKind::Plain,
                ..
            }
        ));

        assert_eq!(second.as_input_str(), "there");
    }

    #[test]
    fn cr_newline() {
        let input = "hello\rthere";

        let tokens: Vec<_> = dbg!(TokenStream::tokenize(input).collect());

        assert_eq!(tokens.len(), 2);

        let first = tokens.first().unwrap();
        assert!(matches!(
            first,
            &Token {
                kind: TokenKind::Plain,
                ..
            }
        ));

        // newline is not present
        assert_eq!(first.as_input_str(), "hello");

        let second = tokens.get(1).unwrap();
        assert!(matches!(
            second,
            &Token {
                kind: TokenKind::Plain,
                ..
            }
        ));

        assert_eq!(second.as_input_str(), "there");
    }

    #[test]
    fn cr_lf_newline() {
        let input = "hello\r\nthere";

        let tokens: Vec<_> = dbg!(TokenStream::tokenize(input).collect());

        assert_eq!(tokens.len(), 2);

        let first = tokens.first().unwrap();
        assert!(matches!(
            first,
            &Token {
                kind: TokenKind::Plain,
                ..
            }
        ));

        // newline is not present
        assert_eq!(first.as_input_str(), "hello");

        let second = tokens.get(1).unwrap();
        assert!(matches!(
            second,
            &Token {
                kind: TokenKind::Plain,
                ..
            }
        ));

        assert_eq!(second.as_input_str(), "there");
    }
}
