use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{Inline, Symbol};

use super::Content;

pub(crate) struct Invalid;
pub(crate) struct Valid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TokenBuilder<K = Invalid, S = Invalid, W = Invalid> {
    kind: TokenKind,
    span: Span,
    spacing: Spacing,
    content: Option<String>,
    _validation: (PhantomData<K>, PhantomData<S>, PhantomData<W>),
}

impl TokenBuilder<Invalid, Invalid, Invalid> {
    pub fn new(kind: TokenKind) -> TokenBuilder<Valid, Invalid, Invalid> {
        let v1: PhantomData<Valid> = PhantomData;
        let v2: PhantomData<Invalid> = PhantomData;
        let v3: PhantomData<Invalid> = PhantomData;

        TokenBuilder {
            kind,
            span: Span::default(),
            spacing: Spacing::None,
            content: None,
            _validation: (v1, v2, v3),
        }
    }
}

impl<K, S, W> TokenBuilder<K, S, W> {
    pub fn with_content(mut self, content: String) -> TokenBuilder<K, S, W> {
        self.content = Some(content);
        self
    }

    pub fn optional_content(
        self,
        content: &[&str],
        content_option: Content,
    ) -> TokenBuilder<K, S, W> {
        match content_option {
            Content::Store => self.with_content(content.concat()),
            _ => self,
        }
    }

    pub fn span(self, span: Span) -> TokenBuilder<K, Valid, W> {
        let span_valid: PhantomData<Valid> = PhantomData;

        TokenBuilder {
            kind: self.kind,
            span,
            spacing: self.spacing,
            content: self.content,
            _validation: (self._validation.0, span_valid, self._validation.2),
        }
    }

    pub fn space(self, spacing: Spacing) -> TokenBuilder<K, S, Valid> {
        let spacing_valid: PhantomData<Valid> = PhantomData;

        TokenBuilder {
            kind: self.kind,
            span: self.span,
            spacing,
            content: self.content,
            _validation: (self._validation.0, self._validation.1, spacing_valid),
        }
    }
}

