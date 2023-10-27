use unimarkup_commons::scanner::{
    position::{Offset, Position},
    token::{Token, TokenKind},
};

/// Token lexed from Unimarkup text.
///
/// # Lifetimes
///
/// * `'input` - lifetime of input the [`Token`] was lexed from.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InlineToken<'input> {
    pub(crate) input: &'input str,
    pub(crate) offset: Offset,
    pub(crate) kind: InlineTokenKind,
    pub(crate) start: Position,
    pub(crate) end: Position,
}

impl<'input> From<&Token<'input>> for InlineToken<'input> {
    fn from(value: &Token<'input>) -> Self {
        InlineToken {
            input: value.input,
            offset: value.offset,
            kind: InlineTokenKind::from(value.kind),
            start: value.start,
            end: value.start,
        }
    }
}

impl<'input> InlineToken<'input> {
    pub fn as_str(&self) -> &str {
        match self.kind {
            InlineTokenKind::Plain
            | InlineTokenKind::EscapedPlain
            | InlineTokenKind::EscapedWhitespace => &self.input[self.offset.start..self.offset.end],
            _ => self.kind.as_str(),
        }
    }
}

/// The kind of the token found in Unimarkup document.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InlineTokenKind {
    /// Bold delimiter token (`**`).
    Bold,

    /// Italic delimiter token (`*`).
    Italic,

    /// Ambiguous token, might be bold, italic, or both (`***`).
    ItalicBold,

    /// Underline delimiter token (`__`);
    Underline,

    /// Subscript delimiter token (`_`);
    Subscript,

    /// Ambiguous token, might be underline, subscript, or both (`___`).
    UnderlineSubscript,

    /// Superscript delimiter token (`^`).
    Superscript,

    /// Overline delimiter token (`‾`).
    Overline,

    /// Strikethrough delimiter token (`~~`).
    Strikethrough,

    /// Highlight delimiter token (`||`).
    Highlight,

    /// Verbatim delimiter token (`` ` ``).
    Verbatim,

    /// Quotation delimiter token (`""`).
    Quote,

    /// Math delimiter token (`$`).
    Math,

    /// Open parenthesis token (`(`).
    OpenParenthesis,

    /// Close parenthesis token (`)`).
    CloseParenthesis,

    /// Open bracket token (`[`).
    OpenBracket,

    /// Close bracket token (`]`).
    CloseBracket,

    /// Open brace token (`{`).
    OpenBrace,

    /// Close brace token (`}`).
    CloseBrace,

    /// Double colon for substitution (`::`).
    NamedSubstitution,

    /// End of line - regular newline token ('\n').
    Newline,

    /// Escaped newline token (`\\n`).
    EscapedNewline,

    /// Any single whitespace token representing a Unicode code point with `WSpace=Y` or `WS`
    /// (e.g. SPACE U+0020: ` `).
    Whitespace,

    /// Escaped whitespace token (``\ ``).
    EscapedWhitespace,

    /// Simple textual token.
    #[default]
    Plain,

    /// Escaped textual token.
    EscapedPlain,

    EOI,

    Comment,

    // For matching
    Any,
    PossibleAttributes,
}

impl InlineTokenKind {
    /// Returns the textual representation of the kind.
    pub const fn as_str(&self) -> &'static str {
        match *self {
            InlineTokenKind::Bold => "**",
            InlineTokenKind::ItalicBold => "***",
            InlineTokenKind::Italic => "*",
            InlineTokenKind::Newline | InlineTokenKind::EscapedNewline => "\n",
            InlineTokenKind::Whitespace | InlineTokenKind::EscapedWhitespace => " ",
            InlineTokenKind::Underline => "__",
            InlineTokenKind::Subscript => "_",
            InlineTokenKind::Superscript => "^",
            InlineTokenKind::UnderlineSubscript => "___",
            InlineTokenKind::Highlight => "||",
            InlineTokenKind::Overline => "‾",
            InlineTokenKind::Strikethrough => "~~",
            InlineTokenKind::Verbatim => "`",
            InlineTokenKind::Quote => "\"\"",
            InlineTokenKind::Math => "$",
            InlineTokenKind::OpenParenthesis => "(",
            InlineTokenKind::CloseParenthesis => ")",
            InlineTokenKind::OpenBracket => "[",
            InlineTokenKind::CloseBracket => "]",
            InlineTokenKind::OpenBrace => "{",
            InlineTokenKind::CloseBrace => "}",
            InlineTokenKind::NamedSubstitution => "::",
            InlineTokenKind::Plain => "",
            _ => "",
        }
    }

    /// Returns the length of this particular [`TokenKind`] occupied in original Unimarkup text.
    ///
    /// [`TokenKind`]: self::TokenKind
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.as_str().len()
    }

    pub fn is_not_keyword(&self) -> bool {
        matches!(
            self,
            InlineTokenKind::Newline
                | InlineTokenKind::Whitespace
                | InlineTokenKind::Plain
                | InlineTokenKind::EOI
                | InlineTokenKind::Comment
                | InlineTokenKind::EscapedNewline
                | InlineTokenKind::EscapedWhitespace
                | InlineTokenKind::EscapedPlain
        )
    }

    pub fn is_keyword(&self) -> bool {
        !self.is_not_keyword()
    }

    pub fn is_open_parenthesis(&self) -> bool {
        matches!(
            self,
            InlineTokenKind::OpenParenthesis
                | InlineTokenKind::OpenBracket
                | InlineTokenKind::OpenBrace
        )
    }

    pub fn is_close_parenthesis(&self) -> bool {
        matches!(
            self,
            InlineTokenKind::CloseParenthesis
                | InlineTokenKind::CloseBracket
                | InlineTokenKind::CloseBrace
        )
    }

    pub fn is_parenthesis(&self) -> bool {
        self.is_open_parenthesis() || self.is_close_parenthesis()
    }

    pub fn is_space(&self) -> bool {
        matches!(self, InlineTokenKind::Newline | InlineTokenKind::Whitespace)
    }

    pub fn is_format_keyword(&self) -> bool {
        matches!(
            self,
            InlineTokenKind::Bold
                | InlineTokenKind::Italic
                | InlineTokenKind::ItalicBold
                | InlineTokenKind::Underline
                | InlineTokenKind::Subscript
                | InlineTokenKind::UnderlineSubscript
                | InlineTokenKind::Superscript
                | InlineTokenKind::Strikethrough
                | InlineTokenKind::Quote
                | InlineTokenKind::Verbatim
                | InlineTokenKind::Highlight
                | InlineTokenKind::Math
                | InlineTokenKind::NamedSubstitution
        )
    }

    pub fn is_scoped_format_keyword(&self) -> bool {
        matches!(
            self,
            InlineTokenKind::Verbatim | InlineTokenKind::Math | InlineTokenKind::NamedSubstitution
        )
    }
}

