# Unimarkup Render

This crate contains traits and implementations to render Unimarkup elements to all supported output formats.
The [`Render`](src/render.rs) trait defined in this crate is used to implement rendering to an output format.
It is already implemented for all supported formats, but it may be used to add support for other formats, or for alternative rendering implementations. 

## Adding new elements

The `Render` trait must be extended to support new Unimarkup elements.
To not break existing rendering implementations, elements have a default render implementation that returns `RenderError::Unimplemented`.
This allows render implementations to gracefully extend the number of supported elements.

```rust
/// Render a Unimarkup [`Heading`] to the output format `T`.
fn render_heading(&mut self, _heading: &Heading, _context: &Context) -> Result<T, RenderError> {
    Err(RenderError::Unimplemented)
}
```

## Add a new Output Format

To add a new output format, the [`Render`](src/render.rs) must be implemented for this new format.

```rust
#[derive(Debug, Default)]
pub struct HtmlRenderer {}

impl Renderer<Html> for HtmlRenderer {
    fn render_paragraph(
        &mut self,
        paragraph: &unimarkup_parser::elements::atomic::Paragraph,
        context: &Context,
    ) -> Result<Html, crate::log_id::RenderError> {
        let inner = self.render_inlines(&paragraph.content, context)?;

        Ok(Html::nested(HtmlTag::P, HtmlAttributes::default(), inner))
    }

    // Implement additional `render_*` functions to support more elements.
}
```

## Syntax Highlighting

The crate provides syntax highlighting functionality in the `html::highlight` module using the [syntect](https://crates.io/crates/syntect) crate.
This highlighting is only available for output formats that can handle HTML content.

# License

Apache 2.0 Licensed
