use crate::{
    lexer::{position::Position, token::Comment},
    logic::LogicAst,
};

use super::rules::AtRuleId;

#[derive(Debug, PartialEq, Clone)]
pub struct AttributeToken {
    kind: AttributeTokenKind,
    start: Position,
    end: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AttributeTokenKind {
    /// Attribute ident that ends with `: `.
    /// The stored ident does **not** include the ending `: `.
    ///
    /// **Note:** The whitespace after `:` is required, but may be any non-escaped whitespace.
    /// This requirement differs from the CSS specification, but makes distinguishing single or nested properties much easier.
    /// It also allows to create properties with object-like arrays as value.
    /// See: https://www.w3.org/TR/css-syntax-3/#parsing, https://github.com/w3c/csswg-drafts/issues/9317
    Ident(Ident),
    /// Value part of a non-nested attribute.
    /// May only be part of the complete value, because the value might be split by newlines or comments.
    ValuePart(TokenPart),
    /// Part that might become a selector part, or property ident with array content.
    /// May only be part of the ident, because it can span multiple lines in case of a selector or quoted identifier.
    NestedIdentPart(TokenPart),
    /// At-rule ident starting with `@`.
    /// The stored ident does **not** include the `@`.
    ///
    /// **Note:** Identifiers must be separated by e.g. whitespace to distinguish between at-rule ident and follow up ident.
    /// This differs from the CSS specification, but helps to handle custom at-rules.
    AtRuleIdent(AtRuleId),
    /// Rule prelude part that is between an at-rule ident and a semicolon or nested block.
    /// May only be part of the prelude, because it can span multiple lines.
    /// e.g. `@<rule ident> <prelude part> {<optional nested block>}`
    AtRulePreludePart(TokenPart),
    /// Tokens surrounded by `{}`.
    /// Nested blocks are implicity closed if the underlying token iterator ends, before `}` is reached.
    Nested(Vec<AttributeToken>, bool),
    Logic(LogicAst),
    /// A Unimarkup comment.
    /// e.g. `;; This is a comment`
    ///
    /// **Note:** CSS comment syntax is **not** supported.
    Comment(Comment),
    /// The `!important` marker.
    /// See: https://www.w3.org/TR/css-syntax-3/#!important-diagram
    Important,
    /// A single comma used as value separator.
    Comma,
    /// A single semicolon used as declaration separator.
    Semicolon,
    /// A quoted value (e.g. `"value"` or `'value'`).
    /// May only be part of the value, because it can span multiple lines.
    /// The `char` contains the quote char (e.g. `"`).
    QuotedValue(Vec<QuotedValuePart>, char),
    /// A single non-escaped whitespace used as value separator.
    Whitespace,
    Newline,
}

#[derive(Debug, PartialEq, Clone)]
pub struct QuotedValuePart {
    kind: QuotedValuePartKind,
    start: Position,
    end: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub enum QuotedValuePartKind {
    /// Contains plain content.
    /// Including escaped whitespaces and escaped non-whitespace graphemes.
    /// e.g. "\ <- escaped whitespace & \n <- escaped `n`."
    Plain(String),
    Logic(LogicAst),
    EscapedNewline,
    Newline,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ident(String);

#[derive(Debug, PartialEq, Clone)]
pub struct TokenPart(String);
