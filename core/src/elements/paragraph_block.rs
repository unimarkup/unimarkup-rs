use std::{collections::HashMap, fmt::Debug};

use crate::{
    backend::{error::BackendError, Render},
    elements::types,
    frontend::{
        error::{custom_pest_error, FrontendError},
        parser::{self, Rule, UmParse},
    },
    log_id::{LogId, SetLog},
};

use pest::iterators::Pairs;
use pest::Span;
use unimarkup_inline::{Inline, ParseUnimarkupInlines};

use super::{error::ElementError, log_id::GeneralErrLogId, types::UnimarkupBlocks};

/// Structure of a Unimarkup paragraph element.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
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

        Ok(vec![paragraph_block.into()])
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

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::ParagraphBlock;
    use crate::backend::Render;
    use unimarkup_inline::{Inline, ParseUnimarkupInlines};

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

        assert_eq!(expected_html, block.render_html().unwrap());
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
                    "<p id='{}'>This is <code>the</code> <em>content</em> <strong>of <sub>the</sub> paragraph</strong></p>",
                    id
                );

        assert_eq!(expected_html, block.render_html().unwrap());
    }
}
