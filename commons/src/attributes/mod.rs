//! Contains structs, enums, parser, and resolver for Unimarkup attributes.
//!
//! Attributes are first tokenized into a vector of [`AttributeToken`](token::AttributeToken).
//! This vector must then be stored in parsed elements.
//! In the resolve step of the compiler, the [`AttributeToken`](token::AttributeToken)s are then `resolved` to [`ResolvedAttributes`](resolved::ResolvedAttributes).
//! These [`ResolvedAttributes`](resolved::ResolvedAttributes) can now be converted to [CSS](CssAttribute), [HTML](html::HtmlAttribute), or [Unimarkup](um::UmAttribute) attributes.

use lightningcss::stylesheet::{ParserFlags, ParserOptions};

pub mod html;
pub mod log_id;
pub mod resolved;
pub mod resolver;
pub mod rules;
pub mod token;
pub mod tokenize;
pub mod um;

pub use lightningcss::properties::Property as CssAttribute;

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
