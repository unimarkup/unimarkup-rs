use std::ops::{Add, AddAssign, Sub, SubAssign};

use unimarkup_commons::scanner::position::Position as CommonsPos;

use super::resolver::Resolved;
use super::ContentOption;
use crate::{Inline, Symbol};

/// Token lexed from Unimarkup text.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: Span,
    pub(crate) spacing: Spacing,
    pub(crate) content: Option<String>,
}

impl Token {
    /// Creates a new [`Token`] with the given [`TokenKind`], [`Span`] that the [`Token`] occupies
    /// and [`Spacing`] that surrounds the [`Token`].
    ///
    /// [`Token`]: self::Token
    /// [`TokenKind`]: self::TokenKind
    /// [`Span`]: self::Span
    /// [`Spacing`]: self::Spacing
    pub fn new(kind: TokenKind, span: Span, spacing: Spacing) -> Self {
        Self {
            kind,
            span,
            spacing,
            content: None,
        }
    }

    /// Creates a new [`Token`] like [`Token::new`] with additional content this [`Token`]
    /// contains.
    ///
    /// [`Token`]: self::Token
    /// [`Token::new`]: self::Token::new
    pub fn with_content(
        kind: TokenKind,
        span: Span,
        spacing: Spacing,
        content: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            span,
            spacing,
            content: Some(content.into()),
        }
    }

    /// Creates a new [`Token`] like [`Token::new`] with content this [`Token`]
    /// contains, based on whether the content option is to store or discard the content.
    ///
    /// [`Token`]: self::Token
    /// [`Token::new`]: self::Token::new
    pub(crate) fn with_conditional_content(
        kind: TokenKind,
        span: Span,
        spacing: Spacing,
        content: impl Into<String>,
        content_option: ContentOption,
    ) -> Self {
        let content = match content_option {
            ContentOption::Store => Some(content.into()),
            ContentOption::Discard => None,
        };

        Self {
            kind,
            span,
            spacing,
            content,
        }
    }

    /// Returns the content of this [`Token`] as [`&str`].
    ///
    /// [`Token`]: self::Token
    /// [`&str`]: &str
    pub fn as_str(&self) -> &str {
        match self.content {
            Some(ref content) => content,
            None => self.kind.as_str(),
        }
    }

    /// Consumes this [`Token`] and returns it's content and the span it occupies.
    ///
    /// [`Token`]: self::Token
    pub fn into_inner(self) -> (String, Span) {
        let content = if let Some(text) = self.content {
            text
        } else {
            String::from(self.as_str())
        };

        (content, self.span)
    }

    /// Returns the [`TokenKind`] of this [`Token`].
    ///
    /// [`Token`]: self::Token
    /// [`TokenKind`]: self::TokenKind
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    /// Returns the [`Spacing`] of this [`Token`].
    ///
    /// [`Token`]: self::Token
    /// [`Spacing`]: self::Spacing
    pub fn spacing(&self) -> Spacing {
        self.spacing
    }

    /// Returns the [`Span`] that this [`Token`] occupies in original input.
    ///
    /// [`Token`]: self::Token
    /// [`Span`]: self::Span
    pub fn span(&self) -> Span {
        self.span
    }

    /// Updates the [`Span`] that this [`Token`] occupies in original input.
    ///
    /// [`Token`]: self::Token
    /// [`Span`]: self::Span
    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    /// Converts the this [`Token`] into a plain [`Token`] with [`TokenKind::Plain`].
    ///
    /// [`Token`]: self::Token
    /// [`TokenKind::Plain`]: self::TokenKind::Plain
    pub fn into_plain(self) -> Self {
        Self {
            kind: TokenKind::Plain,
            ..self
        }
    }

    /// Checks whether this [`Token`] starts a nestable format, i.e. Bold text.
    ///
    /// [`Token`]: self::Token
    pub fn is_nesting_token(&self) -> bool {
        !matches!(
            self.kind,
            TokenKind::Plain | TokenKind::Newline | TokenKind::Whitespace | TokenKind::EndOfLine
        )
    }

    /// Checks whether this [`Token`] is a starting/opening token of some Unimarkup inline format.
    ///
    /// [`Token`]: self::Token
    pub fn opens(&self) -> bool {
        match self.kind() {
            some_kind if some_kind.is_open_bracket() => true,
            _ => {
                let not_followed_by_whitespace =
                    matches!(self.spacing, Spacing::Pre | Spacing::None);

                !self.kind.is_close_bracket()
                    && self.is_nesting_token()
                    && not_followed_by_whitespace
            }
        }
    }

    /// Checks whether this [`Token`] is an ending/closing token of some Unimarkup inline format.
    ///
    /// [`Token`]: self::Token
    pub fn closes(&self) -> bool {
        match self.kind() {
            some_kind if some_kind.is_close_bracket() => true,
            _ => {
                let not_preceded_by_whitespace =
                    matches!(self.spacing, Spacing::Post | Spacing::None);

                !self.kind().is_open_bracket()
                    && self.is_nesting_token()
                    && not_preceded_by_whitespace
            }
        }
    }

    /// Checks whether this token is the same, or partially contains some other token.
    /// i.e. `***` contains both Bold (`**`) and Italic `**` tokens.
    pub fn is_or_contains(&self, other: &Self) -> bool {
        if self.kind() == other.kind() {
            true
        } else {
            match self.kind() {
                TokenKind::ItalicBold => {
                    matches!(other.kind(), TokenKind::Bold | TokenKind::Italic)
                }
                TokenKind::UnderlineSubscript => {
                    matches!(other.kind(), TokenKind::Underline | TokenKind::Subscript)
                }
                _ => false,
            }
        }
    }

    /// Checks whether the two [`Token`]s overlap. Two [`Token`]s overlap if any of the following
    /// is true:
    ///
    /// * have same [`TokenKind`]
    /// * `this` Token contains the other one (i.e. `ItalicBold` contains `Italic`)
    /// * `other` Token contains this Token (i.e. `Bold` is contained in `ItalicBold`)
    ///
    /// [`Token`]: self::Token
    /// [`TokenKind`]: self::Token
    pub fn overlaps(&self, other: &Self) -> bool {
        self.is_or_contains(other) || other.is_or_contains(self)
    }

    /// Checks whether this token is a matching pair of the other token.
    /// i.e. matching token for `(` is `)`.
    pub fn matches_pair(&self, other: &Self) -> bool {
        match self.kind() {
            TokenKind::OpenParens => matches!(other.kind(), TokenKind::CloseParens),
            TokenKind::CloseParens => matches!(other.kind(), TokenKind::OpenParens),
            TokenKind::OpenBracket => matches!(other.kind(), TokenKind::CloseBracket),
            TokenKind::CloseBracket => matches!(other.kind(), TokenKind::OpenBracket),
            TokenKind::OpenBrace => matches!(other.kind(), TokenKind::CloseBrace),
            TokenKind::CloseBrace => matches!(other.kind(), TokenKind::OpenBrace),
            _ => false,
        }
    }

    /// Checks whether the token is ambiguous - might be combination of multiple tokens.
    pub fn is_ambiguous(&self) -> bool {
        matches!(
            self.kind(),
            TokenKind::ItalicBold | TokenKind::UnderlineSubscript
        )
    }

    /// Removes partially the other_token from this token.
    ///
    /// # Panics
    ///
    /// Panics if any of the following invariants are not upheld:
    ///
    /// * This [`Token`] is ambiguous
    /// * The `other_token` is part of this [`Token`] (i.e. Bold (**) is part of ItalicBold (***))
    ///
    /// [`Token`]: self::Token
    pub fn remove_partial(&mut self, other_token: &Token) -> Self {
        let panic_message = "Can't remove partial token, tokens are not overlapping.";

        match self.kind() {
            TokenKind::ItalicBold => match other_token.kind() {
                TokenKind::Italic => self.kind = TokenKind::Bold,
                TokenKind::Bold => self.kind = TokenKind::Italic,
                _ => panic!("{panic_message}"),
            },
            TokenKind::UnderlineSubscript => match other_token.kind() {
                TokenKind::Underline => self.kind = TokenKind::Subscript,
                TokenKind::Subscript => self.kind = TokenKind::Underline,
                _ => panic!("{panic_message}"),
            },
            _ => panic!("{panic_message}"),
        };

        let (resulting_span, removed_span) = self.span.remove(other_token.span());

        self.span = resulting_span;

        Token {
            kind: other_token.kind,
            span: removed_span,
            spacing: other_token.spacing,
            content: None,
        }
    }

    /// Splits ambiguous token into two non-ambiguous [`Token`]s.
    ///
    /// # Panics
    ///
    /// Panics if this [`Token`] is not ambiguous.
    ///
    /// [`Token`]: self::Token
    pub fn split_ambiguous(self) -> (Self, Self) {
        if !self.is_ambiguous() {
            panic!("Cannot meaningfully split a Token that is not ambiguous.");
        } else {
            let (first_kind, second_kind) = match self.kind() {
                TokenKind::ItalicBold => (TokenKind::Italic, TokenKind::Bold),
                TokenKind::UnderlineSubscript => (TokenKind::Subscript, TokenKind::Underline),
                any_other_kind => (any_other_kind, any_other_kind),
            };

            let first_span = Span::from((
                self.span.start(),
                self.span.start() + (0, first_kind.len() - 1),
            ));

            let second_span = Span::from((
                first_span.end() + (0, 1),
                first_span.end() + (0, second_kind.len()),
            ));

            let first_spacing = self.spacing() - Spacing::Post;
            let second_spacing = self.spacing() - Spacing::Pre;

            let first = Self {
                kind: first_kind,
                span: first_span,
                spacing: first_spacing,
                content: None,
            };

            let second = Self {
                kind: second_kind,
                span: second_span,
                spacing: second_spacing,
                content: None,
            };

            (first, second)
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The kind of the token found in Unimarkup document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenKind {
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
    OpenParens,

    /// Close parenthesis token (`)`).
    CloseParens,

    /// Open bracket token (`[`).
    OpenBracket,

    /// Close bracket token (`]`).
    CloseBracket,

    /// Open brace token (`{`).
    OpenBrace,

    /// Close brace token (`}`).
    CloseBrace,

    /// Double colon for substitution (`::`).
    Substitution,

    /// Escaped newline token (`\\n`).
    Newline,

    /// End of line - regular newline token ('\n').
    EndOfLine,

    /// Escaped whitespace token (``\ ``).
    Whitespace,

    /// Simple textual token.
    Plain,
}

impl TokenKind {
    /// Returns the textual representation of the kind.
    pub fn as_str(&self) -> &str {
        match *self {
            TokenKind::Bold => "**",
            TokenKind::ItalicBold => "***",
            TokenKind::Italic => "*",
            TokenKind::Newline | TokenKind::EndOfLine => "\n",
            TokenKind::Whitespace => " ",
            TokenKind::Underline => "__",
            TokenKind::Subscript => "_",
            TokenKind::Superscript => "^",
            TokenKind::UnderlineSubscript => "___",
            TokenKind::Highlight => "||",
            TokenKind::Overline => "‾",
            TokenKind::Strikethrough => "~~",
            TokenKind::Verbatim => "`",
            TokenKind::Quote => "\"\"",
            TokenKind::Math => "$",
            TokenKind::OpenParens => "(",
            TokenKind::CloseParens => ")",
            TokenKind::OpenBracket => "[",
            TokenKind::CloseBracket => "]",
            TokenKind::OpenBrace => "{",
            TokenKind::CloseBrace => "}",
            TokenKind::Substitution => "::",
            TokenKind::Plain => "",
        }
    }

    /// Returns the length of this particular [`TokenKind`] occupied in original Unimarkup text.
    ///
    /// [`TokenKind`]: self::TokenKind
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.as_str().len()
    }

    /// Returns the pair of delimiters for this kind.
    pub fn delimiters(&self) -> TokenDelimiters {
        TokenDelimiters::from(self)
    }

    /// Checks whether the content of this token is significant - should be stored.
    pub(crate) fn content_matters(&self) -> bool {
        matches!(self, TokenKind::Plain)
    }

    /// Returns the [`Content`] for this kind.
    ///
    /// [`Content`]: crate::Content
    pub(crate) fn content_option(&self) -> ContentOption {
        if self.content_matters() {
            ContentOption::Store
        } else {
            ContentOption::Discard
        }
    }

    /// Checks if this is some kind of open parenthesis (`(`, `[` or `{`).
    pub(crate) fn is_open_bracket(&self) -> bool {
        matches!(self, Self::OpenParens | Self::OpenBracket | Self::OpenBrace)
    }

    /// Checks if this is some kind of close parenthesis (`)`, `]` or `]`).
    pub(crate) fn is_close_bracket(&self) -> bool {
        matches!(
            self,
            Self::CloseParens | Self::CloseBracket | Self::CloseBrace
        )
    }

    pub(crate) fn get_ambiguous_variant(&self) -> Option<Self> {
        match self {
            TokenKind::Bold | TokenKind::Italic => Some(Self::ItalicBold),
            TokenKind::Underline | TokenKind::Subscript => Some(Self::UnderlineSubscript),
            _ => None,
        }
    }

    pub(crate) fn is_parenthesis(&self) -> bool {
        self.is_open_bracket() || self.is_close_bracket()
    }
}

impl From<&Inline> for TokenKind {
    fn from(inline: &Inline) -> Self {
        match inline {
            Inline::Bold(_) => Self::Bold,
            Inline::Italic(_) => Self::Italic,
            Inline::Underline(_) => Self::Underline,
            Inline::Subscript(_) => Self::Subscript,
            Inline::Superscript(_) => Self::Superscript,
            Inline::Overline(_) => Self::Overline,
            Inline::Strikethrough(_) => Self::Strikethrough,
            Inline::Highlight(_) => Self::Highlight,
            Inline::Verbatim(_) => Self::Verbatim,
            Inline::Quote(_) => Self::Quote,
            Inline::Math(_) => Self::Math,
            Inline::Parentheses(_) => Self::OpenParens,
            Inline::TextGroup(_) => Self::OpenBracket,
            Inline::Attributes(_) => Self::OpenBrace,
            Inline::Newline(_) => Self::Newline,
            Inline::Whitespace(_) => Self::Whitespace,
            Inline::EndOfLine(_) => Self::EndOfLine,
            Inline::Plain(_) => Self::Plain,
            Inline::Multiple(_) => Self::Plain,
            Inline::Substitution(_) => Self::Substitution,
        }
    }
}

impl From<(Symbol<'_>, usize)> for TokenKind {
    fn from((symbol, len): (Symbol, usize)) -> Self {
        match len {
            1 => match symbol {
                Symbol::Star => Self::Italic,
                Symbol::Underline => Self::Subscript,
                Symbol::Caret => Self::Superscript,
                Symbol::Tick => Self::Verbatim,
                Symbol::Overline => Self::Overline,
                Symbol::Dollar => Self::Math,
                Symbol::OpenParenthesis => Self::OpenParens,
                Symbol::CloseParenthesis => Self::CloseParens,
                Symbol::OpenBracket => Self::OpenBracket,
                Symbol::CloseBracket => Self::CloseBracket,
                Symbol::OpenBrace => Self::OpenBrace,
                Symbol::CloseBrace => Self::CloseBrace,
                _ => Self::Plain,
            },
            2 => match symbol {
                Symbol::Star => Self::Bold,
                Symbol::Underline => Self::Underline,
                Symbol::Pipe => Self::Highlight,
                Symbol::Tilde => Self::Strikethrough,
                Symbol::Quote => Self::Quote,
                Symbol::Colon => Self::Substitution,
                _ => Self::Plain,
            },
            3 => match symbol {
                Symbol::Star => Self::ItalicBold,
                Symbol::Underline => Self::UnderlineSubscript,
                _ => Self::Plain,
            },
            _ => Self::Plain,
        }
    }
}

impl From<TokenDelimiters> for (TokenKind, Option<TokenKind>) {
    fn from(delimiters: TokenDelimiters) -> Self {
        (delimiters.open, delimiters.close)
    }
}

/// Delimiters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenDelimiters {
    open: TokenKind,
    close: Option<TokenKind>,
}

