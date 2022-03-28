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

impl<'a> Tokenize for &'a str {
    fn lex(&self) -> Lexer {
        Lexer {
            input: self,
            pos: Position { line: 0, column: 1 },
        }
    }
}

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

            // update index into current line
            self.index = 0;

            self.pos.line += 1;
            self.pos.column = 1;

            return true;
        }

        false
    }

    fn lex_plain(&mut self) -> Option<Token> {
        let start_pos = self.pos;
        let mut content = String::with_capacity(self.curr.len());

        // multiple cases:
        // 1. got to end of line -> interpret as end of token
        // 2. some keyword found -> end interpretation
        // 3. escape grapheme found -> end interpretation if grapheme is whitespace | newline;
        //    otherwise continue from next character
        // 4. any other grapheme -> consume into plain

        while let Some(grapheme) = self.curr.get(self.index) {
            if grapheme.is_keyword() {
                break;
            } else if grapheme.is_esc() {
                match self.curr.get(self.index + 1) {
                    // character can be consumed if not significant in escape sequence
                    Some(symbol) if symbol.is_significant_esc() => break,
                    Some(symbol) => {
                        self.index += 2; // consume and skip the symbol in next iteration
                        content.push_str(symbol);
                        continue;
                    }
                    None => break,
                }
            } else {
                content.push_str(grapheme);
                self.index += 1;
            }
        }

        // NOTE: index points to the NEXT character, token Span is UP TO that character
        let offset = self.index - self.pos.column;
        let end_pos = self.pos + (0, offset);

        let token = TokenBuilder::new(TokenKind::Plain)
            .with_content(content)
            .span(Span::from((start_pos, end_pos)))
            .space(Spacing::None)
            .build();

        Some(token)
    }

    fn lex_escape_seq(&mut self) -> Option<Token> {
        let grapheme = self.curr.get(self.index)?;

        // NOTE: index here is pointing to the current grapheme
        let start_pos = self.pos; // escape character
        let end_pos = start_pos + (0, grapheme.len());

        let token_kind = if grapheme.is_whitespace() {
            TokenKind::Whitespace
        } else {
            TokenKind::Newline
        };

        let token = TokenBuilder::new(token_kind)
            .with_content(String::from(*grapheme))
            .span(Span::from((start_pos, end_pos)))
            .space(Spacing::None)
            .build();

        self.index += 1;
        Some(token)
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // NOTE: pos.line is updated only in load_next_line()
        self.pos.column = self.index + 1;

        if self.is_end_of_line() && !self.load_next_line() {
            return None;
        }

        // three cases:
        // 1. next grapheme is keyword -> generate some token
        // 2. next grapheme is '\' -> handle escape sequence
        // 3. next grapheme is not a keyword -> it is plain text

        if let Some(grapheme) = self.curr.get(self.index) {
            if grapheme.is_keyword() {
                // TODO: lex the keyword
                println!("keyword found: {grapheme}");
            } else if grapheme.is_esc() {
                // Three cases:
                // 1. next character has significance in escape sequence -> some token
                // 2. next character has no significance -> lex as plain text
                // 3. there is no next character. That implies that we've got to end of line, which
                //    implies that the character following '\' is either '\n' or '\r\t' -> lex newline

                match self.curr.get(self.index + 1) {
                    Some(grapheme) if grapheme.is_significant_esc() => {
                        self.index += 1;
                        return self.lex_escape_seq();
                    }
                    Some(_) => return self.lex_plain(),
                    None => {
                        // is end of line -> newline token!
                        let start_pos = self.pos + (0, 1);
                        let end_pos = start_pos;

                        let token = TokenBuilder::new(TokenKind::Newline)
                            .span(Span::from((start_pos, end_pos)))
                            .space(Spacing::None)
                            .with_content(String::from("\n"))
                            .build();

                        return Some(token);
                    }
                }
            } else {
                return self.lex_plain();
            }
        }

        None
    }
}

/// Extension trait for graphemes (`&str`)
trait IsKeyword {
    /// Checks whether the grapheme is some Unimarkup Inline symbol.
    /// e.g. "*" can be start of Unimarkup Italic or Bold.
    fn is_keyword(&self) -> bool;

    /// Checks whether the grapheme is "\".
    fn is_esc(&self) -> bool;

    /// Checks whether the grapheme is any of the whitespace characters.
    fn is_whitespace(&self) -> bool;

    /// Checks whether the grapheme is Unix or Windows style newline.
    fn is_newline(&self) -> bool;

    /// Checks whether the grapheme has any significance in escape sequence.
    /// e.g. The lexer interprets "\ " as a Whitespace `Token`
    fn is_significant_esc(&self) -> bool {
        self.is_whitespace() || self.is_newline()
    }
}

impl IsKeyword for &str {
    fn is_keyword(&self) -> bool {
        [Lexer::STAR, Lexer::ULINE, Lexer::CARET, Lexer::TICK].contains(self)
    }

    fn is_esc(&self) -> bool {
        *self == Lexer::ESC
    }

    fn is_whitespace(&self) -> bool {
        // NOTE: multi-character grapheme is most probably not a whitespace
        match self.chars().next() {
            Some(character) => character.is_whitespace(),
            None => false,
        }
    }

