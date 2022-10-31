//! Contains the [`Render`] trait definition.

use logid::capturing::MappedLogId;
use crate::html::Html;

/// Abstract type for Unimarkup elements that implement the [`Render`] trait
pub type RenderBlock = Box<dyn Render>;

/// Trait to provide render implementation for supported output formats
pub trait Render {
    /// Generates the HTML representation of an Unimarkup element.
    ///
    /// Returns a [`MappedLogId`] if it's not possible to generate valid HTML, i.e. if
    /// provided id contains characters not allowed in HTML id attribute.
    fn render_html(&self) -> Result<Html, MappedLogId>;
}
