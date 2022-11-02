# Unimarkup Render

This crate defines types and traits used in the [inline](../inline/README.md) and [core](../core/README.md) crate to render Unimarkup elements to all supported output formats.
Therefore, all Unimarkup elements must implement the [`Render`](src/render.rs) trait defined in this crate.

## Add a new Unimarkup Element

Newly added Unimarkup elements must implement the [`Render`](src/render.rs) trait.

```rust
impl Render for NewElement {
  fn render_html(&self) -> Result<Html, LogId> {
    // implement rendering to HTML for this new element
  }
}
```

## Add a new Output Format

To add a new output format, the [`Render`](src/render.rs) trait must be extended.

**Note:** This affects **all** Unimarkup elements implementing the [`Render`](src/render.rs) trait! 

```rust
pub trait Render {
  // ...
  // previous output formats
  // ...

  /// Renders Unimarkup to the new output format
  fn render_new_format(&self) => Result<NewFormat, LogId>;
}
```

## Syntax Highlighting

The crate also provides syntax highlighting functionality in the `highlight` module using the [syntect](https://crates.io/crates/syntect) crate.
