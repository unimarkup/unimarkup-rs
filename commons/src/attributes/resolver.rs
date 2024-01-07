use std::ops::Deref;

use lightningcss::properties::PropertyId;

use crate::{
    lexer::{symbol::SymbolKind, token::TokenKind},
    parsing::ParserError,
};

use super::{
    html::HtmlAttributeId,
    resolved::{
        ResolvedAtRule, ResolvedAttribute, ResolvedAttributeIdent, ResolvedAttributeString,
        ResolvedAttributes, ResolvedFlatAttribute, ResolvedFlatAttributeValue,
        ResolvedNestedAttribute, ResolvedSingleAttribute,
    },
    rules::AtRuleId,
    token::{AttributeToken, AttributeTokenKind, Ident, QuotedPart, TokenPart, ValuePart},
    um::UmAttributeId,
};

#[derive(Debug, Default, Clone)]
pub struct AttributeResolverContext {
    pub um_only: bool,
}

pub struct AttributeResolver<'tslice> {
    tokens: &'tslice [AttributeToken],
}

impl<'tslice> AttributeResolver<'tslice> {
    pub fn new(tokens: &'tslice [AttributeToken]) -> Self {
        Self { tokens }
    }

    pub fn resolve(mut self, context: &AttributeResolverContext) -> ResolvedAttributes<'tslice> {
        let mut attrbs = ResolvedAttributes {
            html: Vec::new(),
            css: Vec::new(),
            um: Vec::new(),
        };

        while let Some(attrb) = self.resolve_next(context) {
            push_attrb(&mut attrbs, attrb);
        }

