use std::{collections::VecDeque, fmt::Debug};

use crate::{
    backend::{BackendError, ParseFromIr, Render},
    frontend::{
        parser::{Rule, UmParse},
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
            let block = ParagraphBlock {
                id: ir_line.id,
                content: ir_line.text,
                attributes: ir_line.attributes,
                line_nr: ir_line.line_nr,
            };

            Ok(block)
        } else {
            Err(BackendError::new(
                "Could not construct ParagraphBlock. \nReason: No content ir lines available.",
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

        html.push_str(&self.content);
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
        backend::ParseFromIr, middleend::ContentIrLine, um_elements::types::UnimarkupType,
        um_error::UmError,
    };

    use super::ParagraphBlock;

    #[test]
    fn parse_from_ir() -> Result<(), UmError> {
        let mut lines: VecDeque<_> = vec![ContentIrLine {
            id: String::from("test-par-id"),
            line_nr: 0,
            um_type: UnimarkupType::Paragraph.to_string(),
            text: String::from("This is an example paragraph\nwhich spans multiple lines"),
            attributes: String::from("{}"),
            ..Default::default()
        }]
        .into();

        let paragraph = ParagraphBlock::parse_from_ir(&mut lines)?;

        assert_eq!(paragraph.id, String::from("test-par-id"));
        assert_eq!(paragraph.line_nr, 0);
        assert_eq!(
            paragraph.content,
            String::from("This is an example paragraph\nwhich spans multiple lines"),
        );
        assert_eq!(paragraph.attributes, String::from("{}"));

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
