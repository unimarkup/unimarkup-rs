use crate::lexer::symbol::SymbolKind;

use super::implicit::ImplicitSubstitutionKind;

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
    Overline(usize),
    Pipe(usize),
    Tilde(usize),
    Dollar(usize),
    Colon(usize),
    Dot(usize),
    ForwardSlash(usize),
    Percentage(usize),
    Ampersand(usize),
    Comma(usize),
    Semicolon(usize),
    ExclamationMark(usize),
    QuestionMark(usize),
    At(usize),
    Lt(usize),
    Gt(usize),
    Eq(usize),

    /// One ASCII digit
    Digit(u8),
    /// One double quote `"`.
    /// Combining multiple double quotes makes attribute and logic parsing harder.
    DoubleQuote,
    /// One single quote `'`.
    /// Combining multiple single quotes makes attribute and logic parsing harder.
    SingleQuote,

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

    // Escaped
    EscapedPlain,
    EscapedWhitespace,
    EscapedNewline,

    // Plain
    #[default]
    Plain,
    TerminalPunctuation,

    // Specials
    ImplicitSubstitution(ImplicitSubstitutionKind),

    // For matching
    Any,
    Space,
    EnclosedBlockEnd,
    PossibleAttributes,
    PossibleDecorator,
    /// Matches one or more contiguous [`TokenKind::Digit`].
    Digits,
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
                | TokenKind::ImplicitSubstitution(_)
                | TokenKind::Any
                | TokenKind::Space
                | TokenKind::EnclosedBlockEnd
                | TokenKind::PossibleAttributes
                | TokenKind::PossibleDecorator
                | TokenKind::Digits
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
            TokenKind::Overline(len) => SymbolKind::Overline.as_str().repeat(len),
            TokenKind::Pipe(len) => SymbolKind::Pipe.as_str().repeat(len),
            TokenKind::Tilde(len) => SymbolKind::Tilde.as_str().repeat(len),
            TokenKind::Dollar(len) => SymbolKind::Dollar.as_str().repeat(len),
            TokenKind::Colon(len) => SymbolKind::Colon.as_str().repeat(len),
            TokenKind::Dot(len) => SymbolKind::Dot.as_str().repeat(len),
            TokenKind::ForwardSlash(len) => SymbolKind::ForwardSlash.as_str().repeat(len),
            TokenKind::Percentage(len) => SymbolKind::Percentage.as_str().repeat(len),
            TokenKind::Comma(len) => SymbolKind::Comma.as_str().repeat(len),
            TokenKind::Semicolon(len) => SymbolKind::Semicolon.as_str().repeat(len),
            TokenKind::ExclamationMark(len) => SymbolKind::ExclamationMark.as_str().repeat(len),
            TokenKind::QuestionMark(len) => SymbolKind::QuestionMark.as_str().repeat(len),
            TokenKind::At(len) => SymbolKind::At.as_str().repeat(len),
            TokenKind::Lt(len) => SymbolKind::Lt.as_str().repeat(len),
            TokenKind::Gt(len) => SymbolKind::Gt.as_str().repeat(len),
            TokenKind::Eq(len) => SymbolKind::Eq.as_str().repeat(len),
            TokenKind::DoubleQuote => SymbolKind::DoubleQuote.as_str().to_string(),
            TokenKind::SingleQuote => SymbolKind::SingleQuote.as_str().to_string(),
            TokenKind::Digit(digit) => SymbolKind::Digit(digit).as_str().to_string(),
            TokenKind::Ampersand(len) => SymbolKind::Ampersand.as_str().repeat(len),
            TokenKind::OpenParenthesis => SymbolKind::OpenParenthesis.as_str().to_string(),
            TokenKind::CloseParenthesis => SymbolKind::CloseParenthesis.as_str().to_string(),
            TokenKind::OpenBracket => SymbolKind::OpenBracket.as_str().to_string(),
            TokenKind::CloseBracket => SymbolKind::CloseBracket.as_str().to_string(),
            TokenKind::OpenBrace => SymbolKind::OpenBrace.as_str().to_string(),
            TokenKind::CloseBrace => SymbolKind::CloseBrace.as_str().to_string(),
            TokenKind::EscapedNewline | TokenKind::Newline | TokenKind::Blankline => {
                SymbolKind::Newline.as_str().to_string()
            }
            TokenKind::Whitespace => SymbolKind::Whitespace.as_str().to_string(),
            TokenKind::Plain
            | TokenKind::TerminalPunctuation
            | TokenKind::EscapedPlain
            | TokenKind::EscapedWhitespace
            | TokenKind::ImplicitSubstitution(_)
            | TokenKind::PossibleAttributes
            | TokenKind::PossibleDecorator
            | TokenKind::Any
            | TokenKind::EnclosedBlockEnd
            | TokenKind::Digits
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
        }
    }
}

