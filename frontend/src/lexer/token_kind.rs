use crate::symbol::SymbolKind;

pub const COMMENT_TOKEN_LEN: usize = 2;

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
    Pipe(usize),
    Tilde(usize),
    Quote(usize),
    Dollar(usize),
    Colon(usize),
    Dot(usize),
    Ampersand(usize),
    Comma(usize),

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
    Eoi,
    Indentation(usize),

    // Escaped
    EscapedPlain,
    EscapedWhitespace,
    EscapedNewline,

    // Plain
    #[default]
    Plain,
    TerminalPunctuation,

    // Specials
    Comment {
        // Set to `true` if comment was implicitly closed at end of line
        implicit_close: bool,
    },
    // ImplicitSubstitution(ImplicitSubstitutionKind),
    DirectUri,

    // For matching
    Any,
    Space,
    EnclosedBlockEnd,
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
                | TokenKind::Eoi
                | TokenKind::EscapedPlain
                | TokenKind::EscapedWhitespace
                | TokenKind::EscapedNewline
                | TokenKind::Plain
                | TokenKind::TerminalPunctuation
                | TokenKind::Comment { .. }
                | TokenKind::DirectUri
                | TokenKind::Any
                | TokenKind::Space
                | TokenKind::EnclosedBlockEnd
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
            TokenKind::Newline | TokenKind::Whitespace | TokenKind::Eoi | TokenKind::Blankline
        )
    }

    pub fn is_plain(&self) -> bool {
        matches!(self, TokenKind::Plain | TokenKind::TerminalPunctuation)
    }
}

impl From<TokenKind> for String {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Star(len) => SymbolKind::Star.as_str().repeat(len),
            TokenKind::Hash(len) => SymbolKind::Hash.as_str().repeat(len),
            TokenKind::Minus(len) => SymbolKind::Minus.as_str().repeat(len),
            TokenKind::Plus(len) => SymbolKind::Plus.as_str().repeat(len),
            TokenKind::Underline(len) => SymbolKind::Underline.as_str().repeat(len),
            TokenKind::Caret(len) => SymbolKind::Caret.as_str().repeat(len),
            TokenKind::Tick(len) => SymbolKind::Tick.as_str().repeat(len),
            TokenKind::Pipe(len) => SymbolKind::Pipe.as_str().repeat(len),
            TokenKind::Tilde(len) => SymbolKind::Tilde.as_str().repeat(len),
            TokenKind::Quote(len) => SymbolKind::Quote.as_str().repeat(len),
            TokenKind::Dollar(len) => SymbolKind::Dollar.as_str().repeat(len),
            TokenKind::Colon(len) => SymbolKind::Colon.as_str().repeat(len),
            TokenKind::Dot(len) => SymbolKind::Dot.as_str().repeat(len),
            TokenKind::Ampersand(len) => SymbolKind::Ampersand.as_str().repeat(len),
            TokenKind::Comma(len) => SymbolKind::Comma.as_str().repeat(len),
            TokenKind::OpenParenthesis => {
                let mut s = String::with_capacity(SymbolKind::OpenParenthesis.as_str().len());
                s.push_str(SymbolKind::OpenParenthesis.as_str());
                s
            }
            TokenKind::CloseParenthesis => {
                let mut s = String::with_capacity(SymbolKind::CloseParenthesis.as_str().len());
                s.push_str(SymbolKind::CloseParenthesis.as_str());
                s
            }
            TokenKind::OpenBracket => {
                let mut s = String::with_capacity(SymbolKind::OpenBracket.as_str().len());
                s.push_str(SymbolKind::OpenBracket.as_str());
                s
            }
            TokenKind::CloseBracket => {
                let mut s = String::with_capacity(SymbolKind::CloseBracket.as_str().len());
                s.push_str(SymbolKind::CloseBracket.as_str());
                s
            }
            TokenKind::OpenBrace => {
                let mut s = String::with_capacity(SymbolKind::OpenBrace.as_str().len());
                s.push_str(SymbolKind::OpenBrace.as_str());
                s
            }
            TokenKind::CloseBrace => {
                let mut s = String::with_capacity(SymbolKind::CloseBrace.as_str().len());
                s.push_str(SymbolKind::CloseBrace.as_str());
                s
            }
            TokenKind::EscapedNewline | TokenKind::Newline | TokenKind::Blankline => {
                // Blankline is also only one newline to handle contiguous blanklines.
                let mut s = String::with_capacity(SymbolKind::Newline.as_str().len());
                s.push_str(SymbolKind::Newline.as_str());
                s
            }
            TokenKind::Whitespace => {
                let mut s = String::with_capacity(SymbolKind::Whitespace.as_str().len());
                s.push_str(SymbolKind::Whitespace.as_str());
                s
            }
            TokenKind::Plain
            | TokenKind::TerminalPunctuation
            | TokenKind::EscapedPlain
            | TokenKind::EscapedWhitespace
            // | TokenKind::ImplicitSubstitution(_)
            | TokenKind::Comment { .. }
            | TokenKind::DirectUri
            | TokenKind::PossibleAttributes
            | TokenKind::PossibleDecorator
            | TokenKind::Any
            | TokenKind::EnclosedBlockEnd
            | TokenKind::Space
            | TokenKind::Eoi => {
                #[cfg(debug_assertions)]
                panic!(
                    "Tried to create String from '{:?}', which has undefined String representation.",
                    value
                );

                #[cfg(not(debug_assertions))]
                String::new()
            }
            TokenKind::Indentation(indent) => " ".repeat(indent),
        }
    }
}

