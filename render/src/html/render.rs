use crate::render::{Context, Renderer};

use super::{Html, HtmlAttribute, HtmlAttributes, HtmlElement};

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
        _verbatim: &unimarkup_parser::elements::enclosed::Verbatim,
        _context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        Err(crate::log_id::RenderError::Unimplemented)
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
        _italic: &unimarkup_inline::NestedContent,
        _context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        Err(crate::log_id::RenderError::Unimplemented)
    }

    fn render_underline(
        &mut self,
        _underline: &unimarkup_inline::NestedContent,
        _context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        Err(crate::log_id::RenderError::Unimplemented)
    }

    fn render_subscript(
        &mut self,
        _subscript: &unimarkup_inline::NestedContent,
        _context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        Err(crate::log_id::RenderError::Unimplemented)
    }

    fn render_superscript(
        &mut self,
        _superscript: &unimarkup_inline::NestedContent,
        _context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        Err(crate::log_id::RenderError::Unimplemented)
    }

    fn render_overline(
        &mut self,
        _overline: &unimarkup_inline::NestedContent,
        _context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        Err(crate::log_id::RenderError::Unimplemented)
    }

    fn render_strikethrough(
        &mut self,
        _strikethrough: &unimarkup_inline::NestedContent,
        _context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        Err(crate::log_id::RenderError::Unimplemented)
    }

    fn render_highlight(
        &mut self,
        _highlight: &unimarkup_inline::NestedContent,
        _context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        Err(crate::log_id::RenderError::Unimplemented)
    }

    fn render_inline_verbatim(
        &mut self,
        _verbatim: &unimarkup_inline::PlainContent,
        _context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        Err(crate::log_id::RenderError::Unimplemented)
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
