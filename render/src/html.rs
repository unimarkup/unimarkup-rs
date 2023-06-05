//! Defines the [`Html`] struct that is returned by the [`Render`](crate::render::Render) trait when rendering Unimarkup to HTML.

use crate::render::RenderBlock;

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
pub fn render_html(blocks: &[RenderBlock]) -> Html {
    let mut html = Html::default();

    for block in blocks {
        let html_part = block.render_html();
        html.body.push_str(&html_part.body);
        html.head.push_str(&html_part.head);
    }

    html
}
