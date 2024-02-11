//! Contains the [`InlineTokenKind`] enum.

use unimarkup_commons::{
    comments::{Comment, COMMENT_TOKEN_KIND},
    lexer::token::{implicit::ImplicitSubstitutionKind, TokenKind},
};

/// The kind of the token found in Unimarkup document.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InlineTokenKind {
    /// Bold delimiter token (`**`).
    Bold,

    /// Italic delimiter token (`*`).
    Italic,

    /// Ambiguous token, might be bold, italic, or both (`***`).
    BoldItalic,

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

    /// Math delimiter token (`$`).
    Math,

    /// Single dot token (`.`)
    Dot,

    /// Citation delimiter token (`&&`)
    Cite,

    /// Single comma token (`,`)
    Comma,

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

    /// A single dot for *dot-notation* (e.g. `&&cite-id.title`).
    SingleDot,

    /// Double quote literal (`"`).
    DoubleQuote,

    /// Single quote literal (`'`).
    SingleQuote,

    /// An ASCII digit.
    Digit(u8),

    /// One or more `<`.
    Lt(usize),

    /// One or more `>`.
    Gt(usize),

    /// Possible start of a media insert (`[!!alt text](url)`).
    MediaInsert,

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

    /// End of input marker.
    Eoi,

    /// A Unimarkup comment.
    Comment,

    /// Implicit substitution (e.g. emojis and arrows)
    ImplicitSubstitution(ImplicitSubstitutionKind),

    // For matching
    Any,
    Space,
    Digits,
    PossibleAttributes,
}

impl InlineTokenKind {
    /// Returns the textual representation of the kind.
    pub fn as_str(&self) -> &'static str {
        match *self {
            InlineTokenKind::Bold => "**",
            InlineTokenKind::BoldItalic => "***",
            InlineTokenKind::Italic => "*",
            InlineTokenKind::Newline | InlineTokenKind::EscapedNewline => "\n",
            InlineTokenKind::Whitespace => " ",
            InlineTokenKind::Underline => "__",
            InlineTokenKind::Subscript => "_",
            InlineTokenKind::Superscript => "^",
            InlineTokenKind::UnderlineSubscript => "___",
            InlineTokenKind::Highlight => "||",
            InlineTokenKind::Overline => "‾",
            InlineTokenKind::Strikethrough => "~~",
            InlineTokenKind::Verbatim => "`",
            InlineTokenKind::Math => "$$",
            InlineTokenKind::Dot => ".",
            InlineTokenKind::Cite => "&&",
            InlineTokenKind::Comma => ",",
            InlineTokenKind::OpenParenthesis => "(",
            InlineTokenKind::CloseParenthesis => ")",
            InlineTokenKind::OpenBracket => "[",
            InlineTokenKind::CloseBracket => "]",
            InlineTokenKind::OpenBrace => "{",
            InlineTokenKind::CloseBrace => "}",
            InlineTokenKind::NamedSubstitution => "::",
            InlineTokenKind::Comment => Comment::keyword(),
            InlineTokenKind::SingleDot => ".",
            InlineTokenKind::DoubleQuote => "\"",
            InlineTokenKind::SingleQuote => "'",
            InlineTokenKind::Digit(digit) => match digit {
                0 => "0",
                1 => "1",
                2 => "2",
                3 => "3",
                4 => "4",
                5 => "5",
                6 => "6",
                7 => "7",
                8 => "8",
                9 => "9",
                _ => {
                    debug_assert!(false, "Tried to convert digit: '{}' to `&str`", digit);
                    ""
                }
            },
            InlineTokenKind::MediaInsert => "!!",
            InlineTokenKind::Eoi => "",
            InlineTokenKind::Plain
            | InlineTokenKind::Lt(_)
            | InlineTokenKind::Gt(_)
            | InlineTokenKind::EscapedPlain
            | InlineTokenKind::EscapedWhitespace
            | InlineTokenKind::ImplicitSubstitution(_)
            | InlineTokenKind::Any
            | InlineTokenKind::Space
            | InlineTokenKind::Digits
            | InlineTokenKind::PossibleAttributes => {
                #[cfg(debug_assertions)]
                panic!(
                    "Tried to create &str from '{:?}', which has undefined &str representation.",
                    self
                );

                #[cfg(not(debug_assertions))]
                ""
            }
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
                | InlineTokenKind::Eoi
                | InlineTokenKind::ImplicitSubstitution(_)
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
        matches!(
            self,
            InlineTokenKind::Newline | InlineTokenKind::Whitespace | InlineTokenKind::Eoi
        )
    }

