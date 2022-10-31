//! Contains the [`Render`] trait definition.

use logid::log_id::LogId;
use crate::html::Html;

/// Abstract type for Unimarkup elements that implement the [`Render`] trait
pub type RenderBlock = Box<dyn Render>;

/// Trait to provide render implementation for supported output formats
pub trait Render {
    /// Generates the HTML representation of an Unimarkup element.
    ///
    /// Returns a [`LogId`] if it's not possible to generate valid HTML, i.e. if
    /// provided id contains characters not allowed in HTML id attribute.
    fn render_html(&self) -> Result<Html, LogId>;
}
