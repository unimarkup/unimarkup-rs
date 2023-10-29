use unimarkup_commons::scanner::{
    position::{Offset, Position},
    token::{implicit::ImplicitSubstitution, Token, TokenKind},
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
            InlineTokenKind::Bold
            | InlineTokenKind::Italic
            | InlineTokenKind::BoldItalic
            | InlineTokenKind::Highlight
            | InlineTokenKind::Underline
            | InlineTokenKind::Subscript
            | InlineTokenKind::UnderlineSubscript
            | InlineTokenKind::Math
            | InlineTokenKind::Verbatim
            | InlineTokenKind::Overline
            | InlineTokenKind::Superscript
            | InlineTokenKind::Quote
            | InlineTokenKind::Strikethrough
            | InlineTokenKind::NamedSubstitution
            | InlineTokenKind::OpenBrace
            | InlineTokenKind::OpenBracket
            | InlineTokenKind::OpenParenthesis
            | InlineTokenKind::CloseBrace
            | InlineTokenKind::CloseBracket
            | InlineTokenKind::CloseParenthesis
            | InlineTokenKind::Whitespace
            | InlineTokenKind::Newline
            | InlineTokenKind::EscapedNewline
            | InlineTokenKind::Eoi => self.kind.as_str(),
            InlineTokenKind::Comment { .. }
            | InlineTokenKind::ImplicitSubstitution(_)
            | InlineTokenKind::Any
            | InlineTokenKind::PossibleAttributes => panic!(
                "Tried to create &str from '{:?}', which has undefined &str representation.",
                self
            ),
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

    Eoi,

    Comment {
        implicit_close: bool,
    },
    ImplicitSubstitution(ImplicitSubstitution),

    // For matching
    Any,
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
            | InlineTokenKind::Any
            | InlineTokenKind::PossibleAttributes => panic!(
                "Tried to create &str from '{:?}', which has undefined &str representation.",
                self
            ),
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
            TokenKind::EOI => InlineTokenKind::Eoi,
            TokenKind::EscapedPlain => InlineTokenKind::EscapedPlain,
            TokenKind::EscapedWhitespace => InlineTokenKind::EscapedWhitespace,
            TokenKind::EscapedNewline => InlineTokenKind::EscapedNewline,
            TokenKind::Comment { implicit_close } => InlineTokenKind::Comment { implicit_close },
            TokenKind::ImplicitSubstitution(impl_subst) => {
                InlineTokenKind::ImplicitSubstitution(impl_subst)
            }

            TokenKind::Any => InlineTokenKind::Any,
            TokenKind::PossibleAttributes => InlineTokenKind::PossibleAttributes,
            TokenKind::Blankline => panic!("Blankline in inline content is not allowed."),

            TokenKind::Plain
            | TokenKind::Dot(_)
            | TokenKind::Hash(_)
            | TokenKind::Minus(_)
            | TokenKind::Plus(_)
            | TokenKind::PossibleDecorator
            | TokenKind::Punctuation => InlineTokenKind::Plain,
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
            InlineTokenKind::Eoi => TokenKind::EOI,
            InlineTokenKind::Comment { implicit_close } => TokenKind::Comment { implicit_close },
            InlineTokenKind::ImplicitSubstitution(impl_subst) => {
                TokenKind::ImplicitSubstitution(impl_subst)
            }
            InlineTokenKind::Any => TokenKind::Any,
            InlineTokenKind::PossibleAttributes => TokenKind::PossibleAttributes,
        }
    }
}
