use crate::error::UmError;

use super::RenderBlock;

/// Used to provide render implementation for supported output formats
pub trait Render {
    /// Generates the HTML representation of a type that implements this trait.
    ///
    /// Returns [`UmError`] if it's not possible to generate valid HTML, i.e. if
    /// provided id contains characters not allowed in HTML id attribute.
    fn render_html(&self) -> Result<String, UmError>;
}

/// Renders all [`RenderBlock`](crate::backend::RenderBlock)s
/// and returns the resulting HTML as [`String`].
///
/// # Arguments
///
/// - `blocks` - array of type [`RenderBlock`]
///
/// Returns [`UmError`], if any of the given blocks returns an [`UmError`]
/// when rendering itself.
pub fn render_html(blocks: &[RenderBlock]) -> Result<String, UmError> {
    let mut html = String::default();

    for block in blocks {
        html.push_str(&block.render_html()?);
    }

    Ok(html)
}
