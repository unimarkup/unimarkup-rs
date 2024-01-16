use std::ops::Deref;

use lightningcss::properties::PropertyId;

use crate::{
    lexer::{symbol::SymbolKind, token::TokenKind},
    parsing::Element,
};

use super::{
    html::HtmlAttributeId,
    resolved::{
        ResolvedAtRule, ResolvedAttribute, ResolvedAttributeIdent, ResolvedAttributeSelectors,
        ResolvedAttributeString, ResolvedAttributes, ResolvedFlatAttribute,
        ResolvedFlatAttributeValue, ResolvedNestedAttribute, ResolvedSingleAttribute,
    },
    rules::AtRuleId,
    token::{
        AttributeToken, AttributeTokenKind, AttributeTokens, IdentOrSelectorPart, QuotedPart,
        ValuePart,
    },
    um::UmAttributeId,
};

#[derive(Debug, Default, Clone)]
pub struct AttributeResolverContext {
    pub um_only: bool,
}

pub struct AttributeResolver<'tslice> {
    tokens: &'tslice [AttributeToken],
    id: Option<&'tslice str>,
}

impl<'tslice> AttributeResolver<'tslice> {
    pub fn new(attrb_tokens: &'tslice AttributeTokens) -> Self {
        Self {
            tokens: &attrb_tokens.tokens,
            id: attrb_tokens.id.as_deref(),
        }
    }

    pub fn resolve(mut self, context: &AttributeResolverContext) -> ResolvedAttributes<'tslice> {
        let mut attrbs = ResolvedAttributes {
            id: self.id,
            html: Vec::new(),
            css: Vec::new(),
            um: Vec::new(),
        };

        while let Some(attrb) = self.resolve_next(context) {
            push_attrb(&mut attrbs, attrb);
        }

        attrbs
    }

    fn next_kind(&mut self) -> Option<&'tslice AttributeTokenKind> {
        while let Some(kind) = self.tokens.first().map(|t| &t.kind) {
            self.tokens = &self.tokens[1..];

            // Skip fill tokens
            if !matches!(
                kind,
                AttributeTokenKind::Comment(_) | AttributeTokenKind::Newline
            ) {
                return Some(kind);
            }
        }

        None
    }

    fn peek_kind(&mut self) -> Option<&'tslice AttributeTokenKind> {
        while let Some(kind) = self.tokens.first().map(|t| &t.kind) {
            // Skip fill tokens
            if !matches!(
                kind,
                AttributeTokenKind::Comment(_) | AttributeTokenKind::Newline
            ) {
                return Some(kind);
            }

            self.tokens = &self.tokens[1..];
        }

        None
    }

    pub fn resolve_next(
        &mut self,
        context: &AttributeResolverContext,
    ) -> Option<ResolvedAttribute<'tslice>> {
        match &self.next_kind()? {
            AttributeTokenKind::IdentOrSelectorPart(IdentOrSelectorPart::Plain(plain_part)) => {
                let next = self.next_kind()?;
                if next == &AttributeTokenKind::IdentMarker {
                    self.resolve_single(plain_part, context)
                } else {
                    // must be selector, because unquoted ident cannot span spaces
                    let mut selectors = next.as_unimarkup();
                    let mut nested = false;
                    while let Some(kind) = self.peek_kind() {
                        if matches!(kind, AttributeTokenKind::Nested(_)) {
                            nested = true;
                            break;
                        } else if kind == &AttributeTokenKind::Semicolon {
                            nested = false;
                            break;
                        }

                        let selector_part = self.next_kind()?.as_unimarkup();
                        selectors.push_str(&selector_part);
                    }

                    if nested {
                        self.resolve_nested(selectors.into(), context)
                    } else {
                        // TODO: set error for bad selector attribute
                        Some(ResolvedAttribute::Invalid)
                    }
                }
            }
            AttributeTokenKind::IdentOrSelectorPart(IdentOrSelectorPart::Quoted(quoted_ident)) => {
                // Quoted ident/selector only allowed for ident
                if self.next_kind()?.clone() != AttributeTokenKind::IdentMarker {
                    return None;
                }

                self.resolve_single(&quoted_ident.ident, context)
            }
            AttributeTokenKind::AtRuleIdent(at_rule_ident) => {
                self.resolve_at_rule(at_rule_ident, context)
            }

            _ => {
                // TODO: set error log for invalid attribute semantics.
                Some(ResolvedAttribute::Invalid)
            }
        }
    }

    fn resolve_ident<'i>(
        &self,
        ident: &'i str,
        context: &AttributeResolverContext,
    ) -> Option<ResolvedAttributeIdent<'i>> {
        let um_ident = UmAttributeId::try_from(ident).ok()?;

        if matches!(um_ident, UmAttributeId::Custom(_)) && !context.um_only {
            if let Ok(html_ident) = HtmlAttributeId::try_from(ident) {
                Some(ResolvedAttributeIdent::Html(html_ident))
            } else {
                let css_ident = PropertyId::from(ident);
                Some(ResolvedAttributeIdent::Css(css_ident))
            }
        } else {
            Some(ResolvedAttributeIdent::Um(um_ident))
        }
    }

    fn resolve_single(
        &mut self,
        ident: &'tslice str,
        context: &AttributeResolverContext,
    ) -> Option<ResolvedAttribute<'tslice>> {
        let resolved_ident = self.resolve_ident(ident, context)?;

        let mut value = ResolvedFlatAttributeValue::Empty;
        let mut important = false;

        while let Some(kind) = self.next_kind() {
            match kind {
                AttributeTokenKind::FlatValue(v) => {
                    value = value + value_to_flat_attrb(v, context);
                }
                AttributeTokenKind::ValueSeparator(separator) => {
                    value = value + separator;
                }
                AttributeTokenKind::Important => {
                    important = true;
                }
                AttributeTokenKind::Semicolon => {
                    break;
                }
                _ => {
                    return None;
                }
            }
        }

        if let ResolvedFlatAttributeValue::Other(o) = value {
            value = ResolvedFlatAttributeValue::Other(o.trim().to_string());
        }

        Some(ResolvedAttribute::Single(ResolvedSingleAttribute::Flat(
            ResolvedFlatAttribute {
                ident: resolved_ident,
                value,
                important,
            },
        )))
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
        selectors: ResolvedAttributeSelectors,
        context: &AttributeResolverContext,
    ) -> Option<ResolvedAttribute<'tslice>> {
        todo!()
    }
}

