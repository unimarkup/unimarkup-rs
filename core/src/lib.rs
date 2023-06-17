use unimarkup_commons::config::output::OutputFormatKind;
use unimarkup_commons::config::Config;
use unimarkup_parser::document::Document;
use unimarkup_parser::parser;
use unimarkup_render::html::render::HtmlRenderer;
use unimarkup_render::html::Html;
use unimarkup_render::log_id::RenderError;

pub struct Unimarkup {
    doc: Document,
}

impl Unimarkup {
    /// Parses Unimarkup content, and returns a [`Unimarkup`] struct to render the content to supported formats.
    ///
    /// # Arguments
    ///
    /// * `um_content` - String containing Unimarkup elements.
    /// * `config` - Unimarkup configuration to be used on top of preambles.
    pub fn parse(um_content: &str, mut config: Config) -> Self {
        if um_content.is_empty() {
            return Unimarkup {
                doc: Document {
                    blocks: vec![],
                    config,
                    ..Default::default()
                },
            };
        }

        Unimarkup {
            doc: parser::parse_unimarkup(um_content, &mut config),
        }
    }

    pub fn get_document(&self) -> &Document {
        &self.doc
    }

    pub fn render_formats(&self) -> Result<Vec<RenderedOutput>, RenderError> {
        let mut outputs = Vec::new();

        for format in self.doc.output_formats() {
            outputs.push(self.render_format(*format)?);
        }

        Ok(outputs)
    }

    pub fn render_format(&self, format: OutputFormatKind) -> Result<RenderedOutput, RenderError> {
        match format {
            OutputFormatKind::Html => Ok(RenderedOutput {
                content: self.render_html()?.to_string(),
                kind: format,
            }),
        }
    }

    pub fn render_html(&self) -> Result<Html, RenderError> {
        unimarkup_render::render::render(&self.doc, HtmlRenderer::default())
    }
}

pub struct RenderedOutput {
    pub content: String,
    pub kind: OutputFormatKind,
}
