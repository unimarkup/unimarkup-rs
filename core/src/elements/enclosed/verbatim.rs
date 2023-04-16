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

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use pest::Parser;

    use super::*;
    use crate::frontend::parser::{Rule, UmParse, UnimarkupParser};

    #[test]
    fn test__render_html__verbatim_with_lang() {
        let id = String::from("verbatim-id");
        let content = String::from(
            "This is content of the verbatim block.
                 It also contains a newline",
        );

        let lang = "rust";

        let attributes = Some(format!("{{ \"language\": \"{}\" }}", lang));

        let block = Verbatim {
            id: id.clone(),
            content: content.clone(),
            attributes,
            line_nr: 0,
        };

        let expected_html = format!(
            "<div id='{}' class='code-block language-{}' >{}</div>",
            id,
            lang,
            &highlight::highlight_html_lines(&content, lang, DEFAULT_THEME)
        );

        assert_eq!(expected_html, block.render_html().unwrap().body);
    }

    #[test]
    fn test__render_html__verbatim_without_lang() {
        let id = String::from("verbatim-id");
        let content = String::from(
            "This is content of the verbatim block.
                 It also contains a newline",
        );

        let attributes = Some(String::from("{}"));

        let block = Verbatim {
            id: id.clone(),
            content: content.clone(),
            attributes,
            line_nr: 0,
        };

        let expected_html = format!(
            "<div id='{}' class='code-block language-plain' >{}</div>",
            id,
            &highlight::highlight_html_lines(&content, PLAIN_SYNTAX, DEFAULT_THEME)
        );

        assert_eq!(expected_html, block.render_html().unwrap().body);
    }

    #[test]
    fn test__parse__verbatim() {
        let input = "~~~\nfn main() {\n  println!(\"Hello World!\");\n}\n~~~";

        let expected = Verbatim {
            id: format!("verbatim-{}", 1),
            content: "fn main() {\n  println!(\"Hello World!\");\n}".to_owned(),
            attributes: None,
            line_nr: 1,
        };

        try_parse(input, vec![expected.into()])
    }

    #[test]
    fn test__parse__verbatim_with_lang() {
        let input = "~~~rust\nfn main() {\n  println!(\"Hello World!\");\n}\n~~~";

        let expected = Verbatim {
            id: format!("verbatim-{}", 1),
            content: "fn main() {\n  println!(\"Hello World!\");\n}".to_owned(),
            attributes: Some("{ \"language\": \"rust\" }".to_owned()),
            line_nr: 1,
        };

        try_parse(input, vec![expected.into()])
    }

    #[test]
    fn test__parse_verbatim__with_attrs() {
        let input = "~~~{ \"language\": \"rust\", \"id\": \"custom-id\" }\nfn main() {\n  println!(\"Hello World!\");\n}\n~~~";

        let expected = Verbatim {
            id: "custom-id".to_owned(),
            content: "fn main() {\n  println!(\"Hello World!\");\n}".to_owned(),
            attributes: Some("{\"id\":\"custom-id\",\"language\":\"rust\"}".to_owned()),
            line_nr: 1,
        };

        try_parse(input, vec![expected.into()])
    }

    #[test]
    #[should_panic]
    fn test__parse__invalid_verbatim() {
        let input = "~~~
                            some content ~~~";

        try_parse(input, Blocks::default());
    }

    fn try_parse(input: &str, expected_block: Blocks) {
        let mut unimarkup = UnimarkupParser::parse(Rule::unimarkup, input).unwrap();

        assert_eq!(unimarkup.clone().count(), 1, "Number of pairs not equal 1");

        let mut inner_pairs = unimarkup.next().unwrap().into_inner();

        assert_eq!(
            inner_pairs.clone().count(),
            2,
            "Number of inner pairs not equal 2"
        );

        let enclosed = inner_pairs.next().unwrap();

        assert_eq!(
            enclosed.as_rule(),
            Rule::enclosed_block,
            "Inner pair is not a enclosed_block"
        );

        let verbatim_res = UnimarkupParser::parse(Rule::verbatim, enclosed.as_str());

        assert!(verbatim_res.is_ok(), "Cause: {}", verbatim_res.unwrap_err());

        let mut input_pairs = verbatim_res.unwrap();

        let block_res = <Verbatim as UmParse>::parse(&mut input_pairs, enclosed.as_span());

        assert!(block_res.is_ok(), "Cause: {:?}", block_res.unwrap_err());

        let list = block_res.unwrap();
        assert_eq!(
            list.len(),
            1,
            "Number of Unimarkup blocks in Verbatim not equal 1"
        );

        assert_eq!(list, expected_block, "Parsed input not equal to expected");
    }
}
