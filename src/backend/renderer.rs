use crate::um_error::UmError;

use super::RenderBlock;

/// Used to provide render implementation into various formats
/// for the given UnimarkupBlock
pub trait Render {
    /// Generates the HTML representation of the UnimarkupBlock
    ///
    /// Returns [`UmError`] if it's not possible to generate valid HTML, i.e. if
    /// provided id contains characters not allowed in HTML id attribute.
    fn render_html(&self) -> Result<String, UmError>;
}

/// Renders all [`UnimarkupType`](crate::um_elements::types::UnimarkupType)
/// and returns resulting HTML as [`String`].
///
/// Returns [`UmError`] any of the UnimarkupBlocks returns an [`UmError`]
/// when rendering itself.
pub fn render_html(blocks: &[RenderBlock]) -> Result<String, UmError> {
    let mut html = String::default();

    for block in blocks {
        html.push_str(&block.render_html()?);
    }

    Ok(html)
}
