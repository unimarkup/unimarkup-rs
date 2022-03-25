use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    span: Span,
    spacing: Spacing,
    content: Option<String>,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Self {
            kind,
            span: Span::default(),
            spacing: Spacing::default(),
            content: None,
        }
    }

    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    pub fn space(mut self, spacing: Spacing) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn spacing(&self) -> Spacing {
        self.spacing
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        match self.content {
            Some(ref content) => content,
            None => self.kind.as_ref(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Bold,
    Italic,
    Newline,
    Plain,
}

impl AsRef<str> for TokenKind {
    fn as_ref(&self) -> &str {
        match *self {
            TokenKind::Bold => "**",
            TokenKind::Italic => "*",
            TokenKind::Newline => "\n",
            TokenKind::Plain => "",
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

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.line += rhs.line;
        self.column += rhs.column;
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

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.line -= rhs.line;
        self.column -= rhs.column;
    }
}
