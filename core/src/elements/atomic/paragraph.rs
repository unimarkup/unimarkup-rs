use std::{
    collections::HashMap,
    fmt::Debug,
};

use logid::{
    capturing::{LogIdTracing, MappedLogId},
    log_id::LogId,
};
use pest::iterators::Pairs;
use pest::Span;
use unimarkup_inline::{Inline, ParseUnimarkupInlines};
use unimarkup_render::{html::Html, render::Render};

use crate::elements::{inlines, log_id::GeneralErrLogId, Blocks};
use crate::{
    elements::types,
    frontend::parser::{self, custom_pest_error, Rule, UmParse},
    log_id::CORE_LOG_ID_MAP,
};

/// Structure of a Unimarkup paragraph element.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Paragraph {
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

impl UmParse for Paragraph {
    fn parse(pairs: &mut Pairs<Rule>, span: Span) -> Result<Blocks, MappedLogId>
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

        let paragraph_block = Paragraph {
            id,
            content,
            attributes: serde_json::to_string(&attributes.unwrap_or_default()).unwrap(),
            line_nr,
        };

        Ok(vec![paragraph_block.into()])
    }
}

impl Render for Paragraph {
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

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use unimarkup_inline::{Inline, ParseUnimarkupInlines};
    use unimarkup_render::render::Render;
    use super::Paragraph;

    #[test]
    fn test__render_html__paragraph() {
        let id = String::from("paragraph-id");
        let content: Vec<Inline> = "This is the content of the paragraph"
            .parse_unimarkup_inlines()
            .collect();

        let block = Paragraph {
            id: id.clone(),
            content: content.clone(),
            attributes: "{}".into(),
            line_nr: 0,
        };

        let expected_html = format!("<p id='{}'>{}</p>", id, content[0].as_string());

        assert_eq!(expected_html, block.render_html().unwrap().body);
    }

    #[test]
    fn test__render_html__paragraph_with_inline() {
        let id = String::from("paragraph-id");
        let content = "This is `the` *content* **of _the_ paragraph**"
            .parse_unimarkup_inlines()
            .collect();

        let block = Paragraph {
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
}
