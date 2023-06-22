use crate::render::{Context, Renderer};

use super::{highlight, Html, HtmlAttribute, HtmlAttributes, HtmlElement, HtmlHead};

#[derive(Debug, Default)]
pub struct HtmlRenderer {}

impl Renderer<Html> for HtmlRenderer {
    fn render_paragraph(
        &mut self,
        paragraph: &unimarkup_parser::elements::atomic::Paragraph,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_inlines(&paragraph.content, context)?;

        Ok(Html::nested("p", HtmlAttributes::default(), inner))
    }

    fn render_heading(
        &mut self,
        heading: &unimarkup_parser::elements::atomic::Heading,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_inlines(&heading.content, context)?;
        let tag = format!("h{}", u8::from(heading.level));

        let attributes = HtmlAttributes::from(vec![HtmlAttribute {
            name: "id".to_string(),
            value: Some(heading.id.clone()),
        }]);

        Ok(Html::nested(&tag, attributes, inner))
    }

    fn render_verbatim_block(
        &mut self,
        verbatim: &unimarkup_parser::elements::enclosed::Verbatim,
        _context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        // TODO: improve handling of attributes
        // let attributes = serde_json::from_str::<VerbatimAttributes>(
        //     &verbatim.attributes.as_ref().cloned().unwrap_or_default(),
        // )
        // .ok();

        // let language = match attributes.as_ref() {
        //     Some(attrs) => attrs.language.clone().unwrap_or(PLAIN_SYNTAX.to_string()),
        //     None => PLAIN_SYNTAX.to_string(),
        // };
        let language = "rust";

        let inner = Html::with(
            HtmlHead {
                syntax_highlighting_used: true,
                ..Default::default()
            },
            HtmlElement {
                name: "code".to_string(),
                attributes: HtmlAttributes::default(),
                content: Some(
                    highlight::highlight_content(&verbatim.content, language)
                        .unwrap_or(verbatim.content.clone()),
                ),
            },
        );

        Ok(Html::nested("pre", HtmlAttributes::default(), inner))
    }

    fn render_bold(
        &mut self,
        bold: &unimarkup_inline::NestedContent,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(bold, context)?;

        Ok(Html::nested("strong", HtmlAttributes::default(), inner))
    }

    fn render_italic(
        &mut self,
        italic: &unimarkup_inline::NestedContent,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(italic, context)?;

        Ok(Html::nested("em", HtmlAttributes::default(), inner))
    }

    fn render_underline(
        &mut self,
        underline: &unimarkup_inline::NestedContent,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(underline, context)?;
        let mut attributes = HtmlAttributes::default();
        attributes.push(HtmlAttribute {
            name: "style".to_string(),
            value: Some("text-decoration: underline;".to_string()),
        });

        Ok(Html::nested("span", attributes, inner))
    }

    fn render_subscript(
        &mut self,
        subscript: &unimarkup_inline::NestedContent,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(subscript, context)?;

        Ok(Html::nested("sub", HtmlAttributes::default(), inner))
    }

    fn render_superscript(
        &mut self,
        superscript: &unimarkup_inline::NestedContent,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(superscript, context)?;

        Ok(Html::nested("sup", HtmlAttributes::default(), inner))
    }

    fn render_overline(
        &mut self,
        overline: &unimarkup_inline::NestedContent,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(overline, context)?;
        let mut attributes = HtmlAttributes::default();
        attributes.push(HtmlAttribute {
            name: "style".to_string(),
            value: Some("text-decoration: overline;".to_string()),
        });

        Ok(Html::nested("span", attributes, inner))
    }

    fn render_strikethrough(
        &mut self,
        strikethrough: &unimarkup_inline::NestedContent,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(strikethrough, context)?;
        let mut attributes = HtmlAttributes::default();
        attributes.push(HtmlAttribute {
            name: "style".to_string(),
            value: Some("text-decoration: line-through;".to_string()),
        });

        Ok(Html::nested("span", attributes, inner))
    }

    fn render_highlight(
        &mut self,
        highlight: &unimarkup_inline::NestedContent,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(highlight, context)?;

        Ok(Html::nested("mark", HtmlAttributes::default(), inner))
    }

    fn render_quote(
        &mut self,
        quote: &unimarkup_inline::NestedContent,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(quote, context)?;

        Ok(Html::nested("q", HtmlAttributes::default(), inner))
    }

    fn render_inline_verbatim(
        &mut self,
        verbatim: &unimarkup_inline::PlainContent,
        _context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let html = Html::with_body(HtmlElement {
            name: "code".to_string(),
            attributes: HtmlAttributes::default(),
            content: Some(verbatim.as_string()),
        });

        Ok(html)
    }

    fn render_plain(
        &mut self,
        plain: &unimarkup_inline::PlainContent,
        _context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let html = Html::with_body(HtmlElement {
            name: String::default(),
            attributes: HtmlAttributes::default(),
            content: Some(plain.as_string()),
        });

        Ok(html)
    }
}
