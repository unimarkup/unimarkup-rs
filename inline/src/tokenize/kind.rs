//! Contains the [`InlineTokenKind`] enum.

use unimarkup_commons::lexer::token::{implicit::ImplicitSubstitutionKind, TokenKind};

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

    /// End of input marker.
    Eoi,

    /// A Unimarkup comment.
    Comment {
        implicit_close: bool,
    },
    /// Implicit substitution (e.g. emojis and arrows)
    ImplicitSubstitution(ImplicitSubstitutionKind),
    /// Direct URI
    Directuri,

    // For matching
    Any,
    Space,
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
            InlineTokenKind::Quote => "\"\"",
            InlineTokenKind::Math => "$$",
            InlineTokenKind::OpenParenthesis => "(",
            InlineTokenKind::CloseParenthesis => ")",
            InlineTokenKind::OpenBracket => "[",
            InlineTokenKind::CloseBracket => "]",
            InlineTokenKind::OpenBrace => "{",
            InlineTokenKind::CloseBrace => "}",
            InlineTokenKind::NamedSubstitution => "::",
            InlineTokenKind::Eoi => "",
            InlineTokenKind::Plain
            | InlineTokenKind::EscapedPlain
            | InlineTokenKind::EscapedWhitespace
            | InlineTokenKind::Comment { .. }
            | InlineTokenKind::ImplicitSubstitution(_)
            | InlineTokenKind::Directuri
            | InlineTokenKind::Any
            | InlineTokenKind::Space
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
                | InlineTokenKind::Comment { .. }
                | InlineTokenKind::ImplicitSubstitution(_)
                | InlineTokenKind::Directuri
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
                | InlineTokenKind::Quote
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
            InlineTokenKind::Verbatim | InlineTokenKind::Math | InlineTokenKind::NamedSubstitution
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
pub const QUOTE_KEYWORD_LEN: usize = 2;
pub const MATH_KEYWORD_LEN: usize = 2;
pub const NAMED_SUBSTITUTION_KEYWORD_LEN: usize = 2;
pub const SUPERSCRIPT_KEYWORD_LEN: usize = 1;
pub const VERBATIM_KEYWORD_LEN: usize = 1;
pub const OVERLINE_KEYWORD_LEN: usize = 1;

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
            TokenKind::Quote(len) => {
                if len == QUOTE_KEYWORD_LEN {
                    InlineTokenKind::Quote
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
            TokenKind::Comment { implicit_close } => InlineTokenKind::Comment { implicit_close },
            TokenKind::ImplicitSubstitution(impl_subst) => {
                InlineTokenKind::ImplicitSubstitution(impl_subst)
            }
            TokenKind::DirectUri => InlineTokenKind::Directuri,

            TokenKind::Any => InlineTokenKind::Any,
            TokenKind::Space => InlineTokenKind::Space,
            TokenKind::PossibleAttributes => InlineTokenKind::PossibleAttributes,

            TokenKind::Plain
            | TokenKind::Dot(_)
            | TokenKind::Hash(_)
            | TokenKind::Minus(_)
            | TokenKind::Plus(_)
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
            InlineTokenKind::Quote => TokenKind::Quote(QUOTE_KEYWORD_LEN),
            InlineTokenKind::Math => TokenKind::Dollar(MATH_KEYWORD_LEN),
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
            InlineTokenKind::Comment { implicit_close } => TokenKind::Comment { implicit_close },
            InlineTokenKind::ImplicitSubstitution(impl_subst) => {
                TokenKind::ImplicitSubstitution(impl_subst)
            }
            InlineTokenKind::Directuri => TokenKind::DirectUri,
            InlineTokenKind::Any => TokenKind::Any,
            InlineTokenKind::Space => TokenKind::Space,
            InlineTokenKind::PossibleAttributes => TokenKind::PossibleAttributes,
        }
    }
}
