use std::{collections::VecDeque, fmt::Debug};

use pest::iterators::Pair;

use crate::{
    frontend::{
        parser::{Rule},
    }};

use super::Render;
#[derive(Debug)]
pub struct Plain {
    pub content: String
}
#[derive(Debug)]
pub struct Bold {
    pub content: VecDeque<FormatTypes>
}
#[derive(Debug)]
pub struct Italic {
    pub content: VecDeque<FormatTypes>
}
#[derive(Debug)]
pub struct Subscript {
    pub content: VecDeque<FormatTypes>
}
#[derive(Debug)]
pub struct Superscript {
    pub content: VecDeque<FormatTypes>
}
#[derive(Debug)]
pub struct Verbatim {
    pub content: String
}
#[derive(Debug)]
pub struct Raw {
    pub content: String
}
#[derive(Debug)]
pub enum FormatTypes {
    Bold(Bold),
    Italic(Italic),
    Subscript(Subscript),
    Superscript(Superscript),
    Verbatim(Verbatim),
    Plain(Plain),
}

pub fn create_format_types(pair: Pair<Rule>) -> VecDeque<FormatTypes> {
    get_nested_inline(pair)
}

fn get_nested_inline(pair: Pair<Rule>) -> VecDeque<FormatTypes> {
    let mut content: VecDeque<FormatTypes> = VecDeque::<FormatTypes>::new();

    match pair.as_rule() {
        
        Rule::text => {
            let plain = Plain { content: pair.as_str().to_string(),};
            content.push_back(FormatTypes::Plain(plain));
        },
        Rule::italic => {
            let inner = pair.into_inner();
            let mut vector = VecDeque::new();

            for pair in inner {
                vector.append(&mut get_nested_inline(pair));
            }
            let italic = Italic{content: vector};
            content.push_back(FormatTypes::Italic(italic));
        },
        Rule::subscript => {
            let inner = pair.into_inner();
            let mut vector = VecDeque::new();

            for pair in inner {
                vector.append(&mut get_nested_inline(pair));
            }
            let subscript = Subscript{content: vector};
            content.push_back(FormatTypes::Subscript(subscript));
        },
        Rule::superscript => {
            let inner = pair.into_inner();
            let mut vector = VecDeque::new();

            for pair in inner {
                vector.append(&mut get_nested_inline(pair));
            }
            let superscript = Superscript{content: vector};
            content.push_back(FormatTypes::Superscript(superscript));
        },
        Rule::bold => {
            let inner = pair.into_inner();
            let mut vector = VecDeque::new();

            for pair in inner {
                vector.append(&mut get_nested_inline(pair));
            }
            let bold = Bold{content: vector};
            content.push_back(FormatTypes::Bold(bold));
        },
        Rule::verbatim => {
            let verbatim = Verbatim{content: pair.into_inner().as_str().to_string(),};
            content.push_back(FormatTypes::Verbatim(verbatim));
        },
        _ => unreachable!("No other inline types allowed.")
    }
    
    content
}

impl Render for Bold {
    fn render_html(&self) -> Result<String, crate::um_error::UmError> {
        
        let mut html = String::default();
        html.push_str("<b>");
        for element in &self.content {
            html.push_str(&element.render_html().expect("At least one or more formatting types expected"));
        }
        html.push_str("</b>");
        Ok(html)
    }
}

impl Render for Italic {
    fn render_html(&self) -> Result<String, crate::um_error::UmError> {
        
        let mut html = String::default();
        html.push_str("<i>");
        for element in &self.content {
            html.push_str(&element.render_html().expect("At least one or more formatting types expected"));
        }
        html.push_str("</i>");
        Ok(html)
    }
}
impl Render for Subscript {
    fn render_html(&self) -> Result<String, crate::um_error::UmError> {
        
        let mut html = String::default();
        html.push_str("<sub>");
        for element in &self.content {
            html.push_str(&element.render_html().expect("At least one or more formatting types expected"));
        }
        html.push_str("</sub>");
        Ok(html)
    }
}
impl Render for Superscript {
    fn render_html(&self) -> Result<String, crate::um_error::UmError> {
        
        let mut html = String::default();
        html.push_str("<sup>");
        for element in &self.content {
            html.push_str(&element.render_html().expect("At least one or more formatting types expected"));
        }
        html.push_str("</sup>");
        Ok(html)
    }
}
impl Render for Verbatim {
    fn render_html(&self) -> Result<String, crate::um_error::UmError> {
        
        let mut html = String::default();
        html.push_str("<pre>");
        html.push_str(&self.content);
        html.push_str("</pre>");
        Ok(html)
    }
}
impl Render for Plain {
    fn render_html(&self) -> Result<String, crate::um_error::UmError> {
        
        let mut html = String::default();
        html.push_str(&self.content);
        Ok(html)
    }
}
impl Render for FormatTypes {
    fn render_html(&self) -> Result<String, crate::um_error::UmError> {
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

pub fn render_inline_umblocks(html: &mut String, inline_format:VecDeque<FormatTypes>) {

    for element in inline_format {
        html.push_str(&element.render_html().expect("Rendered format types expected"));
    }
}