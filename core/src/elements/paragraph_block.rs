use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

use crate::{
    backend::{error::BackendError, ParseFromIr, Render},
    elements::types::{self, UnimarkupBlocks, UnimarkupType},
    frontend::{
        error::{custom_pest_error, FrontendError},
        parser::{self, Rule, UmParse},
    },
    log_id::{LogId, SetLog},
    middleend::{AsIrLines, ContentIrLine},
};

use pest::iterators::Pairs;
use pest::Span;
use unimarkup_inline::{Inline, ParseUnimarkupInlines};

use super::{error::ElementError, log_id::GeneralErrLogId};

/// Structure of a Unimarkup paragraph element.
#[derive(Debug, Default, Clone)]
pub struct ParagraphBlock {
    /// Unique identifier for a paragraph.
    pub id: String,

    /// The content of the paragraph.
    pub content: Vec<Inline>,

    /// Attributes of the paragraph.
    pub attributes: String,

    /// Line number, where the paragraph occurs in
    /// the Unimarkup document.
    pub line_nr: usize,
}

impl UmParse for ParagraphBlock {
    fn parse(pairs: &mut Pairs<Rule>, span: Span) -> Result<UnimarkupBlocks, FrontendError>
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
                    ElementError::Atomic(
                        (GeneralErrLogId::InvalidAttribute as LogId)
                            .set_log(
                                &custom_pest_error(
                                    "Paragraph attributes are not valid JSON.",
                                    attributes.as_span(),
                                ),
                                file!(),
                                line!(),
                            )
                            .add_info(&format!("Cause: {}", err)),
                    )
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
                delim = types::DELIMITER
            ))
            .unwrap(),
        };

        let paragraph_block = ParagraphBlock {
            id,
            content,
            attributes: serde_json::to_string(&attributes.unwrap_or_default()).unwrap(),
            line_nr,
        };

        Ok(vec![Box::new(paragraph_block)])
    }
}

impl ParseFromIr for ParagraphBlock {
    fn parse_from_ir(content_lines: &mut VecDeque<ContentIrLine>) -> Result<Self, BackendError>
    where
        Self: Sized,
    {
        if let Some(ir_line) = content_lines.pop_front() {
            let expected_type = UnimarkupType::Paragraph.to_string();

            if ir_line.um_type != expected_type {
                return Err(ElementError::Atomic(
                    (GeneralErrLogId::InvalidElementType as LogId).set_log(
                        &format!(
                            "Expected paragraph type to parse, instead got: '{}'",
                            ir_line.um_type
                        ),
                        file!(),
                        line!(),
                    ),
                )
                .into());
            }

            let content = if !ir_line.text.is_empty() {
                &*ir_line.text
            } else {
                &*ir_line.fallback_text
            }
            .parse_unimarkup_inlines()
            .collect();

            let attributes = if !ir_line.attributes.is_empty() {
                ir_line.attributes
            } else {
                ir_line.fallback_attributes
            };

            let block = ParagraphBlock {
                id: ir_line.id,
                content,
                attributes,
                line_nr: ir_line.line_nr,
            };

            Ok(block)
        } else {
            Err(ElementError::Atomic(
                (GeneralErrLogId::FailedBlockCreation as LogId)
                    .set_log("Could not construct ParagraphBlock.", file!(), line!())
                    .add_info("Cause: No content ir line available."),
            )
            .into())
        }
    }
}

impl Render for ParagraphBlock {
    fn render_html(&self) -> Result<String, BackendError> {
        let mut html = String::default();

        html.push_str("<p");
        html.push_str(" id='");
        html.push_str(&self.id);
        html.push_str("'>");

        let inlines = {
            let mut inline_html = String::new();
            for inline in &self.content {
                inline_html.push_str(&inline.render_html()?);
            }

            inline_html
        };

        html.push_str(&inlines);

        html.push_str("</p>");

        Ok(html)
    }
}

impl AsIrLines<ContentIrLine> for ParagraphBlock {
    fn as_ir_lines(&self) -> Vec<ContentIrLine> {
        let line = ContentIrLine::new(
            &self.id,
            self.line_nr,
            UnimarkupType::Paragraph.to_string(),
            &self
                .content
                .iter()
                .map(|inline| inline.as_string())
                .collect::<String>(),
            "",
            &self.attributes,
            "",
        );

        vec![line]
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{
        backend::{ParseFromIr, Render},
        elements::types::UnimarkupType,
        middleend::ContentIrLine,
    };

    use super::ParagraphBlock;

    #[test]
    fn test__render_html__paragraph() {
        let id = String::from("paragraph-id");
        let content = String::from("This is the content of the paragraph");

        let block = ParagraphBlock {
            id: id.clone(),
            content: content.clone(),
            attributes: "{}".into(),
            line_nr: 0,
        };

        let expected_html = format!("<p id='{}'>{}</p>", id, content);

        assert_eq!(expected_html, block.render_html().unwrap());
    }

    #[test]
    fn test__parse_from_ir__paragraph() {
        let test_id = String::from("test-par-id");
        let content = String::from("This is an example paragraph\nwhich spans multiple lines");

        let mut lines: VecDeque<_> = vec![ContentIrLine {
            id: test_id.clone(),
            line_nr: 0,
            um_type: UnimarkupType::Paragraph.to_string(),
            text: content.clone(),
            attributes: String::from("{}"),
            ..Default::default()
        }]
        .into();

        let paragraph = ParagraphBlock::parse_from_ir(&mut lines).unwrap();

        assert_eq!(paragraph.id, test_id);
        assert_eq!(paragraph.line_nr, 0);
        assert_eq!(paragraph.content, content);
        assert_eq!(paragraph.attributes, String::from("{}"));
    }

    #[test]
    fn test__render_html__paragraph_with_inline() {
        let id = String::from("paragraph-id");
        let content = String::from("This is `the` *content* **of _the_ paragraph**");

        let block = ParagraphBlock {
            id: id.clone(),
            content,
            attributes: "{}".into(),
            line_nr: 0,
        };

        let expected_html = format!(
            "<p id='{}'>This is <pre>the</pre> <i>content</i> <b>of <sub>the</sub> paragraph</b></p>",
            id
        );

        assert_eq!(expected_html, block.render_html().unwrap());
    }

    #[test]
    fn test__parse_from_ir__invalid_paragraph() {
        let mut lines = vec![].into();

        let block_res = ParagraphBlock::parse_from_ir(&mut lines);

        assert!(block_res.is_err());

        let ir_line_bad_type = ContentIrLine {
            id: String::from("some-id"),
            line_nr: 2,
            um_type: format!("{}-more-info", UnimarkupType::Paragraph.to_string()),
            text: String::from("This is the text of this paragraph"),
            ..Default::default()
        };

        lines.push_front(ir_line_bad_type);

        let block_res = ParagraphBlock::parse_from_ir(&mut lines);

        assert!(block_res.is_err());
    }
}
