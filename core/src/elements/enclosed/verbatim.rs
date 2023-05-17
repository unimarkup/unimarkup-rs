use logid::capturing::MappedLogId;
use serde::{Deserialize, Serialize};
use unimarkup_render::highlight::{self, DEFAULT_THEME, PLAIN_SYNTAX};
use unimarkup_render::html::Html;
use unimarkup_render::render::Render;

use crate::elements::blocks::Block;
use crate::elements::Blocks;
use crate::parser::{ElementParser, TokenizeOutput};
use unimarkup_commons::scanner::{Symbol, SymbolKind};

/// Structure of a Unimarkup verbatim block element.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Verbatim {
    /// Unique identifier for a verbatim block.
    pub id: String,

    /// The content of the verbatim block.
    pub content: String,

    /// Attributes of the verbatim block.
    // TODO: make attributes data structure
    pub attributes: Option<String>,

    /// Line number, where the verbatim block occurs in
    /// the Unimarkup document.
    pub line_nr: usize,
}

pub(crate) enum Token<'a> {
    Delimiter { line: usize },
    Content(&'a [Symbol<'a>]),
}

impl ElementParser for Verbatim {
    type Token<'a> = self::Token<'a>;

    fn tokenize<'i>(input: &'i [Symbol<'i>]) -> Option<TokenizeOutput<'i, Self::Token<'i>>> {
        let start_delim = input
            .iter()
            .take_while(|symbol| matches!(symbol.kind, SymbolKind::Tick))
            .count();

        if start_delim < 3 {
            return None;
        };

        // we know there are at least 3
        let first_delim = input[0];

        // TODO: handle language attribute
        let content_count = input
            .iter()
            .skip(start_delim)
            .take_while(|symbol| !matches!(symbol.kind, SymbolKind::Tick))
            .count();

        let end_delim = input
            .iter()
            .skip(start_delim + content_count)
            .take_while(|sym| matches!(sym.kind, SymbolKind::Tick))
            .count();

        if end_delim != start_delim {
            return None;
        }

        let start_content = start_delim;
        let end_content = start_content + content_count;
        let content = &input[start_content..end_content];
        let rest = &input[end_content + end_delim..];

        let last_delim = input[end_content];

        let output = TokenizeOutput {
            tokens: vec![
                Token::Delimiter {
                    line: first_delim.start.line,
                },
                Token::Content(content),
                Token::Delimiter {
                    line: last_delim.start.line,
                },
            ],
            rest_of_input: rest,
        };

        Some(output)
    }

    fn parse(input: Vec<Self::Token<'_>>) -> Option<Blocks> {
        let Token::Delimiter { line } = input.get(0)? else {return None};
        let Token::Content(symbols) = input.get(1)? else { return None };
        let content = Symbol::flatten(symbols);

        let block = Self {
            id: String::default(),
            content: String::from(content),
            attributes: None,
            line_nr: *line,
        };

        Some(vec![Block::Verbatim(block)])
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct VerbatimAttributes {
    language: Option<String>,
}

impl Render for Verbatim {
    fn render_html(&self) -> Result<Html, MappedLogId> {
        let mut res = String::with_capacity(self.content.capacity());

        // TODO: improve handling of attributes
        let attributes = serde_json::from_str::<VerbatimAttributes>(
            &self.attributes.as_ref().cloned().unwrap_or_default(),
        )
        .ok();

        let language = match attributes.as_ref() {
            Some(attrs) => attrs.language.clone().unwrap_or(PLAIN_SYNTAX.to_string()),
            None => PLAIN_SYNTAX.to_string(),
        };

        res.push_str(&format!(
            "<div id='{}' class='code-block language-{}' >",
            &self.id, &language
        ));
        res.push_str(&highlight::highlight_html_lines(
            &self.content,
            &language,
            DEFAULT_THEME,
        ));
        res.push_str("</div>");

        Ok(Html {
            body: res,
            ..Default::default()
        })
    }
}
