mod kind;
use itertools::PeekingNext;
pub use kind::*;

use crate::lexer::position::Offset;

use super::{iterator::implicit::TokenIteratorImplicits, Token, TokenKind};

pub fn get_implicit<'input>(
    implicit_iter: &mut TokenIteratorImplicits<'input>,
) -> Option<Token<'input>> {
    if let Some(arrow_token) = get_implicit_arrow(implicit_iter) {
        Some(arrow_token)
    } else if let Some(emoji_token) = get_implicit_emoji(implicit_iter) {
        Some(emoji_token)
    } else if let Some(direct_uri_token) = get_direct_uri(implicit_iter) {
        Some(direct_uri_token)
    } else {
        let mut first_token = implicit_iter.base_iter.peeking_next(|_| true)?;
        match first_token.kind {
            // Might be: Trademark, Copyright, Registered, plusminus
            TokenKind::OpenParenthesis => {
                if let Some(implicit_kind) =
                    get_trademark_copyright_registered_plusmins_kind(implicit_iter)
                {
                    let closing = implicit_iter.base_iter.peeking_next(|_| true)?;
                    if closing.kind == TokenKind::CloseParenthesis {
                        return Some(Token {
                            input: first_token.input,
                            offset: Offset {
                                start: first_token.offset.start,
                                end: closing.offset.end,
                            },
                            kind: TokenKind::ImplicitSubstitution(implicit_kind),
                            start: first_token.start,
                            end: closing.end,
                        });
                    }
                }

                return None;
            }
            TokenKind::Dot(3) => {
                first_token.kind =
                    TokenKind::ImplicitSubstitution(ImplicitSubstitutionKind::HorizontalEllipsis);
            }
            TokenKind::Minus(2) => {
                first_token.kind =
                    TokenKind::ImplicitSubstitution(ImplicitSubstitutionKind::EnDash);
            }
            TokenKind::Minus(3) => {
                first_token.kind =
                    TokenKind::ImplicitSubstitution(ImplicitSubstitutionKind::EmDash);
            }
            _ => return None,
        }

        Some(first_token)
    }
}

fn get_trademark_copyright_registered_plusmins_kind(
    implicit_iter: &mut TokenIteratorImplicits,
) -> Option<ImplicitSubstitutionKind> {
    // First open parenthesis already consumed, lase closing parenthesis is checked in `get_implicit`
    let outer_token = implicit_iter.base_iter.peeking_next(|_| true)?;
    match outer_token.kind {
        // Might be: Copyright or Registered
        TokenKind::OpenParenthesis => {
            let inner_token = implicit_iter.base_iter.peeking_next(|_| true)?;
            if inner_token.kind == TokenKind::Plain {
                let content = String::from(inner_token).to_lowercase();
                let subst = if content == "c" {
                    Some(ImplicitSubstitutionKind::Copyright)
                } else if content == "r" {
                    Some(ImplicitSubstitutionKind::Registered)
                } else {
                    return None;
                };

                if implicit_iter.base_iter.peeking_next(|_| true)?.kind
                    == TokenKind::CloseParenthesis
                {
                    subst
                } else {
                    None
                }
            } else {
                None
            }
        }
        // Might be: trademark or plusminus
        TokenKind::Plain => {
            let content = String::from(outer_token).to_lowercase();
            if content == "tm" {
                Some(ImplicitSubstitutionKind::Trademark)
            } else if content == "+-" || content == "-+" {
                Some(ImplicitSubstitutionKind::PlusMinus)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn get_implicit_arrow<'input>(
    _implicit_iter: &mut TokenIteratorImplicits<'input>,
) -> Option<Token<'input>> {
    None
}

fn get_implicit_emoji<'input>(
    _implicit_iter: &mut TokenIteratorImplicits<'input>,
) -> Option<Token<'input>> {
    None
}

fn get_direct_uri<'input>(
    _implicit_iter: &mut TokenIteratorImplicits<'input>,
) -> Option<Token<'input>> {
    None
}
