use unimarkup_inline::types::*;

use crate::render::{Context, Renderer};

use super::{
    highlight, tag::HtmlTag, Html, HtmlAttribute, HtmlAttributes, HtmlBody, HtmlElement, HtmlHead,
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

        Ok(Html::nested(HtmlTag::P, HtmlAttributes::default(), inner))
    }

    fn render_heading(
        &mut self,
        heading: &unimarkup_parser::elements::atomic::Heading,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_inlines(&heading.content, context)?;
        let tag = HtmlTag::from(heading.level);

        let attributes = HtmlAttributes::from(vec![HtmlAttribute {
            name: "id".to_string(),
            value: Some(heading.id.clone()),
        }]);

        Ok(Html::nested(tag, attributes, inner))
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
            HtmlBody::from(HtmlElement {
                tag: HtmlTag::Code,
                attributes: HtmlAttributes::default(),
                content: Some(
                    highlight::highlight_content(&verbatim.content, language)
                        .unwrap_or(verbatim.content.clone()),
                ),
            }),
        );

        Ok(Html::nested(HtmlTag::Pre, HtmlAttributes::default(), inner))
    }

    fn render_bold(
        &mut self,
        bold: &Bold,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(bold.inner(), context)?;

        Ok(Html::nested(
            HtmlTag::Strong,
            HtmlAttributes::default(),
            inner,
        ))
    }

    fn render_italic(
        &mut self,
        italic: &Italic,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(italic.inner(), context)?;

        Ok(Html::nested(HtmlTag::Em, HtmlAttributes::default(), inner))
    }

    fn render_underline(
        &mut self,
        underline: &Underline,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(underline.inner(), context)?;
        let mut attributes = HtmlAttributes::default();
        attributes.push(HtmlAttribute {
            name: "style".to_string(),
            value: Some("text-decoration: underline;".to_string()),
        });

        Ok(Html::nested(HtmlTag::Span, attributes, inner))
    }

    fn render_subscript(
        &mut self,
        subscript: &Subscript,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(subscript.inner(), context)?;

        Ok(Html::nested(HtmlTag::Sub, HtmlAttributes::default(), inner))
    }

    fn render_superscript(
        &mut self,
        superscript: &Superscript,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(superscript.inner(), context)?;

        Ok(Html::nested(HtmlTag::Sup, HtmlAttributes::default(), inner))
    }

    fn render_overline(
        &mut self,
        overline: &Overline,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(overline.inner(), context)?;
        let mut attributes = HtmlAttributes::default();
        attributes.push(HtmlAttribute {
            name: "style".to_string(),
            value: Some("text-decoration: overline;".to_string()),
        });

        Ok(Html::nested(HtmlTag::Span, attributes, inner))
    }

    fn render_strikethrough(
        &mut self,
        strikethrough: &Strikethrough,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(strikethrough.inner(), context)?;
        let mut attributes = HtmlAttributes::default();
        attributes.push(HtmlAttribute {
            name: "style".to_string(),
            value: Some("text-decoration: line-through;".to_string()),
        });

        Ok(Html::nested(HtmlTag::Span, attributes, inner))
    }

    fn render_highlight(
        &mut self,
        highlight: &Highlight,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(highlight.inner(), context)?;

        Ok(Html::nested(
            HtmlTag::Mark,
            HtmlAttributes::default(),
            inner,
        ))
    }

    fn render_quote(
        &mut self,
        quote: &Quote,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_nested_inline(quote.inner(), context)?;

        Ok(Html::nested(HtmlTag::Q, HtmlAttributes::default(), inner))
    }

    fn render_inline_verbatim(
        &mut self,
        verbatim: &Verbatim,
        _context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let html = Html::with_body(HtmlBody::from(HtmlElement {
            tag: HtmlTag::Code,
            attributes: HtmlAttributes::default(),
            content: Some(verbatim.inner().to_string()),
        }));

        Ok(html)
    }

    fn render_plain(
        &mut self,
        plain: &Plain,
        _context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let html = Html::with_body(HtmlBody::from(HtmlElement {
            tag: HtmlTag::PlainContent,
            attributes: HtmlAttributes::default(),
            content: Some(plain.inner().to_string()),
        }));

        Ok(html)
    }

    fn render_newline(
        &mut self,
        _newline: &Newline,
        _context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let html = Html::with_body(HtmlBody::from(HtmlElement {
            tag: HtmlTag::PlainContent,
            attributes: HtmlAttributes::default(),
            content: Some(unimarkup_inline::TokenKind::Whitespace.as_str().to_string()),
        }));

        Ok(html)
    }

    fn render_escaped_newline(
        &mut self,
        _escaped_newline: &EscapedNewline,
        _context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let html = Html::with_body(HtmlBody::from(HtmlElement {
            tag: HtmlTag::Br,
            attributes: HtmlAttributes::default(),
            content: None,
        }));

        Ok(html)
    }

    fn render_escaped_whitespace(
        &mut self,
        escaped_whitespace: &EscapedWhitespace,
        _context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let html = Html::with_body(HtmlBody::from(HtmlElement {
            tag: HtmlTag::Span,
            attributes: HtmlAttributes(vec![HtmlAttribute {
                name: "style".to_string(),
                value: Some("white-space: pre-wrap;".to_string()),
            }]),
            content: Some(escaped_whitespace.inner().to_string()),
        }));

        Ok(html)
    }
}