impl TokenBuilder<Valid, Valid, Valid> {
    pub fn build(self) -> Token {
        Token {
            kind: self.kind,
            span: self.span,
            spacing: self.spacing,
            content: self.content,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    span: Span,
    spacing: Spacing,
    content: Option<String>,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, spacing: Spacing) -> Self {
        Self {
            kind,
            span,
            spacing,
            content: None,
        }
    }

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

    pub fn as_str(&self) -> &str {
        match self.content {
            Some(ref content) => content,
            None => self.kind.as_str(),
        }
    }

    pub fn into_inner(self) -> (String, Span) {
        let content = if let Some(text) = self.content {
            text
        } else {
            String::default()
        };

        (content, self.span)
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn spacing(&self) -> Spacing {
        self.spacing
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    pub fn into_plain(self) -> Self {
        Self {
            kind: TokenKind::Plain,
            ..self
        }
    }

    pub fn is_nesting_token(&self) -> bool {
        !matches!(
            self.kind,
            TokenKind::Plain | TokenKind::Newline | TokenKind::Whitespace
        )
    }

    pub fn opens(&self) -> bool {
        if self.kind().is_open_parentheses() {
            true
        } else {
            let not_followed_by_whitespace = matches!(self.spacing, Spacing::Pre | Spacing::None);

            self.is_nesting_token() && not_followed_by_whitespace
        }
    }

    pub fn closes(&self) -> bool {
        if self.kind().is_close_parentheses() {
            true
        } else {
            let not_preceded_by_whitespace = matches!(self.spacing, Spacing::Post | Spacing::None);

            self.is_nesting_token() && not_preceded_by_whitespace
        }
    }

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

    pub fn is_ambiguous(&self) -> bool {
        matches!(
            self.kind(),
            TokenKind::ItalicBold | TokenKind::UnderlineSubscript
        )
    }

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

        TokenBuilder::new(other_token.kind())
            .span(removed_span)
            .space(other_token.spacing())
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Bold,
    Italic,
    ItalicBold,
    Underline,
    Subscript,
    UnderlineSubscript,
    Superscript,
    Overline,
    Strikethrough,
    Highlight,
    Verbatim,
    Quote,
    Math,
    OpenParens,
    CloseParens,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Newline,
    Whitespace,
    Plain,
}

impl TokenKind {
    pub fn as_str(&self) -> &str {
        match *self {
            TokenKind::Bold => "**",
            TokenKind::ItalicBold => "***",
            TokenKind::Italic => "*",
            TokenKind::Newline => "\n",
            TokenKind::Whitespace => " ",
            TokenKind::Underline => "__",
            TokenKind::Subscript => "_",
            TokenKind::Superscript => "^",
            TokenKind::UnderlineSubscript => "___",
            TokenKind::Highlight => "||",
            TokenKind::Overline => "‾",
            TokenKind::Strikethrough => "~~",
            TokenKind::Verbatim => "`",
            TokenKind::Quote => "\"",
            TokenKind::Math => "$",
            TokenKind::OpenParens => "(",
            TokenKind::CloseParens => ")",
            TokenKind::OpenBracket => "[",
            TokenKind::CloseBracket => "]",
            TokenKind::OpenBrace => "{",
            TokenKind::CloseBrace => "}",
            TokenKind::Plain => "",
        }
    }

    pub fn delimiters(&self) -> (&str, &str) {
        match self {
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
            | TokenKind::Math => (self.as_str(), self.as_str()),

            TokenKind::Newline | TokenKind::Whitespace | TokenKind::Plain => ("", ""),

            TokenKind::OpenParens | TokenKind::CloseParens => {
                (Self::OpenParens.as_str(), Self::CloseParens.as_str())
            }
            TokenKind::OpenBracket | TokenKind::CloseBracket => {
                (Self::OpenBracket.as_str(), Self::CloseBracket.as_str())
            }
            TokenKind::OpenBrace | TokenKind::CloseBrace => {
                (Self::OpenBrace.as_str(), Self::CloseBrace.as_str())
            }
        }
    }

    pub(crate) fn content_matters(&self) -> bool {
        matches!(self, TokenKind::Plain)
    }

    pub(crate) fn content_option(&self) -> Content {
        if self.content_matters() {
            Content::Store
        } else {
            Content::Auto
        }
    }

    pub(crate) fn is_open_parentheses(&self) -> bool {
        matches!(self, Self::OpenParens | Self::OpenBracket | Self::OpenBrace)
    }

    pub(crate) fn is_close_parentheses(&self) -> bool {
        matches!(
            self,
            Self::CloseParens | Self::CloseBracket | Self::CloseBrace
        )
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
            Inline::Parens(_) => Self::OpenParens,
            Inline::TextGroup(_) => Self::OpenBracket,
            Inline::Attributes(_) => Self::OpenBrace,
            Inline::Newline(_) => Self::Newline,
            Inline::Whitespace(_) => Self::Whitespace,
            Inline::Plain(_) => Self::Plain,
            Inline::Multiple(_) => Self::Plain,
        }
    }
}

impl From<(Symbol, usize)> for TokenKind {
    fn from((symbol, len): (Symbol, usize)) -> Self {
        match len {
            1 => match symbol {
                Symbol::Star => Self::Italic,
                Symbol::Underline => Self::Subscript,
                Symbol::Caret => Self::Superscript,
                Symbol::Tick => Self::Verbatim,
                Symbol::Overline => Self::Overline,
                Symbol::Dollar => Self::Math,
                Symbol::OpenParens => Self::OpenParens,
                Symbol::CloseParens => Self::CloseParens,
                Symbol::OpenBracket => Self::OpenBracket,
                Symbol::CloseBracket => Self::CloseBracket,
                Symbol::OpenBrace => Self::OpenBrace,
                Symbol::CloseBrace => Self::CloseBrace,
                Symbol::Esc | Symbol::Pipe | Symbol::Tilde | Symbol::Quote | Symbol::Plain => {
                    Self::Plain
                }
            },
            2 => match symbol {
                Symbol::Star => Self::Bold,
                Symbol::Underline => Self::Underline,
                Symbol::Pipe => Self::Highlight,
                Symbol::Tilde => Self::Strikethrough,
                Symbol::Quote => Self::Quote,
                Symbol::Esc
                | Symbol::Caret
                | Symbol::Tick
                | Symbol::Overline
                | Symbol::Dollar
                | Symbol::OpenParens
                | Symbol::CloseParens
                | Symbol::OpenBracket
                | Symbol::CloseBracket
                | Symbol::OpenBrace
                | Symbol::CloseBrace
                | Symbol::Plain => Self::Plain,
            },
            3 => match symbol {
                Symbol::Star => Self::ItalicBold,
                Symbol::Underline => Self::UnderlineSubscript,
                Symbol::Esc
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
                | Symbol::Plain => Self::Plain,
            },
            _ => Self::Plain,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spacing {
    Pre,
    Post,
    Both,
    None,
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
            Spacing::Both => *self = Spacing::None,
            Spacing::None => {}
            Spacing::Pre => match rhs {
                Spacing::Pre | Spacing::Both => *self = Spacing::None,
                _ => {}
            },
            Spacing::Post => match rhs {
                Spacing::Post | Spacing::Both => *self = Spacing::None,
                _ => {}
            },
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    start: Position,
    end: Position,
}

impl Span {
    pub fn start(&self) -> Position {
        self.start
    }

    pub fn end(&self) -> Position {
        self.end
    }

    /// Returns the difference between end and start [`Position`] of this [`Span`].
    ///
    /// [`Position`]: crate::Position
    /// [`Span`]: crate::Span
    pub fn len(&self) -> Position {
        self.end - self.start
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
    /// Tuple containing the resulting span and the removed span.
    fn remove(self, other: Span) -> (Span, Span) {
        let other = if self.overlaps(other) {
            other
        } else if other.end < self.start {
            let start = self.start;
            let end = start + other.len();

            Span::from((start, end))
        } else {
            // !self.overlaps implies that in this case other.start > self.end
            let end = self.end;
            let start = end - other.len();

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
}

impl From<(Position, Position)> for Span {
    fn from((start, end): (Position, Position)) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub line: usize,
    pub column: usize,
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
