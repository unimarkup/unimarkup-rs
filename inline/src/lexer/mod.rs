use std::{cmp::Ordering, str::Lines};

use unicode_segmentation::*;

mod token;

pub use token::*;

pub trait Tokenize {
    fn lex(&self) -> Lexer;

    fn lex_with_offs(&self, pos: Position) -> Lexer {
        Lexer { pos, ..self.lex() }
    }

    fn lex_iter(&self) -> TokenIterator;
    fn lex_iter_with_offs(&self, pos: Position) -> TokenIterator {
        let mut iter = self.lex_iter();
        let line_offs = pos.line;

        for _ in 0..=line_offs {
            iter.load_next_line();
        }

        iter.index += pos.column;

        iter.pos = pos;

        iter
    }
}

impl<'a> Tokenize for &'a str {
    fn lex(&self) -> Lexer {
        Lexer {
            input: self,
            pos: Position { line: 0, column: 1 },
        }
    }

    fn lex_iter(&self) -> TokenIterator {
        self.lex().iter()
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
    const OLINE: &'static str = "‾";
    const VLINE: &'static str = "|";
    const TILDE: &'static str = "~";
    const QUOTE: &'static str = "\"";
    const DOLLAR: &'static str = "$";
    // const COLON: &'static str = ":";
    const OPEN_PAREN: &'static str = "(";
    const CLOSE_PAREN: &'static str = ")";
    const OPEN_BRACKET: &'static str = "[";
    const CLOSE_BRACKET: &'static str = "]";
    const OPEN_BRACE: &'static str = "{";
    const CLOSE_BRACE: &'static str = "}";

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Content {
    Store,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum LexLength {
    Unlimited,
    Exact(usize),
}

impl LexLength {
    pub(crate) fn allows_len(&self, len: usize) -> bool {
        match *self {
            LexLength::Unlimited => true,
            LexLength::Exact(exact_len) => len == exact_len,
        }
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

    fn lex_keyword(&mut self) -> Option<Token> {
        let first = self.curr.get(self.index)?;

        // possible options:
        // - '*' -> Italic, Bold or both
        // - '_' -> Underline, Subscript or both
        // - '‾' -> Overline,
        // - '^' -> Superscript
        // - '~' -> Strikethrough
        // - '|' -> Highlight
        // - '`' -> Verbatim
        // - '"' -> Quote
        // - '$' -> Math
        //
        // NOT YET IMPLEMENTED :
        // - ":" -> Custom Emoji, e.g. ::heart::
        // - '[' | ']' -> OpenBracket, CloseBracket
        // - '(' | ')' -> OpenParens, CloseParens
        // - '{' | '}' -> OpenBrace, CloseBrace
        // ... and more

        match *first {
            Lexer::STAR => self.lex_italic_bold(),
            Lexer::ULINE => self.lex_underline_subscript(),
            Lexer::OLINE => self.lex_overline(),
            Lexer::CARET => self.lex_token(Lexer::CARET, 1, TokenKind::Superscript),
            Lexer::TILDE => {
                self.lex_token_exact(Lexer::TILDE, LexLength::Exact(2), TokenKind::Strikethrough)
            }
            Lexer::TICK => {
                self.lex_token_exact(Lexer::TICK, LexLength::Exact(1), TokenKind::Verbatim)
            }
            Lexer::QUOTE => {
                self.lex_token_exact(Lexer::TICK, LexLength::Exact(2), TokenKind::Quote)
            }
            Lexer::DOLLAR => {
                self.lex_token_exact(Lexer::DOLLAR, LexLength::Exact(1), TokenKind::Math)
            }
            Lexer::OPEN_PAREN => self.lex_token_exact(
                Lexer::OPEN_PAREN,
                LexLength::Exact(1),
                TokenKind::OpenParens,
            ),
            Lexer::CLOSE_PAREN => self.lex_token_exact(
                Lexer::CLOSE_PAREN,
                LexLength::Exact(1),
                TokenKind::OpenParens,
            ),
            Lexer::OPEN_BRACKET => self.lex_token_exact(
                Lexer::OPEN_BRACKET,
                LexLength::Exact(1),
                TokenKind::OpenParens,
            ),
            Lexer::CLOSE_BRACKET => self.lex_token_exact(
                Lexer::CLOSE_BRACKET,
                LexLength::Exact(1),
                TokenKind::OpenParens,
            ),
            Lexer::OPEN_BRACE => self.lex_token_exact(
                Lexer::OPEN_BRACE,
                LexLength::Exact(1),
                TokenKind::OpenParens,
            ),
            Lexer::CLOSE_BRACE => self.lex_token_exact(
                Lexer::CLOSE_BRACE,
                LexLength::Exact(1),
                TokenKind::OpenParens,
            ),
            Lexer::VLINE => self.lex_late_token(Lexer::VLINE, 2, TokenKind::Highlight),
            _ => None,
        }
    }

    fn lex_token(&mut self, symbol: &str, len: usize, kind: TokenKind) -> Option<Token> {
        let token = self.lex_by_symbol(symbol, Content::Auto, |lexed_len| {
            match lexed_len.cmp(&len) {
                Ordering::Equal => kind,
                _ => TokenKind::Plain,
            }
        });

        Some(token)
    }

    fn lex_token_exact(&mut self, symbol: &str, len: LexLength, kind: TokenKind) -> Option<Token> {
        let kind_from_len = |lexed_len| {
            if len.allows_len(lexed_len) {
                kind
            } else {
                TokenKind::Plain
            }
        };

        let token = self.lex_token_with_len(symbol, Content::Auto, kind_from_len, len);

        Some(token)
    }

    fn lex_italic_bold(&mut self) -> Option<Token> {
        let token = self.lex_by_symbol(Lexer::STAR, Content::Store, |len| match len {
            1 => TokenKind::Italic,
            2 => TokenKind::Bold,
            _ => TokenKind::ItalicBold,
        });

        Some(token)
    }

    fn lex_late_token(&mut self, symbol: &str, len: usize, kind: TokenKind) -> Option<Token> {
        // if symbol repeats itself more than len times, then leave later symbols as the token
        // itself, and use all of the earlier symbols as plain.
        // Example: |||| -> First 2 || will be lexed as plain, the later one as "Highlight"
        // token (at next iteration)

        let end_pos = self.find_symbol_end_pos(symbol, LexLength::Unlimited);
        let lexed_len = end_pos - self.index;

        let mut to_lex_len = len;
        let mut kind = kind;

        if lexed_len != len {
            to_lex_len = lexed_len - len;
            kind = TokenKind::Plain;
        }

        self.lex_token_exact(symbol, LexLength::Exact(to_lex_len), kind)
    }

    fn lex_underline_subscript(&mut self) -> Option<Token> {
        let token = self.lex_by_symbol(Lexer::ULINE, Content::Store, |len| match len {
            2 => TokenKind::Underline,
            1 => TokenKind::Subscript,
            _ => TokenKind::UnderlineSubscript,
        });

        Some(token)
    }

    fn lex_overline(&mut self) -> Option<Token> {
        let token = self.lex_by_symbol(Lexer::OLINE, Content::Auto, |len| -> TokenKind {
            match len {
                1 => TokenKind::Overline,
                _ => TokenKind::Plain,
            }
        });

        Some(token)
    }

    fn find_symbol_end_pos(&self, symbol: &str, lex_len: LexLength) -> usize {
        let mut pos = self.index;

        loop {
            match self.curr.get(pos) {
                Some(grapheme) if *grapheme == symbol => pos += 1,
                _ => break pos,
            }

            match lex_len {
                LexLength::Exact(len) => {
                    if pos - self.index == len {
                        break pos;
                    }
                }
                _ => continue,
            }
        }
    }

    fn lex_by_symbol<F>(&mut self, symbol: &str, content_option: Content, kind_from_len: F) -> Token
    where
        F: Fn(usize) -> TokenKind,
    {
        self.lex_token_with_len(symbol, content_option, kind_from_len, LexLength::Unlimited)
    }

    fn lex_token_with_len<F>(
        &mut self,
        symbol: &str,
        content_option: Content,
        kind_from_len: F,
        lex_len: LexLength,
    ) -> Token
    where
        F: Fn(usize) -> TokenKind,
    {
        let pos = self.find_symbol_end_pos(symbol, lex_len);

        let len = pos - self.index;

        let kind = kind_from_len(len);
        let start = self.pos;
        let end = start + (0, len - 1);

        let spacing = self.spacing_around(len);

        let token_builder = TokenBuilder::new(kind)
            .span(Span::from((start, end)))
            .space(spacing)
            .optional_content(&self.curr[self.index..pos], content_option);

        self.index = pos;

        token_builder.build()
    }

    fn spacing_around(&self, len: usize) -> Spacing {
        let mut spacing = Spacing::None;

        if self.is_whitespace_at_offs(-1) {
            spacing += Spacing::Pre;
        }
        if self.is_whitespace_at_offs(len as isize) {
            spacing += Spacing::Post;
        }

        spacing
    }

    /// Check if character at cursor position with offset is whitespace.
    fn is_whitespace_at_offs(&self, offset: isize) -> bool {
        if offset < 0 && offset.abs() as usize > self.index {
            false
        } else {
            let pos = if offset < 0 {
                self.index - offset.abs() as usize
            } else {
                self.index + offset as usize
            };

            self.curr.get(pos).map_or(false, |ch| ch.is_whitespace())
        }
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
                return self.lex_keyword();
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
                        let start_pos = self.pos;
                        let end_pos = start_pos + (0, 1);

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
        [
            Lexer::STAR,
            Lexer::ULINE,
            Lexer::OLINE,
            Lexer::CARET,
            Lexer::TICK,
            Lexer::TILDE,
            Lexer::VLINE,
            Lexer::OPEN_PAREN,
            Lexer::CLOSE_PAREN,
            Lexer::OPEN_BRACKET,
            Lexer::OPEN_BRACKET,
            Lexer::OPEN_BRACE,
            Lexer::CLOSE_BRACE,
        ]
        .contains(self)
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
mod tests;
