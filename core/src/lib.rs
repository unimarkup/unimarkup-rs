use std::ffi::OsStr;

pub use unimarkup_commons as commons;
pub use unimarkup_inline as inline;
pub use unimarkup_parser as parser;
pub use unimarkup_render as render;

use crate::commons::config::output::OutputFormatKind;
use crate::commons::config::Config;
use crate::parser::document::Document;
use crate::render::html::render::HtmlRenderer;
use crate::render::html::Html;
use crate::render::log_id::RenderError;
use crate::render::render::{OutputFormat, Renderer};
use crate::render::umi::render::UmiRenderer;
use crate::render::umi::Umi;

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
        match config.input.extension().and_then(OsStr::to_str) {
            Some("umi") => Unimarkup {
                doc: Umi::create_um(um_content, &mut config).unwrap(),
            },
            _ => Unimarkup {
                doc: parser::parse_unimarkup(um_content, &mut config),
            },
        }
    }

    pub fn get_document(&self) -> &Document {
        &self.doc
    }

    pub fn get_formats(&self) -> impl Iterator<Item = &OutputFormatKind> {
        self.doc.output_formats()
    }

    pub fn render<T: OutputFormat>(
        &self,
        format: OutputFormatKind,
        renderer: impl Renderer<T>,
    ) -> Result<T, RenderError> {
        unimarkup_render::render::render(&self.doc, format, renderer)
    }

    pub fn render_html(&self) -> Result<Html, RenderError> {
        self.render(OutputFormatKind::Html, HtmlRenderer::default())
    }

    pub fn render_umi(&self) -> Result<Umi, RenderError> {
        self.render(OutputFormatKind::Umi, UmiRenderer::default())
    }
}
