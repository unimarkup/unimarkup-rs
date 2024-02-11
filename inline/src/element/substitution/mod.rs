use crate::parser::InlineParser;
use crate::InlineTokenKind;
use unimarkup_commons::lexer::token::iterator::PeekingNext;
use unimarkup_commons::lexer::{position::Position, token::implicit::ImplicitSubstitutionKind};

use super::{Inline, InlineElement};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistinctReference {
    id: String,
    fields: Vec<String>,
    start: Position,
    end: Position,
}

impl DistinctReference {
    pub fn new(id: String, fields: Vec<String>, start: Position, end: Position) -> Self {
        Self {
            id,
            fields,
            start,
            end,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn fields(&self) -> &Vec<String> {
        &self.fields
    }
}

impl InlineElement for DistinctReference {
    fn as_unimarkup(&self) -> String {
        format!("{}.{}", self.id.clone(), self.fields.clone().join("."))
    }

    fn start(&self) -> unimarkup_commons::lexer::position::Position {
        self.start
    }

    fn end(&self) -> unimarkup_commons::lexer::position::Position {
        self.end
    }
}

// Parses distinct reference
pub(crate) fn parse_distinct_reference<'s, 'i>(
    mut parser: InlineParser<'s, 'i>,
) -> (InlineParser<'s, 'i>, Option<Inline>) {
    let open_token_opt = parser.iter.peeking_next(|_| true);
    if open_token_opt.is_none() {
        return (parser, None);
    }

    let open_token = open_token_opt.expect("Checked above to be not None.");

    parser.iter.next(); // consume open token => now it will lead to Some(inline)

    let mut parsed_token_strings = Vec::new();
    let mut entries = Vec::new();

    while let Some(kind) = parser.iter.peek_kind() {
        let token = parser.iter.next().unwrap();
        if kind == InlineTokenKind::Eoi || kind == InlineTokenKind::Cite {
            break;
        }
        if kind == InlineTokenKind::Dot || token.as_str() == "." {
            entries.push(parsed_token_strings.join(""));
            parsed_token_strings = Vec::new();
        } else {
            parsed_token_strings.push(token.as_str().to_string());
        }
    }

    let parsed_str = parsed_token_strings.join("");
    if !parsed_str.is_empty() {
        entries.push(parsed_str);
    }

    if entries.len() < 2 {
        entries.push("authors".to_string());
    }
    let id = entries[0].clone();
    entries.remove(0);

    let prev_token = parser
        .iter
        .prev_token()
        .expect("Previous token must exist, because peek above would else have returned None.");

    let end = if parser.iter.end_reached() {
        //TODO: Check for optional attributes here
        prev_token.end
    } else {
        crate::element::helper::implicit_end_using_prev(&prev_token)
    };

    (
        parser,
        Some(Inline::DistinctReference(DistinctReference::new(
            id,
            entries,
            open_token.start,
            end,
        ))),
    )
}