    pub fn is_format_keyword(&self) -> bool {
        matches!(
            self,
            InlineTokenKind::Bold
                | InlineTokenKind::Italic
                | InlineTokenKind::BoldItalic
                | InlineTokenKind::Underline
                | InlineTokenKind::Subscript
                | InlineTokenKind::UnderlineSubscript
                | InlineTokenKind::Superscript
                | InlineTokenKind::Strikethrough
                | InlineTokenKind::DoubleQuote
                | InlineTokenKind::Overline
                | InlineTokenKind::Verbatim
                | InlineTokenKind::Highlight
                | InlineTokenKind::Math
                | InlineTokenKind::NamedSubstitution
        )
    }

    pub fn is_scoped_format_keyword(&self) -> bool {
        matches!(
            self,
            InlineTokenKind::Verbatim
                | InlineTokenKind::Math
                | InlineTokenKind::NamedSubstitution
                | InlineTokenKind::Comment
                | InlineTokenKind::Cite
        )
    }
}

pub const ITALIC_KEYWORD_LEN: usize = 1;
pub const BOLD_KEYWORD_LEN: usize = 2;
pub const BOLDITALIC_KEYWORD_LEN: usize = 3;
pub const SUBSCRIPT_KEYWORD_LEN: usize = 1;
pub const UNDERLINE_KEYWORD_LEN: usize = 2;
pub const UNDERLINESUBSCRIPT_KEYWORD_LEN: usize = 3;
pub const STRIKETHROUGH_KEYWORD_LEN: usize = 2;
pub const HIGHLIGHT_KEYWORD_LEN: usize = 2;
pub const MATH_KEYWORD_LEN: usize = 2;
pub const NAMED_SUBSTITUTION_KEYWORD_LEN: usize = 2;
pub const SUPERSCRIPT_KEYWORD_LEN: usize = 1;
pub const VERBATIM_KEYWORD_LEN: usize = 1;
pub const OVERLINE_KEYWORD_LEN: usize = 1;
pub const INLINE_INSERT_KEYWORD_LEN: usize = 2;
pub const DOT_KEYWORD_LEN: usize = 1;
pub const CITE_KEYWORD_LEN: usize = 2;
pub const COMMA_KEYWORD_LEN: usize = 1;

impl From<TokenKind> for InlineTokenKind {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Star(len) => {
                if len == ITALIC_KEYWORD_LEN {
                    InlineTokenKind::Italic
                } else if len == BOLD_KEYWORD_LEN {
                    InlineTokenKind::Bold
                } else if len == BOLDITALIC_KEYWORD_LEN {
                    InlineTokenKind::BoldItalic
                } else {
                    InlineTokenKind::Plain
                }
            }

            TokenKind::Underline(len) => {
                if len == SUBSCRIPT_KEYWORD_LEN {
                    InlineTokenKind::Subscript
                } else if len == UNDERLINE_KEYWORD_LEN {
                    InlineTokenKind::Underline
                } else if len == UNDERLINESUBSCRIPT_KEYWORD_LEN {
                    InlineTokenKind::UnderlineSubscript
                } else {
                    InlineTokenKind::Plain
                }
            }

