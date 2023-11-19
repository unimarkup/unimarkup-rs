use unimarkup_commons::lexer::{position::Position, token::implicit::ImplicitSubstitutionKind};

use super::InlineElement;

pub mod named;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplicitSubstitution {
    kind: ImplicitSubstitutionKind,
    start: Position,
    end: Position,
}

impl ImplicitSubstitution {
    pub fn new(kind: ImplicitSubstitutionKind, start: Position, end: Position) -> Self {
        Self { kind, start, end }
    }

    pub fn orig(&self) -> &'static str {
        self.kind.orig()
    }

    pub fn subst(&self) -> &'static str {
        self.kind.subst()
    }
}

impl InlineElement for ImplicitSubstitution {
    fn as_unimarkup(&self) -> String {
        self.kind.orig().to_string()
    }

    fn start(&self) -> unimarkup_commons::lexer::position::Position {
        self.start
    }

    fn end(&self) -> unimarkup_commons::lexer::position::Position {
        self.end
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectUri {
    uri: String,
    start: Position,
    end: Position,
}

impl DirectUri {
    pub fn new(uri: String, start: Position, end: Position) -> Self {
        Self { uri, start, end }
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }
}

impl InlineElement for DirectUri {
    fn as_unimarkup(&self) -> String {
        self.uri.clone()
    }

    fn start(&self) -> unimarkup_commons::lexer::position::Position {
        self.start
    }

    fn end(&self) -> unimarkup_commons::lexer::position::Position {
        self.end
    }
}