impl From<&TokenKind> for TokenDelimiters {
    fn from(kind: &TokenKind) -> Self {
        match kind {
            TokenKind::Bold
            | TokenKind::Italic
            | TokenKind::ItalicBold
            | TokenKind::Underline
            | TokenKind::Subscript
            | TokenKind::UnderlineSubscript
            | TokenKind::Superscript
            | TokenKind::Overline
            | TokenKind::Strikethrough
            | TokenKind::Highlight
            | TokenKind::Verbatim
            | TokenKind::Quote
            | TokenKind::Substitution
            | TokenKind::Math => Self {
                open: *kind,
                close: Some(*kind),
            },

            TokenKind::OpenParens | TokenKind::CloseParens => Self {
                open: TokenKind::OpenParens,
                close: Some(TokenKind::CloseParens),
            },
            TokenKind::OpenBracket | TokenKind::CloseBracket => Self {
                open: TokenKind::OpenBracket,
                close: Some(TokenKind::CloseBracket),
            },
            TokenKind::OpenBrace | TokenKind::CloseBrace => Self {
                open: TokenKind::OpenBrace,
                close: Some(TokenKind::CloseBrace),
            },
            TokenKind::Newline
            | TokenKind::EndOfLine
            | TokenKind::Whitespace
            | TokenKind::Plain => Self {
                open: TokenKind::Plain,
                close: Some(TokenKind::Plain),
            },
        }
    }
}

