use std::collections::{HashMap, VecDeque};

use logid::capturing::{LogIdTracing, MappedLogId};
use logid::log_id::LogId;
use pest::iterators::Pairs;
use serde::{Deserialize, Serialize};
use unimarkup_render::html::Html;
use unimarkup_render::render::Render;

use crate::backend::ParseFromIr;
use crate::elements::log_id::EnclosedErrLogId;
use crate::elements::types::ElementType;
use crate::frontend::parser::{custom_pest_error, Rule, UmParse};
use crate::highlight::{self, DEFAULT_THEME, PLAIN_SYNTAX};
use crate::log_id::CORE_LOG_ID_MAP;
use crate::middleend::{AsIrLines, ContentIrLine};

use super::log_id::GeneralErrLogId;
use super::UnimarkupBlocks;

/// Structure of a Unimarkup verbatim block element.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VerbatimBlock {
    /// Unique identifier for a verbatim block.
    pub id: String,

    /// The content of the verbatim block.
    pub content: String,

    /// Attributes of the verbatim block.
    pub attributes: String,

    /// Line number, where the verbatim block occurs in
    /// the Unimarkup document.
    pub line_nr: usize,
}

impl UmParse for VerbatimBlock {
    fn parse(pairs: &mut Pairs<Rule>, span: pest::Span) -> Result<UnimarkupBlocks, MappedLogId>
    where
        Self: Sized,
    {
        let verbatim = pairs
            .next()
            .expect("Tried to parse invalid verbatim block.");

        let (line_nr, _column_nr) = span.start_pos().line_col();

        let mut block = VerbatimBlock {
            id: format!("verbatim-{}", line_nr),
            content: String::new(),
            attributes: String::new(),
            line_nr,
        };

        for rule in verbatim.into_inner() {
            match rule.as_rule() {
                Rule::verbatim_lang => {
                    let attr = format!("{{ \"language\": \"{}\" }}", rule.as_str().trim());

                    block.attributes = attr;
                }
                Rule::verbatim_content => {
                    block.content = String::from(rule.as_str().trim());
                }
                Rule::verbatim_delimiter | Rule::verbatim_end => continue,
                Rule::attributes => {
                    let attributes: HashMap<&str, &str> = serde_json::from_str(rule.as_str())
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

                    block.attributes = serde_json::to_string(&attributes).unwrap();
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

        Ok(vec![Box::new(block)])
    }
}

impl AsIrLines<ContentIrLine> for VerbatimBlock {
    fn as_ir_lines(&self) -> Vec<ContentIrLine> {
        let line = ContentIrLine::new(
            &self.id,
            self.line_nr,
            ElementType::VerbatimBlock.to_string(),
            &self.content,
            "",
            &self.attributes,
            "",
        );

        vec![line]
    }
}

impl ParseFromIr for VerbatimBlock {
    fn parse_from_ir(content_lines: &mut VecDeque<ContentIrLine>) -> Result<Self, MappedLogId>
    where
        Self: Sized,
    {
        if let Some(ir_line) = content_lines.pop_front() {
            let expected_type = ElementType::VerbatimBlock.to_string();

            if ir_line.um_type != expected_type {
                return Err(
                    (GeneralErrLogId::InvalidElementType as LogId).set_event_with(
                        &CORE_LOG_ID_MAP,
                        &format!(
                            "Expected verbatim type to parse, instead got: '{}'",
                            ir_line.um_type
                        ),
                        file!(),
                        line!(),
                    ),
                );
            }

            let content = if !ir_line.text.is_empty() {
                ir_line.text
            } else {
                ir_line.fallback_text
            };

            let attributes = if !ir_line.attributes.is_empty() {
                ir_line.attributes
            } else {
                ir_line.fallback_attributes
            };

            let block = VerbatimBlock {
                id: ir_line.id,
                content,
                attributes,
                line_nr: ir_line.line_nr,
            };

            Ok(block)
        } else {
            Err((GeneralErrLogId::FailedBlockCreation as LogId)
                .set_event_with(
                    &CORE_LOG_ID_MAP,
                    "Could not construct VerbatimBlock.",
                    file!(),
                    line!(),
                )
                .add_info("Cause: No content ir line available."))
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct VerbatimAttributes {
    language: Option<String>,
}

impl Render for VerbatimBlock {
    fn render_html(&self) -> Result<Html, MappedLogId> {
        let mut res = String::with_capacity(self.content.capacity());

        let attributes =
            serde_json::from_str::<VerbatimAttributes>(&self.attributes).unwrap_or_default();

        let language = match attributes.language {
            Some(language) => language,
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
    use std::collections::{HashMap, VecDeque};

    use pest::Parser;

    use super::*;
    use crate::backend::ParseFromIr;
    use crate::elements::types::ElementType;
    use crate::frontend::parser::{Rule, UmParse, UnimarkupParser};
    use crate::middleend::*;

    #[test]
    fn test__render_html__verbatim_with_lang() {
        let id = String::from("verbatim-id");
        let content = String::from(
            "This is content of the verbatim block.
                 It also contains a newline",
        );

        let lang = "rust";

        let attributes = format!("{{ \"language\": \"{}\" }}", lang);

        let block = VerbatimBlock {
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

        let attributes = String::from("{}");

        let block = VerbatimBlock {
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
    fn test__parse_from_ir__valid_verbatim() {
        let test_id = String::from("test-id");
        let content = String::from(
            "This is an example verbatim
                which spans multiple lines",
        );

        let mut lines: VecDeque<_> = vec![ContentIrLine {
            id: test_id.clone(),
            line_nr: 0,
            um_type: ElementType::VerbatimBlock.to_string(),
            text: content.clone(),
            attributes: String::from("{}"),
            ..Default::default()
        }]
        .into();

        let verbatim = VerbatimBlock::parse_from_ir(&mut lines).unwrap();

        assert_eq!(verbatim.id, test_id);
        assert_eq!(verbatim.line_nr, 0);
        assert_eq!(verbatim.content, content);
        assert_eq!(verbatim.attributes, String::from("{}"));
    }

    #[should_panic]
    #[test]
    fn test__parse_from_ir__invalid_verbatim() {
        let mut lines = vec![].into();

        let block_res = VerbatimBlock::parse_from_ir(&mut lines);

        assert!(block_res.is_err());

        let ir_line_bad_type = ContentIrLine {
            id: String::from("some-id"),
            line_nr: 2,
            um_type: format!("{}-more-info", ElementType::VerbatimBlock),
            text: String::from("This is the text of this verbatim"),
            ..Default::default()
        };

        lines.push_front(ir_line_bad_type);

        VerbatimBlock::parse_from_ir(&mut lines).unwrap();
    }

    #[test]
    fn test__content_ir_lines__valid_verbatim() {
        let id = String::from("verbatim-id");
        let content = String::from("This is placeholder content");
        let attributes = String::from("{}");
        let line_nr = 0;

        let block = VerbatimBlock {
            id,
            content,
            attributes,
            line_nr,
        };

        let ir_lines = block.as_ir_lines();

        assert_eq!(ir_lines.len(), 1);

        let line = ir_lines.get(0).unwrap();

        assert_eq!(line.id, block.id);
        assert_eq!(line.line_nr, block.line_nr);
        assert_eq!(line.um_type, ElementType::VerbatimBlock.to_string());
        assert_eq!(line.text, block.content);
        assert!(line.fallback_text.is_empty());
        assert_eq!(line.attributes, block.attributes);
        assert!(line.fallback_attributes.is_empty());
    }

    #[test]
    fn test__parse__verbatim() {
        let input = "~~~
                            fn main() {
                                println!(\"Hello World!\");
                            }\n~~~";

        let expected_text = r#"fn main() {
                                println!("Hello World!");
                            }"#;

        let expected_line = ContentIrLine::new(
            "verbatim-1",
            1,
            ElementType::VerbatimBlock.to_string(),
            expected_text,
            "",
            "",
            "",
        );

        try_parse(input, expected_line)
    }

    #[test]
    fn test__parse__verbatim_with_lang() {
        let input = "~~~rust
                            fn main() {
                                println!(\"Hello World!\");
                            }\n~~~";

        let expected_text = r#"fn main() {
                                println!("Hello World!");
                            }"#;

        let expected_line = ContentIrLine::new(
            "verbatim-1",
            1,
            ElementType::VerbatimBlock.to_string(),
            expected_text,
            "",
            "{ \"language\": \"rust\" }",
            "",
        );

        try_parse(input, expected_line)
    }

    #[test]
    fn test__parse_verbatim__with_attrs() {
        let input = "~~~{ \"language\": \"rust\", \"id\": \"custom-id\" }
                            fn main() {
                                println!(\"Hello World!\");
                            }\n~~~";

        let expected_text = r#"fn main() {
                                println!("Hello World!");
                            }"#;

        let mut expected_attrs = HashMap::new();

        expected_attrs.insert("id", "custom-id");
        expected_attrs.insert("language", "rust");

        let expected_line = ContentIrLine::new(
            "custom-id",
            1,
            ElementType::VerbatimBlock.to_string(),
            expected_text,
            "",
            serde_json::to_string(&expected_attrs).unwrap(),
            "",
        );

        try_parse(input, expected_line)
    }

    #[test]
    #[should_panic]
    fn test__parse__invalid_verbatim() {
        let input = "~~~
                            some content ~~~";

        try_parse(input, ContentIrLine::default());
    }

    fn try_parse(input: &str, mut expected_line: ContentIrLine) {
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

        let block_res = VerbatimBlock::parse(&mut input_pairs, enclosed.as_span());

        assert!(block_res.is_ok(), "Cause: {:?}", block_res.unwrap_err());

        let list = block_res.unwrap();
        assert_eq!(
            list.len(),
            1,
            "Number of UnimarkupBlocks in VerbatimBlock not equal 1"
        );

        let mut ir_lines = list.get(0).unwrap().as_ir_lines();

        assert_eq!(ir_lines.len(), 1, "Number of ir_lines not equal 1");

        let mut line = ir_lines.pop().unwrap();

        check_lines(&mut line, &mut expected_line);
    }

    fn check_lines(first: &mut ContentIrLine, second: &mut ContentIrLine) {
        if !first.attributes.is_empty() {
            let is_attrs: HashMap<&str, &str> = serde_json::from_str(&first.attributes).unwrap();
            let expect_attrs: HashMap<&str, &str> =
                serde_json::from_str(&second.attributes).unwrap();
            assert_eq!(is_attrs, expect_attrs, "Attributes do not match");
        }

        // test attributes manually because HashMap is not sorted
        // that makes the test fail depending on the sorting of attributes
        // even if they contain the same keys with same values
        first.attributes = String::default();
        second.attributes = String::default();

        assert_eq!(first, second, "ContentIrLine does not match");
    }
}
