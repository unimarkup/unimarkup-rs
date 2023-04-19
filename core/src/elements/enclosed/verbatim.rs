use std::collections::BTreeMap;

use logid::capturing::{LogIdTracing, MappedLogId};
use logid::log_id::LogId;
use pest::iterators::Pairs;
use serde::{Deserialize, Serialize};
use unimarkup_render::highlight::{self, DEFAULT_THEME, PLAIN_SYNTAX};
use unimarkup_render::html::Html;
use unimarkup_render::render::Render;

use crate::elements::blocks::Block;
use crate::elements::enclosed::log_id::EnclosedErrLogId;
use crate::elements::log_id::GeneralErrLogId;
use crate::elements::Blocks;
use crate::frontend::parser::{custom_pest_error, Rule, UmParse};
use crate::log_id::CORE_LOG_ID_MAP;
use crate::parser::symbol::{Symbol, SymbolKind};
use crate::parser::{ElementParser, TokenizeOutput};

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
            .take_while(|symbol| matches!(symbol.kind, SymbolKind::Verbatim))
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
            .take_while(|symbol| !matches!(symbol.kind, SymbolKind::Verbatim))
            .count();

        let end_delim = input
            .iter()
            .skip(start_delim + content_count)
            .take_while(|sym| matches!(sym.kind, SymbolKind::Verbatim))
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

impl UmParse for Verbatim {
    fn parse(pairs: &mut Pairs<Rule>, span: pest::Span) -> Result<Blocks, MappedLogId>
    where
        Self: Sized,
    {
        let verbatim = pairs
            .next()
            .expect("Tried to parse invalid verbatim block.");

        let (line_nr, _column_nr) = span.start_pos().line_col();

        let mut block = Verbatim {
            id: format!("verbatim-{}", line_nr),
            content: String::new(),
            attributes: None,
            line_nr,
        };

        for rule in verbatim.into_inner() {
            match rule.as_rule() {
                Rule::verbatim_lang => {
                    let attr = format!("{{ \"language\": \"{}\" }}", rule.as_str().trim());

                    block.attributes = Some(attr);
                }
                Rule::verbatim_content => {
                    block.content = String::from(rule.as_str().trim());
                }
                Rule::verbatim_delimiter | Rule::verbatim_end => continue,
                Rule::attributes => {
                    let attributes: BTreeMap<&str, &str> = serde_json::from_str(rule.as_str())
                        .map_err(|_| {
                            (GeneralErrLogId::InvalidAttribute as LogId).set_event_with(
                                &CORE_LOG_ID_MAP,
                                &custom_pest_error(
                                    "Verbatim block attributes are not valid JSON",
                                    rule.as_span(),
                                ),
                                file!(),
                                line!(),
                            )
                        })?;

                    if let Some(&id) = attributes.get("id") {
                        block.id = String::from(id);
                    }

                    block.attributes = serde_json::to_string(&attributes).ok();
                }
                other => {
                    use pest::error;

                    let err_variant = error::ErrorVariant::ParsingError {
                        positives: vec![
                            Rule::verbatim_lang,
                            Rule::verbatim_content,
                            Rule::verbatim_delimiter,
                        ],
                        negatives: vec![other],
                    };

                    let pest_err = error::Error::new_from_span(err_variant, rule.as_span());

                    return Err((EnclosedErrLogId::FailedParsing as LogId)
                        .set_event_with(
                            &CORE_LOG_ID_MAP,
                            "Could not parse verbatim block.",
                            file!(),
                            line!(),
                        )
                        .add_info(&format!("Cause: {}", pest_err)));
                }
            }
        }

        Ok(vec![block.into()])
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
