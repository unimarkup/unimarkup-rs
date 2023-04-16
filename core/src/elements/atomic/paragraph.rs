use std::{collections::HashMap, fmt::Debug};

use logid::{
    capturing::{LogIdTracing, MappedLogId},
    log_id::LogId,
};
use pest::iterators::Pairs;
use pest::Span;
use unimarkup_inline::{Inline, ParseUnimarkupInlines};
use unimarkup_render::{html::Html, render::Render};

use crate::{
    elements::{blocks::Block, types},
    frontend::parser::{self, custom_pest_error, Rule, UmParse},
    log_id::CORE_LOG_ID_MAP,
    parser::{
        symbol::{Symbol, SymbolKind},
        TokenizeOutput,
    },
};
use crate::{
    elements::{inlines, log_id::GeneralErrLogId, Blocks},
    parser::ElementParser,
};

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
        let content = Symbol::flatten(value).parse_unimarkup_inlines().collect();
        let line_nr = value.get(0).map(|symbol| symbol.start.line).unwrap_or(0);

        let id = parser::generate_id(&format!(
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

impl UmParse for Paragraph {
    fn parse(pairs: &mut Pairs<Rule>, span: Span) -> Result<Blocks, MappedLogId>
    where
        Self: Sized,
    {
        let (line_nr, _column_nr) = span.start_pos().line_col();

        let mut paragraph_rules = pairs
            .next()
            .expect("paragraph must be there at this point")
            .into_inner();

        let content = paragraph_rules
            .next()
            .expect("Invalid paragraph: content expected")
            .as_str()
            .parse_unimarkup_inlines()
            .collect();

        let attributes = if let Some(attributes) = paragraph_rules.next() {
            let attr: HashMap<&str, &str> =
                serde_json::from_str(attributes.as_str()).map_err(|err| {
                    (GeneralErrLogId::InvalidAttribute as LogId)
                        .set_event_with(
                            &CORE_LOG_ID_MAP,
                            &custom_pest_error(
                                "Paragraph attributes are not valid JSON.",
                                attributes.as_span(),
                            ),
                            file!(),
                            line!(),
                        )
                        .add_info(&format!("Cause: {}", err))
                })?;

            Some(attr)
        } else {
            None
        };

        let id = match attributes {
            Some(ref attrs) if attrs.get("id").is_some() => attrs.get("id").unwrap().to_string(),
            _ => parser::generate_id(&format!(
                "paragraph{delim}{}",
                line_nr,
                delim = types::ELEMENT_TYPE_DELIMITER
            ))
            .unwrap(),
        };

        let paragraph_block = Paragraph {
            id,
            content,
            attributes: serde_json::to_string(&attributes.unwrap_or_default()).ok(),
            line_nr,
        };

        Ok(vec![paragraph_block.into()])
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

pub(crate) struct Token<'a> {
    kind: TokenKind<'a>,
}

impl ElementParser for Paragraph {
    type Token<'a> = self::Token<'a>;

    fn tokenize<'input>(
        input: &'input [Symbol<'input>],
    ) -> Option<TokenizeOutput<Self::Token<'input>>> {
        let iter = input.iter();

        let taken = iter.take_while(not_closing_symbol).count() + 1;

        let tokens = vec![
            Token {
                kind: TokenKind::Start,
            },
            Token {
                kind: TokenKind::Text(&input[..taken]),
            },
            Token {
                kind: TokenKind::End,
            },
        ];

        let input = &input[taken..];

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
    fn render_html(&self) -> Result<Html, MappedLogId> {
        let mut html = Html::default();

        html.body.push_str("<p");
        html.body.push_str(" id='");
        html.body.push_str(&self.id);
        html.body.push_str("'>");

        inlines::push_inlines(&mut html, &self.content)?;

        html.body.push_str("</p>");

        Ok(html)
    }
}