            TokenKind::Pipe(len) => {
                if len == HIGHLIGHT_KEYWORD_LEN {
                    InlineTokenKind::Highlight
                } else {
                    InlineTokenKind::Plain
                }
            }
            TokenKind::Tilde(len) => {
                if len == STRIKETHROUGH_KEYWORD_LEN {
                    InlineTokenKind::Strikethrough
                } else {
                    InlineTokenKind::Plain
                }
            }
            TokenKind::Dollar(len) => {
                if len == MATH_KEYWORD_LEN {
                    InlineTokenKind::Math
                } else {
                    InlineTokenKind::Plain
                }
            }
            TokenKind::Colon(len) => {
                if len == NAMED_SUBSTITUTION_KEYWORD_LEN {
                    InlineTokenKind::NamedSubstitution
                } else {
                    InlineTokenKind::Plain
                }
            }

            TokenKind::Caret(len) => {
                if len == SUPERSCRIPT_KEYWORD_LEN {
                    InlineTokenKind::Superscript
                } else {
                    InlineTokenKind::Plain
                }
            }
            TokenKind::Tick(len) => {
                if len == VERBATIM_KEYWORD_LEN {
                    InlineTokenKind::Verbatim
                } else {
                    InlineTokenKind::Plain
                }
            }
            TokenKind::Overline(len) => {
                if len == OVERLINE_KEYWORD_LEN {
                    InlineTokenKind::Overline
                } else {
                    InlineTokenKind::Plain
                }
            }
            TokenKind::Semicolon(len) => {
                if len == Comment::keyword_len() {
                    InlineTokenKind::Comment
                } else {
                    InlineTokenKind::Plain
                }
            }
            TokenKind::Dot(len) => {
                if len == DOT_KEYWORD_LEN {
                    InlineTokenKind::Dot
                } else {
                    InlineTokenKind::Plain
                }
            }
            TokenKind::ExclamationMark(len) => {
                if len == INLINE_INSERT_KEYWORD_LEN {
                    InlineTokenKind::MediaInsert
                } else {
                    InlineTokenKind::Plain
                }
            }
            TokenKind::Ampersand(len) => {
                if len == CITE_KEYWORD_LEN {
                    InlineTokenKind::Cite
                } else {
                    InlineTokenKind::Plain
                }
            }
            TokenKind::Lt(len) => InlineTokenKind::Lt(len),
            TokenKind::Gt(len) => InlineTokenKind::Gt(len),
            TokenKind::DoubleQuote => InlineTokenKind::DoubleQuote,
            TokenKind::SingleQuote => InlineTokenKind::SingleQuote,
            TokenKind::Digit(digit) => InlineTokenKind::Digit(digit),
            TokenKind::Comma(len) => {
                if len == COMMA_KEYWORD_LEN {
                    InlineTokenKind::Comma
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
            TokenKind::Eoi | TokenKind::Blankline => InlineTokenKind::Eoi, // Blankline is not allowed in inlines => treat as inline end
            TokenKind::EscapedPlain => InlineTokenKind::EscapedPlain,
            TokenKind::EscapedWhitespace => InlineTokenKind::EscapedWhitespace,
            TokenKind::EscapedNewline => InlineTokenKind::EscapedNewline,
            TokenKind::ImplicitSubstitution(impl_subst) => {
                InlineTokenKind::ImplicitSubstitution(impl_subst)
            }
            TokenKind::Any => InlineTokenKind::Any,
            TokenKind::Space => InlineTokenKind::Space,
            TokenKind::Digits => InlineTokenKind::Digits,
            TokenKind::PossibleAttributes => InlineTokenKind::PossibleAttributes,

            TokenKind::Plain
            | TokenKind::Hash(_)
            | TokenKind::Minus(_)
            | TokenKind::Plus(_)
            | TokenKind::ForwardSlash(_)
            | TokenKind::Percentage(_)
            | TokenKind::QuestionMark(_)
            | TokenKind::At(_)
            | TokenKind::Eq(_)
            | TokenKind::EnclosedBlockEnd
            | TokenKind::PossibleDecorator
            | TokenKind::TerminalPunctuation => InlineTokenKind::Plain,
        }
    }
}

