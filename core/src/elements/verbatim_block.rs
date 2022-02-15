use std::collections::{HashMap, VecDeque};

use pest::iterators::Pairs;
use serde::{Deserialize, Serialize};

use crate::backend::{error::BackendError, ParseFromIr, Render};
use crate::elements::log_id::EnclosedErrLogId;
use crate::elements::types::{UnimarkupBlocks, UnimarkupType};
use crate::frontend::error::{custom_pest_error, FrontendError};
use crate::frontend::parser::{Rule, UmParse};
use crate::log_id::{LogId, SetLog};
use crate::middleend::{AsIrLines, ContentIrLine};

use super::error::ElementError;
use super::log_id::GeneralErrLogId;

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
    fn parse(pairs: &mut Pairs<Rule>, span: pest::Span) -> Result<UnimarkupBlocks, FrontendError>
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
                            ElementError::Enclosed(
                                (GeneralErrLogId::InvalidAttribute as LogId).set_log(
                                    &custom_pest_error(
                                        "Verbatim block attributes are not valid JSON",
                                        rule.as_span(),
                                    ),
                                    file!(),
                                    line!(),
                                ),
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

                    return Err(ElementError::Enclosed(
                        (EnclosedErrLogId::FailedParsing as LogId)
                            .set_log("Could not parse verbatim block.", file!(), line!())
                            .add_info(&format!("Cause: {}", pest_err)),
                    )
                    .into());
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
            UnimarkupType::VerbatimBlock.to_string(),
            &self.content,
            "",
            &self.attributes,
            "",
        );

        vec![line]
    }
}

