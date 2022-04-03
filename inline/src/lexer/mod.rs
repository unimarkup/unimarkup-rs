use std::{cmp::Ordering, str::Lines};

use unicode_segmentation::*;

mod token;

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
    const OLINE: &'static str = "‾";
    const VLINE: &'static str = "|";
    const TILDE: &'static str = "~";

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
    #[allow(unused)]
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
        //
        // NOT YET IMPLEMENTED :
        // - '`' -> Verbatim
        // - '|' -> Highlight
        // - '"' -> Quote
        // - '$' -> Math
        // - ":" -> Custom Emoji, e.g. ::heart::
        // - '[' | ']' -> OpenBracket, CloseBracket
        // - '(' | ')' -> OpenParens, CloseParens
        // - '{' | '}' -> OpenBrace, CloseBrace
        // ... and more

        match *first {
            Lexer::STAR => return self.lex_italic_bold(),
            Lexer::ULINE => return self.lex_underline_subscript(),
            Lexer::OLINE => return self.lex_overline(),
            Lexer::CARET => return self.lex_token(Lexer::CARET, 1, TokenKind::Superscript),
            Lexer::TILDE => {
                return self.lex_token_exact(
                    Lexer::TILDE,
                    LexLength::Exact(2),
                    TokenKind::Strikethrough,
                )
            }
            Lexer::VLINE => return self.lex_late_token(Lexer::VLINE, 2, TokenKind::Highlight),
            _ => {}
        }

        None
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
            _ => TokenKind::UnderlineCombo,
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
        [
            Lexer::STAR,
            Lexer::ULINE,
            Lexer::OLINE,
            Lexer::CARET,
            Lexer::TICK,
            Lexer::TILDE,
            Lexer::VLINE,
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
mod tests {
    use super::*;

    macro_rules! assert_token {
        ($token:ident with $kind:expr, $spacing:expr, $span:expr) => {
            assert_eq!($token.kind(), $kind);
            assert_eq!($token.spacing(), $spacing);
            assert_eq!($token.span(), Span::from($span));
            true
        };

        ($token:ident with $kind:expr, $spacing:expr, $span:expr, $content:expr) => {
            assert_token!($token with $kind, $spacing, $span);
            assert_eq!($token.as_str(), $content);
            true
        }
    }

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
        let start = Position::default();
        let end = start + (0, 11 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "Some string"
        };
    }

    #[test]
    fn lex_plain_with_esc() {
        let input = "Some string \\* with escaped character";
        let lexer = input.lex();

        let mut iter = lexer.into_iter();

        assert_eq!(lexer.into_iter().count(), 1);

        let token = iter.next().unwrap();
        let start = Position::default();
        let end = start + (0, 37 - 1);

        println!("Parsed token {token:?}");

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "Some string * with escaped character"
        };
    }

    #[test]
    fn lex_plain_with_esc_begin() {
        let input = "\\*This string has escape sequence at the beginning";
        let lexer = input.lex();

        let mut iter = lexer.into_iter();

        assert_eq!(lexer.into_iter().count(), 1);

        let token = iter.next().unwrap();

        println!("Parsed token {token:?}");

        let start = Position::default();
        let end = start + (0, 50 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "*This string has escape sequence at the beginning"
        };
    }

    #[test]
    fn lex_simple_space() {
        let input = "\\ ";
        let lexer = input.lex();

        let mut iter = lexer.into_iter();

        let token = iter.next().unwrap();

        let start = Position::default();
        let end = start + (0, 2 - 1);

        assert_token! {
            token with
                TokenKind::Whitespace,
                Spacing::None,
                (start, end),
                " "
        };
    }

    #[test]
    fn lex_tab_whitespace() {
        let input = "\\\t";
        let lexer = input.lex();

        let mut iter = lexer.into_iter();

        let token = iter.next().unwrap();

        let start = Position::default();
        let end = start + (0, 2 - 1);

        assert_token! {
            token with
                TokenKind::Whitespace,
                Spacing::None,
                (start, end),
                "\t"
        };
    }

    #[test]
    fn lex_newline() {
        let input = "\\\n";
        let lexer = input.lex();

        let mut iter = lexer.into_iter();

        let token = iter.next().unwrap();

        let start = Position::default();
        let end = start + (0, 2 - 1);

        assert_token! {
            token with
                TokenKind::Newline,
                Spacing::None,
                (start, end),
                "\n"
        };
    }

    #[test]
    fn lex_whitespace_plain_combined() {
        let input = "This is some text \\ with whitespace token";
        let lexer = input.lex();

        assert_eq!(lexer.iter().count(), 3);

        let mut iter = lexer.iter();

        // PLAIN
        let first = iter.next().unwrap();
        let start = Position::default();
        let end = start + (0, 18 - 1);

        assert_token! {
            first with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "This is some text "
        };

        // WHITESPACE
        let second = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 2 - 1);

        assert_token! {
            second with
                TokenKind::Whitespace,
                Spacing::None,
                (start, end),
                " "
        };

        // PLAIN
        let third = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 21 - 1);

        assert_token! {
            third with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "with whitespace token"
        };
    }

    #[test]
    fn lex_bold() {
        let input = "**This is text in bold** whereas this isn't.";

        let lexer = input.lex();

        let mut iter = lexer.iter();

        assert_eq!(lexer.iter().count(), 4);

        let token = iter.next().unwrap();
        let start = Position::default();
        let end = start + (0, 1);

        assert_token! {
            token with
                TokenKind::Bold,
                Spacing::None,
                (start, end)
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 20 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "This is text in bold"
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 2 - 1);

        assert_token! {
            token with
                TokenKind::Bold,
                Spacing::Post,
                (start, end)
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 20 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                " whereas this isn't."
        };
    }

    #[test]
    fn lex_bold_italic_combined() {
        let input = "****bold and italic***";

        let lexer = input.lex();

        let iter = lexer.iter();

        assert_eq!(iter.count(), 3);

        for token in &lexer {
            println!("{token:#?}");
        }
    }

    #[test]
    fn lex_underline() {
        let mut iter = "__This is underlined__".lex().iter();

        let token = iter.next().unwrap();
        let start = Position::default();
        let end = start + (0, 2 - 1);

        assert_token! {
            token with
                TokenKind::Underline,
                Spacing::None,
                (start, end)
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 18 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "This is underlined"
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 2 - 1);

        assert_token! {
            token with
                TokenKind::Underline,
                Spacing::None,
                (start, end)
        };
    }

    #[test]
    fn lex_subscript() {
        let mut iter = "_This is underlined_".lex().iter();

        let token = iter.next().unwrap();
        let start = Position::default();
        let end = start + (0, 1 - 1);

        assert_token! {
            token with
                TokenKind::Subscript,
                Spacing::None,
                (start, end)
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 18 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "This is underlined"
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 1 - 1);

        assert_token! {
            token with
                TokenKind::Subscript,
                Spacing::None,
                (start, end)
        };
    }

    #[test]
    fn lex_underline_subscript() {
        let mut iter = "___Bla some__ text_ hehe _bla".lex().iter();

        let token = iter.next().unwrap();
        let start = Position::default();
        let end = start + (0, 3 - 1);

        assert_token! {
            token with
                TokenKind::UnderlineCombo,
                Spacing::None,
                (start, end)
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 8 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "Bla some"
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 2 - 1);

        assert_token! {
            token with
                TokenKind::Underline,
                Spacing::Post,
                (start, end)
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 5 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                " text"
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 1 - 1);

        assert_token! {
            token with
                TokenKind::Subscript,
                Spacing::Post,
                (start, end)
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 6 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                " hehe "
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 1 - 1);

        assert_token! {
            token with
                TokenKind::Subscript,
                Spacing::Pre,
                (start, end)
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 3 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "bla"
        };
    }

    #[test]
    fn lex_overline() {
        let input = "‾This is overlined‾";

        let mut iter = input.lex().iter();

        let token = iter.next().unwrap();
        let start = Position::default();
        let end = start + (0, 1 - 1);

        assert_token! {
            token with
                TokenKind::Overline,
                Spacing::None,
                (start, end)
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 17 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "This is overlined"
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 1 - 1);

        assert_token! {
            token with
                TokenKind::Overline,
                Spacing::None,
                (start, end)
        };
    }

    #[test]
    fn lex_overline_too_long() {
        let input = "‾‾This is overlined‾‾‾";

        let mut iter = input.lex().iter();

        let token = iter.next().unwrap();
        let start = Position::default();
        let end = start + (0, 2 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "‾‾"
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 17 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "This is overlined"
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 3 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "‾‾‾"
        };
    }

    #[test]
    fn lex_superscript() {
        let input = "^This is superscript^";

        let mut iter = input.lex().iter();

        let token = iter.next().unwrap();
        let start = Position::default();
        let end = start + (0, 1 - 1);

        assert_token! {
            token with
                TokenKind::Superscript,
                Spacing::None,
                (start, end)
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 19 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "This is superscript"
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 1 - 1);

        assert_token! {
            token with
                TokenKind::Superscript,
                Spacing::None,
                (start, end)
        };
    }

    #[test]
    fn lex_superscript_too_long() {
        let input = "^^This is superscript^^^";

        let mut iter = input.lex().iter();

        let token = iter.next().unwrap();
        let start = Position::default();
        let end = start + (0, 2 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "^^"
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 19 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "This is superscript"
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 3 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "^^^"
        };
    }

    #[test]
    fn lex_strikethrough() {
        let input = "~~This is strikethrough~~";

        let mut iter = input.lex().iter();

        let token = iter.next().unwrap();
        let start = Position::default();
        let end = start + (0, 2 - 1);

        assert_token! {
            token with
                TokenKind::Strikethrough,
                Spacing::None,
                (start, end)
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 21 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "This is strikethrough"
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 2 - 1);

        assert_token! {
            token with
                TokenKind::Strikethrough,
                Spacing::None,
                (start, end)
        };
    }

    #[test]
    fn lex_strikethrough_too_long() {
        let input = "~This is not strikethrough~~~";

        let mut iter = input.lex().iter();

        let token = iter.next().unwrap();
        let start = Position::default();
        let end = start + (0, 1 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "~"
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 25 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "This is not strikethrough"
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 2 - 1);

        assert_token! {
            token with
                TokenKind::Strikethrough,
                Spacing::None,
                (start, end)
        };

        let token = iter.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 1 - 1);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end)
        };
    }
}