impl From<SymbolKind> for TokenKind {
    fn from(value: SymbolKind) -> Self {
        match value {
            SymbolKind::Plain | SymbolKind::Backslash => TokenKind::Plain, // Backslash is incorrect, but will be corrected in iterator
            SymbolKind::TerminalPunctuation => TokenKind::TerminalPunctuation,
            SymbolKind::Whitespace => TokenKind::Whitespace,
            SymbolKind::Newline => TokenKind::Newline,
            SymbolKind::Eoi => TokenKind::Eoi,
            SymbolKind::Hash => TokenKind::Hash(1),
            SymbolKind::Star => TokenKind::Star(1),
            SymbolKind::Minus => TokenKind::Minus(1),
            SymbolKind::Plus => TokenKind::Plus(1),
            SymbolKind::Underline => TokenKind::Underline(1),
            SymbolKind::Caret => TokenKind::Caret(1),
            SymbolKind::Tick => TokenKind::Tick(1),
            SymbolKind::Pipe => TokenKind::Pipe(1),
            SymbolKind::Tilde => TokenKind::Tilde(1),
            SymbolKind::Quote => TokenKind::Quote(1),
            SymbolKind::Dollar => TokenKind::Dollar(1),
            SymbolKind::Colon => TokenKind::Colon(1),
            SymbolKind::Dot => TokenKind::Colon(1),
            SymbolKind::Ampersand => TokenKind::Ampersand(1),
            SymbolKind::Comma => TokenKind::Comma(1),
            SymbolKind::OpenParenthesis => TokenKind::OpenParenthesis,
            SymbolKind::CloseParenthesis => TokenKind::CloseParenthesis,
            SymbolKind::OpenBracket => TokenKind::OpenBracket,
            SymbolKind::CloseBracket => TokenKind::CloseBracket,
            SymbolKind::OpenBrace => TokenKind::OpenBrace,
            SymbolKind::CloseBrace => TokenKind::CloseBrace,
            SymbolKind::Space => TokenKind::Indentation(1),
        }
    }
}

impl From<(SymbolKind, usize)> for TokenKind {
    fn from(value: (SymbolKind, usize)) -> Self {
        let kind = value.0;
        let len = value.1;

        match kind {
            SymbolKind::Plain | SymbolKind::Backslash => TokenKind::Plain, // Backslash is incorrect, but will be corrected in iterator
            SymbolKind::TerminalPunctuation => TokenKind::TerminalPunctuation,
            SymbolKind::Whitespace => TokenKind::Whitespace,
            SymbolKind::Newline => TokenKind::Newline,
            SymbolKind::Eoi => TokenKind::Eoi,
            SymbolKind::Hash => TokenKind::Hash(len),
            SymbolKind::Star => TokenKind::Star(len),
            SymbolKind::Minus => TokenKind::Minus(len),
            SymbolKind::Plus => TokenKind::Plus(len),
            SymbolKind::Underline => TokenKind::Underline(len),
            SymbolKind::Caret => TokenKind::Caret(len),
            SymbolKind::Tick => TokenKind::Tick(len),
            SymbolKind::Pipe => TokenKind::Pipe(len),
            SymbolKind::Tilde => TokenKind::Tilde(len),
            SymbolKind::Quote => TokenKind::Quote(len),
            SymbolKind::Dollar => TokenKind::Dollar(len),
            SymbolKind::Colon => TokenKind::Colon(len),
            SymbolKind::Dot => TokenKind::Dot(len),
            SymbolKind::Ampersand => TokenKind::Ampersand(len),
            SymbolKind::Comma => TokenKind::Comma(len),
            SymbolKind::OpenParenthesis => TokenKind::OpenParenthesis,
            SymbolKind::CloseParenthesis => TokenKind::CloseParenthesis,
            SymbolKind::OpenBracket => TokenKind::OpenBracket,
            SymbolKind::CloseBracket => TokenKind::CloseBracket,
            SymbolKind::OpenBrace => TokenKind::OpenBrace,
            SymbolKind::CloseBrace => TokenKind::CloseBrace,
            SymbolKind::Space => TokenKind::Indentation(1),
        }
    }
}