    fn is_newline(&self) -> bool {
        ["\n", "\r\n"].contains(self)
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
    fn lex_plain() {
        let lexer = "Some string".lex();
        let mut iter = lexer.into_iter();

        let token = iter.next().unwrap();

        assert_eq!(token.kind(), TokenKind::Plain);
        assert_eq!(token.spacing(), Spacing::None);

        let start_pos = Position::default();
        let end_pos = Position {
            line: 1,
            column: 11,
        };

        assert_eq!(token.span(), Span::from((start_pos, end_pos)));
        assert_eq!(token.as_str(), "Some string");
    }

    #[test]
    fn lex_plain_with_esc() {
        let input = "Some string \\* with escaped character";
        let lexer = input.lex();

        let mut iter = lexer.into_iter();

        assert_eq!(lexer.into_iter().count(), 1);

        let token = iter.next().unwrap();

        println!("Parsed token {token:?}");

        assert_eq!(token.kind(), TokenKind::Plain);
        assert_eq!(token.spacing(), Spacing::None);

        let start_pos = Position { line: 1, column: 1 };
        let end_pos = Position {
            line: 1,
            column: 37,
        };
        assert_eq!(token.span(), Span::from((start_pos, end_pos)));

        let expect_output = "Some string * with escaped character";
        assert_eq!(token.as_str(), expect_output);
    }

    #[test]
    fn lex_plain_with_esc_begin() {
        let input = "\\*This string has escape sequence at the beginning";
        let lexer = input.lex();

        let mut iter = lexer.into_iter();

        assert_eq!(lexer.into_iter().count(), 1);

        let token = iter.next().unwrap();

        println!("Parsed token {token:?}");

        assert_eq!(token.kind(), TokenKind::Plain);
        assert_eq!(token.spacing(), Spacing::None);

        let start_pos = Position { line: 1, column: 1 };
        let end_pos = Position {
            line: 1,
            column: 50,
        };
        assert_eq!(token.span(), Span::from((start_pos, end_pos)));

        let expect_output = "*This string has escape sequence at the beginning";
        assert_eq!(token.as_str(), expect_output);
    }

    #[test]
    fn lex_simple_space() {
        let input = "\\ ";
        let lexer = input.lex();

        let mut iter = lexer.into_iter();

        let token = iter.next().unwrap();

        assert_eq!(token.kind(), TokenKind::Whitespace);

        let start_pos = Position { line: 1, column: 1 };
        let end_pos = start_pos + (0, 1);
        assert_eq!(token.span(), Span::from((start_pos, end_pos)));

        assert_eq!(token.spacing(), Spacing::None);
        assert_eq!(token.as_str(), " ");
    }

    #[test]
    fn lex_tab_whitespace() {
        let input = "\\\t";
        let lexer = input.lex();

        println!("Input: {input}");

        let grapheme = input.graphemes(true).next().unwrap();

        println!("Grapheme: {grapheme}");

        let mut iter = lexer.into_iter();

        let token = iter.next().unwrap();

        assert_eq!(token.kind(), TokenKind::Whitespace);

        let start_pos = Position { line: 1, column: 1 };
        let end_pos = start_pos + (0, 1);
        assert_eq!(token.span(), Span::from((start_pos, end_pos)));

        assert_eq!(token.spacing(), Spacing::None);
        assert_eq!(token.as_str(), "\t");
    }

    #[test]
    fn lex_newline() {
        let input = "\\\n";
        let lexer = input.lex();

        let mut iter = lexer.into_iter();

        let token = iter.next().unwrap();

        assert_eq!(token.kind(), TokenKind::Newline);

        let start_pos = Position { line: 1, column: 2 };
        let end_pos = start_pos;
        assert_eq!(token.span(), Span::from((start_pos, end_pos)));

        assert_eq!(token.spacing(), Spacing::None);
        assert_eq!(token.as_str(), "\n");
    }

    #[test]
    fn lex_whitespace_plain_combined() {
        let input = "This is some text \\ with whitespace token";
        let lexer = input.lex();

        assert_eq!(lexer.iter().count(), 3);

        let mut iter = lexer.iter();

        // PLAIN
        let first = iter.next().unwrap();

        assert_eq!(first.kind(), TokenKind::Plain);
        assert_eq!(first.spacing(), Spacing::None);

        let start = Position { line: 1, column: 1 };
        let end = start + (0, 18 - 1);
        assert_eq!(first.span(), Span::from((start, end)));
        assert_eq!(first.as_str(), "This is some text ");

        // WHITESPACE
        let second = iter.next().unwrap();

        assert_eq!(second.kind(), TokenKind::Whitespace);
        assert_eq!(second.spacing(), Spacing::None);

        let start = end + (0, 1);
        let end = start + (0, 1);
        assert_eq!(second.span(), Span::from((start, end)));
        assert_eq!(second.as_str(), " ");

        // PLAIN
        let third = iter.next().unwrap();

        assert_eq!(third.kind(), TokenKind::Plain);
        assert_eq!(third.spacing(), Spacing::None);

        let start = end + (0, 1);
        let end = start + (0, 21 - 1);
        assert_eq!(third.span(), Span::from((start, end)));
        assert_eq!(third.as_str(), "with whitespace token");
    }
}
