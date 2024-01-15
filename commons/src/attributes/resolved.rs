use cssparser::ParseError;
use lightningcss::{
    error::ParserError, properties::PropertyId, selector::SelectorList, stylesheet::ParserOptions,
    traits::ParseWithOptions,
};

use super::{
    html::HtmlAttributeId,
    token::{AttributeToken, ValueSeparator},
    um::UmAttributeId,
};

/// Resolved attributes have replaced all logic elements by their respective return value.
/// Resolved attributes still preserve the attribute order in the given content.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResolvedAttributes<'tslice> {
    pub html: Vec<ResolvedAttribute<'tslice>>,
    pub css: Vec<ResolvedAttribute<'tslice>>,
    pub um: Vec<ResolvedAttribute<'tslice>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ResolvedAttribute<'tslice> {
    Single(ResolvedSingleAttribute<'tslice>),
    AtRule(ResolvedAtRule<'tslice>),
    Nested(ResolvedNestedAttribute<'tslice>),
    Invalid,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ResolvedNestedAttribute<'tslice> {
    pub(crate) selectors: ResolvedAttributeSelectors,
    pub(crate) body: Vec<ResolvedAttribute<'tslice>>,
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
pub struct ResolvedAttributeSelectors(String);

impl ResolvedAttributeSelectors {
    #[inline]
    pub fn selectors<'a>(
        &'a self,
        options: ParserOptions<'_, 'a>,
    ) -> Result<SelectorList<'a>, ParseError<'_, ParserError<'_>>> {
        SelectorList::<'a>::parse_string_with_options(&self, options)
    }
}

impl From<String> for ResolvedAttributeSelectors {
    fn from(value: String) -> Self {
        ResolvedAttributeSelectors(value)
    }
}

impl std::ops::Deref for ResolvedAttributeSelectors {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ResolvedAttributeIdent<'tslice> {
    Html(HtmlAttributeId<'tslice>),
    Css(PropertyId<'tslice>),
    Um(UmAttributeId<'tslice>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ResolvedSingleAttribute<'tslice> {
    Flat(ResolvedFlatAttribute<'tslice>),
    Array(ResolvedArrayAttribute<'tslice>),
}

impl ResolvedSingleAttribute<'_> {
    pub fn ident(&self) -> &ResolvedAttributeIdent {
        match self {
            ResolvedSingleAttribute::Flat(flat) => &flat.ident,
            ResolvedSingleAttribute::Array(array) => &array.ident,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ResolvedFlatAttribute<'tslice> {
    pub(crate) ident: ResolvedAttributeIdent<'tslice>,
    pub(crate) value: ResolvedFlatAttributeValue,
    pub(crate) important: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ResolvedAttributeString<'tslice> {
    pub(crate) tokens: &'tslice [AttributeToken],
    /// Mapping from a token `T` in the `tokens` slice to the length the token resolved to in the resulting string.
    /// First entry in the vector corresponds to the first entry in the slice.
    ///
    /// This is needed to get correct error mapping from `cssparser` to parsed [`AttributeToken`]s or [`QuotedValuePart`].
    pub(crate) resolved_length: Vec<ResolvedTokenLength>,
    /// The string the referenced tokens resolved to.
    pub(crate) value: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ResolvedTokenLength {
    pub(crate) len_utf8: usize,
    pub(crate) len_utf16: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ResolvedFlatAttributeValue {
    /// A single (`'`) or double (`"`) quoted value (e.g. `prop: "some value"`)
    /// Note: The quote char is already part of the string.
    Quoted(String),
    Float(f64),
    Int(isize),
    Bool(bool),
    /// A value that is not a bool, number, quoted string, or whitespace/comma separated.
    /// e.g. `prop: #ffffff`
    Other(String),
    Empty,
}

impl std::ops::Add<ResolvedFlatAttributeValue> for ResolvedFlatAttributeValue {
    type Output = ResolvedFlatAttributeValue;

    fn add(self, rhs: ResolvedFlatAttributeValue) -> Self::Output {
        debug_assert!(
            matches!(
                self,
                ResolvedFlatAttributeValue::Other(_) | ResolvedFlatAttributeValue::Empty
            ),
            "Separator must have been pushed before, turning self into variant `Other`, or no value was set."
        );

        let mut s = self.to_string();
        s.push_str(&rhs.to_string());

        // Always "Other", because two values get combined
        ResolvedFlatAttributeValue::Other(s)
    }
}

impl std::ops::Add<&ValueSeparator> for ResolvedFlatAttributeValue {
    type Output = ResolvedFlatAttributeValue;

    fn add(self, rhs: &ValueSeparator) -> Self::Output {
        let mut s = self.to_string();
        s.push_str(rhs.as_str());

        // Always "Other", because separator is added to value
        ResolvedFlatAttributeValue::Other(s)
    }
}

impl std::fmt::Display for ResolvedFlatAttributeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ResolvedFlatAttributeValue::Quoted(q) => q.clone(),
            ResolvedFlatAttributeValue::Float(f) => f.to_string(),
            ResolvedFlatAttributeValue::Int(i) => i.to_string(),
            ResolvedFlatAttributeValue::Bool(b) => b.to_string(),
            ResolvedFlatAttributeValue::Other(o) => o.clone(),
            ResolvedFlatAttributeValue::Empty => String::default(),
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ResolvedArrayAttribute<'tslice> {
    ident: ResolvedAttributeIdent<'tslice>,
    value: Vec<ResolvedSingleAttribute<'tslice>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ResolvedAtRule<'tslice> {
    ident: ResolvedAttributeIdent<'tslice>,
    prelude: Option<String>,
    block: Option<Vec<ResolvedAttribute<'tslice>>>,
}
