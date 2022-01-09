use std::{collections::VecDeque, fmt::Debug};

use crate::{
    backend::{BackendError, ParseFromIr, Render, inline_formatting::render_inline_umblocks},
    frontend::{
        parser::{Rule, UmParse, self},
        UnimarkupBlocks,
    },
    middleend::{AsIrLines, ContentIrLine},
    um_elements::types::{self, UnimarkupType},
    um_error::UmError,
};

use pest::iterators::Pairs;
use pest::Span;

#[derive(Debug, Default)]
pub struct ParagraphBlock {
    pub id: String,
    pub content: String,
    pub attributes: String,
    pub line_nr: usize,
}

impl UmParse for ParagraphBlock {
    fn parse(pairs: &mut Pairs<Rule>, span: Span) -> Result<UnimarkupBlocks, UmError>
    where
        Self: Sized,
    {
        let paragraph = pairs.next().expect("paragraph must be there at this point");

        let (line_nr, _column_nr) = span.start_pos().line_col();

        let mut id: String = "".to_string();
        id.push_str("paragraph");
        id.push(types::DELIMITER);
        id.push_str(&line_nr.to_string());

        let paragraph_block = ParagraphBlock {
            id,
            content: paragraph.as_str().into(),
            attributes: "{}".into(),
            line_nr,
        };

        Ok(vec![Box::new(paragraph_block)])
    }
}

impl ParseFromIr for ParagraphBlock {
    fn parse_from_ir(content_lines: &mut VecDeque<ContentIrLine>) -> Result<Self, UmError>
    where
        Self: Sized,
    {
        if let Some(ir_line) = content_lines.pop_front() {
            let expected_type = UnimarkupType::Paragraph.to_string();

            if ir_line.um_type != expected_type {
                return Err(BackendError::new(format!(
                    "Expected paragraph type to parse, instead got: '{}'",
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

            let block = ParagraphBlock {
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

impl Render for ParagraphBlock {
    fn render_html(&self) -> Result<String, UmError> {
        
        
        let mut html = String::default();

        html.push_str("<p");
        html.push_str(" id='");
        html.push_str(&self.id);
        html.push_str("'>");

        let inline = parser::parse_inline(&self.content).expect("Inline formatting or plain text expected.");
        render_inline_umblocks(&mut html, inline);

        html.push_str("</p>");

        Ok(html)
    }
}

impl AsIrLines for ParagraphBlock {
    fn as_ir_lines(&self) -> Vec<ContentIrLine> {
        let line = ContentIrLine::new(
            &self.id,
            self.line_nr,
            UnimarkupType::Paragraph.to_string(),
            &self.content,
            "",
            &self.attributes,
            "",
        );

        vec![line]
    }
}

#[cfg(test)]
mod paragraph_tests {
    use std::collections::VecDeque;

    use crate::{
        backend::{ParseFromIr, Render},
        middleend::ContentIrLine,
        um_elements::types::UnimarkupType,
        um_error::UmError,
    };

    use super::ParagraphBlock;

    #[test]
    fn render_paragraph_html() -> Result<(), UmError> {
        let id = String::from("paragraph-id");
        let content = String::from("This is the content of the heading");

        let block = ParagraphBlock {
            id: id.clone(),
            content: content.clone(),
            attributes: "{}".into(),
            line_nr: 0,
        };

        let expected_html = format!("<p id='{}'>{}</p>", id, content);

        assert_eq!(expected_html, block.render_html()?);

        Ok(())
    }

    #[test]
    fn parse_from_ir() -> Result<(), UmError> {
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

        let paragraph = ParagraphBlock::parse_from_ir(&mut lines)?;

        assert_eq!(paragraph.id, test_id);
        assert_eq!(paragraph.line_nr, 0);
        assert_eq!(paragraph.content, content);
        assert_eq!(paragraph.attributes, String::from("{}"));

        Ok(())
    }

    #[test]
    fn render_paragraph_with_inline_html() -> Result<(), UmError> {
        let id = String::from("paragraph-id");
        let content = String::from("This is `the` *content* **of _the_ heading**");

        let block = ParagraphBlock {
            id: id.clone(),
            content,
            attributes: "{}".into(),
            line_nr: 0,
        };

        let expected_html = format!("<p id='{}'>This is <pre>the</pre> <i>content</i> <b>of <sub>the</sub> heading</b></p>", id);

        assert_eq!(expected_html, block.render_html()?);

        Ok(())
    }

    #[test]
    fn parse_from_ir_bad() {
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
