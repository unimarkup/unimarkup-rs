use crate::render::{Context, Renderer};

use super::{
    highlight::{self, DEFAULT_THEME},
    Html, HtmlAttribute, HtmlAttributes, HtmlElement,
};

#[derive(Debug, Default)]
pub struct HtmlRenderer {}

impl Renderer<Html> for HtmlRenderer {
    fn render_paragraph(
        &mut self,
        paragraph: &unimarkup_parser::elements::atomic::Paragraph,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_inlines(&paragraph.content, context)?;

        Ok(Html::new_nested("p", HtmlAttributes::default(), inner))
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

        Ok(Html::new_nested(&tag, attributes, inner))
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
        let language = "auto";

        let html = Html::new_with_body(HtmlElement {
            name: "div".to_string(),
            attributes: HtmlAttributes::default(),
            content: Some(highlight::highlight_html_lines(
                &verbatim.content,
                language,
                DEFAULT_THEME,
            )),
        });

        Ok(html)
    }

    fn render_bold(
        &mut self,
        bold: &unimarkup_inline::NestedContent,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(bold, context)?;

        Ok(Html::new_nested("strong", HtmlAttributes::default(), inner))
    }

    fn render_italic(
        &mut self,
        italic: &unimarkup_inline::NestedContent,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(italic, context)?;

        Ok(Html::new_nested("em", HtmlAttributes::default(), inner))
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

        Ok(Html::new_nested("span", attributes, inner))
    }

    fn render_subscript(
        &mut self,
        subscript: &unimarkup_inline::NestedContent,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(subscript, context)?;

        Ok(Html::new_nested("sub", HtmlAttributes::default(), inner))
    }

    fn render_superscript(
        &mut self,
        superscript: &unimarkup_inline::NestedContent,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(superscript, context)?;

        Ok(Html::new_nested("sup", HtmlAttributes::default(), inner))
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

        Ok(Html::new_nested("span", attributes, inner))
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

        Ok(Html::new_nested("span", attributes, inner))
    }

    fn render_highlight(
        &mut self,
        highlight: &unimarkup_inline::NestedContent,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(highlight, context)?;

        Ok(Html::new_nested("mark", HtmlAttributes::default(), inner))
    }

    fn render_quote(
        &mut self,
        quote: &unimarkup_inline::NestedContent,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(quote, context)?;

        Ok(Html::new_nested("q", HtmlAttributes::default(), inner))
    }

    fn render_inline_verbatim(
        &mut self,
        verbatim: &unimarkup_inline::PlainContent,
        _context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let html = Html::new_with_body(HtmlElement {
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
        let html = Html::new_with_body(HtmlElement {
            name: String::default(),
            attributes: HtmlAttributes::default(),
            content: Some(plain.as_string()),
        });

        Ok(html)
    }
}
