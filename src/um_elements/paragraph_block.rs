use std::fmt::Debug;

use crate::{
    backend::Render,
    frontend::parser::{Rule, UmParse},
    middleend::{AsIrLines, ContentIrLine},
    um_elements::types::{self, UnimarkupType, DELIMITER},
};

use pest::iterators::{Pair, Pairs};
use pest::Span;

pub struct ParagraphBlock {
    pub id: String,
    pub content: String,
    pub attributes: String,
    pub line_nr: usize,
}

impl ParagraphBlock {
    fn parse_single(pair: &Pair<Rule>) -> Self {
        let mut id: String = "".to_string();
        id.push_str("paragraph");
        id.push(DELIMITER);

        ParagraphBlock {
            id,
            content: pair.as_str().into(),
            attributes: "{}".into(),
            line_nr: 0,
        }
    }
}

impl UmParse for ParagraphBlock {
    fn parse(
        pairs: &mut Pairs<Rule>,
        span: Span,
    ) -> Result<crate::frontend::UnimarkupBlocks, crate::um_error::UmError>
    where
        Self: Sized,
    {
        let paragraph = pairs.next().expect("hmm");

        let (line_nr, _column_nr) = span.start_pos().line_col();

        let mut paragraph_block = ParagraphBlock::parse_single(&paragraph);

        paragraph_block.line_nr = line_nr;
        paragraph_block.id.push_str(&line_nr.to_string());

        Ok(vec![Box::new(paragraph_block)])
    }
}

impl From<&ParagraphBlock> for Vec<ContentIrLine> {
    fn from(paragraph_block: &ParagraphBlock) -> Self {
        let mut um_type = UnimarkupType::Paragraph.to_string();

        um_type.push(types::DELIMITER);

        let line = ContentIrLine::new(
            &paragraph_block.id,
            paragraph_block.line_nr,
            um_type,
            &paragraph_block.content,
            "",
            &paragraph_block.attributes,
            "",
        );

        vec![line]
    }
}

impl Render for ParagraphBlock {
    fn render_html(&self) -> Result<String, crate::um_error::UmError> {
        let mut html = String::default();

        html.push_str("<p");
        html.push_str(" id='");
        html.push_str(&self.id);
        html.push_str("'>");

        html.push_str(&self.content);
        html.push_str("</p");
        html.push('>');

        Ok(html)
    }
}

impl AsIrLines for ParagraphBlock {
    fn as_ir_lines(&self) -> Vec<crate::middleend::ContentIrLine> {
        self.into()
    }
}

impl Debug for ParagraphBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParagraphBlock")
            .field("id", &self.id)
            .field("content", &self.content)
            .field("attributes", &self.attributes)
            .field("line_nr", &self.line_nr)
            .finish()
    }
}
