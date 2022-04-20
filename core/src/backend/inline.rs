use unimarkup_inline::{Inline, InlineKind, NestedInline};

use super::{Render, error::BackendError};


impl Render for Inline {
  fn render_html(&self) -> Result<String, BackendError> {
    let mut output = String::new();

    for inline in self {
      match inline {
        InlineKind::Bold(bold) => {
          output.push_str("<strong>");
          output.push_str(&bold.render_html()?);
          output.push_str("</strong>");
        },
        InlineKind::Italic(italic) => {
          output.push_str("<em>");
          output.push_str(&italic.render_html()?);
          output.push_str("</em>");
        },
        InlineKind::BoldItalic(bold_italic) => {
          output.push_str("<strong><em>");
          output.push_str(&bold_italic.render_html()?);
          output.push_str("</em></strong>");
        },
        InlineKind::Verbatim(verbatim) => {
          output.push_str("<pre>");
          output.push_str(&verbatim.content);
          output.push_str("</pre>");
        },
        InlineKind::Plain(plain)
        | InlineKind::PlainNewLine(plain) => {
          output.push_str(&plain.content);
        },
        InlineKind::EscapedNewLine(_) => {
          output.push_str("<br/>");
        },
        InlineKind::EscapedSpace(_) => {
          output.push_str("&nbsp;")
        },
      }
    }

    Ok(output)
  }
}

impl Render for NestedInline {
  fn render_html(&self) -> Result<String, BackendError> {
    self.content.render_html()
  }
}
