use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

use crate::{
    backend::ParseFromIr,
    elements::types::{self, ElementType},
    frontend::parser::{self, custom_pest_error, Rule, UmParse},
    log_id::CORE_LOG_ID_MAP,
    middleend::{AsIrLines, ContentIrLine},
};

use logid::{
    capturing::{LogIdTracing, MappedLogId},
    log_id::LogId,
};
use pest::iterators::Pairs;
use pest::Span;
use unimarkup_inline::{Inline, ParseUnimarkupInlines};
use unimarkup_render::{html::Html, render::Render};

use super::{inlines, log_id::GeneralErrLogId, UnimarkupBlocks};

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
    fn parse(pairs: &mut Pairs<Rule>, span: Span) -> Result<UnimarkupBlocks, MappedLogId>
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
                    (GeneralErrLogId::InvalidAttribute as LogId)
                        .set_event_with(
                            &CORE_LOG_ID_MAP,
                            &custom_pest_error(
                                "Paragraph attributes are not valid JSON.",
                                attributes.as_span(),
                            ),
                            file!(),
                            line!(),
                        )
                        .add_info(&format!("Cause: {}", err))
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
                delim = types::ELEMENT_TYPE_DELIMITER
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
    fn parse_from_ir(content_lines: &mut VecDeque<ContentIrLine>) -> Result<Self, MappedLogId>
    where
        Self: Sized,
    {
        if let Some(ir_line) = content_lines.pop_front() {
            let expected_type = ElementType::Paragraph.to_string();

            if ir_line.um_type != expected_type {
                return Err(
                    (GeneralErrLogId::InvalidElementType as LogId).set_event_with(
                        &CORE_LOG_ID_MAP,
                        &format!(
                            "Expected paragraph type to parse, instead got: '{}'",
                            ir_line.um_type
                        ),
                        file!(),
                        line!(),
                    ),
                );
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
            Err((GeneralErrLogId::FailedBlockCreation as LogId)
                .set_event_with(
                    &CORE_LOG_ID_MAP,
                    "Could not construct ParagraphBlock.",
                    file!(),
                    line!(),
                )
                .add_cause("No content ir line available."))
        }
    }
}

impl Render for ParagraphBlock {
    fn render_html(&self) -> Result<Html, MappedLogId> {
        let mut html = Html::default();

        html.body.push_str("<p");
        html.body.push_str(" id='");
        html.body.push_str(&self.id);
        html.body.push_str("'>");

        inlines::push_inlines(&mut html, &self.content)?;

        html.body.push_str("</p>");

        Ok(html)
    }
}

impl AsIrLines<ContentIrLine> for ParagraphBlock {
    fn as_ir_lines(&self) -> Vec<ContentIrLine> {
        let line = ContentIrLine::new(
            &self.id,
            self.line_nr,
            ElementType::Paragraph.to_string(),
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

    use unimarkup_inline::{Inline, ParseUnimarkupInlines};
    use unimarkup_render::render::Render;

    use crate::{backend::ParseFromIr, elements::types::ElementType, middleend::ContentIrLine};

    use super::ParagraphBlock;

    #[test]
    fn test__render_html__paragraph() {
        let id = String::from("paragraph-id");
        let content: Vec<Inline> = "This is the content of the paragraph"
            .parse_unimarkup_inlines()
            .collect();

        let block = ParagraphBlock {
            id: id.clone(),
            content: content.clone(),
            attributes: "{}".into(),
            line_nr: 0,
        };

        let expected_html = format!("<p id='{}'>{}</p>", id, content[0].as_string());

        assert_eq!(expected_html, block.render_html().unwrap().body);
    }

    #[test]
    fn test__parse_from_ir__paragraph() {
        let test_id = String::from("test-par-id");
        let content: Vec<Inline> = "This is an example paragraph\nwhich spans multiple lines"
            .parse_unimarkup_inlines()
            .collect();

        let mut lines: VecDeque<_> = vec![ContentIrLine {
            id: test_id.clone(),
            line_nr: 0,
            um_type: ElementType::Paragraph.to_string(),
            text: content.iter().map(|inline| inline.as_string()).collect(),
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
        let content = "This is `the` *content* **of _the_ paragraph**"
            .parse_unimarkup_inlines()
            .collect();

        let block = ParagraphBlock {
            id: id.clone(),
            content,
            attributes: "{}".into(),
            line_nr: 0,
        };

        let expected_html = format!(
                    "<p id='{}'>This is <pre><code>the</code></pre> <em>content</em> <strong>of <sub>the</sub> paragraph</strong></p>",
                    id
                );

        assert_eq!(expected_html, block.render_html().unwrap().body);
    }

    #[test]
    fn test__parse_from_ir__invalid_paragraph() {
        let mut lines = vec![].into();

        let block_res = ParagraphBlock::parse_from_ir(&mut lines);

        assert!(block_res.is_err());

        let ir_line_bad_type = ContentIrLine {
            id: String::from("some-id"),
            line_nr: 2,
            um_type: format!("{}-more-info", ElementType::Paragraph),
            text: String::from("This is the text of this paragraph"),
            ..Default::default()
        };

        lines.push_front(ir_line_bad_type);

        let block_res = ParagraphBlock::parse_from_ir(&mut lines);

        assert!(block_res.is_err());
    }
}