fn value_to_flat_attrb(
    value: &ValuePart,
    _context: &AttributeResolverContext,
) -> ResolvedFlatAttributeValue {
    match value {
        ValuePart::Plain(val) => ResolvedFlatAttributeValue::Other(val.clone()),
        ValuePart::Float(val) => ResolvedFlatAttributeValue::Float(*val),
        ValuePart::Int(val) => ResolvedFlatAttributeValue::Int(*val),
        ValuePart::Bool(val) => ResolvedFlatAttributeValue::Bool(*val),
        // TODO: use own variant for CssFn
        ValuePart::CssFn(css_fn) => ResolvedFlatAttributeValue::Other(css_fn.as_unimarkup()),
        ValuePart::Quoted(quoted) => ResolvedFlatAttributeValue::Quoted(quoted.resolve()),
    }
}

// fn resolve_quoted(quoted: &QuotedPart, _context: &AttributeResolverContext) -> String {
//     let mut s = String::new();

//     for q in &quoted.parts {
//         match &q.kind {
//             super::token::QuotedPartKind::Plain(plain) => s.push_str(plain),
//             super::token::QuotedPartKind::ImplicitSubstitution(impl_subst) => {
//                 s.push_str(impl_subst.subst())
//             }
//             super::token::QuotedPartKind::NamedSubstitution(_) => todo!(),
//             super::token::QuotedPartKind::Logic(_) => todo!(),
//             super::token::QuotedPartKind::EscapedNewline => {
//                 s.push_str(SymbolKind::Newline.as_str())
//             }
//             super::token::QuotedPartKind::Newline => s.push_str(SymbolKind::Whitespace.as_str()),
//         }
//     }

//     s
// }

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
        ResolvedAttribute::Invalid => {}
    }
}

fn push_single_attrb<'tslice>(
    attrbs: &mut ResolvedAttributes<'tslice>,
    single: ResolvedSingleAttribute<'tslice>,
) {
    let v = match single.ident() {
        super::resolved::ResolvedAttributeIdent::Html(HtmlAttributeId::Id) => {
            // TODO: handle 'id' attribute properly
            // Ignored here, because the id attribute is retrieved before resolving
            return;
        }
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
        id: None,
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

#[cfg(test)]
mod test {

    use crate::{
        attributes::{
            resolver::{AttributeResolver, AttributeResolverContext},
            token::AttributeTokens,
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
        let attrb_tokens =
            attrb_tokens("{color: red; disabled: false; class: first-class second-class}").unwrap();

        let resolver = AttributeResolver::new(&attrb_tokens);
        let resolved = resolver.resolve(&AttributeResolverContext::default());
        dbg!(resolved);
    }
}
