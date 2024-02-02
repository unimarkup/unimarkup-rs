use std::rc::Rc;

use unimarkup_commons::lexer::{
    position::Position,
    token::iterator::{EndMatcher, Itertools},
};

use crate::{
    element::{Inline, InlineElement},
    parser::InlineParser,
    tokenize::InlineToken,
    InlineTokenKind,
};

/// Represents the citation element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Citation {
    /// The entries of this citation.
    entries: Vec<CitationEntry>,
    /// The start of this citation in the original content.
    start: Position,
    /// The end of this citation in the original content.
    end: Position,
}

impl Citation {
    pub fn new(entries: Vec<CitationEntry>, start: Position, end: Position) -> Self {
        Self {
            entries,
            start,
            end,
        }
    }

    /// Returns the list of [`CitationEntry`] inside the citation.
    pub fn entries(&self) -> &Vec<CitationEntry> {
        &self.entries
    }

    pub(crate) fn parse<'slice, 'input>(
        mut parser: InlineParser<'slice, 'input>,
    ) -> (InlineParser<'slice, 'input>, Option<Inline>) {
        let open_bracket = parser
            .iter
            .prev_token()
            .expect("OpenBracket was consumed before parsing Citation.");

        let (mut scoped_parser, outer_open_formats) =
            parser.nest_scoped(Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.consumed_matches(&[InlineTokenKind::CloseBracket.into()])
            })));

        let cite_token_opt = scoped_parser.iter.next();
        if cite_token_opt.is_none() {
            return (scoped_parser.unfold_scoped(outer_open_formats), None);
        }

        let cite_token = cite_token_opt.expect("Checked above to be not None.");

        debug_assert_eq!(
            cite_token.kind,
            InlineTokenKind::Cite,
            "Called citation parser on kind '{:?}'.",
            cite_token.kind
        );

        let mut entries = Vec::new();
        while scoped_parser.iter.peek().is_some() {
            let (updated_parser, entry_opt) = CitationEntry::parse(scoped_parser);
            scoped_parser = updated_parser;

            match entry_opt {
                Some(entry) => entries.push(entry),
                // One wrong citation entry invalidates the citation
                None => return (scoped_parser.unfold_scoped(outer_open_formats), None),
            }
        }

        if entries.is_empty() {
            return (scoped_parser.unfold_scoped(outer_open_formats), None);
        }

        let end = entries.last().expect("At least one entry exists.").end;
        let cited_ids = entries.iter().map(|entry| entry.id.clone()).collect_vec();
        scoped_parser.context.citations.push(cited_ids);

        parser = scoped_parser.unfold_scoped(outer_open_formats);
        parser.iter.next(); // Consume closing bracket

        (
            parser,
            Some(Citation::new(entries, open_bracket.start, end).into()),
        )
    }
}

impl From<Citation> for Inline {
    fn from(value: Citation) -> Self {
        Inline::Citation(value)
    }
}

impl InlineElement for Citation {
    fn as_unimarkup(&self) -> String {
        format!(
            "[&&{}]",
            self.entries
                .iter()
                .map(|entry| entry.as_unimarkup())
                .join(", ")
        )
    }

    fn start(&self) -> Position {
        self.start
    }

    fn end(&self) -> Position {
        self.end
    }
}

/// Represents a citation entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CitationEntry {
    /// The citation ID.
    id: String,
    /// Optional attributes of the ciration entry.
    attributes: Vec<Inline>,
    /// The start of this entry in the original content.
    start: Position,
    /// The end of this entry in the original content.
    end: Position,
}

impl CitationEntry {
    pub fn as_unimarkup(&self) -> String {
        self.id.clone() // TODO: add attributes once implemented
    }

    pub(crate) fn parse<'slice, 'input>(
        parser: InlineParser<'slice, 'input>,
    ) -> (InlineParser<'slice, 'input>, Option<CitationEntry>) {
        let mut id_parser = parser.nest(Some(Rc::new(|matcher: &mut dyn EndMatcher| {
            matcher.consumed_matches(&[
                InlineTokenKind::Comma.into(),
                InlineTokenKind::Whitespace.into(),
            ]) || matcher.consumed_matches(&[
                InlineTokenKind::Comma.into(),
                InlineTokenKind::Newline.into(),
            ]) || matcher.matches(&[InlineTokenKind::Newline.into()])
                || matcher.matches(&[InlineTokenKind::EscapedNewline.into()])
                || matcher.matches(&[InlineTokenKind::Whitespace.into()])
                || matcher.matches(&[InlineTokenKind::EscapedWhitespace.into()])
                || matcher.matches(&[InlineTokenKind::Dot.into()])
                || matcher.matches(&[InlineTokenKind::Cite.into()])
                || matcher.outer_end() // TODO: match for attributes
        })));

        let id_parts = id_parser.iter.take_to_end();
        match InlineToken::flatten(&id_parts) {
            Some(id) => {
                // Invalid entry ID
                if id.is_empty()
                    || matches!(
                        id_parser.iter.peek_kind(),
                        Some(InlineTokenKind::Newline)
                            | Some(InlineTokenKind::EscapedNewline)
                            | Some(InlineTokenKind::Whitespace)
                            | Some(InlineTokenKind::EscapedWhitespace)
                            | Some(InlineTokenKind::Dot)
                            | Some(InlineTokenKind::Cite)
                    )
                {
                    return (id_parser.into_inner(), None);
                }

                let start = id_parts
                    .first()
                    .expect("At least one token exists, because ID is not empty.")
                    .start;
                let end = id_parts
                    .last()
                    .expect("At least one token exists, because ID is not empty.")
                    .end;

                (
                    id_parser.into_inner(),
                    Some(CitationEntry {
                        id: id.to_string(),
                        attributes: Vec::new(),
                        start,
                        end,
                    }),
                )
            }
            None => return (id_parser.into_inner(), None),
        }
    }
}