impl ParseFromIr for VerbatimBlock {
    fn parse_from_ir(content_lines: &mut VecDeque<ContentIrLine>) -> Result<Self, BackendError>
    where
        Self: Sized,
    {
        if let Some(ir_line) = content_lines.pop_front() {
            let expected_type = UnimarkupType::VerbatimBlock.to_string();

            if ir_line.um_type != expected_type {
                return Err(ElementError::Enclosed(
                    (GeneralErrLogId::InvalidElementType as LogId).set_log(
                        &format!(
                            "Expected verbatim type to parse, instead got: '{}'",
                            ir_line.um_type
                        ),
                        file!(),
                        line!(),
                    ),
                )
                .into());
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
            Err(ElementError::Enclosed(
                (GeneralErrLogId::FailedBlockCreation as LogId)
                    .set_log("Could not construct VerbatimBlock.", file!(), line!())
                    .add_info("Cause: No content ir line available."),
            )
            .into())
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct VerbatimAttributes {
    language: Option<String>,
}

impl Render for VerbatimBlock {
    fn render_html(&self) -> Result<String, BackendError> {
        let mut res = String::with_capacity(self.content.capacity());

        let attributes =
            serde_json::from_str::<VerbatimAttributes>(&self.attributes).unwrap_or_default();

        res.push_str("<pre><code");
        res.push_str(" id='");
        res.push_str(&self.id);

        if let Some(language) = attributes.language {
            res.push_str("' class='language-");
            res.push_str(&language.trim().to_lowercase());
        }

        res.push_str("'>");
        res.push_str(&self.content);
        res.push_str("</code></pre>");

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use pest::Parser;

    use crate::backend::{ParseFromIr, Render};
    use crate::elements::types::UnimarkupType;
    use crate::elements::*;
    use crate::frontend::parser::{Rule, UmParse, UnimarkupParser};
    use crate::middleend::*;
    mod render {
        use super::*;

        #[test]
        fn with_lang() {
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
                "<pre><code id='{}' class='language-{}'>{}</code></pre>",
                id, lang, content
            );

            assert_eq!(expected_html, block.render_html().unwrap());
        }

        #[test]
        fn without_lang() {
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

            let expected_html = format!("<pre><code id='{}'>{}</code></pre>", id, content);

            assert_eq!(expected_html, block.render_html().unwrap());
        }
    }

    mod parse_from_ir {
        use super::*;

        #[test]
        fn parse() {
            let test_id = String::from("test-id");
            let content = String::from(
                "This is an example verbatim
                which spans multiple lines",
            );

            let mut lines: VecDeque<_> = vec![ContentIrLine {
                id: test_id.clone(),
                line_nr: 0,
                um_type: UnimarkupType::VerbatimBlock.to_string(),
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

        #[test]
        fn parse_bad() {
            let mut lines = vec![].into();

            let block_res = VerbatimBlock::parse_from_ir(&mut lines);

            assert!(block_res.is_err());

            let ir_line_bad_type = ContentIrLine {
                id: String::from("some-id"),
                line_nr: 2,
                um_type: format!("{}-more-info", UnimarkupType::VerbatimBlock.to_string()),
                text: String::from("This is the text of this verbatim"),
                ..Default::default()
            };

            lines.push_front(ir_line_bad_type);

            let block_res = VerbatimBlock::parse_from_ir(&mut lines);

            assert!(block_res.is_err());
        }
    }

    mod as_ir_lines {
        use super::*;

        #[test]
        fn lines() {
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
            assert_eq!(line.um_type, UnimarkupType::VerbatimBlock.to_string());
            assert_eq!(line.text, block.content);
            assert!(line.fallback_text.is_empty());
            assert_eq!(line.attributes, block.attributes);
            assert!(line.fallback_attributes.is_empty());
        }
    }

    mod um_parse {
        use std::collections::HashMap;

        use super::*;

        #[test]
        fn parse() {
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
                UnimarkupType::VerbatimBlock.to_string(),
                expected_text,
                "",
                "",
                "",
            );

            try_parse(input, expected_line)
        }

        #[test]
        fn parse_with_lang() {
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
                UnimarkupType::VerbatimBlock.to_string(),
                expected_text,
                "",
                "{ \"language\": \"rust\" }",
                "",
            );

            try_parse(input, expected_line)
        }

        #[test]
        fn parse_with_attrs() {
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
                UnimarkupType::VerbatimBlock.to_string(),
                expected_text,
                "",
                serde_json::to_string(&expected_attrs).unwrap(),
                "",
            );

            try_parse(input, expected_line)
        }

        #[test]
        #[should_panic]
        fn parse_bad() {
            let input = "~~~
                            some content ~~~";

            try_parse(input, ContentIrLine::default());
        }

        fn try_parse(input: &str, mut expected_line: ContentIrLine) {
            let mut unimarkup = UnimarkupParser::parse(Rule::unimarkup, input).unwrap();

            assert_eq!(unimarkup.clone().count(), 1);

            let mut inner_pairs = unimarkup.next().unwrap().into_inner();

            assert_eq!(inner_pairs.clone().count(), 2);

            let enclosed = inner_pairs.next().unwrap();

            assert_eq!(enclosed.as_rule(), Rule::enclosed_block);

            let verbatim_res = UnimarkupParser::parse(Rule::verbatim, enclosed.as_str());

            assert!(verbatim_res.is_ok());

            let mut input_pairs = verbatim_res.unwrap();

            let block_res = VerbatimBlock::parse(&mut input_pairs, enclosed.as_span());

            assert!(block_res.is_ok());

            let list = block_res.unwrap();
            assert_eq!(list.len(), 1);

            let mut ir_lines = list.get(0).unwrap().as_ir_lines();

            assert_eq!(ir_lines.len(), 1);

            let mut line = ir_lines.pop().unwrap();

            check_lines(&mut line, &mut expected_line);
        }

        fn check_lines(first: &mut ContentIrLine, second: &mut ContentIrLine) {
            if !first.attributes.is_empty() {
                let is_attrs: HashMap<&str, &str> =
                    serde_json::from_str(&first.attributes).unwrap();
                let expect_attrs: HashMap<&str, &str> =
                    serde_json::from_str(&second.attributes).unwrap();
                assert_eq!(is_attrs, expect_attrs);
            }

            // test attributes manually because HashMap is not sorted
            // that makes the test fail depending on the sorting of attributes
            // even if they contain the same keys with same values
            first.attributes = String::default();
            second.attributes = String::default();

            assert_eq!(first, second);
        }
    }
}