//! Defines the [`Html`] struct that is returned when rendering Unimarkup to HTML.

use crate::render::OutputFormat;

use self::tag::HtmlTag;

pub(crate) mod citeproc;
pub mod highlight;
pub mod render;
pub mod tag;

#[derive(Debug, Default)]
pub struct HtmlAttribute {
    pub name: String,
    pub value: Option<String>,
}

/// Represents a HTML element.
///
/// **Note:** If `tag = Html::PlainContent`, the element is displayed without a HTML tag.
#[derive(Debug, Default)]
pub struct HtmlElement {
    pub tag: HtmlTag,
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
    pub paged_js_used: bool,
    pub styles: HtmlAttributes, //TODO: replace with CSS struct
}

impl HtmlHead {
    fn merge(&mut self, mut other: Self) {
        self.elements.append(&mut other.elements);
        self.styles.append(&mut other.styles);
        self.syntax_highlighting_used |= other.syntax_highlighting_used;
        self.paged_js_used |= other.paged_js_used;
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

    pub fn with_body(body: HtmlBody) -> Self {
        Html {
            body,
            ..Default::default()
        }
    }

    pub fn with(head: HtmlHead, body: HtmlBody) -> Self {
        Html {
            head,
            body,
            ..Default::default()
        }
    }

    pub fn nested(outer_tag: HtmlTag, outer_attributes: HtmlAttributes, inner: Self) -> Self {
        let mut html = Html::with_body(HtmlBody::from(HtmlElement {
            tag: outer_tag,
            attributes: outer_attributes,
            content: Some(inner.body.elements.to_string()),
        }));
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
                paged_js_used: false,
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
        if let (HtmlTag::PlainContent, Some(content)) = (&self.tag, &self.content) {
            return write!(f, "{}", content);
        } else if self.tag == HtmlTag::PlainContent {
            return write!(f, "");
        }

        let mut element = format!("<{}", self.tag.as_str());

        if !self.attributes.is_empty() {
            element.push_str(&format!("{}", self.attributes));
        }

        match &self.content {
            Some(content) => element.push_str(&format!(">{}</{}>", content, self.tag.as_str())),
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
        let highlighting = if self.paged_js_used {
            let _ = write!(
                f,
                "<script>{}</script>",
                include_str!("paged.polyfill.min.js")
            );
            include_str!("../../styles/syntax_highlighting_paged_js.css")
        } else {
            include_str!("../../styles/syntax_highlighting.css")
        };
        if self.syntax_highlighting_used {
            write!(f, "<style>{}</style>", highlighting)?;
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

impl From<Vec<HtmlElement>> for HtmlBody {
    fn from(value: Vec<HtmlElement>) -> Self {
        HtmlBody {
            elements: HtmlElements(value),
        }
    }
}

impl From<HtmlElement> for HtmlBody {
    fn from(value: HtmlElement) -> Self {
        HtmlBody {
            elements: HtmlElements(vec![value]),
        }
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