        attrbs
    }

    fn skip_fill_tokens(&mut self) {
        while let Some(kind) = self.tokens.first().map(|t| &t.kind) {
            match kind {
                AttributeTokenKind::Comment(_)
                | AttributeTokenKind::Newline
                | AttributeTokenKind::Whitespace => {
                    self.tokens = &self.tokens[1..];
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn next_kind(&mut self) -> Option<&'tslice AttributeTokenKind> {
        let next_kind = self.tokens.first().map(|t| &t.kind)?;
        self.tokens = &self.tokens[1..];
        Some(next_kind)
    }

    pub fn resolve_next(
        &mut self,
        context: &AttributeResolverContext,
    ) -> Option<ResolvedAttribute<'tslice>> {
        self.skip_fill_tokens();

        match &self.next_kind()? {
            AttributeTokenKind::Ident(ident) => self.resolve_single(ident, context),
            AttributeTokenKind::AtRuleIdent(at_rule_ident) => {
                self.resolve_at_rule(at_rule_ident, context)
            }
            AttributeTokenKind::SelectorPart(selector_part) => {
                self.resolve_nested(selector_part, context)
            }
            _ => {
                // TODO: set error log for invalid attribute semantics.
                None
            }
        }
    }

    fn resolve_ident<'i>(
        &self,
        ident: &'i Ident,
        context: &AttributeResolverContext,
    ) -> Option<ResolvedAttributeIdent<'i>> {
        let um_ident = UmAttributeId::try_from(ident.as_str()).ok()?;

        if matches!(um_ident, UmAttributeId::Custom(_)) && !context.um_only {
            if let Ok(html_ident) = HtmlAttributeId::try_from(ident.as_str()) {
                Some(ResolvedAttributeIdent::Html(html_ident))
            } else {
                let css_ident = PropertyId::from(ident.as_str());
                Some(ResolvedAttributeIdent::Css(css_ident))
            }
        } else {
            Some(ResolvedAttributeIdent::Um(um_ident))
        }
    }

    fn resolve_single(
        &mut self,
        ident: &'tslice Ident,
        context: &AttributeResolverContext,
    ) -> Option<ResolvedAttribute<'tslice>> {
        let resolved_ident = self.resolve_ident(ident, context)?;

        self.skip_fill_tokens();

        let value_resolver = self.get_kinds_until_semicolon()?;

        match value_resolver.kind {
            AttributeValueResolverKind::Empty => {
                //TODO: set error for empty value
                None
            }
            AttributeValueResolverKind::Single(single) => {
                let value = get_flat_attrb_value(&single, context)?;
                Some(ResolvedAttribute::Single(ResolvedSingleAttribute::Flat(
                    ResolvedFlatAttribute {
                        ident: resolved_ident,
                        value,
                        important: value_resolver.has_important,
                    },
                )))
            }
            AttributeValueResolverKind::Array(array) => {
                let mut array_parts = Vec::new();
                for part in array {
                    array_parts.push(get_flat_attrb_value(&part, context)?);
                }
                Some(ResolvedAttribute::Single(ResolvedSingleAttribute::Flat(
                    ResolvedFlatAttribute {
                        ident: resolved_ident,
                        value: ResolvedFlatAttributeValue::Array(array_parts),
                        important: value_resolver.has_important,
                    },
                )))
            }
            AttributeValueResolverKind::Nested(nested) => todo!(),
        }
    }

    fn get_kinds_until_semicolon(&mut self) -> Option<AttributeValueResolver> {
        let mut value_resolver = AttributeValueResolver::new();
        let mut must_be_nested = false;

        while let Some(kind) = self.next_kind() {
            match kind {
                AttributeTokenKind::Semicolon => break,
                AttributeTokenKind::Comment(_) => {}
                AttributeTokenKind::Newline | AttributeTokenKind::Whitespace => {
                    value_resolver.push(&AttributeTokenKind::Whitespace);
                }
                AttributeTokenKind::Comma | AttributeTokenKind::ValuePart(ValuePart::Important) => {
                    value_resolver.push(kind);
                }
                AttributeTokenKind::Nested(_) => {
                    if value_resolver.has_important {
                        //TODO: set error that "!important" must be last proper value
                        return None;
                    }

                    must_be_nested = true;
                    value_resolver.push(kind);
                }
                AttributeTokenKind::Ident(_)
                | AttributeTokenKind::AtRuleIdent(_)
                | AttributeTokenKind::AtRulePreludePart(_)
                | AttributeTokenKind::SelectorPart(_) => {
                    //TODO: set error for invalid token
                    return None;
                }
                AttributeTokenKind::Logic(_) => {
                    //TODO: handle logic
                    return None;
                }
                AttributeTokenKind::QuotedPart(_) | AttributeTokenKind::ValuePart(_) => {
                    if must_be_nested || value_resolver.has_important {
                        // TODO: set error that only nested may follow on nested
                        // set error that "!important" must be last proper value
                        return None;
                    }

                    value_resolver.push(kind);
                }
            }
        }

        Some(value_resolver)
    }

    fn resolve_at_rule(
        &mut self,
        at_rule: &'tslice AtRuleId,
        context: &AttributeResolverContext,
    ) -> Option<ResolvedAttribute<'tslice>> {
        todo!()
    }

    fn resolve_nested(
        &mut self,
        selector: &'tslice TokenPart,
        context: &AttributeResolverContext,
    ) -> Option<ResolvedAttribute<'tslice>> {
        todo!()
    }
}

