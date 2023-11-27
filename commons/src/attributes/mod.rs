//! Contains the [`Attributes`] struct and parser for Unimarkup attributes.

use cssparser::ParseError;
use lightningcss::{
    error::ParserError,
    selector::SelectorList,
    stylesheet::{ParserFlags, ParserOptions},
    traits::ParseWithOptions,
};

use crate::lexer::position::Position;

use self::{css::CssAttribute, html::HtmlAttribute, um::UmAttribute};

pub mod css;
pub mod html;
pub mod log_id;
pub mod parser;
pub mod um;

pub const ATTRIBUTE_PARSER_OPTIONS: ParserOptions = ParserOptions {
    filename: String::new(),
    css_modules: None,
    // 0 because correct global positions must be calculated manually.
    source_index: 0,
    // No recovery, because errors must be translated to warnings on Unimarkup side.
    error_recovery: false,
    warnings: None,
    flags: ParserFlags::NESTING,
};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Attributes {
    css: Vec<CssAttribute>,
    html: Vec<HtmlAttribute>,
    um: Vec<UmAttribute>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct AttributeIdent {
    ident: String,
    start: Position,
    end: Position,
}

impl From<(&str, Position, Position)> for AttributeIdent {
    /// Tries to convert a given `str` to an [`AttributeIdent`].
    /// The positions are the `start` and `end` of the given `str`.
    ///
    /// Usage: `AttributeIdent::try_from("my-ident", <start pos>, <end pos>)`
    fn from(value: (&str, Position, Position)) -> Self {
        AttributeIdent {
            ident: value.0.to_string(),
            start: value.1,
            end: value.2,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct AttributeSelectors {
    selectors: String,
    start: Position,
    end: Position,
}

impl AttributeSelectors {
    #[inline]
    pub fn selectors<'a>(
        &'a self,
        options: ParserOptions<'_, 'a>,
    ) -> Result<SelectorList<'a>, ParseError<'_, ParserError<'_>>> {
        SelectorList::<'a>::parse_string_with_options(&self.selectors, options)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct NestedAttribute<T> {
    selector: AttributeSelectors,
    inner: Vec<T>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct AttributeValue {
    value: Vec<AttributeValueToken>,
    start: Position,
    end: Position,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AttributeValueToken {
    kind: AttributeValueTokenKind,
    start: Position,
    end: Position,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AttributeValueTokenKind {
    Content(String),
    Logic(String), // TODO: replace with logic AST
    Newline,
}
