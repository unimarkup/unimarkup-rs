use crate::{
    attributes::{AttributeIdent, AttributeValue},
    lexer::position::Position,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CssProperty {
    ident: CssPropertyId,
    value: AttributeValue,
    start: Position,
    end: Position,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CssPropertyId {
    // TODO: Add standard css properties
    /// `All` property used by `lightningcss`.
    All,
    /// A custom css property starting with `--`.
    /// e.g. `--my-prop`
    Custom(Box<AttributeIdent>),
    /// An unknown css property.
    /// This property is neither a supported HTML attribute nor special Unimarkup attribute.
    Unknown(Box<AttributeIdent>),
}