impl From<SymbolKind> for TokenKind {
    fn from(value: SymbolKind) -> Self {
        match value {
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
            SymbolKind::Dollar => TokenKind::Dollar(1),
            SymbolKind::Colon => TokenKind::Colon(1),
            SymbolKind::Dot => TokenKind::Colon(1),
            SymbolKind::ForwardSlash => TokenKind::ForwardSlash(1),
            SymbolKind::Percentage => TokenKind::Percentage(1),
            SymbolKind::Ampersand => TokenKind::Ampersand(1),
            SymbolKind::Comma => TokenKind::Comma(1),
            SymbolKind::Semicolon => TokenKind::Semicolon(1),
            SymbolKind::ExclamationMark => TokenKind::ExclamationMark(1),
            SymbolKind::QuestionMark => TokenKind::QuestionMark(1),
            SymbolKind::At => TokenKind::At(1),
            SymbolKind::Lt => TokenKind::Lt(1),
            SymbolKind::Gt => TokenKind::Gt(1),
            SymbolKind::Eq => TokenKind::Eq(1),
            SymbolKind::DoubleQuote => TokenKind::DoubleQuote,
            SymbolKind::SingleQuote => TokenKind::SingleQuote,
            SymbolKind::Plain | SymbolKind::Backslash => TokenKind::Plain, // Backslash is incorrect, but is corrected in `super::next_token()`
            SymbolKind::TerminalPunctuation => TokenKind::TerminalPunctuation,
            SymbolKind::Whitespace => TokenKind::Whitespace,
            SymbolKind::Newline => TokenKind::Newline,
            SymbolKind::Eoi => TokenKind::Eoi,
            SymbolKind::Digit(digit) => TokenKind::Digit(digit),
            SymbolKind::OpenParenthesis => TokenKind::OpenParenthesis,
            SymbolKind::CloseParenthesis => TokenKind::CloseParenthesis,
            SymbolKind::OpenBracket => TokenKind::OpenBracket,
            SymbolKind::CloseBracket => TokenKind::CloseBracket,
            SymbolKind::OpenBrace => TokenKind::OpenBrace,
            SymbolKind::CloseBrace => TokenKind::CloseBrace,
        }
    }
}

pub enum ConversionError {
    CannotMergeSymbol,
}

impl TryFrom<(SymbolKind, usize)> for TokenKind {
    type Error = ConversionError;

    fn try_from(value: (SymbolKind, usize)) -> Result<Self, Self::Error> {
        let kind = value.0;
        let len = value.1;

        let token = match kind {
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
            SymbolKind::Dollar => TokenKind::Dollar(len),
            SymbolKind::Colon => TokenKind::Colon(len),
            SymbolKind::Dot => TokenKind::Dot(len),
            SymbolKind::ForwardSlash => TokenKind::ForwardSlash(len),
            SymbolKind::Percentage => TokenKind::Percentage(len),
            SymbolKind::Comma => TokenKind::Comma(len),
            SymbolKind::Ampersand => TokenKind::Ampersand(len),
            SymbolKind::Semicolon => TokenKind::Semicolon(len),
            SymbolKind::ExclamationMark => TokenKind::ExclamationMark(len),
            SymbolKind::QuestionMark => TokenKind::QuestionMark(len),
            SymbolKind::At => TokenKind::At(len),
            SymbolKind::Lt => TokenKind::Lt(len),
            SymbolKind::Gt => TokenKind::Gt(len),
            SymbolKind::Eq => TokenKind::Eq(len),
            SymbolKind::Plain
            | SymbolKind::Backslash
            | SymbolKind::TerminalPunctuation
            | SymbolKind::Whitespace
            | SymbolKind::Newline
            | SymbolKind::Eoi
            | SymbolKind::OpenParenthesis
            | SymbolKind::CloseParenthesis
            | SymbolKind::OpenBracket
            | SymbolKind::CloseBracket
            | SymbolKind::OpenBrace
            | SymbolKind::CloseBrace
            | SymbolKind::SingleQuote
            | SymbolKind::DoubleQuote
            | SymbolKind::Digit(_) => {
                return Err(ConversionError::CannotMergeSymbol);
            }
        };

        Ok(token)
    }
}
