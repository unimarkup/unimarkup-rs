use std::collections::VecDeque;

use pest::iterators::Pairs;
use serde::{Deserialize, Serialize};

use crate::backend::{BackendError, ParseFromIr, Render};
use crate::frontend::parser::{Rule, UmParse};
use crate::frontend::UnimarkupBlocks;
use crate::middleend::{AsIrLines, ContentIrLine};
use crate::um_elements::types::UnimarkupType;
use crate::um_error::UmError;

/// Structure of a Unimarkup verbatim block element.
#[derive(Debug, PartialEq, Eq)]
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
    fn parse(pairs: &mut Pairs<Rule>, span: pest::Span) -> Result<UnimarkupBlocks, UmError>
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

                    return Err(UmError::General {
                        msg: String::from("Could not parse verbatim block."),
                        error: Box::new(pest_err),
                    });
                }
            }
        }

        Ok(vec![Box::new(block)])
    }
}

impl AsIrLines for VerbatimBlock {
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
    fn parse_from_ir(content_lines: &mut VecDeque<ContentIrLine>) -> Result<Self, UmError>
    where
        Self: Sized,
    {
        if let Some(ir_line) = content_lines.pop_front() {
            let expected_type = UnimarkupType::VerbatimBlock.to_string();

            if ir_line.um_type != expected_type {
                return Err(BackendError::new(format!(
                    "Expected verbatim type to parse, instead got: '{}'",
                    ir_line.um_type
                ))
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
            Err(BackendError::new(
                "Could not construct VerbatimBlock. \nReason: No content ir line available.",
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
    fn render_html(&self) -> Result<String, UmError> {
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
    use crate::frontend::parser::{Rule, UmParse, UnimarkupParser};
    use crate::middleend::*;
    use crate::um_elements::types::UnimarkupType;
    use crate::um_elements::*;
    use crate::um_error::UmError;
    mod render {
        use super::*;

        #[test]
        fn with_lang() -> Result<(), UmError> {
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

            assert_eq!(expected_html, block.render_html()?);

            Ok(())
        }

        #[test]
        fn without_lang() -> Result<(), UmError> {
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

            assert_eq!(expected_html, block.render_html()?);

            Ok(())
        }
    }

    mod parse_from_ir {
        use super::*;

        #[test]
        fn parse() -> Result<(), UmError> {
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

            let verbatim = VerbatimBlock::parse_from_ir(&mut lines)?;

            assert_eq!(verbatim.id, test_id);
            assert_eq!(verbatim.line_nr, 0);
            assert_eq!(verbatim.content, content);
            assert_eq!(verbatim.attributes, String::from("{}"));

            Ok(())
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
        use super::*;

        #[test]
        fn parse() -> Result<(), UmError> {
            let input = "~~~rust
                            fn main() {
                                println!(\"Hello World!\");
                            }\n~~~";

            let expected_id = "verbatim-1";
            let expected_line_nr = 1;
            let expected_type = UnimarkupType::VerbatimBlock.to_string();

            let expected_text = r#"fn main() {
                                println!("Hello World!");
                            }"#;

            let expected_attributes = "{ \"language\": \"rust\" }";

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

            let ir_lines = list.get(0).unwrap().as_ir_lines();

            assert_eq!(ir_lines.len(), 1);

            let line = ir_lines.get(0).unwrap();

            assert_eq!(line.id, expected_id);
            assert_eq!(line.line_nr, expected_line_nr);
            assert_eq!(line.um_type, expected_type);
            assert_eq!(line.text, expected_text);
            assert!(line.fallback_text.is_empty());
            assert_eq!(line.attributes, expected_attributes);
            assert!(line.fallback_attributes.is_empty());

            Ok(())
        }
    }
}