fn get_flat_attrb_value(
    kinds: &[&AttributeTokenKind],
    context: &AttributeResolverContext,
) -> Option<ResolvedFlatAttributeValue> {
    if kinds.is_empty() {
        return None;
    }

    if kinds.len() == 1 {
        match kinds.first()? {
            AttributeTokenKind::QuotedPart(quoted) => Some(ResolvedFlatAttributeValue::Quoted(
                resolve_quoted(quoted, context),
                quoted.quote,
            )),
            AttributeTokenKind::ValuePart(value) => match value {
                ValuePart::Plain(val) => Some(ResolvedFlatAttributeValue::Other(val.clone())),
                ValuePart::Float(val) => Some(ResolvedFlatAttributeValue::Float(*val)),
                ValuePart::Int(val) => Some(ResolvedFlatAttributeValue::Int(*val)),
                ValuePart::Bool(val) => Some(ResolvedFlatAttributeValue::Bool(*val)),
                ValuePart::Important => {
                    unreachable!("'!important' part not passed to flatten fn.");
                }
            },
            _ => {
                unreachable!("Only quoted and value parts are passed.")
            }
        }
    } else {
        let s = kinds.iter().fold(String::new(), |mut s, kind| match kind {
            AttributeTokenKind::QuotedPart(quoted) => {
                s.push_str(&format!(
                    "{}{}{}",
                    quoted.quote,
                    resolve_quoted(quoted, context),
                    quoted.quote
                ));
                s
            }
            AttributeTokenKind::ValuePart(value) => {
                s.push_str(&value.as_unimarkup());
                s
            }
            _ => {
                unreachable!("Only quoted and value parts are passed.")
            }
        });

        Some(ResolvedFlatAttributeValue::Other(s))
    }
}

fn resolve_quoted(quoted: &QuotedPart, _context: &AttributeResolverContext) -> String {
    let mut s = String::new();

    for q in &quoted.parts {
        match &q.kind {
            super::token::QuotedPartKind::Plain(plain) => s.push_str(plain),
            super::token::QuotedPartKind::ImplicitSubstitution(impl_subst) => {
                s.push_str(impl_subst.subst())
            }
            super::token::QuotedPartKind::NamedSubstitution(_) => todo!(),
            super::token::QuotedPartKind::Logic(_) => todo!(),
            super::token::QuotedPartKind::EscapedNewline => {
                s.push_str(SymbolKind::Newline.as_str())
            }
            super::token::QuotedPartKind::Newline => s.push_str(SymbolKind::Whitespace.as_str()),
        }
    }

    s
}

fn push_attrb<'tslice>(
    attrbs: &mut ResolvedAttributes<'tslice>,
    attrb: ResolvedAttribute<'tslice>,
) {
    match attrb {
        ResolvedAttribute::Single(single) => {
            push_single_attrb(attrbs, single);
        }
        ResolvedAttribute::AtRule(at_rule) => {
            push_at_rule_attrb(attrbs, at_rule);
        }
        ResolvedAttribute::Nested(nested) => {
            push_nested_attrb(attrbs, nested);
        }
    }
}

fn push_single_attrb<'tslice>(
    attrbs: &mut ResolvedAttributes<'tslice>,
    single: ResolvedSingleAttribute<'tslice>,
) {
    let v = match single.ident() {
        super::resolved::ResolvedAttributeIdent::Html(_) => &mut attrbs.html,
        super::resolved::ResolvedAttributeIdent::Css(_) => &mut attrbs.css,
        super::resolved::ResolvedAttributeIdent::Um(_) => &mut attrbs.um,
    };

    v.push(ResolvedAttribute::Single(single));
}

fn push_at_rule_attrb<'tslice>(
    attrbs: &mut ResolvedAttributes<'tslice>,
    at_rule: ResolvedAtRule<'tslice>,
) {
    // Only allow css attributes inside at-rules for now
    attrbs.css.push(ResolvedAttribute::AtRule(at_rule));
}

fn push_nested_attrb<'tslice>(
    attrbs: &mut ResolvedAttributes<'tslice>,
    nested: ResolvedNestedAttribute<'tslice>,
) {
    let mut inner_attrbs = ResolvedAttributes {
        html: Vec::new(),
        css: Vec::new(),
        um: Vec::new(),
    };

    for nested_attrb in nested.body {
        push_attrb(&mut inner_attrbs, nested_attrb);
    }

    if !inner_attrbs.html.is_empty() {
        attrbs
            .html
            .push(ResolvedAttribute::Nested(ResolvedNestedAttribute {
                selectors: nested.selectors.clone(),
                body: inner_attrbs.html,
            }));
    }

    if !inner_attrbs.css.is_empty() {
        attrbs
            .css
            .push(ResolvedAttribute::Nested(ResolvedNestedAttribute {
                selectors: nested.selectors.clone(),
                body: inner_attrbs.css,
            }));
    }

    if !inner_attrbs.um.is_empty() {
        attrbs
            .um
            .push(ResolvedAttribute::Nested(ResolvedNestedAttribute {
                selectors: nested.selectors,
                body: inner_attrbs.um,
            }));
    }
}

