use std::marker::PhantomData;

use itertools::Itertools;

use crate::{
    attributes::rules,
    lexer::token::{iterator::TokenIterator, TokenKind},
    parsing::{Element, Parser, ParserError},
};

use super::{
    rules::AtRuleId,
    token::{AttributeToken, AttributeTokenKind, AttributeTokens},
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AttributeContext {
    in_page_rule: bool,
}

pub struct AttributeTokenizer<'slice, 'input, P, T, C>
where
    // The parser to use to parse Unimarkup content inside logic elements.
    P: Parser<'slice, 'input, T, C>,
    T: Element,
    C: Default,
{
    pub iter: TokenIterator<'slice, 'input>,
    /// Context for attribute tokenization.
    pub context: AttributeContext,
    um_parser_context: Option<C>,
    um_parser: PhantomData<P>,
    um_parser_ok_result: PhantomData<T>,
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

impl<'slice, 'input, P, T, C> Parser<'slice, 'input, AttributeTokens, AttributeContext>
    for AttributeTokenizer<'slice, 'input, P, T, C>
where
    P: Parser<'slice, 'input, T, C>,
    T: Element,
    C: Default,
{
    fn new(iter: TokenIterator<'slice, 'input>, context: AttributeContext) -> Self {
        Self {
            iter,
            context,
            um_parser_context: None,
            um_parser: PhantomData,
            um_parser_ok_result: PhantomData,
        }
    }

    fn parse(mut self) -> (Self, Result<AttributeTokens, ParserError>) {
        // Start: Ident | SelectorPart | AtRuleIdent | Logic | Comment | Whitespace | Newline
        // => after Ident: (Comment | Whitespace | Newline)* (ValuePart | Nested | Logic | QuotedValue) (Comment | Whitespace | Newline)*
        //    => after ValuePart: (ValuePart | Logic | QuotedValue | Comment | Whitespace | Newline)* Important? Semicolon
        //    => after Nested: (Comment | Whitespace | Newline)* (Nested | <Start>)
        //    => after Logic: <after Ident> | Important? (Comment | Whitespace | Newline)* Semicolon
        // => after SelectorPart: (Comment | Whitespace | Newline)* (SelectorPart | Nested | Logic)
        //    => after Nested: (Comment | Whitespace | Newline)* <Start>
        //    => after Logic: (Comment | Whitespace | Newline)* (SelectorPart | Nested | Logic)
        // => after AtRuleIdent: (Comment | Whitespace | Newline)* (AtRulePreludePart | Nested)
        //    => after AtRulePreludePart: <after AtRuleIdent> | Semicolon
        //    => after Nested: <Start>
        // => after Logic | Comment | Whitespace | Newline: <Start>
        //
        // Nested: `{` <Start>* `}`
        // QuotedValue: (`"` QuotedValuePartKind* `"`) | (`'` QuotedValuePartKind* `'`)

        let mut attrb_tokens = Vec::new();
        let open_token = match self.iter.next() {
            Some(token) if token.kind == TokenKind::OpenBrace => token,
            Some(_) | None => {
                return (self, Err(ParserError::InvalidStart));
            }
        };
        let next_kind = self.iter.peek_kind();

        // Would be logic start
        if next_kind == Some(TokenKind::Dollar(1)) {
            return (self, Err(ParserError::InvalidStart));
        }

        while let Some(next_kind) = self.iter.peek_kind() {
            match next_kind {
                TokenKind::At(len) => {
                    if len != 1 {
                        // TODO: set log error for multiple `@`
                        return (self, Err(ParserError::SyntaxViolation));
                    }
                    let at_rule_parsed = rules::parse_at_rule(&mut self, &mut attrb_tokens);
                    if !at_rule_parsed {
                        return (self, Err(ParserError::SyntaxViolation));
                    }
                }
                _ => {
                    let attribute_parsed = parse_single_or_nested(&mut self, &mut attrb_tokens);
                    if !attribute_parsed {
                        return (self, Err(ParserError::SyntaxViolation));
                    }
                }
            }
        }

        let end = attrb_tokens.last().map(|t| t.end).unwrap_or(open_token.end);
        (
            self,
            Ok(AttributeTokens {
                tokens: attrb_tokens,
                implicit_closed: true, // TODO: handle implicit closed correctly
                start: open_token.start,
                end,
            }),
        )
    }

    fn context(&self) -> &AttributeContext {
        &self.context
    }

    fn context_mut(&mut self) -> &mut AttributeContext {
        &mut self.context
    }

    fn iter(&mut self) -> &mut TokenIterator<'slice, 'input> {
        &mut self.iter
    }

    fn into_inner(self) -> (TokenIterator<'slice, 'input>, AttributeContext) {
        (self.iter, self.context)
    }
}

fn parse_single_or_nested<'slice, 'input, P, T, C>(
    tokenizer: &mut AttributeTokenizer<'slice, 'input, P, T, C>,
    attrb_tokens: &mut Vec<AttributeToken>,
) -> bool
where
    P: Parser<'slice, 'input, T, C>,
    T: Element,
    C: Default,
{
    todo!()
}
