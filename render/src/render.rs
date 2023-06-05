//! Contains the [`Render`] trait definition.

use crate::html::Html;

/// Abstract type for Unimarkup elements that implement the [`Render`] trait
pub type RenderBlock = Box<dyn Render>;

/// Trait to provide render implementation for supported output formats
pub trait Render {
    // Note: Rendering must not fail => Type directly instead of Result or Option

    /// Generates the HTML representation of an Unimarkup element.
    fn render_html(&self) -> Html;
}