impl From<InlineTokenKind> for TokenKind {
    fn from(value: InlineTokenKind) -> Self {
        match value {
            InlineTokenKind::Bold => TokenKind::Star(BOLD_KEYWORD_LEN),
            InlineTokenKind::Italic => TokenKind::Star(ITALIC_KEYWORD_LEN),
            InlineTokenKind::BoldItalic => TokenKind::Star(BOLDITALIC_KEYWORD_LEN),
            InlineTokenKind::Underline => TokenKind::Underline(UNDERLINE_KEYWORD_LEN),
            InlineTokenKind::Subscript => TokenKind::Underline(SUBSCRIPT_KEYWORD_LEN),
            InlineTokenKind::UnderlineSubscript => {
                TokenKind::Underline(UNDERLINESUBSCRIPT_KEYWORD_LEN)
            }
            InlineTokenKind::Superscript => TokenKind::Caret(SUPERSCRIPT_KEYWORD_LEN),
            InlineTokenKind::Overline => TokenKind::Overline(OVERLINE_KEYWORD_LEN),
            InlineTokenKind::Strikethrough => TokenKind::Tilde(STRIKETHROUGH_KEYWORD_LEN),
            InlineTokenKind::Highlight => TokenKind::Pipe(HIGHLIGHT_KEYWORD_LEN),
            InlineTokenKind::Verbatim => TokenKind::Tick(VERBATIM_KEYWORD_LEN),
            InlineTokenKind::DoubleQuote => TokenKind::DoubleQuote,
            InlineTokenKind::SingleQuote => TokenKind::SingleQuote,
            InlineTokenKind::Math => TokenKind::Dollar(MATH_KEYWORD_LEN),
            InlineTokenKind::Dot => TokenKind::Dot(DOT_KEYWORD_LEN),
            InlineTokenKind::Cite => TokenKind::Ampersand(CITE_KEYWORD_LEN),
            InlineTokenKind::Comma => TokenKind::Comma(COMMA_KEYWORD_LEN),
            InlineTokenKind::OpenParenthesis => TokenKind::OpenParenthesis,
            InlineTokenKind::CloseParenthesis => TokenKind::CloseParenthesis,
            InlineTokenKind::OpenBracket => TokenKind::OpenBracket,
            InlineTokenKind::CloseBracket => TokenKind::CloseBracket,
            InlineTokenKind::OpenBrace => TokenKind::OpenBrace,
            InlineTokenKind::CloseBrace => TokenKind::CloseBrace,
            InlineTokenKind::NamedSubstitution => TokenKind::Colon(NAMED_SUBSTITUTION_KEYWORD_LEN),
            InlineTokenKind::Newline => TokenKind::Newline,
            InlineTokenKind::EscapedNewline => TokenKind::EscapedNewline,
            InlineTokenKind::Whitespace => TokenKind::Whitespace,
            InlineTokenKind::EscapedWhitespace => TokenKind::EscapedWhitespace,
            InlineTokenKind::Plain => TokenKind::Plain,
            InlineTokenKind::EscapedPlain => TokenKind::EscapedPlain,
            InlineTokenKind::Eoi => TokenKind::Eoi,
            InlineTokenKind::Comment => COMMENT_TOKEN_KIND,
            InlineTokenKind::ImplicitSubstitution(impl_subst) => {
                TokenKind::ImplicitSubstitution(impl_subst)
            }
            InlineTokenKind::SingleDot => TokenKind::Dot(1),
            InlineTokenKind::Digit(digit) => TokenKind::Digit(digit),
            InlineTokenKind::Lt(len) => TokenKind::Lt(len),
            InlineTokenKind::Gt(len) => TokenKind::Gt(len),
            InlineTokenKind::MediaInsert => TokenKind::ExclamationMark(INLINE_INSERT_KEYWORD_LEN),
            InlineTokenKind::Any => TokenKind::Any,
            InlineTokenKind::Space => TokenKind::Space,
            InlineTokenKind::PossibleAttributes => TokenKind::PossibleAttributes,
            InlineTokenKind::Digits => TokenKind::Digits,
        }
    }
}
