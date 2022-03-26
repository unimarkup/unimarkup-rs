use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Sub, SubAssign};

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
    pub fn as_str(&self) -> &str {
        match self.content {
            Some(ref content) => content,
            None => self.kind.as_str(),
        }
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Bold,
    Italic,
    Newline,
    Whitespace,
    Plain,
}

impl TokenKind {
    fn as_str(&self) -> &str {
        match *self {
            TokenKind::Bold => "**",
            TokenKind::Italic => "*",
            TokenKind::Newline => "\n",
            TokenKind::Plain => "",
            TokenKind::Whitespace => " ",
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    start: Position,
    end: Position,
}

impl From<(Position, Position)> for Span {
    fn from((start, end): (Position, Position)) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            line: self.line + rhs.line,
            column: self.column + rhs.column,
        }
    }
}

impl Add<(usize, usize)> for Position {
    type Output = Self;

    fn add(self, (line, column): (usize, usize)) -> Self::Output {
        Self {
            line: self.line + line,
            column: self.column + column,
        }
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

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            line: self.line - rhs.line,
            column: self.column - rhs.column,
        }
    }
}

impl Sub<(usize, usize)> for Position {
    type Output = Self;

    fn sub(self, (line, column): (usize, usize)) -> Self::Output {
        Self {
            line: self.line - line,
            column: self.column - column,
        }
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
