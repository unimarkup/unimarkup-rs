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
                _ => continue,
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
                "Could not construct ParagraphBlock. \nReason: No content ir line available.",
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
        res.push_str(" 'id=");
        res.push_str(&self.id);

        if let Some(language) = attributes.language {
            res.push_str(" class='language-");
            res.push_str(&language.trim().to_lowercase());
        }

        res.push_str("'>");
        res.push_str(&self.content);
        res.push_str("</code></pre>");

        Ok(res)
    }
}
