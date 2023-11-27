use self::{props::CssProperty, rules::CssAtRule};

use super::NestedAttribute;

pub mod props;
pub mod rules;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CssAttribute {
    Property(CssProperty),
    AtRule(CssAtRule),
    Nested(Box<NestedAttribute<CssAttribute>>),
}
