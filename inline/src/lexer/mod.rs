use std::str::Lines;

use unicode_segmentation::*;

mod token;

pub use token::*;

/// Used to create a Unimarkup [`Lexer`] over some data structure, most typically over some kind of
/// string, i.e. [`&str`].
///
/// [`Lexer`]: crate::Lexer
pub trait Tokenize {
    /// Creates the `Lexer` from this type.
    fn lex(&self) -> Lexer;

    /// Creates the `Lexer` from this type starting at the given offset.
    fn lex_with_offs(&self, pos: Position) -> Lexer {
        Lexer { pos, ..self.lex() }
    }

    /// Creates an [`TokenIterator`] from this type.
    ///
    /// [`TokenIterator`]: crate::TokenIterator
    fn lex_iter(&self) -> TokenIterator;

    /// Creates an [`TokenIterator`] from this type starting at the given offset.
    ///
    /// [`TokenIterator`]: crate::TokenIterator
    fn lex_iter_with_offs(&self, pos: Position) -> TokenIterator {
        let lexer = self.lex_with_offs(pos);

        lexer.iter()
    }
}

impl<'a> Tokenize for &'a str {
    fn lex(&self) -> Lexer {
        Lexer {
            input: self,
            pos: Position { line: 1, column: 1 },
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

/// Symbols with significance in Unimarkup inline formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Symbol {
    /// The backslash (`\`) is used for escaping other symbols.
    Esc,
    /// The start (`*`) literal is used for bold and/or italic formatting.
    Star,
    /// The underline (`_`) literal is used for undeline and/or subscript formatting.
    Underline,
    /// The caret (`^`) literal is used for superscript formatting.
    Caret,
    /// The tick (```) literal is used for verbatim formatting.
    Tick,
    /// The overline (`‾`) literal is used for overline formatting.
    Overline,
    /// The pipe (`|`) literal is used for highlight formatting.
    Pipe,
    /// The tilde (`~`) literal is used for strikethrough formatting.
    Tilde,
    /// The quote (`"`) literal is used for quotation formatting.
    Quote,
    /// The dollar (`$`) literal is used for math mode formatting.
    Dollar,
    /// The open parentheses (`(`) literal is used for additional data to text group elements (e.g.
    /// image insert).
    OpenParens,
    /// The close parentheses (`)`) literal is used to close the additional data to text group.
    CloseParens,
    /// The open bracket (`[`) literal is used for text group elements.
    OpenBracket,
    /// The close bracket (`]`) literal is used for text group elements.
    CloseBracket,
    /// The open brace (`{`) literal is used for inline attributes.
    OpenBrace,
    /// The close brace (`}`) literal is used for inline attributes.
    CloseBrace,
    /// The plain symbol represents any other literal with no significance in Unimarkup inline
    /// formatting.
    Plain,
    // Colon,
}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        match self {
            Symbol::Esc => "\\",
            Symbol::Star => "*",
            Symbol::Underline => "_",
            Symbol::Caret => "^",
            Symbol::Tick => "`",
            Symbol::Overline => "‾",
            Symbol::Pipe => "|",
            Symbol::Tilde => "~",
            Symbol::Quote => "\"",
            Symbol::Dollar => "$",
            Symbol::OpenParens => "(",
            Symbol::CloseParens => ")",
            Symbol::OpenBracket => "[",
            Symbol::CloseBracket => "]",
            Symbol::OpenBrace => "{",
            Symbol::CloseBrace => "}",
            Symbol::Plain => "",
            // Symbol::Colon => ":",
        }
    }
}

impl From<&str> for Symbol {
    fn from(input: &str) -> Self {
        match input {
            "\\" => Symbol::Esc,
            "*" => Symbol::Star,
            "_" => Symbol::Underline,
            "^" => Symbol::Caret,
            "`" => Symbol::Tick,
            "‾" => Symbol::Overline,
            "|" => Symbol::Pipe,
            "~" => Symbol::Tilde,
            "\"" => Symbol::Quote,
            "$" => Symbol::Dollar,
            "(" => Symbol::OpenParens,
            ")" => Symbol::CloseParens,
            "[" => Symbol::OpenBracket,
            "]" => Symbol::CloseBracket,
            "{" => Symbol::OpenBrace,
            "}" => Symbol::CloseBrace,
            _ => Symbol::Plain,
        }
    }
}

