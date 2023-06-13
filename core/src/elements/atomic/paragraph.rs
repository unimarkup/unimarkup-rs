use std::fmt::Debug;

use unimarkup_inline::{Inline, ParseInlines};
use unimarkup_render::{html::Html, render::Render};

use crate::{
    elements::{blocks::Block, types},
    parser::{self, TokenizeOutput},
};
use crate::{
    elements::{inlines, Blocks},
    parser::ElementParser,
};
use unimarkup_commons::scanner::{Symbol, SymbolKind};

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

impl From<&[Symbol<'_>]> for Paragraph {
    fn from(value: &[Symbol<'_>]) -> Self {
        let content = value.parse_inlines().collect();
        let line_nr = value.get(0).map(|symbol| symbol.start.line).unwrap_or(0);

        let id = parser::generate_id::generate_id(&format!(
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

enum TokenKind<'a> {
    Start,
    End,
    Text(&'a [Symbol<'a>]),
}

pub(crate) struct ParagraphToken<'a> {
    kind: TokenKind<'a>,
}

impl ElementParser for Paragraph {
    type Token<'a> = self::ParagraphToken<'a>;

    fn tokenize<'input>(
        input: &'input [Symbol<'input>],
    ) -> Option<TokenizeOutput<Self::Token<'input>>> {
        let iter = input.iter();

        let taken = iter.take_while(not_closing_symbol).count();
        let end_of_input = taken.min(input.len());

        let tokens = vec![
            ParagraphToken {
                kind: TokenKind::Start,
            },
            ParagraphToken {
                kind: TokenKind::Text(&input[..end_of_input]),
            },
            ParagraphToken {
                kind: TokenKind::End,
            },
        ];

        let input = &input[end_of_input..];

        let output = TokenizeOutput {
            tokens,
            rest_of_input: input,
        };

        Some(output)
    }

    fn parse(input: Vec<Self::Token<'_>>) -> Option<Blocks> {
        let content = match input[1].kind {
            TokenKind::Start => &[],
            TokenKind::End => &[],
            TokenKind::Text(symbols) => symbols,
        };

        let block = Block::Paragraph(Paragraph::from(content));

        Some(vec![block])
    }
}

impl Render for Paragraph {
    fn render_html(&self) -> Html {
        let mut html = Html::default();

        html.body.push_str("<p");
        html.body.push_str(" id='");
        html.body.push_str(&self.id);
        html.body.push_str("'>");

        inlines::push_inlines(&mut html, &self.content);

        html.body.push_str("</p>");

        html
    }
}