impl From<&Token> for TokenDelimiters {
    fn from(token: &Token) -> Self {
        Self::from(&token.kind)
    }
}

impl TokenDelimiters {
    /// Returns the [`&str`] representation of opening and, if available, closing delimiter.
    pub fn as_str(&self) -> (&str, Option<&str>) {
        (
            self.open.as_str(),
            self.close.as_ref().map(TokenKind::as_str),
        )
    }

    /// Returns the opening [`TokenKind`]
    ///
    /// [`TokenKind`]: self::TokenKind
    pub fn open(&self) -> TokenKind {
        self.open
    }

    /// Returns the opening [`TokenKind`] if available.
    ///
    /// [`TokenKind`]: self::TokenKind
    pub fn close(&self) -> Option<TokenKind> {
        self.close
    }
}

/// Enum representing the spacing surrounding a particular token in Unimarkup document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Spacing {
    /// Whitespace before the token.
    Pre,

    /// Whitespace after the token.
    Post,

    /// Whitespace both before and after the token.
    Both,

    /// Whitespace neither before nor after the token.
    None,
}

impl From<Resolved> for Spacing {
    fn from(resolved: Resolved) -> Self {
        match resolved {
            Resolved::Open => Spacing::Pre,
            Resolved::Close => Spacing::Post,
            Resolved::Neither => Spacing::Both,
        }
    }
}

