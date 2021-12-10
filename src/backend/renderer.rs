use crate::um_error::UmError;

use super::RenderBlock;

pub trait Render {
    fn render_html(&self) -> Result<String, UmError>;
}

/// # Render HTML
///
/// Render all [`UnimarkupType`](crate::um_elements::types::UnimarkupType) and return resulting HTML as [`String`]
pub fn render_html(blocks: &[RenderBlock]) -> Result<String, UmError> {
    let mut html = String::default();

    for block in blocks {
        html.push_str(&block.render_html()?);
    }

    Ok(html)
}
