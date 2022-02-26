#![allow(dead_code)]

use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    index: usize,
}

impl Lexer {
    const ESC: char = '\\';

    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            index: 0,
        }
    }

    fn identify_token(&mut self) -> Token {
        // consume all chars of token
        let first_char = self.input.get(self.index).unwrap(); // this one is guaranteed
        let mut count = 1;

        let space_before = self.index == 0
            || self
                .get(self.index - 1)
                .map(|ch| ch.is_whitespace())
                .unwrap_or(false);

        self.index += 1;

        while let Some(ch) = self.input.get(self.index) {
            if *ch == *first_char {
                self.index += 1;
                count += 1;
            } else {
                break;
            }
        }

        let space_after = self
            .get(self.index + 1)
            .map(|ch| ch.is_whitespace())
            .unwrap_or(false);

        if space_before && space_after {
            return Token::Plain;
        }

        if *first_char == '*' && count == 2 {
            if space_before {
                Token::BoldBegin
            } else {
                Token::BoldEnd
            }
        } else {
            Token::Plain
        }
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
        // 2. First char is part of a keyword -> is some other token

        // let begin_index = self.index;
        let first = self.get(self.index)?; // return None if there is no next char

        if first.is_keyword() {
            // parse token
            return Some(self.identify_token());
        } else {
            // parse until keyword
            while let Some(ch) = self.get(self.index) {
                match ch {
                    _ if ch.is_keyword() => {
                        // return characters as plain text
                        return Some(Token::Plain);
                    }
                    _ if *ch == Lexer::ESC => {
                        // escape character
                        // skip escape character, add next character to plain text
                        // and go to character after it => index + 2
                        self.index += 2;
                    }
                    _ => {
                        self.index += 1;
                    }
                }
            }
        }

        Some(Token::Plain)
    }
}

impl Display for Lexer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ch in &self.input {
            f.write_fmt(format_args!("{}", ch))?;
        }

        Ok(())
    }
}

trait IsKeyword {
    fn is_keyword(&self) -> bool;
}

impl IsKeyword for char {
    fn is_keyword(&self) -> bool {
        *self == '*'
    }
}

struct Spacing {
    before: bool,
    after: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    BoldBegin,
    BoldEnd,
    Plain,
}

// pub enum TokenKind {
//     BoldBegin,
//     BoldEnd,
//     ItalicBegin,
//     ItalicEnd,
// }

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    #[test]
    fn lex_bold() {
        let input = "*\\*This is some** text with some more **text";

        let lexer = Lexer::new(input);

        let tokens: Vec<_> = lexer.collect();
        println!("\nTokens in '{}':", input);
        println!("{:#?}", tokens);

        assert_eq!("haha", "haha");
    }
}
