use super::{iterator::implicits::TokenIteratorImplicits, TokenKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImplicitSubstitution {
    Arrow(ArrowSubsitution),
    Emoji(EmojiSubstitution),
    Trademark,
    Copyright,
    Registered,
    HorizontalEllipsis,
    PlusMinus,
    EnDash,
    EmDash,
    DirectUri,
}

pub fn get_implicit(implicit_iter: &mut TokenIteratorImplicits) -> Option<ImplicitSubstitution> {
    if let Some(arrow) = get_implicit_arrow(implicit_iter) {
        Some(ImplicitSubstitution::Arrow(arrow))
    } else if let Some(emoji) = get_implicit_emoji(implicit_iter) {
        Some(ImplicitSubstitution::Emoji(emoji))
    } else {
        match implicit_iter.next()?.kind {
            // Might be: Trademark, Copyright, Registered, plusminus
            TokenKind::OpenParenthesis => {
                let tm_c_r_pm_opt = get_trademark_copyright_registered_plusmins(implicit_iter);
                if implicit_iter.next()?.kind == TokenKind::CloseParenthesis {
                    tm_c_r_pm_opt
                } else {
                    None
                }
            }
            TokenKind::Dot(3) => Some(ImplicitSubstitution::HorizontalEllipsis),
            TokenKind::Minus(2) => Some(ImplicitSubstitution::EnDash),
            TokenKind::Minus(3) => Some(ImplicitSubstitution::EmDash),
            _ => None,
        }
    }
}

fn get_trademark_copyright_registered_plusmins(
    implicit_iter: &mut TokenIteratorImplicits,
) -> Option<ImplicitSubstitution> {
    // First open parenthesis already consumed, lase closing parenthesis is checked in `get_implicit`
    let outer_token = implicit_iter.next()?;
    match outer_token.kind {
        // Might be: Copyright or Registered
        TokenKind::OpenParenthesis => {
            let inner_token = implicit_iter.next()?;
            if inner_token.kind == TokenKind::Plain {
                let content = String::from(inner_token).to_lowercase();
                let subst = if content == "c" {
                    Some(ImplicitSubstitution::Copyright)
                } else if content == "r" {
                    Some(ImplicitSubstitution::Registered)
                } else {
                    return None;
                };

                if implicit_iter.next()?.kind == TokenKind::CloseParenthesis {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArrowSubsitution {}

fn get_implicit_arrow(implicit_iter: &mut TokenIteratorImplicits) -> Option<ArrowSubsitution> {
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EmojiSubstitution {}

fn get_implicit_emoji(implicit_iter: &mut TokenIteratorImplicits) -> Option<EmojiSubstitution> {
    None
}
