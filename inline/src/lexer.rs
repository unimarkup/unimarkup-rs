#![allow(dead_code)]

use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

mod spacing;
mod token;

pub use spacing::*;
pub use token::*;

/// Lexer of the Unimarkup inline format types.
#[derive(Debug)]
pub struct Lexer {
    /// Input as characters
    input: Vec<char>,

    /// Cursor used for indexing into input
    index: usize,
}

impl Lexer {
    const ESC: char = '\\';
    const STAR: char = '*';
    const ULINE: char = '_';
    const CARET: char = '^';
    const TILDE: char = '~';

    /// Create a lexer over given input.
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            index: 0,
        }
    }

    /// Get token length starting from the current cursor position.
    fn token_len(&self, token_char: &char) -> usize {
        let mut count = 1;

        while let Some(ch) = self.input.get(self.index + count) {
            if *ch == *token_char {
                count += 1;
            } else {
                break;
            }
        }

        count
    }

    /// Check if character at cursor position with offset is whitespace.
    fn is_whitespace(&self, offset: isize) -> bool {
        if offset < 0 && offset.abs() as usize > self.index {
            false
        } else {
            let pos = if offset < 0 {
                self.index - offset.abs() as usize
            } else {
                self.index + offset as usize
            };

            self.get(pos).map_or(false, |ch| ch.is_whitespace())
        }
    }

    /// Identifies the token under current cursor position.
    fn identify_token(&mut self) -> Token {
        let first_char = self
            .input
            .get(self.index)
            .expect("Expected at least one character");

        let start_index = self.index;

        let mut spacing = Spacing::Neither;

        if self.is_whitespace(-1) {
            spacing += Spacing::Pre;
        }

        let mut len = self.token_len(first_char);

        // check if ambiguous token
        match len.cmp(&3) {
            Ordering::Equal => {
                if self.is_whitespace(len as isize) {
                    spacing += Spacing::Post;
                }

                if let Some(amb_token) = self.identify_ambiguous(first_char, spacing) {
                    self.index += len;
                    return amb_token;
                }
            }
            Ordering::Greater => {
                // 4 or more tokens can be open - close of same token
                // e.g. **** -> open bold, close bold
                len = 2;
                self.index += len;
            }
            Ordering::Less => {
                self.index += len;
            }
        }

        if self.is_whitespace(0) {
            spacing += Spacing::Post;
        }

        if spacing == Spacing::Both {
            // is plain text
            let content = String::from_iter(self.input[start_index..self.index].iter());

            Token::new(TokenKind::Plain(content), Spacing::Both)
        } else {
            let kind = match *first_char {
                Lexer::STAR => match len {
                    2 => TokenKind::Bold,
                    _ => TokenKind::Italic,
                },
                Lexer::ULINE => match len {
                    2 => TokenKind::Underline,
                    _ => TokenKind::Subscript,
                },
                Lexer::TILDE => TokenKind::Verbatim,
                Lexer::CARET => TokenKind::Superscript,
                _ => self.input[start_index..self.index].into(),
            };

            Token::new(kind, spacing)
        }
    }

    /// Identifies the [`AmbiguousToken`] under the current cursor position.
    fn identify_ambiguous(&self, token_char: &char, spacing: Spacing) -> Option<Token> {
        let mut kind: Option<TokenKind> = None;

        match *token_char {
            Self::STAR => {
                kind = Some(TokenKind::Ambiguous(AmbiguousToken::new(
                    TokenKind::Bold,
                    TokenKind::Italic,
                )))
            }
            Self::ULINE => {
                kind = Some(TokenKind::Ambiguous(AmbiguousToken::new(
                    TokenKind::Underline,
                    TokenKind::Subscript,
                )))
            }
            _ => {}
        }

        kind.map(|kind| Token::new(kind, spacing))
    }
}

impl From<&[char]> for TokenKind {
    fn from(input: &[char]) -> Self {
        TokenKind::Plain(String::from_iter(input))
    }
}

impl Deref for Lexer {
    type Target = Vec<char>;

    fn deref(&self) -> &Self::Target {
        &self.input
    }
}

impl DerefMut for Lexer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.input
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // two cases:
        // 1. First char is not part of keyword -> it's plain text token
        // 2. First char is part of a keyword -> is potentially some other token

        let first = self.get(self.index)?; // return None if there is no next char

        if first.is_keyword() {
            // parse token
            Some(self.identify_token())
        } else {
            // parse plain until keyword
            let mut content = String::new();
            let mut begin_index = self.index;

            while let Some(ch) = self.get(self.index) {
                match ch {
                    _ if ch.is_keyword() => {
                        // stop parsing of plain
                        break;
                    }
                    _ if *ch == Lexer::ESC => {
                        // escape character

                        content.extend(self.input[begin_index..self.index].iter());

                        // next character which should be added into content is the one after the
                        // escape character
                        begin_index = self.index + 1;

                        // continue parsing from character after the escape character
                        self.index += 2;
                    }
                    _ => {
                        self.index += 1;
                    }
                }
            }

            content.extend(self.input[begin_index..self.index].iter());

            let token = Token::new(TokenKind::Plain(content), Spacing::Neither);

            Some(token)
        }
    }
}

/// Extension trait for char
trait IsKeyword {
    /// Checks if the given `char` is potentially a keyword.
    fn is_keyword(&self) -> bool;
}

impl IsKeyword for char {
    fn is_keyword(&self) -> bool {
        [Lexer::STAR, Lexer::CARET, Lexer::ULINE, Lexer::TILDE].contains(self)
    }
}

#[cfg(test)]
mod tests;
