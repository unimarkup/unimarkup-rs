use std::fmt::Debug;
use std::rc::Rc;

use unimarkup_inline::{Inline, ParseInlines};

use crate::{elements::Blocks, parser::ElementParser};
use crate::{
    elements::{blocks::Block, types},
    parser::TokenizeOutput,
};
use unimarkup_commons::scanner::{EndMatcher, Symbol, SymbolIterator};

/// Structure of a Unimarkup paragraph element.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Paragraph {
    /// Unique identifier for a paragraph.
    pub id: String,

    /// The content of the paragraph.
    pub content: Vec<Inline>,

    /// Attributes of the paragraph.
    pub attributes: Option<String>,

    /// Line number, where the paragraph occurs in
    /// the Unimarkup document.
    pub line_nr: usize,
}

impl Paragraph {}

impl From<Vec<&'_ Symbol<'_>>> for Paragraph {
    fn from(value: Vec<&'_ Symbol<'_>>) -> Self {
        let content = value
            .iter()
            .map(|&s| *s)
            .collect::<Vec<Symbol<'_>>>()
            .parse_inlines()
            .collect();
        let line_nr = value.get(0).map(|symbol| symbol.start.line).unwrap_or(0);

        let id = crate::generate_id::generate_id(&format!(
            "paragraph{delim}{}",
            line_nr,
            delim = types::ELEMENT_TYPE_DELIMITER
        ))
        .unwrap();

        Paragraph {
            id,
            content,
            attributes: None,
            line_nr,
        }
    }
}

impl ElementParser for Paragraph {
    type Token<'a> = &'a Symbol<'a>;

    fn tokenize<'i>(input: &mut SymbolIterator<'i>) -> Option<TokenizeOutput<Self::Token<'i>>> {
        let mut content_iter = input.nest(
            None,
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.consumed_is_empty_line()
            })),
        );
        let content = content_iter.take_to_end();
        content_iter.update(input);

        if content.is_empty() {
            return None;
        }

        Some(TokenizeOutput { tokens: content })
    }

    fn parse(input: Vec<Self::Token<'_>>) -> Option<Blocks> {
        let block = Block::Paragraph(Paragraph::from(input));

        Some(vec![block])
    }
}
