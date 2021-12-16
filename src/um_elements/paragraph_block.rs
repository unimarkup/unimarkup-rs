use std::fmt::Debug;

use crate::{
    backend::Render,
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
        let mut um_type = UnimarkupType::Paragraph.to_string();

        um_type.push(types::DELIMITER);

        let line = ContentIrLine::new(
            &self.id,
            self.line_nr,
            um_type,
            &self.content,
            "",
            &self.attributes,
            "",
        );

        vec![line]
    }
}
