use std::fmt::Debug;

use unimarkup_inline::{Inline, ParseInlines};

use crate::{elements::Blocks, parser::ElementParser};
use crate::{
    elements::{blocks::Block, types},
    parser::TokenizeOutput,
};
use unimarkup_commons::scanner::{Symbol, SymbolIterator, SymbolKind};

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

fn not_closing_symbol(symbol: &&Symbol) -> bool {
    [SymbolKind::Blankline, SymbolKind::EOI]
        .iter()
        .all(|closing| *closing != symbol.kind)
}

impl ElementParser for Paragraph {
    type Token<'a> = &'a Symbol<'a>;

    fn tokenize<'i>(input: &mut SymbolIterator<'i>) -> Option<TokenizeOutput<Self::Token<'i>>> {
        let content = input.by_ref().take_while(not_closing_symbol).collect();

        let output = TokenizeOutput { tokens: content };

        Some(output)
    }

    fn parse(input: Vec<Self::Token<'_>>) -> Option<Blocks> {
        let block = Block::Paragraph(Paragraph::from(input));

        Some(vec![block])
    }
}