impl From<TokenKind> for InlineTokenKind {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Star(len) => {
                if len == 1 {
                    InlineTokenKind::Italic
                } else if len == 2 {
                    InlineTokenKind::Bold
                } else if len == 3 {
                    InlineTokenKind::ItalicBold
                } else {
                    InlineTokenKind::Plain
                }
            }

            TokenKind::Underline(len) => {
                if len == 1 {
                    InlineTokenKind::Subscript
                } else if len == 2 {
                    InlineTokenKind::Underline
                } else if len == 3 {
                    InlineTokenKind::UnderlineSubscript
                } else {
                    InlineTokenKind::Plain
                }
            }

            TokenKind::Pipe(len) => {
                if len == 2 {
                    InlineTokenKind::Highlight
                } else {
                    InlineTokenKind::Plain
                }
            }
            TokenKind::Tilde(len) => {
                if len == 2 {
                    InlineTokenKind::Strikethrough
                } else {
                    InlineTokenKind::Plain
                }
            }
            TokenKind::Quote(len) => {
                if len == 2 {
                    InlineTokenKind::Quote
                } else {
                    InlineTokenKind::Plain
                }
            }
            TokenKind::Dollar(len) => {
                if len == 2 {
                    InlineTokenKind::Math
                } else {
                    InlineTokenKind::Plain
                }
            }
            TokenKind::Colon(len) => {
                if len == 2 {
                    InlineTokenKind::NamedSubstitution
                } else {
                    InlineTokenKind::Plain
                }
            }

            TokenKind::Caret(len) => {
                if len == 1 {
                    InlineTokenKind::Superscript
                } else {
                    InlineTokenKind::Plain
                }
            }
            TokenKind::Tick(len) => {
                if len == 1 {
                    InlineTokenKind::Verbatim
                } else {
                    InlineTokenKind::Plain
                }
            }
            TokenKind::Overline(len) => {
                if len == 1 {
                    InlineTokenKind::Overline
                } else {
                    InlineTokenKind::Plain
                }
            }

            TokenKind::OpenParenthesis => InlineTokenKind::OpenParenthesis,
            TokenKind::CloseParenthesis => InlineTokenKind::CloseParenthesis,
            TokenKind::OpenBracket => InlineTokenKind::OpenBracket,
            TokenKind::CloseBracket => InlineTokenKind::CloseBracket,
            TokenKind::OpenBrace => InlineTokenKind::OpenBrace,
            TokenKind::CloseBrace => InlineTokenKind::CloseBrace,
            TokenKind::Whitespace => InlineTokenKind::Whitespace,
            TokenKind::Newline => InlineTokenKind::Newline,
            TokenKind::EOI => InlineTokenKind::EOI,
            TokenKind::EscapedPlain => InlineTokenKind::EscapedPlain,
            TokenKind::EscapedWhitespace => InlineTokenKind::EscapedWhitespace,
            TokenKind::EscapedNewline => InlineTokenKind::EscapedNewline,
            TokenKind::Comment => InlineTokenKind::Comment,
            TokenKind::Any => InlineTokenKind::Any,
            TokenKind::PossibleAttributes => InlineTokenKind::PossibleAttributes,
            TokenKind::Blankline => panic!("Blankline in inline content is not allowed."),

            _ => InlineTokenKind::Plain,
        }
    }
}
