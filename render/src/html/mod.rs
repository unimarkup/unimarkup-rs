//! Defines the [`Html`] struct that is returned when rendering Unimarkup to HTML.

use unimarkup_commons::attributes::{
    resolved::{
        ResolvedAttribute, ResolvedAttributeIdent, ResolvedAttributes, ResolvedFlatAttributeValue,
        ResolvedSingleAttribute,
    },
    resolver::{AttributeResolver, AttributeResolverContext},
    token::AttributeTokens,
};

use crate::render::OutputFormat;

use self::tag::HtmlTag;

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
pub struct HtmlAttributes(Vec<HtmlAttribute>, Option<HtmlAttribute>);

impl HtmlAttributes {
    pub fn push_style(&mut self, value: &str) {
        let new_styles = match &mut self.1 {
            Some(styles) => styles.value.as_mut().map_or(value, |s| {
                s.push_str(value);
                s
            }),
            None => value,
        };
        self.1 = Some(HtmlAttribute {
            name: "style".to_string(),
            value: Some(new_styles.to_string()),
        })
    }
}

impl From<&AttributeTokens> for HtmlAttributes {
    fn from(value: &AttributeTokens) -> Self {
        let resolved = AttributeResolver::new(value).resolve(&AttributeResolverContext::default());
        resolved.into()
    }
}

const HTML_ATTRB_QUOTES: &str = "'";

impl From<ResolvedAttributes<'_>> for HtmlAttributes {
    fn from(value: ResolvedAttributes<'_>) -> Self {
        let mut attrbs = Vec::new();

        if let Some(id) = value.id {
            attrbs.push(HtmlAttribute {
                name: "id".to_string(),
                value: Some(id.to_string()),
            })
        }

        for html_attrb in value.html {
            if let ResolvedAttribute::Single(ResolvedSingleAttribute::Flat(flat)) = html_attrb {
                let ResolvedAttributeIdent::Html(ident) = flat.ident else {
                    unreachable!("Idents in HTML attributes must be valid HTML idents.");
                };
                if let ResolvedFlatAttributeValue::Bool(true) = flat.value {
                    attrbs.push(HtmlAttribute {
                        name: ident.as_str().to_string(),
                        value: None,
                    })
                } else {
                    attrbs.push(HtmlAttribute {
                        name: ident.as_str().to_string(),
                        value: Some(
                            flat.value
                                .to_string()
                                .replace(HTML_ATTRB_QUOTES, &format!("\\{HTML_ATTRB_QUOTES}")),
                        ), // escape quote, because HTML attributes are wrapped in single quotes
                    })
                }
            } else {
                //TODO: set warn for unsupported attrb for now
            }
        }

        let mut style_value = String::new();
        for css_attrb in value.css {
            if let ResolvedAttribute::Single(ResolvedSingleAttribute::Flat(flat)) = css_attrb {
                let ResolvedAttributeIdent::Css(ident) = flat.ident else {
                    unreachable!("Idents in CSS attributes must be valid CSS idents.");
                };

                style_value.push_str(&format!(
                    "{}:{}{};",
                    ident.name(), //TODO: add vendorprefix
                    flat.value
                        .to_string()
                        .replace(HTML_ATTRB_QUOTES, &format!("\\{HTML_ATTRB_QUOTES}")),
                    if flat.important { " !important" } else { "" }
                ));
            } else {
                //TODO: set warn for unsupported attrb for now
            }
        }
        let style = if !style_value.is_empty() {
            Some(HtmlAttribute {
                name: "style".to_string(),
                value: Some(style_value),
            })
        } else {
            None
        };

        HtmlAttributes(attrbs, style)
    }
}

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
                styles: HtmlAttributes(Vec::new(), None),
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

        if !self.attributes.is_empty() || self.attributes.1.is_some() {
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
            Some(value) => write!(
                f,
                "{}={}{}{}",
                self.name, HTML_ATTRB_QUOTES, value, HTML_ATTRB_QUOTES
            ),
            None => write!(f, "{}", self.name),
        }
    }
}

impl From<Vec<HtmlAttribute>> for HtmlAttributes {
    fn from(value: Vec<HtmlAttribute>) -> Self {
        HtmlAttributes(value, None)
    }
}

impl std::fmt::Display for HtmlAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for attrb in &self.0 {
            // Note: *whitespace* at beginning is important, because `HtmlElement` does not set one after open tag.
            // This way no unnecessary whitespace is left at the end of the open tag.
            write!(f, " {}", attrb)?;
        }
        if let Some(style) = &self.1 {
            write!(f, " {}", style)?;
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
