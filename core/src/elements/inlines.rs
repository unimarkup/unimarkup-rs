//! Contains helper functions to integrate inlines into the core crate.

use unimarkup_inline::Inline;
use unimarkup_render::{html::Html, render::Render};

/// Helper function to push inline elements to an existing HTML structure.
///
/// # Arguments
///
/// - `html` - the HTML structure inlines are pushed to
/// - `inlines` - the list of inline elements that are pushed on the HTML structure
///
/// Returns [`MappedLogId`] if rendering to HTML fails for one inline element.
pub(crate) fn push_inlines(html: &mut Html, inlines: &Vec<Inline>) {
    let inlines: Html = {
        let mut inline_html = Html::default();

        for inline in inlines {
            let inline_part = inline.render_html();
            inline_html.body.push_str(&inline_part.body);
            inline_html.head.push_str(&inline_part.head);
        }

        inline_html
    };
    html.body.push_str(&inlines.body);
    html.head.push_str(&inlines.head);
}
