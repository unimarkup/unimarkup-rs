mod kind;
pub use kind::*;

use crate::scanner::position::Offset;

use self::iterator::TokenIteratorImplicits;

use super::{Token, TokenKind};

pub mod iterator;

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
        let mut first_token = implicit_iter.base_iter.next()?;
        match first_token.kind {
            // Might be: Trademark, Copyright, Registered, plusminus
            TokenKind::OpenParenthesis => {
                if let Some(implicit_kind) =
                    get_trademark_copyright_registered_plusmins_kind(implicit_iter)
                {
                    let closing = implicit_iter.base_iter.next()?;
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
                    TokenKind::ImplicitSubstitution(ImplicitSubstitution::HorizontalEllipsis);
            }
            TokenKind::Minus(2) => {
                first_token.kind = TokenKind::ImplicitSubstitution(ImplicitSubstitution::EnDash);
            }
            TokenKind::Minus(3) => {
                first_token.kind = TokenKind::ImplicitSubstitution(ImplicitSubstitution::EmDash);
            }
            _ => return None,
        }

        Some(first_token)
    }
}

fn get_trademark_copyright_registered_plusmins_kind(
    implicit_iter: &mut TokenIteratorImplicits,
) -> Option<ImplicitSubstitution> {
    // First open parenthesis already consumed, lase closing parenthesis is checked in `get_implicit`
    let outer_token = implicit_iter.base_iter.next()?;
    match outer_token.kind {
        // Might be: Copyright or Registered
        TokenKind::OpenParenthesis => {
            let inner_token = implicit_iter.base_iter.next()?;
            if inner_token.kind == TokenKind::Plain {
                let content = String::from(inner_token).to_lowercase();
                let subst = if content == "c" {
                    Some(ImplicitSubstitution::Copyright)
                } else if content == "r" {
                    Some(ImplicitSubstitution::Registered)
                } else {
                    return None;
                };

                if implicit_iter.base_iter.next()?.kind == TokenKind::CloseParenthesis {
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
                Some(ImplicitSubstitution::Trademark)
            } else if content == "+-" || content == "-+" {
                Some(ImplicitSubstitution::PlusMinus)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn get_implicit_arrow<'input>(
    implicit_iter: &mut TokenIteratorImplicits<'input>,
) -> Option<Token<'input>> {
    None
}

fn get_implicit_emoji<'input>(
    implicit_iter: &mut TokenIteratorImplicits<'input>,
) -> Option<Token<'input>> {
    None
}

fn get_direct_uri<'input>(
    implicit_iter: &mut TokenIteratorImplicits<'input>,
) -> Option<Token<'input>> {
    None
}