impl Symbol {
    pub(crate) fn allowed_len(&self) -> LexLength {
        match self {
            Symbol::Star | Symbol::Underline => LexLength::Limited(3),

            Symbol::Esc
            | Symbol::Caret
            | Symbol::Overline
            | Symbol::Tick
            | Symbol::Dollar
            | Symbol::OpenParens => LexLength::Limited(1),

            Symbol::CloseParens
            | Symbol::OpenBracket
            | Symbol::CloseBracket
            | Symbol::OpenBrace
            | Symbol::CloseBrace => LexLength::Exact(1),

            Symbol::Pipe | Symbol::Tilde | Symbol::Quote => LexLength::Limited(2),

            Symbol::Plain => LexLength::Unlimited,
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn iter(&self) -> TokenIterator<'a> {
        let ignored_line_upto_index = self.pos.line.saturating_sub(1);
        let mut lines = self.input.lines();

        let curr = if let Some(line) = lines.nth(ignored_line_upto_index) {
            Vec::from_iter(line.graphemes(true))
        } else {
            Vec::default()
        };

        TokenIterator {
            lines,
            curr,
            index: self.pos.column - 1,
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
pub(crate) enum LexLength {
    /// Any length allowed.
    Unlimited,
    /// Exact length allowed.
    Exact(usize),
    /// Any length up to the limit.
    Limited(usize),
}

impl From<usize> for LexLength {
    fn from(len: usize) -> Self {
        Self::Exact(len)
    }
}

#[derive(Debug, Clone)]
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

    fn next_line(&mut self) -> bool {
        // remove last line from cache
        self.curr.clear();

        if let Some(next_line) = self.lines.next() {
            // load next line into cache
            self.curr.extend(next_line.graphemes(true));

            // update index into current line
            self.index = 0;

            self.pos.line += 1;
            self.pos.column = 1;

            true
        } else {
            false
        }
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
        // - '[' | ']' -> OpenBracket, CloseBracket
        // - '(' | ')' -> OpenParens, CloseParens
        // - '{' | '}' -> OpenBrace, CloseBrace
        //
        // NOT YET IMPLEMENTED :
        // - ":" -> Custom Emoji, e.g. ::heart::
        // ... and more

        // NOTE: General variant of lexing:
        // If some literal occurs the maximal symbol length + 1 times, then it's lexed as plain.

        let symbol = Symbol::from(*first);

        let lex_len = symbol.allowed_len();

        let symbol_len = self.symbol_len(symbol, lex_len);

        let start_pos = self.pos;
        let end_pos = start_pos + (0, symbol_len - 1);

        let spacing = self.spacing_around(symbol_len);

        let kind = TokenKind::from((symbol, symbol_len));

        let pos = self.index + symbol_len;

        let token = TokenBuilder::new(kind)
            .span(Span::from((start_pos, end_pos)))
            .space(spacing)
            .optional_content(&self.curr[self.index..pos], kind.content_option())
            .build();

        self.index = pos;

        Some(token)
    }

    fn symbol_len(&self, symbol: Symbol, lex_len: LexLength) -> usize {
        let end_pos = self.literal_end_index(symbol);
        let scanned_len = end_pos - self.index;

        match lex_len {
            // check if potentially less literals found
            LexLength::Exact(len) => scanned_len.min(len),
            _ => scanned_len,
        }
    }

    fn literal_end_index(&self, symbol: impl AsRef<str>) -> usize {
        let mut pos = self.index;
        let literal = symbol.as_ref();

        loop {
            match self.curr.get(pos) {
                Some(grapheme) if *grapheme == literal => pos += 1,
                _ => break pos,
            }
        }
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
        // NOTE: pos.line is updated only in next_line() function
        self.pos.column = self.index + 1;

        if self.is_end_of_line() && !self.next_line() {
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
        matches!(
            Symbol::from(*self),
            Symbol::Star
                | Symbol::Underline
                | Symbol::Caret
                | Symbol::Tick
                | Symbol::Overline
                | Symbol::Pipe
                | Symbol::Tilde
                | Symbol::Quote
                | Symbol::Dollar
                | Symbol::OpenParens
                | Symbol::CloseParens
                | Symbol::OpenBracket
                | Symbol::CloseBracket
                | Symbol::OpenBrace
                | Symbol::CloseBrace
        )
    }

    fn is_esc(&self) -> bool {
        matches!(Symbol::from(*self), Symbol::Esc)
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
