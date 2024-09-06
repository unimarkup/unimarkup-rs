mod token;
mod token_kind;

use icu_segmenter::GraphemeClusterSegmenter;
use ribbon::{Enroll, Tape};
use token::Token;

use crate::scanner::SymbolStream;
use crate::symbol::SymbolKind;

fn indentation<'input>(_sym_stream: &mut Tape<SymbolStream<'_, 'input>>) -> Token<'input> {
    todo!()
}

fn tokenize(input: &str) -> Vec<Token> {
    let segmenter = GraphemeClusterSegmenter::new();

    let mut sym_stream = super::scanner::scan_str(input, &segmenter).tape();
    let mut start_of_line = true;

    while let Some(sym) = sym_stream.next() {
        match sym.kind {
            SymbolKind::Space => {
                if start_of_line {
                    start_of_line = false;
                    indentation(&mut sym_stream);
                } else {
                    todo!("Create a space token")
                }
            }

            SymbolKind::Plain => todo!(),
            SymbolKind::TerminalPunctuation => todo!(),
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
        }
    }

    todo!()
}

// /// Gets the next [`Token`] from the underlying symbols.
// fn next_token<'input>(
//     sym_iter: &mut SymbolIterator<'_, 'input>,
//     first_symbol: Symbol<'input>,
// ) -> Token<'input> {
//     let first_kind = first_symbol.kind;
//     let mut token = Token {
//         input: first_symbol.input,
//         offset: first_symbol.offset,
//         kind: TokenKind::from(first_symbol.kind),
//         start: first_symbol.start,
//         end: first_symbol.end,
//     };
//
//     match first_kind {
//         SymbolKind::Eoi => token.kind = TokenKind::Eoi,
//         SymbolKind::Plain => {
//             // Consume contiguous plain symbols
//             if let Some(last_symbol) = sym_iter.peeking_take_while(|s| s.kind == first_kind).last()
//             {
//                 // Consume peeked symbols without iterating over them again
//                 sym_iter.set_index(sym_iter.peek_index());
//                 token.offset.extend(last_symbol.offset);
//                 token.end = last_symbol.end;
//             }
//         }
//         SymbolKind::Whitespace => {
//             // Multiple whitespace cannot be consumed, because most prefix matching is done per single space
//             // Kind is already set in From impl above.
//         }
//         SymbolKind::Backslash => {
//             let escaped_symbol_opt = sym_iter.next();
//
//             match escaped_symbol_opt {
//                 Some(escaped_symbol) => {
//                     match escaped_symbol.kind {
//                         SymbolKind::Whitespace => {
//                             token.kind = TokenKind::EscapedWhitespace;
//                         }
//                         SymbolKind::Newline | SymbolKind::Eoi => {
//                             // Only escape non-blanklines, to get correct block-end matching
//                             match make_blankline(sym_iter, token) {
//                                 Some(blankline) => {
//                                     token = blankline;
//                                 }
//                                 None => {
//                                     token.kind = TokenKind::EscapedNewline;
//                                 }
//                             }
//                         }
//                         _ => {
//                             token.kind = TokenKind::EscapedPlain;
//                         }
//                     };
//                     token.offset.extend(escaped_symbol.offset);
//                     token.end = escaped_symbol.end;
//                 }
//                 // No Symbol after backslash => not possible, because last is always EOI
//                 None => {
//                     unreachable!("No symbol after backslash! Missing EOI symbol at end.");
//                 }
//             }
//         }
//
//         SymbolKind::Newline => match sym_iter.peek() {
//             // Skip over Newline before EOI
//             Some(symbol) if symbol.kind == SymbolKind::Eoi => {
//                 sym_iter.set_index(sym_iter.peek_index() + 1); // consume Newline in sym iter
//                 token.kind = TokenKind::Eoi;
//                 token.end = symbol.end;
//             }
//             _ => match make_blankline(sym_iter, token) {
//                 Some(blankline) => {
//                     token = blankline;
//                 }
//                 None => {
//                     token.kind = TokenKind::Newline;
//                 }
//             },
//         },
//
//         SymbolKind::TerminalPunctuation => {
//             token.kind = TokenKind::TerminalPunctuation;
//         }
//
//         _ if first_kind.is_parenthesis() => {
//             // TokenKind already set in `From` impl
//             // Multiple parenthesis are not combined, because each parenthesis may create a new scope
//         }
//
//         // Might be inline formatting token
//         _ if first_kind.is_keyword() => {
//             let mut contiguous_keyword_cnt = 0;
//             let contgiuous_keywords = sym_iter.peeking_take_while(|s| {
//                 let accept = s.kind == first_kind;
//                 if accept {
//                     contiguous_keyword_cnt += 1;
//                 }
//                 accept
//             });
//
//             if let Some(last_symbol) = contgiuous_keywords.last() {
//                 // Consume peeked symbols without iterating over them again
//                 sym_iter.set_index(sym_iter.peek_index());
//                 token.kind = TokenKind::from((first_kind, contiguous_keyword_cnt + 1)); // +1 because first symbol is same keyword
//                 token.offset.extend(last_symbol.offset);
//                 token.end = last_symbol.end;
//             }
//         }
//         _ => {
//             token.kind = TokenKind::Plain;
//         }
//     }
//
//     token
// }
//
// /// Converts a [`TokenKind::Newline`] into a [`TokenKind::Blankline`] if there are only whitespaces until the next [`TokenKind::Newline`].
// fn make_blankline<'input>(
//     sym_iter: &mut SymbolIterator<'_, 'input>,
//     mut token: Token<'input>,
// ) -> Option<Token<'input>> {
//     let _whitespaces = sym_iter
//         .peeking_take_while(|s| s.kind == SymbolKind::Whitespace)
//         .count();
//
//     let symbol_opt = sym_iter.peek(); // Do not consume last newline/EOI, to allow chaining contiguous blanklines
//
//     if symbol_opt.map_or(false, |s| {
//         s.kind == SymbolKind::Newline || s.kind == SymbolKind::Eoi
//     }) {
//         // Consume peeking_next symbols without iterating over them again
//         sym_iter.set_index(sym_iter.peek_index());
//
//         let symbol = symbol_opt.expect("Checked above to be some symbol.");
//         token.offset.extend(symbol.offset);
//         token.end = symbol.start; // Start position, because last newline/EOI is not consumed
//         token.kind = TokenKind::Blankline;
//         Some(token)
//     } else {
//         // No blankline => do not skip whitespaces
//         None
//     }
// }
