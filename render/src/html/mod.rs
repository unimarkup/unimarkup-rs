//! Defines the [`Html`] struct that is returned when rendering Unimarkup to HTML.

use crate::render::OutputFormat;

pub mod highlight;
pub mod render;

#[derive(Debug, Default)]
pub struct HtmlAttribute {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Default)]
pub struct HtmlElement {
    pub name: String,
    pub attributes: HtmlAttributes,
    pub content: Option<String>,
}

// Note: Both structs below are needed to implement `Display` for `Vec`.

#[derive(Debug, Default)]
pub struct HtmlAttributes(Vec<HtmlAttribute>);
#[derive(Debug, Default)]
pub struct HtmlElements(Vec<HtmlElement>);

#[derive(Debug, Default)]
pub struct Html {
    pub head: HtmlElements,
    pub body: HtmlElements,
    pub lang: String,
}

impl Html {
    pub fn with_head(element: HtmlElement) -> Self {
        let mut html = Html::default();
        html.head.push(element);
        html
    }

    pub fn with_body(element: HtmlElement) -> Self {
        let mut html = Html::default();
        html.body.push(element);
        html
    }

    pub fn with(head: HtmlElement, body: HtmlElement) -> Self {
        let mut html = Html::default();
        html.head.push(head);
        html.body.push(body);
        html
    }

    pub fn nested(outer_name: &str, outer_attributes: HtmlAttributes, mut inner: Self) -> Self {
        let mut html = Html::with_body(HtmlElement {
            name: outer_name.to_string(),
            attributes: outer_attributes,
            content: Some(inner.body.to_string()),
        });
        html.head.append(&mut inner.head);

        html
    }
}

impl OutputFormat for Html {
    fn new(_context: &crate::render::Context) -> Self {
        Html {
            head: HtmlElements(Vec::new()),
            body: HtmlElements(Vec::new()),
            lang: "en-US".to_string(),
        }
    }

    fn append(&mut self, mut other: Self) -> Result<(), crate::log_id::RenderError> {
        self.head.append(&mut other.head);
        self.body.append(&mut other.body);

        Ok(())
    }
}

impl std::fmt::Display for Html {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<!DOCTYPE HTML><html lang=\"{}\"><head>{}</head><body>{}</body></html>",
            self.lang, self.head, self.body
        )
    }
}

impl std::fmt::Display for HtmlElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // No name -> treat as plain content
        if let (true, Some(content)) = (self.name.is_empty(), &self.content) {
            return write!(f, "{}", content);
        }

        let mut element = format!("<{}", self.name);

        if !self.attributes.is_empty() {
            element.push_str(&format!("{}", self.attributes));
        }

        match &self.content {
            Some(content) => element.push_str(&format!(">{}</{}>", content, self.name)),
            None => element.push_str("/>"),
        }

        write!(f, "{}", element)
    }
}

impl From<Vec<HtmlElement>> for HtmlElements {
    fn from(value: Vec<HtmlElement>) -> Self {
        HtmlElements(value)
    }
}

impl std::fmt::Display for HtmlElements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for elem in &self.0 {
            write!(f, "{}", elem)?;
        }
        Ok(())
    }
}

impl std::ops::Deref for HtmlElements {
    type Target = Vec<HtmlElement>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for HtmlElements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for HtmlAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(value) => write!(f, "{}='{}'", self.name, value),
            None => write!(f, "{}", self.name),
        }
    }
}

impl From<Vec<HtmlAttribute>> for HtmlAttributes {
    fn from(value: Vec<HtmlAttribute>) -> Self {
        HtmlAttributes(value)
    }
}

impl std::fmt::Display for HtmlAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for attrb in &self.0 {
            // Note: *whitespace* at beginning is important, because `HtmlElement` does not set one after open tag.
            // This way no unnecessary whitespace is left at the end of the open tag.
            write!(f, " {}", attrb)?;
        }
        Ok(())
    }
}

impl std::ops::Deref for HtmlAttributes {
    type Target = Vec<HtmlAttribute>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for HtmlAttributes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
