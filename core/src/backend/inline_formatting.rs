use std::{collections::VecDeque, fmt::Debug};

use pest::iterators::Pair;
use pest::Parser;

use crate::backend::BackendError;
use crate::frontend::parser::{Rule, UnimarkupParser};
use crate::log_id::{LogId, SetLog};

use super::{Render, InlineErrLogId};
/// [`Plain`] is one of the inline formatting types, which contains the raw text as String.
#[derive(Debug)]
pub struct Plain {
    content: String,
}
/// [`Bold`] is one of the inline formatting types
#[derive(Debug)]
pub struct Bold {
    content: VecDeque<FormatTypes>,
}
/// [`Italic`] is one of the inline formatting types.
#[derive(Debug)]
pub struct Italic {
    content: VecDeque<FormatTypes>,
}
/// [`Subscript`] is one of the inline formatting types.
#[derive(Debug)]
pub struct Subscript {
    content: VecDeque<FormatTypes>,
}
/// [`Superscript`] is one of the inline formatting types.
#[derive(Debug)]
pub struct Superscript {
    content: VecDeque<FormatTypes>,
}
/// [`Verbatim`] is one of the inline formatting types.
#[derive(Debug)]
pub struct Verbatim {
    content: String,
}
/// [`FormatTypes`] is an enum of all the inline formatting types.
#[derive(Debug)]
pub enum FormatTypes {
    /// Represents the [`Bold`] FormatType
    Bold(Bold),
    /// Represents the [`Italic`] FormatType
    Italic(Italic),
    /// Represents the [`Subscript`] FormatType
    Subscript(Subscript),
    /// Represents the [`Superscript`] FormatType
    Superscript(Superscript),
    /// Represents the [`Verbatim`] FormatType
    Verbatim(Verbatim),
    /// Represents the [`Plain`] FormatType
    Plain(Plain),
}

/// [`parse_inline`] parses through the content of a [`UnimarkupBlock`] and returns a VecDeque of Formattypes
pub fn parse_inline(source: &str) -> Result<VecDeque<FormatTypes>, BackendError> {
    let mut rule_pairs =
        UnimarkupParser::parse(Rule::inline_format, source).map_err(|err| BackendError::General(
            (InlineErrLogId::NoInlineDetected as LogId).set_log("No inline format detected!", file!(), line!())
            .add_to_log(&format!("Given: {}", source))
            .add_to_log(&format!("Cause: {}", err))
        ))?;

    let mut inline_format = VecDeque::<FormatTypes>::new();

    if let Some(inline) = rule_pairs.next() {
        inline_format.append(&mut pair_into_format_types(inline));
    }

    Ok(inline_format)
}

fn create_format_types(pair: Pair<Rule>) -> VecDeque<FormatTypes> {
    let mut content: VecDeque<FormatTypes> = VecDeque::<FormatTypes>::new();

    match pair.as_rule() {
        Rule::plain => {
            let plain = Plain {
                content: pair.as_str().to_string(),
            };
            content.push_back(FormatTypes::Plain(plain));
        }
        Rule::italic_inline => {
            let italic = Italic {
                content: pair_into_format_types(pair),
            };
            content.push_back(FormatTypes::Italic(italic));
        }
        Rule::subscript_inline => {
            let subscript = Subscript {
                content: pair_into_format_types(pair),
            };
            content.push_back(FormatTypes::Subscript(subscript));
        }
        Rule::superscript_inline => {
            let superscript = Superscript {
                content: pair_into_format_types(pair),
            };
            content.push_back(FormatTypes::Superscript(superscript));
        }
        Rule::bold_inline => {
            let bold = Bold {
                content: pair_into_format_types(pair),
            };
            content.push_back(FormatTypes::Bold(bold));
        }
        Rule::verbatim_inline => {
            let verbatim = Verbatim {
                content: pair.into_inner().as_str().to_string(),
            };
            content.push_back(FormatTypes::Verbatim(verbatim));
        }
        _ => unreachable!("No other inline types allowed."),
    }

    content
}

fn pair_into_format_types(pair: Pair<Rule>) -> VecDeque<FormatTypes> {
    pair.into_inner().flat_map(create_format_types).collect()
}

impl Render for Bold {
    fn render_html(&self) -> Result<String, BackendError> {
        let mut html = String::default();
        html.push_str("<b>");
        for element in &self.content {
            html.push_str(
                &element
                    .render_html()
                    .expect("At least one or more formatting types expected"),
            );
        }
        html.push_str("</b>");
        Ok(html)
    }
}

impl Render for Italic {
    fn render_html(&self) -> Result<String, BackendError> {
        let mut html = String::default();
        html.push_str("<i>");
        for element in &self.content {
            html.push_str(
                &element
                    .render_html()
                    .expect("At least one or more formatting types expected"),
            );
        }
        html.push_str("</i>");
        Ok(html)
    }
}

impl Render for Subscript {
    fn render_html(&self) -> Result<String, BackendError> {
        let mut html = String::default();
        html.push_str("<sub>");
        for element in &self.content {
            html.push_str(
                &element
                    .render_html()
                    .expect("At least one or more formatting types expected"),
            );
        }
        html.push_str("</sub>");
        Ok(html)
    }
}

impl Render for Superscript {
    fn render_html(&self) -> Result<String, BackendError> {
        let mut html = String::default();
        html.push_str("<sup>");
        for element in &self.content {
            html.push_str(
                &element
                    .render_html()
                    .expect("At least one or more formatting types expected"),
            );
        }
        html.push_str("</sup>");
        Ok(html)
    }
}

impl Render for Verbatim {
    fn render_html(&self) -> Result<String, BackendError> {
        let mut html = String::default();
        html.push_str("<pre>");
        html.push_str(&self.content);
        html.push_str("</pre>");
        Ok(html)
    }
}

impl Render for Plain {
    fn render_html(&self) -> Result<String, BackendError> {
        Ok(self.content.clone())
    }
}

impl Render for FormatTypes {
    fn render_html(&self) -> Result<String, BackendError> {
        match self {
            FormatTypes::Bold(content) => content.render_html(),
            FormatTypes::Italic(content) => content.render_html(),
            FormatTypes::Subscript(content) => content.render_html(),
            FormatTypes::Superscript(content) => content.render_html(),
            FormatTypes::Verbatim(content) => content.render_html(),
            FormatTypes::Plain(content) => content.render_html(),
        }
    }
}

impl Render for VecDeque<FormatTypes> {
    fn render_html(&self) -> Result<String, BackendError> {
        let mut html = String::default();

        for element in self {
            html.push_str(
                &element
                    .render_html()
                    .expect("Rendered format types expected"),
            );
        }
        Ok(html)
    }
}
