use crate::scanner::SymbolKind;

/// The kind of the token found in Unimarkup document.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenKind {
    // Keywords
    Star(usize),
    Hash(usize),
    Minus(usize),
    Plus(usize),
    Underline(usize),
    Caret(usize),
    Tick(usize),
    Overline(usize),
    Pipe(usize),
    Tilde(usize),
    Quote(usize),
    Dollar(usize),
    Colon(usize),

    // parenthesis
    OpenParenthesis,
    CloseParenthesis,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,

    // Spaces
    Whitespace,
    Newline,
    Blankline,
    EOI,

    // Escaped
    EscapedPlain,
    EscapedWhitespace,
    EscapedNewline,

    // Plain
    #[default]
    Plain,

    // Comments
    Comment,

    // For matching
    Any,
    PossibleAttributes,
    PossibleDecorator,
}

impl TokenKind {
    pub fn is_keyword(&self) -> bool {
        !self.is_not_keyword()
    }

    pub fn is_not_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Whitespace
                | TokenKind::Newline
                | TokenKind::Blankline
                | TokenKind::EOI
                | TokenKind::EscapedPlain
                | TokenKind::EscapedWhitespace
                | TokenKind::EscapedNewline
                | TokenKind::Plain
                | TokenKind::Comment
                | TokenKind::Any
                | TokenKind::PossibleAttributes
                | TokenKind::PossibleDecorator
        )
    }

    pub fn is_open_parenthesis(&self) -> bool {
        matches!(
            self,
            TokenKind::OpenParenthesis | TokenKind::OpenBracket | TokenKind::OpenBrace
        )
    }

    pub fn is_close_parenthesis(&self) -> bool {
        matches!(
            self,
            TokenKind::CloseParenthesis | TokenKind::CloseBracket | TokenKind::CloseBrace
        )
    }

    pub fn is_parenthesis(&self) -> bool {
        self.is_open_parenthesis() || self.is_close_parenthesis()
    }

    pub fn is_space(&self) -> bool {
        matches!(
            self,
            TokenKind::Newline | TokenKind::Whitespace | TokenKind::EOI | TokenKind::Blankline
        )
    }
}

impl From<SymbolKind> for TokenKind {
    fn from(value: SymbolKind) -> Self {
        match value {
            SymbolKind::Plain | SymbolKind::Backslash => TokenKind::Plain, // Backslash is incorrect, but will be corrected in iterator
            SymbolKind::Whitespace => TokenKind::Whitespace,
            SymbolKind::Newline => TokenKind::Newline,
            SymbolKind::EOI => TokenKind::EOI,
            SymbolKind::Hash => TokenKind::Hash(1),
            SymbolKind::Star => TokenKind::Star(1),
            SymbolKind::Minus => TokenKind::Minus(1),
            SymbolKind::Plus => TokenKind::Plus(1),
            SymbolKind::Underline => TokenKind::Underline(1),
            SymbolKind::Caret => TokenKind::Caret(1),
            SymbolKind::Tick => TokenKind::Tick(1),
            SymbolKind::Overline => TokenKind::Overline(1),
            SymbolKind::Pipe => TokenKind::Pipe(1),
            SymbolKind::Tilde => TokenKind::Tilde(1),
            SymbolKind::Quote => TokenKind::Quote(1),
            SymbolKind::Dollar => TokenKind::Dollar(1),
            SymbolKind::Colon => TokenKind::Colon(1),
            SymbolKind::OpenParenthesis => TokenKind::OpenParenthesis,
            SymbolKind::CloseParenthesis => TokenKind::CloseParenthesis,
            SymbolKind::OpenBracket => TokenKind::OpenBracket,
            SymbolKind::CloseBracket => TokenKind::CloseBracket,
            SymbolKind::OpenBrace => TokenKind::OpenBrace,
            SymbolKind::CloseBrace => TokenKind::CloseBrace,
        }
    }
}

impl From<(SymbolKind, usize)> for TokenKind {
    fn from(value: (SymbolKind, usize)) -> Self {
        let kind = value.0;
        let len = value.1;

        match kind {
            SymbolKind::Plain | SymbolKind::Backslash => TokenKind::Plain, // Backslash is incorrect, but will be corrected in iterator
            SymbolKind::Whitespace => TokenKind::Whitespace,
            SymbolKind::Newline => TokenKind::Newline,
            SymbolKind::EOI => TokenKind::EOI,
            SymbolKind::Hash => TokenKind::Hash(len),
            SymbolKind::Star => TokenKind::Star(len),
            SymbolKind::Minus => TokenKind::Minus(len),
            SymbolKind::Plus => TokenKind::Plus(len),
            SymbolKind::Underline => TokenKind::Underline(len),
            SymbolKind::Caret => TokenKind::Caret(len),
            SymbolKind::Tick => TokenKind::Tick(len),
            SymbolKind::Overline => TokenKind::Overline(len),
            SymbolKind::Pipe => TokenKind::Pipe(len),
            SymbolKind::Tilde => TokenKind::Tilde(len),
            SymbolKind::Quote => TokenKind::Quote(len),
            SymbolKind::Dollar => TokenKind::Dollar(len),
            SymbolKind::Colon => TokenKind::Colon(len),
            SymbolKind::OpenParenthesis => TokenKind::OpenParenthesis,
            SymbolKind::CloseParenthesis => TokenKind::CloseParenthesis,
            SymbolKind::OpenBracket => TokenKind::OpenBracket,
            SymbolKind::CloseBracket => TokenKind::CloseBracket,
            SymbolKind::OpenBrace => TokenKind::OpenBrace,
            SymbolKind::CloseBrace => TokenKind::CloseBrace,
        }
    }
}
