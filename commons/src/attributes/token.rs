use crate::{
    comments::Comment,
    lexer::{
        position::Position,
        symbol::SymbolKind,
        token::{implicit::ImplicitSubstitutionKind, TokenKind},
    },
    logic::LogicAst,
    parsing::Element,
};

use super::rules::AtRuleId;

#[derive(Debug, PartialEq, Clone)]
pub struct AttributeTokens {
    pub(crate) tokens: Vec<AttributeToken>,
    pub(crate) implicit_closed: bool,
    pub(crate) start: Position,
    pub(crate) end: Position,
}

impl Element for AttributeTokens {
    fn as_unimarkup(&self) -> String {
        self.tokens.iter().fold(String::new(), |mut s, t| {
            s.push_str(&t.as_unimarkup());
            s
        })
    }

    fn start(&self) -> crate::lexer::position::Position {
        self.start
    }

    fn end(&self) -> crate::lexer::position::Position {
        self.end
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AttributeToken {
    pub(crate) kind: AttributeTokenKind,
    pub(crate) start: Position,
    pub(crate) end: Position,
}

impl Element for AttributeToken {
    fn as_unimarkup(&self) -> String {
        self.kind.as_unimarkup()
    }

    fn start(&self) -> Position {
        self.start
    }

    fn end(&self) -> Position {
        self.end
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AttributeTokenKind {
    /// Marks previous [`SelectorIdentPart`]s to be for an attribute ident that ends with `: `.
    /// The stored ident does **not** include the ending `: `.
    /// Quoted idents are allowed for Unimarkup attributes, but must not span multiple lines.
    /// e.g. `"quoted ident": 2023`
    ///
    /// **Note:** The whitespace after `:` is required, but may be any non-escaped whitespace.
    /// This requirement differs from the CSS specification, but makes distinguishing single or nested properties much easier.
    /// It also allows to create attributes with arrays using `[]` instead of implicit arrays via comma or space.
    /// See: https://www.w3.org/TR/css-syntax-3/#parsing, https://github.com/w3c/csswg-drafts/issues/9317
    ///
    /// **Note:** There is no `SelectorMarker`, because `Nested` implicitly marks previous parts as selector parts.
    /// Consequently, all ident or selector parts up until either an `IdentMarker` or `Nested` token are part of the same ident/selector.
    IdentMarker,
    /// Ident or selector part for a flat or nested attribute.
    /// May only be part of the selector/ident, because a selector may span multiple lines,
    /// or an idenat having comments between ident and the colon to mark it as ident instead of selector.
    IdentOrSelectorPart(IdentOrSelectorPart),
    /// At-rule ident starting with `@`.
    /// The stored ident does **not** include the `@`.
    ///
    /// **Note:** Identifiers must be separated by e.g. whitespace to distinguish between at-rule ident and follow up ident.
    /// This differs from the CSS specification, but helps to handle custom at-rules.
    AtRuleIdent(AtRuleId),
    /// Rule prelude part that is between an at-rule ident and a semicolon or nested block.
    /// May only be part of the prelude, because it can span multiple lines.
    /// e.g. `@<rule ident> <prelude part> {<optional nested block>}`
    AtRulePreludePart(String),
    /// Flat value of an attribute that is not nested and is not an array.
    FlatValue(ValuePart),
    /// Value separator for flat values.
    /// e.g. whitespace for `margin: 2px 3px;`
    ValueSeparator(ValueSeparator),
    /// Tokens surrounded by `{}`.
    /// Nested blocks are implicity closed if the underlying token iterator ends, before `}` is reached.
    /// A semicolon is optional after the closing `}`.
    Nested(AttributeTokens),
    /// Tokens surrounded by `[]`.
    /// Arrays are implicity closed if the underlying token iterator ends, before `]` is reached.
    Array(AttributeTokens),
    /// A logic element that may resolve to one or more attributes, or a value of an attribute.
    ///
    /// ```text
    /// {
    ///   #id-selector {
    ///     {$attrb_var};
    ///   }
    ///   {$other_attrbs};
    ///   class: {$my_classes};
    /// }
    /// ```
    Logic(LogicAst),
    /// A Unimarkup comment.
    /// e.g. `;; This is a comment`
    ///
    /// **Note:** CSS comment syntax is **not** supported.
    Comment(Comment),
    /// A single semicolon used as declaration separator.
    Semicolon,
    /// A single comma used as value separator.
    Comma,
    /// The `!important` marker.
    /// See: https://www.w3.org/TR/css-syntax-3/#!important-diagram
    Important,
    Newline,
    /// Invalid token found while tokenization.
    /// Contains the [`TokenKind`] and content causing the invalid attribute token.
    Invalid(TokenKind, String),
}

impl AttributeTokenKind {
    pub fn as_unimarkup(&self) -> String {
        match self {
            AttributeTokenKind::IdentMarker => ": ".to_string(),
            AttributeTokenKind::FlatValue(value_part) => value_part.as_unimarkup(),
            AttributeTokenKind::ValueSeparator(separator) => separator.as_unimarkup(),
            AttributeTokenKind::IdentOrSelectorPart(part) => part.as_unimarkup(),
            AttributeTokenKind::AtRuleIdent(at_rule_ident) => {
                format!("@{}", at_rule_ident.as_str())
            }
            AttributeTokenKind::AtRulePreludePart(at_rule_prelude_part) => {
                at_rule_prelude_part.clone()
            }
            AttributeTokenKind::Nested(inner) => {
                format!(
                    "{{{}{}",
                    inner.as_unimarkup(),
                    if inner.implicit_closed {
                        ""
                    } else {
                        SymbolKind::CloseBrace.as_str()
                    }
                )
            }
            AttributeTokenKind::Array(inner) => {
                format!(
                    "[{}{}",
                    inner.as_unimarkup(),
                    if inner.implicit_closed {
                        ""
                    } else {
                        SymbolKind::CloseBracket.as_str()
                    }
                )
            }
            AttributeTokenKind::Logic(logic) => logic.as_unimarkup(),
            AttributeTokenKind::Comment(comment) => comment.as_unimarkup(),
            AttributeTokenKind::Semicolon => SymbolKind::Semicolon.as_str().to_string(),
            AttributeTokenKind::Comma => SymbolKind::Comma.as_str().to_string(),
            AttributeTokenKind::Important => "!important".to_string(),
            AttributeTokenKind::Newline => SymbolKind::Newline.as_str().to_string(),
            AttributeTokenKind::Invalid(_, c) => c.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IdentOrSelectorPart {
    Plain(String),
    /// A quoted part (e.g. `"value"` or `'value'`).
    Quoted(QuotedIdent),
}

impl IdentOrSelectorPart {
    pub fn as_unimarkup(&self) -> String {
        match self {
            IdentOrSelectorPart::Plain(plain) => plain.clone(),
            IdentOrSelectorPart::Quoted(q) => q.as_unimarkup(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct QuotedIdent {
    pub(crate) ident: String,
    pub(crate) quote: char,
    pub(crate) implicit_closed: bool,
}

impl QuotedIdent {
    pub fn as_unimarkup(&self) -> String {
        self.ident.clone()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct QuotedPart {
    pub(crate) parts: Vec<QuotedValuePart>,
    pub(crate) quote: char,
    pub(crate) implicit_closed: bool,
}

impl QuotedPart {
    pub fn resolve(&self) -> String {
        let quote = self.quote;
        let inner = self.parts.iter().fold(String::new(), |mut s, q| {
            s.push_str(&q.kind.resolve());
            s
        });

        if self.implicit_closed {
            format!("{quote}{inner}")
        } else {
            format!("{quote}{inner}{quote}")
        }
    }
}

impl Element for QuotedPart {
    fn as_unimarkup(&self) -> String {
        let quote = self.quote;
        let inner = self.parts.iter().fold(String::new(), |mut s, q| {
            s.push_str(&q.kind.as_unimarkup());
            s
        });

        if self.implicit_closed {
            format!("{quote}{inner}")
        } else {
            format!("{quote}{inner}{quote}")
        }
    }

    fn start(&self) -> Position {
        self.parts.first().map(|p| p.start).unwrap_or_default()
    }

    fn end(&self) -> Position {
        self.parts.last().map(|p| p.end).unwrap_or_default()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct QuotedValuePart {
    pub(crate) kind: QuotedPartKind,
    pub(crate) start: Position,
    pub(crate) end: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub enum QuotedPartKind {
    /// Contains plain content.
    /// Including escaped graphemes and whitespaces.
    /// Escaped graphemes are added to the content **without** the leading backslash.
    /// e.g. "\ " <- escaped whitespace & "\n" <- escaped `n`.
    Plain(String),
    /// Implicit substitutions except `DirectUri` are converted to their *rendered* representations.
    ImplicitSubstitution(ImplicitSubstitutionKind),
    /// Named substitutions are converted to their *rendered* representations.
    /// This is possible, because the content of named susbtitutions may only consist of plain content, whitespaces, newlines, escaped variants, or implicit substitutions.
    NamedSubstitution(String),
    Logic(LogicAst),
    EscapedNewline,
    Newline,
}

impl QuotedPartKind {
    pub fn as_unimarkup(&self) -> String {
        match self {
            QuotedPartKind::Plain(plain) => plain.clone(),
            QuotedPartKind::ImplicitSubstitution(implicit_subst) => {
                implicit_subst.orig().to_string()
            }
            QuotedPartKind::NamedSubstitution(named_subst) => named_subst.clone(),
            QuotedPartKind::Logic(logic) => logic.as_unimarkup(),
            QuotedPartKind::EscapedNewline | QuotedPartKind::Newline => {
                SymbolKind::Newline.as_str().to_string()
            }
        }
    }

    pub fn resolve(&self) -> String {
        match self {
            QuotedPartKind::Plain(plain) => plain.clone(),
            QuotedPartKind::ImplicitSubstitution(implicit_subst) => {
                implicit_subst.subst().to_string()
            }
            QuotedPartKind::NamedSubstitution(named_subst) => named_subst.clone(),
            QuotedPartKind::Logic(logic) => logic.as_unimarkup(),
            QuotedPartKind::EscapedNewline | QuotedPartKind::Newline => {
                SymbolKind::Newline.as_str().to_string()
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValuePart {
    CssFn(CssFn),
    /// A quoted part (e.g. `"value"` or `'value'`).
    Quoted(QuotedPart),
    Plain(String),
    Float(f64),
    Int(isize),
    Bool(bool),
}

impl ValuePart {
    pub fn as_unimarkup(&self) -> String {
        match self {
            ValuePart::CssFn(css_fn) => css_fn.as_unimarkup(),
            ValuePart::Quoted(q) => q.as_unimarkup(),
            ValuePart::Plain(plain) => plain.clone(),
            ValuePart::Float(val) => val.to_string(),
            ValuePart::Int(val) => val.to_string(),
            ValuePart::Bool(val) => val.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValueSeparator {
    Whitespace,
    Comma,
}

impl ValueSeparator {
    pub fn as_unimarkup(&self) -> String {
        self.as_str().to_string()
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ValueSeparator::Whitespace => SymbolKind::Whitespace.as_str(),
            ValueSeparator::Comma => SymbolKind::Comma.as_str(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CssFn {
    pub(crate) name: String,
    pub(crate) inner: Vec<AttributeToken>,
    pub(crate) implicit_closed: bool,
}

impl CssFn {
    pub fn as_unimarkup(&self) -> String {
        let s = self.inner.iter().fold(String::new(), |mut s, i| {
            s.push_str(&i.as_unimarkup());
            s
        });
        format!(
            "{}({}{}",
            self.name,
            s,
            if self.implicit_closed { "" } else { ")" }
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum EnclosedSymbolKind {
    Brace,
    Bracket,
    Parenthesis,
    /// `<` or `>`
    Angle,
}

impl EnclosedSymbolKind {
    pub fn open_symbol(&self) -> SymbolKind {
        match self {
            EnclosedSymbolKind::Brace => SymbolKind::OpenBrace,
            EnclosedSymbolKind::Bracket => SymbolKind::OpenBracket,
            EnclosedSymbolKind::Parenthesis => SymbolKind::OpenParenthesis,
            EnclosedSymbolKind::Angle => SymbolKind::Lt,
        }
    }

    pub fn close_symbol(&self) -> SymbolKind {
        match self {
            EnclosedSymbolKind::Brace => SymbolKind::CloseBrace,
            EnclosedSymbolKind::Bracket => SymbolKind::CloseBracket,
            EnclosedSymbolKind::Parenthesis => SymbolKind::CloseParenthesis,
            EnclosedSymbolKind::Angle => SymbolKind::Gt,
        }
    }
}
