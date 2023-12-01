use cssparser::ParseError;
use lightningcss::{
    error::ParserError, selector::SelectorList, stylesheet::ParserOptions, traits::ParseWithOptions,
};

use super::token::{AttributeToken, QuotedValuePart};

/// Resolved attributes have replaced all logic elements by their respective return value.
/// Resolved attributes still preserve the attribute order in the given content.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResolvedAttributes<'tslice>(Vec<ResolvedAttribute<'tslice>>);

#[derive(Debug, PartialEq, Clone)]
pub enum ResolvedAttribute<'tslice> {
    Property(ResolvedProperty<'tslice>),
    AtRule(ResolvedAtRule<'tslice>),
    Nested(ResolvedNestedAttribute<'tslice>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ResolvedNestedAttribute<'tslice> {
    selectors: ResolvedAttributeSelectors<'tslice>,
    body: Vec<ResolvedAttribute<'tslice>>,
}

impl ResolvedNestedAttribute<'_> {
    #[inline]
    pub fn selectors<'a>(
        &'a self,
        options: ParserOptions<'_, 'a>,
    ) -> Result<SelectorList<'a>, ParseError<'_, ParserError<'_>>> {
        self.selectors.selectors(options)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ResolvedAttributeSelectors<'tslice> {
    selectors: ResolvedAttributeString<'tslice, AttributeToken>,
}

impl ResolvedAttributeSelectors<'_> {
    #[inline]
    pub fn selectors<'a>(
        &'a self,
        options: ParserOptions<'_, 'a>,
    ) -> Result<SelectorList<'a>, ParseError<'_, ParserError<'_>>> {
        SelectorList::<'a>::parse_string_with_options(&self.selectors.value, options)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ResolvedAttributeIdent<'tslice> {
    /// The parsed token this ident is associated with.
    /// Ident can only be from one token.
    ///
    /// Quoted identifiers are allowed, but must **not** span multiple lines.
    /// e.g. `"custom ident": <some value>`
    ident: &'tslice AttributeToken,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ResolvedProperty<'tslice> {
    Single(ResolvedSingleProperty<'tslice>),
    Array(ResolvedArrayProperty<'tslice>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ResolvedSingleProperty<'tslice> {
    ident: ResolvedAttributeIdent<'tslice>,
    value: ResolvedSinglePropertyValue<'tslice>,
    important: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ResolvedAttributeString<'tslice, T> {
    tokens: &'tslice [T],
    /// Mapping from a token `T` in the `tokens` slice to the length the token resolved to in the resulting string.
    /// First entry in the vector corresponds to the first entry in the slice.
    ///
    /// This is needed to get correct error mapping from `cssparser` to parsed [`AttributeToken`]s or [`QuotedValuePart`].
    resolved_length: Vec<ResolvedTokenLength>,
    /// The string the referenced tokens resolved to.
    value: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ResolvedTokenLength {
    pub(crate) len_utf8: usize,
    pub(crate) len_utf16: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ResolvedSinglePropertyValue<'tslice> {
    /// A single (`'`) or double (`"`) quoted value (e.g. `prop: "some value"`)
    Quoted(ResolvedAttributeString<'tslice, QuotedValuePart>, char),
    /// A whitespace or comma separated value (e.g. `prop: "some" "value"`)
    Array(Vec<ResolvedSinglePropertyValue<'tslice>>),
    Float(f64),
    Int(u64),
    Bool(bool),
    /// A value that is not a bool, number, quoted string, or whitespace/comma separated.
    /// e.g. `prop: #ffffff`
    Other(ResolvedAttributeString<'tslice, AttributeToken>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ResolvedArrayProperty<'tslice> {
    ident: ResolvedAttributeIdent<'tslice>,
    value: Vec<ResolvedProperty<'tslice>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ResolvedAtRule<'tslice> {
    ident: ResolvedAttributeIdent<'tslice>,
    prelude: Option<String>,
    block: Option<Vec<ResolvedAttribute<'tslice>>>,
}
