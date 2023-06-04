//! Defines the [`Html`] struct that is returned by the [`Render`](crate::render::Render) trait when rendering Unimarkup to HTML.

use crate::render::{RenderBlock, RenderErr};

#[derive(Debug, Default)]
pub struct Html {
    pub head: String,
    pub body: String,
}

/// Renders all [`RenderBlock`]s and returns the resulting [`Html`] structure.
///
/// # Arguments
///
/// - `blocks` - array of type [`RenderBlock`]
///
/// Returns a [`RenderErr`], if any of the given blocks return a [`MappedLogId`]
/// when rendering themself.
pub fn render_html(blocks: &[RenderBlock]) -> Result<Html, RenderErr> {
    let mut html = Html::default();

    for block in blocks {
        let html_part = block.render_html()?;
        html.body.push_str(&html_part.body);
        html.head.push_str(&html_part.head);
    }

    Ok(html)
}