enum AttributeValueResolverKind<'tslice> {
    Empty,
    Single(Vec<&'tslice AttributeTokenKind>),
    Array(Vec<Vec<&'tslice AttributeTokenKind>>),
    Nested(Vec<&'tslice AttributeTokenKind>),
}

struct AttributeValueResolver<'tslice> {
    kind: AttributeValueResolverKind<'tslice>,
    has_important: bool,
    prev_was_separator: bool,
}

impl<'tslice> AttributeValueResolver<'tslice> {
    fn new() -> Self {
        Self {
            kind: AttributeValueResolverKind::Empty,
            has_important: false,
            prev_was_separator: false,
        }
    }

    fn push(&mut self, kind: &'tslice AttributeTokenKind) {
        let is_array_separator = matches!(
            kind,
            AttributeTokenKind::Comma | AttributeTokenKind::Whitespace
        );
        let is_important = matches!(kind, AttributeTokenKind::ValuePart(ValuePart::Important));
        if is_important {
            self.has_important = true;
        }

        if !is_array_separator && !is_important {
            match &mut self.kind {
                AttributeValueResolverKind::Empty => {
                    if matches!(kind, AttributeTokenKind::Nested(_)) {
                        self.kind = AttributeValueResolverKind::Nested(vec![kind])
                    } else {
                        self.kind = AttributeValueResolverKind::Single(vec![kind])
                    }
                }
                AttributeValueResolverKind::Single(single) if self.prev_was_separator => {
                    self.kind =
                        AttributeValueResolverKind::Array(vec![std::mem::take(single), vec![kind]]);
                }
                AttributeValueResolverKind::Single(ref mut single) => {
                    single.push(kind);
                }
                AttributeValueResolverKind::Array(ref mut array) => {
                    if self.prev_was_separator {
                        array.push(vec![]);
                    }
                    // safe to unwrap, because at least one value was pushed for "Array" kind
                    array.last_mut().unwrap().push(kind);
                }
                AttributeValueResolverKind::Nested(ref mut nested) => {
                    if !matches!(kind, AttributeTokenKind::Nested(_)) {
                        panic!("Only nested attribute tokens allowed, once nested is encountered.");
                    }
                    nested.push(kind);
                }
            }
        }

        self.prev_was_separator = is_array_separator;
    }
}

#[cfg(test)]
mod test {

    use crate::{
        attributes::{
            resolver::{AttributeResolver, AttributeResolverContext},
            token::{AttributeTokenKind, AttributeTokens, Ident, QuotedPartKind, TokenPart},
            tokenize::{AttributeContext, AttributeTokenizer},
        },
        lexer::token::iterator::TokenIterator,
        parsing::{Parser, ParserError},
    };

    fn attrb_tokens(s: &str) -> Result<AttributeTokens, ParserError> {
        let tokens = crate::lexer::token::lex_str(s);
        let attrb_tokenizer: AttributeTokenizer<'_, '_> =
            AttributeTokenizer::new(TokenIterator::from(&*tokens), AttributeContext::default());

        let (_, res) = attrb_tokenizer.parse();
        res
    }

    #[test]
    fn dummy_test() {
        let attrb_tokens = attrb_tokens("{color: red; id: 'my-id'}").unwrap();

        let resolver = AttributeResolver::new(&attrb_tokens.tokens);
        let resolved = resolver.resolve(&AttributeResolverContext::default());
        dbg!(resolved);
    }
}
