use crate::lexer::position::Position;

use super::CssAttribute;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CssAtRule {
    Charset(Box<CssAtRuleCharset>),
    Custom(Box<CssCustomAtRule>), // TODO: add standard at-rules
}

/// Represents the `@charset "<charset>";` at-rule.
/// See: https://developer.mozilla.org/en-US/docs/Web/CSS/@charset.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct CssAtRuleCharset {
    charset: String,
    start: Position,
    end: Position,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct CssCustomAtRule {
    ident: CssCustomAtRuleIdent,
    rule: Option<CssCustomAtRuleRule>,
    body: Option<CssCustomAtRuleBody>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct CssCustomAtRuleIdent {
    name: String,
    start: Position,
    end: Position,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct CssCustomAtRuleRule {
    name: String,
    start: Position,
    end: Position,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct CssCustomAtRuleBody {
    body: Vec<CssAttribute>,
    start: Position,
    end: Position,
}
