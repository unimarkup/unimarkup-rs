use std::rc::Rc;
use unimarkup_commons::lexer::{position::Position, token::implicit::ImplicitSubstitutionKind};
use unimarkup_commons::lexer::token::iterator::{EndMatcher, PeekingNext};
use crate::InlineTokenKind;
use crate::parser::InlineParser;

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
    start: Position,
    end: Position,
}

impl DistinctReference {
    pub fn new(id: String, start: Position, end: Position) -> Self {
        Self { id, start, end }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl InlineElement for DistinctReference {
    fn as_unimarkup(&self) -> String {
        self.id.clone()
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

    // No need to check for correct opening format, because parser is only assigned for valid opening tokens.
    if parser.iter.peek_kind().map_or(true, |t| t.is_space()) {
        return (parser, None);
    }

    parser.iter.next(); // consume open token => now it will lead to Some(inline)

    // ignore implicits, because only escapes and logic elements are allowed in following inline verbatim
    let prev_context_flags = parser.context.flags;

    let (mut scoped_parser, outer_open_formats) =
        parser.nest_scoped(Some(Rc::new(|matcher: &mut dyn EndMatcher| {
            !matcher.prev_is_space()
                && matcher.consumed_matches(&[InlineTokenKind::Cite.into()])
        })));
    scoped_parser.context.flags.allow_implicits = false;
    scoped_parser.context.flags.keep_whitespaces = true;
    scoped_parser.context.flags.logic_only = true;

    let (updated_parser, inner) = InlineParser::parse(scoped_parser);
    scoped_parser = updated_parser;

    let end_reached = scoped_parser.iter.end_reached();
    parser = scoped_parser.unfold_scoped(outer_open_formats);
    parser.context.flags = prev_context_flags;

    let prev_token = parser.iter.prev_token().expect(
        "Previous token must exist, because peek above would else have returned None.",
    );

    let (end, implicit_end) = if end_reached {
        //TODO: Check for optional attributes here
        (prev_token.end, false)
    } else {
        (
            crate::element::helper::implicit_end_using_prev(&prev_token),
            true,
        )
    };

    (
        parser,
        Some(Inline::DistinctReference(DistinctReference::new(inner[0].as_unimarkup(), open_token.start, end)))
    )
}