impl Default for Spacing {
    fn default() -> Self {
        Self::None
    }
}

impl AddAssign for Spacing {
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Spacing::Both => {}
            Spacing::None => *self = rhs,
            Spacing::Pre => match rhs {
                Spacing::Post | Spacing::Both => *self = Spacing::Both,
                _ => {}
            },
            Spacing::Post => match rhs {
                Spacing::Pre | Spacing::Both => *self = Spacing::Both,
                _ => {}
            },
        };
    }
}

impl SubAssign for Spacing {
    fn sub_assign(&mut self, rhs: Self) {
        match self {
            Spacing::Both => match rhs {
                Spacing::Pre => *self = Spacing::Post,
                Spacing::Post => *self = Spacing::Pre,
                Spacing::Both => *self = Spacing::None,
                Spacing::None => {}
            },
            Spacing::Pre => match rhs {
                Spacing::Pre | Spacing::Both => *self = Spacing::None,
                _ => {}
            },
            Spacing::Post => match rhs {
                Spacing::Post | Spacing::Both => *self = Spacing::None,
                _ => {}
            },
            Spacing::None => {}
        };
    }
}

impl Add for Spacing {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl Sub for Spacing {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

/// Span used to store information about the space some [`Token`] occupies in Unimarkup document.
///
/// [`Token`]: self::Token
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    pub(crate) start: Position,
    pub(crate) end: Position,
}

// TODO: remove this once we switch to `unimarkup_commons::scanner::position::Position` everywhere.
impl From<CommonsPos> for Position {
    fn from(value: CommonsPos) -> Self {
        Self {
            line: value.line,
            column: value.col_grapheme,
        }
    }
}

impl Span {
    /// Returns the start position of this span.
    pub fn start(&self) -> Position {
        self.start
    }

