mod token;

use std::str::Lines;

use unicode_segmentation::*;

pub use token::*;

pub trait Tokenize {
    fn lex(&self) -> Lexer;

    fn lex_with_offs(&self, pos: Position) -> Lexer {
        Lexer { pos, ..self.lex() }
    }
}

impl<'a, T> Tokenize for T
where
    T: AsRef<str> + 'a,
{
    fn lex(&self) -> Lexer {
        Lexer {
            input: self.as_ref(),
            pos: Position { line: 0, column: 1 },
        }
    }
}

/*
 * This is a paragraph with \n newline inside
 *
 * OUTPUT: This is a paragraph with n newline inside
 *
 * .chars() -> Chars: Iterator
 *
 * \t, \n, \\, \*
 *
 * **Some text* bla**
 *
 * Bold, Plain, Italic
 *
 *
 *
 * */

pub struct Lexer<'a> {
    input: &'a str,
    pos: Position,
}

impl<'a> Lexer<'a> {
    const ESC: &'static str = "\\";
    const STAR: &'static str = "*";
    const ULINE: &'static str = "_";
    const CARET: &'static str = "^";
    const TICK: &'static str = "`";

    pub fn iter(&self) -> TokenIterator<'a> {
        TokenIterator {
            lines: self.input.lines(),
            curr: Vec::new(),
            index: 0,
            pos: self.pos,
        }
    }
}

impl<'a> IntoIterator for &'a Lexer<'a> {
    type Item = Token;

    type IntoIter = TokenIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct TokenIterator<'a> {
    lines: Lines<'a>,
    curr: Vec<&'a str>,
    index: usize,
    pos: Position, // in input text
}

impl TokenIterator<'_> {
    fn is_end_of_line(&self) -> bool {
        self.index >= self.curr.len()
    }

    fn load_next_line(&mut self) -> bool {
        // remove last line from cache
        self.curr.clear();

        if let Some(next_line) = self.lines.next() {
            // load next line into cache
            self.curr.extend(next_line.graphemes(true));

            // update position
            self.pos.line += 1;
            self.pos.column = 1;

            // update index into current line
            self.index = 0;

            return true;
        }

        // two cases:
        // 1. next grapheme is keyword -> generate some token
        // 2. next grapheme is not a keyword -> it is plain text

        false
    }

    fn lex_plain(&mut self) -> Option<Token> {
        let start_pos = self.pos;
        let mut content = String::with_capacity(self.curr.len());

        // multiple cases:
        // 1. got to end of line -> interpret as end of token
        // 2. escape grapheme found -> end interpretation
        // 3. some keyword found -> end interpretation
        // 4. any other grapheme -> consume into plain

        while let Some(grapheme) = self.curr.get(self.index) {
            if grapheme.is_esc() || grapheme.is_keyword() {
                // skip the escape character and continue from next character
                break;
            } else {
                content.push_str(grapheme);
                self.index += 1;
            }
        }

        let mut end_pos = self.pos;
        end_pos.column += self.index;

        let token = Token::new(TokenKind::Plain)
            .with_content(content)
            .span(Span::from((start_pos, end_pos)))
            .space(Spacing::None);

        Some(token)
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end_of_line() && !self.load_next_line() {
            return None;
        }

        if let Some(grapheme) = self.curr.get(self.index) {
            if grapheme.is_keyword() {
                // TODO: lex the keyword
                todo!()
            } else if grapheme.is_esc() {
                // TODO: lex escape
            } else {
                return self.lex_plain();
            }
        }

        None
    }
}

trait IsKeyword {
    fn is_keyword(&self) -> bool;
    fn is_esc(&self) -> bool;
}

impl IsKeyword for &str {
    fn is_keyword(&self) -> bool {
        [Lexer::STAR, Lexer::ULINE, Lexer::CARET, Lexer::TICK].contains(self)
    }

    fn is_esc(&self) -> bool {
        *self == "\\"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines() {
        let input = r#"first line

            third line"#;

        assert_eq!(input.lines().count(), 3);
    }

    #[test]
    fn into_iter() {
        let lexer = "Some string".lex();

        for token in &lexer {
            println!("{:?}", token);
        }

        assert_eq!(lexer.into_iter().count(), 1);
    }

    #[test]
    fn consume_plain() {
        let lexer = "Some string".lex();

        println!("\nTesting consume_plain: \n\n");

        for token in &lexer {
            println!("{token:?}");
            // assert_eq!(token.kind(), TokenKind::Plain)
        }

        println!("\nEnd consume_plain test\n\n");
    }
}
