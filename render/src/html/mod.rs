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

#[derive(Debug, Default)]
pub struct HtmlAttributes(Vec<HtmlAttribute>);
#[derive(Debug, Default)]
pub struct HtmlElements(Vec<HtmlElement>);

#[derive(Debug, Default)]
pub struct HtmlHead {
    pub elements: HtmlElements,
    pub syntax_highlighting_used: bool,
    pub styles: HtmlAttributes, //TODO: replace with CSS struct
}

impl HtmlHead {
    fn merge(&mut self, mut other: Self) {
        self.elements.append(&mut other.elements);
        self.styles.append(&mut other.styles);
        self.syntax_highlighting_used |= other.syntax_highlighting_used;
    }
}

#[derive(Debug, Default)]
pub struct HtmlBody {
    pub elements: HtmlElements,
}

#[derive(Debug, Default)]
pub struct Html {
    pub head: HtmlHead,
    pub body: HtmlBody,
    pub lang: String,
}

impl Html {
    pub fn with_head(head: HtmlHead) -> Self {
        Html {
            head,
            ..Default::default()
        }
    }

    pub fn with_body(element: HtmlElement) -> Self {
        let mut html = Html::default();
        html.body.elements.push(element);
        html
    }

    pub fn with(head: HtmlHead, element: HtmlElement) -> Self {
        let mut html = Html {
            head,
            ..Default::default()
        };
        html.body.elements.push(element);
        html
    }

    pub fn nested(outer_name: &str, outer_attributes: HtmlAttributes, inner: Self) -> Self {
        let mut html = Html::with_body(HtmlElement {
            name: outer_name.to_string(),
            attributes: outer_attributes,
            content: Some(inner.body.elements.to_string()),
        });
        html.head.merge(inner.head);

        html
    }
}

impl OutputFormat for Html {
    fn new(context: &crate::render::Context) -> Self {
        Html {
            head: HtmlHead {
                elements: HtmlElements(Vec::new()),
                syntax_highlighting_used: false,
                styles: HtmlAttributes(Vec::new()),
            },
            body: HtmlBody {
                elements: HtmlElements(Vec::new()),
            },
            lang: context.get_lang().to_string(),
        }
    }

    fn append(&mut self, mut other: Self) -> Result<(), crate::log_id::RenderError> {
        self.head.merge(other.head);

        self.body.elements.append(&mut other.body.elements);

        Ok(())
    }
}

impl std::fmt::Display for Html {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<!DOCTYPE HTML><html lang=\"{}\">{}{}</html>",
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

impl std::fmt::Display for HtmlHead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<head>{}", self.elements)?;

        if self.syntax_highlighting_used {
            write!(
                f,
                "<style>{}</style>",
                include_str!("../../styles/syntax_highlighting.css")
            )?;
        }

        //TODO: write other head styles (try to use LightningCss optimizations)

        write!(f, "</head>")?;
        Ok(())
    }
}

impl std::fmt::Display for HtmlBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<body>{}</body>", self.elements)?;
        Ok(())
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