    /// Returns the end position of this span.
    pub fn end(&self) -> Position {
        self.end
    }

    /// Returns the difference between end and start [`Position`] of this [`Span`].
    ///
    /// # Panics
    ///
    /// * if [`Span`] occupies multiple lines in original document. Length cannot be approximated
    ///   in such case, since it is unknown how long each of the lines was.
    ///
    /// [`Position`]: self::Position
    /// [`Span`]: self::Span
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        if self.start.line != self.end.line {
            panic!("Length cannot be approximated for Spans over multiple lines.");
        }

        self.end.column - self.start.column
    }

    fn overlaps(&self, other: Span) -> bool {
        (self.start >= other.start && self.start <= other.end)
            || (self.end >= other.start && self.end <= other.end)
    }

    /// Removes the `other` [`Span`] from `self`. In case the spans do not overlap, the `other`
    /// span will be laid over `self` in following manner:
    ///
    /// 1. `other` starts at the beginning of `self` if it originally comes before the `self` span.
    /// 2. `other` ends at the end of `self` if it originally comes after the `self` span.
    ///
    /// In both cases, the lenght of the `other` span will not be changed.
    ///
    /// # Returns
    ///
    /// Tuple containing the resulting span and the removed span.
    ///
    /// [`Span`]: self::Span
    fn remove(self, other: Span) -> (Span, Span) {
        let other = if self.overlaps(other) {
            other
        } else if other.end < self.start {
            let start = self.start;
            let end = start + (0, other.len());

            Span::from((start, end))
        } else {
            // !self.overlaps implies that in this case other.start > self.end
            let end = self.end;
            let start = end - (0, other.len());

            Span::from((start, end))
        };

        let start = if self.start < other.start {
            self.start
        } else {
            other.end + (0, 1)
        };

        let end = if self.end > other.end {
            self.end
        } else {
            other.start - (0, 1)
        };

        let removed_span = other;
        let resulting_span = Span::from((start, end));

        (resulting_span, removed_span)
    }

