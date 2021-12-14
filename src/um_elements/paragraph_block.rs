use std::fmt::Debug;

use pest::iterators::{Pair, Pairs};
use crate::{frontend::{parser::{Rule, UmParse}, UnimarkupBlocks}, backend::Render, middleend::{AsIrLines, ContentIrLine}, um_elements::types::{UnimarkupType, self}};
use pest::Span;

pub struct ParagraphBlock {
    pub id: String,
    pub content: String,
    pub attributes: String,
    pub line_nr: usize,
}

impl UmParse for ParagraphBlock {

    fn parse_multiple(pairs: &mut Pairs<Rule>, span: Span) -> Result<crate::frontend::UnimarkupBlocks, crate::um_error::UmError>
    where
        Self: Sized 
    {
        let paragraph_pairs = pairs
        .next()
        .expect("At least one pair available")
        .into_inner();

        let mut paragraphs: UnimarkupBlocks = Vec::new();

        let (line_nr, _column_nr) = span.start_pos().line_col();

        for pair in paragraph_pairs {
            let mut pargraph = Self::parse(pair);
            pargraph.line_nr += line_nr;
            paragraphs.push(Box::new(pargraph));
        }

        Ok(paragraphs)

    }

    fn parse(pair: Pair<Rule>) -> Self
    where
        Self: Sized,
    {
        let mut paragraph_data = pair.into_inner();
        let paragraph_content = paragraph_data.next().expect( "paragraph rule has paragraph_content");
        let (line_nr, _) = paragraph_content.as_span().start_pos().line_col();

        ParagraphBlock {
            id: line_nr.to_string(), //explicit ID as String, default ID is line_nr
            content: paragraph_content.as_str().into(),
            attributes: "{}".into(),
            line_nr,
        }
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
        f.debug_struct("ParagraphBlock").field("id", &self.id).field("content", &self.content).field("attributes", &self.attributes).field("line_nr", &self.line_nr).finish()
    }
}