    pub(crate) fn swapped(&self, other: &Self) -> (Self, Self) {
        let (mut first, mut second) = if self.start.column < other.start.column {
            (*self, *other)
        } else {
            (*other, *self)
        };

        let first_len = first.len();
        let second_len = second.len();

        first.end.column = first.start.column + second_len;
        second.start.column = first.end.column + 1;
        second.end.column = second.start.column + first_len;

        (second, first)
    }
}

impl From<(Position, Position)> for Span {
    fn from((start, end): (Position, Position)) -> Self {
        Self { start, end }
    }
}

/// Representation of a position in Unimarkup input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    /// Represents the line in Unimarkup input.
    pub line: usize,

    /// Represents the column in Unimarkup input.
    pub column: usize,
}

impl Position {
    /// Creates a new position with given line and column.
    pub fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.line += rhs.line;
        self.column += rhs.column;
    }
}

impl AddAssign<(usize, usize)> for Position {
    fn add_assign(&mut self, (line, column): (usize, usize)) {
        self.line += line;
        self.column += column;
    }
}

impl<T> Add<T> for Position
where
    Position: AddAssign<T>,
{
    type Output = Position;

    fn add(mut self, rhs: T) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.line -= rhs.line;
        self.column -= rhs.column;
    }
}

impl SubAssign<(usize, usize)> for Position {
    fn sub_assign(&mut self, (line, column): (usize, usize)) {
        self.line -= line;
        self.column -= column;
    }
}

impl<T> Sub<T> for Position
where
    Position: SubAssign<T>,
{
    type Output = Position;

    fn sub(mut self, rhs: T) -> Self::Output {
        self -= rhs;
        self
    }
}
