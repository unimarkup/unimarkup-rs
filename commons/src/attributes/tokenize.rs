use std::marker::PhantomData;

use itertools::{Itertools, PeekingNext};

use crate::{
    attributes::rules,
    comments::{Comment, COMMENT_TOKEN_KIND},
    lexer::{
        position::Position,
        token::{
            iterator::{EndMatcher, IteratorEndFn, TokenIterator},
            Token, TokenKind,
        },
    },
    parsing::{Element, Parser, ParserError},
};

use super::{
    rules::AtRuleId,
    token::{
        AttributeToken, AttributeTokenKind, AttributeTokens, QuotedPart, QuotedValuePart, TokenPart,
    },
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
                TokenKind::Semicolon(len) => {
                    self.parse_semicolon(&mut attrb_tokens, len);
                }
                TokenKind::OpenBrace => {
                    // Only logic allowed at this point
                    // TODO: add logic parser
                    self.iter
                        .next()
                        .expect("Peek was some, so next must return value.");
                }
                TokenKind::CloseBrace => {
                    // Must be closing brace, because inner braces are consumed in recursive fn calls.
                    let token = self
                        .iter
                        .next()
                        .expect("Peek was some, so next must return value.");

                    return (
                        self,
                        Ok(AttributeTokens {
                            tokens: attrb_tokens,
                            implicit_closed: false,
                            start: open_token.start,
                            end: token.end,
                        }),
                    );
                }
                TokenKind::Whitespace => {
                    let token = self
                        .iter
                        .next()
                        .expect("Peek was some, so next must return value.");
                    attrb_tokens.push(AttributeToken {
                        kind: AttributeTokenKind::Whitespace,
                        start: token.start,
                        end: token.end,
                    })
                }
                TokenKind::Newline => {
                    let token = self
                        .iter
                        .next()
                        .expect("Peek was some, so next must return value.");
                    attrb_tokens.push(AttributeToken {
                        kind: AttributeTokenKind::Newline,
                        start: token.start,
                        end: token.end,
                    })
                }
                TokenKind::EscapedWhitespace | TokenKind::EscapedNewline => {
                    // TODO: set error that escaped whitespace/newline is not allowed as identifier start
                    return (self, Err(ParserError::SyntaxViolation));
                }
                _ => {
                    // Note: Quotes may be part of the ident
                    let res = self.parse_single_or_nested(&mut attrb_tokens);
                    if let Err(err) = res {
                        return (self, Err(err));
                    }
                }
            }
        }

        let end = attrb_tokens.last().map(|t| t.end).unwrap_or(open_token.end);
        (
            self,
            Ok(AttributeTokens {
                tokens: attrb_tokens,
                implicit_closed: true,
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

impl<'slice, 'input, P, T, C> AttributeTokenizer<'slice, 'input, P, T, C>
where
    P: Parser<'slice, 'input, T, C>,
    T: Element,
    C: Default,
{
    fn parse_single_or_nested(
        &mut self,
        attrb_tokens: &mut Vec<AttributeToken>,
    ) -> Result<(), ParserError> {
        let ident_part = self.parse_ident_part(attrb_tokens)?;

        self.parse_comments(attrb_tokens)?; // To consume any amount of comments, spaces, newlines between ident and value

        match ident_part {
            IdentSelectorKind::Ident => {
                // parse value or nested
                if self.iter.peek_kind() == Some(TokenKind::OpenBrace) {
                    self.parse_nested(attrb_tokens)
                } else {
                    self.parse_value(attrb_tokens)
                }
            }
            IdentSelectorKind::Selector => {
                // parse nested
                // next token must either be OpenBrace or None, because ident_parse() wouldn't have stopped otherwise
                self.parse_nested(attrb_tokens)
            }
            IdentSelectorKind::NoValue => {
                // either reached semicolon or EOI
                Ok(())
            }
        }
    }

    fn parse_value(&mut self, attrb_tokens: &mut Vec<AttributeToken>) -> Result<(), ParserError> {
        while let Some(token) = self.iter.peek() {
            match token.kind {
                TokenKind::OpenBrace => {
                    todo!()
                }
                TokenKind::Semicolon(len) => {
                    todo!()
                }
                _ => {
                    todo!()
                }
            }
        }

        todo!()
    }

    fn parse_nested(&mut self, attrb_tokens: &mut Vec<AttributeToken>) -> Result<(), ParserError> {
        let open_token = self.iter.next().ok_or(ParserError::InvalidStart)?;
        debug_assert_eq!(
            open_token.kind,
            TokenKind::OpenBrace,
            "Nested attribute parser called for token other than 'OpenBrace'."
        );

        let mut end = open_token.end;

        match self.iter.peek_kind() {
            Some(kind) if kind == TokenKind::Dollar(1) => {
                // parse logic
                todo!()
            }
            Some(kind) if kind == TokenKind::CloseBrace => {
                let next_token = self
                    .iter
                    .next()
                    .expect("Peeked above that `next` is some token.");
                attrb_tokens.push(AttributeToken {
                    kind: AttributeTokenKind::Nested(AttributeTokens {
                        tokens: vec![],
                        implicit_closed: false,
                        start: open_token.start,
                        end: next_token.end,
                    }),
                    start: open_token.start,
                    end: next_token.end,
                });
                return Ok(());
            }
            Some(_) => {
                todo!()
            }
            None => {
                attrb_tokens.push(AttributeToken {
                    kind: AttributeTokenKind::Nested(AttributeTokens {
                        tokens: vec![],
                        implicit_closed: true,
                        start: open_token.start,
                        end,
                    }),
                    start: open_token.start,
                    end,
                });
                return Ok(());
            }
        }
    }

    fn parse_comments(
        &mut self,
        attrb_tokens: &mut Vec<AttributeToken>,
    ) -> Result<(), ParserError> {
        while let Some(token) = self.iter.peek() {
            match token.kind {
                TokenKind::Semicolon(len) => {
                    if len == Comment::keyword_len() {
                        self.parse_semicolon(attrb_tokens, len);
                    } else {
                        return Ok(());
                    }
                }
                TokenKind::Newline => {
                    attrb_tokens.push(AttributeToken {
                        kind: AttributeTokenKind::Newline,
                        start: token.start,
                        end: token.end,
                    });
                    self.iter.next();
                }
                TokenKind::Whitespace => {
                    attrb_tokens.push(AttributeToken {
                        kind: AttributeTokenKind::Whitespace,
                        start: token.start,
                        end: token.end,
                    });
                    self.iter.next();
                }
                _ => {
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    fn parse_ident_part(
        &mut self,
        attrb_tokens: &mut Vec<AttributeToken>,
    ) -> Result<IdentSelectorKind, ParserError> {
        let mut ident_or_selector = String::new();

        let start_token = self.iter.peek().expect("Peeked Some in `parse()` loop.");
        let mut start_pos = start_token.start;
        let mut start_offset = start_token.offset.start;
        let mut end_offset = start_token.offset.end;
        let mut end = start_token.end;
        let mut is_selector = false;
        let mut quote_parsing = QuoteParsing::Ident;

        while let Some(token) = self.iter.peek() {
            match token.kind {
                TokenKind::Colon(1) => {
                    ident_or_selector.push_str(&token.input[start_offset..token.offset.start]);
                    self.iter.next();

                    if let Some(next_token) = self.iter.peeking_next(|t| {
                        matches!(t.kind, TokenKind::Whitespace | TokenKind::Newline)
                    }) {
                        self.iter.skip_to_peek();
                        attrb_tokens.push(AttributeToken {
                            kind: AttributeTokenKind::Ident(ident_or_selector.into()),
                            start: start_pos,
                            end: token.end,
                        });
                        return Ok(IdentSelectorKind::Ident);
                    }
                    ident_or_selector.push(':');
                    end = token.end;
                }
                TokenKind::Newline => {
                    self.iter.next();

                    // must be selector, because valid idents do not span multiple lines
                    ident_or_selector.push_str(&token.input[start_offset..token.offset.start]);
                    attrb_tokens.push(AttributeToken {
                        kind: AttributeTokenKind::SelectorPart(ident_or_selector.into()),
                        start: start_pos,
                        end: token.start,
                    });
                    attrb_tokens.push(AttributeToken {
                        kind: AttributeTokenKind::Newline,
                        start: token.start,
                        end: token.end,
                    });

                    is_selector = true; // Ident does not allow newlines
                    ident_or_selector = String::new();
                    start_pos = token.end;
                    start_offset = token.offset.end;
                    end = token.end;
                }
                TokenKind::EscapedNewline => {
                    return Err(ParserError::SyntaxViolation);
                }
                TokenKind::Semicolon(len) => {
                    ident_or_selector.push_str(&token.input[start_offset..token.offset.start]);
                    is_selector = true;

                    if !ident_or_selector.is_empty() {
                        // Must be selector, because Colon must come directly after ident
                        let kind = AttributeTokenKind::SelectorPart(ident_or_selector.into());
                        attrb_tokens.push(AttributeToken {
                            kind,
                            start: start_pos,
                            end: token.start,
                        });
                        ident_or_selector = String::new();
                    }

                    if len != Comment::keyword_len() {
                        // attribute without value => will be handled during resolve step
                        // semicolon is parsed in main `parse` fn
                        return Ok(IdentSelectorKind::NoValue);
                    }

                    self.parse_semicolon(attrb_tokens, len);

                    if let Some(next_token) = self.iter.peek() {
                        start_pos = next_token.start;
                        start_offset = next_token.offset.start;
                        end = next_token.start;
                    }
                }
                TokenKind::OpenBrace => {
                    break;
                }
                TokenKind::SingleQuote | TokenKind::DoubleQuote => {
                    ident_or_selector.push_str(&token.input[start_offset..token.offset.start]);

                    if !ident_or_selector.is_empty() {
                        let kind = AttributeTokenKind::SelectorPart(ident_or_selector.into());
                        attrb_tokens.push(AttributeToken {
                            kind,
                            start: start_pos,
                            end: token.start,
                        });
                        is_selector = true;
                        ident_or_selector = String::new();
                    }

                    attrb_tokens.push(
                        self.parse_quote(quote_parsing)
                            .expect("Must be quoted content, because peek is quote."),
                    );
                }
                _ => {
                    // treat quoted content in attribute selectors (`[]`) as values
                    if token.kind == TokenKind::OpenBracket {
                        quote_parsing = QuoteParsing::Value;
                        is_selector = true;
                    } else if token.kind == TokenKind::CloseBracket {
                        quote_parsing = QuoteParsing::Ident;
                        is_selector = true;
                    }

                    self.iter.next();
                    end = token.end;
                }
            }

            end_offset = token.offset.end;
        }

        ident_or_selector.push_str(&start_token.input[start_offset..end_offset]);

        if !ident_or_selector.is_empty() {
            // always selector, because a valid ident must end with `: `.
            attrb_tokens.push(AttributeToken {
                kind: AttributeTokenKind::SelectorPart(ident_or_selector.into()),
                start: start_pos,
                end,
            });
        }

        return Ok(IdentSelectorKind::Selector);
    }

    fn parse_quote(&mut self, variant: QuoteParsing) -> Result<AttributeToken, ParserError> {
        let open_quote = self.iter.next().ok_or(ParserError::InvalidStart)?;
        let quote_char = if open_quote.kind == TokenKind::DoubleQuote {
            '"'
        } else {
            '\''
        };
        let mut quote_tokens = Vec::new();
        let mut start_offset = open_quote.offset.end;
        let mut start = open_quote.start;
        let mut end = open_quote.end;

        while let Some(token) = self.iter.peeking_next(|_| true) {
            if (variant == QuoteParsing::Value
                && matches!(token.kind, TokenKind::ImplicitSubstitution(_)))
                || matches!(token.kind, TokenKind::Newline | TokenKind::EscapedNewline)
                || token.kind == open_quote.kind
            {
                let plain = &token.input[start_offset..token.offset.start];
                quote_tokens.push(QuotedValuePart {
                    kind: super::token::QuotedPartKind::Plain(plain.to_string()),
                    start,
                    end,
                });
                start_offset = token.offset.start;
                start = token.start;
                end = token.end;
            }

            match token.kind {
                TokenKind::ImplicitSubstitution(subst) if variant == QuoteParsing::Value => {
                    quote_tokens.push(QuotedValuePart {
                        kind: super::token::QuotedPartKind::ImplicitSubstitution(subst),
                        start,
                        end,
                    });
                    start_offset = token.offset.end;
                    start = token.end;
                }
                TokenKind::OpenBrace if variant == QuoteParsing::Value => {
                    // might be logic
                    // TODO implement logic parsing
                }
                TokenKind::Colon(2) if variant == QuoteParsing::Value => {
                    // might be named subst
                    // TODO: implement named subst
                }
                TokenKind::Newline => {
                    quote_tokens.push(QuotedValuePart {
                        kind: super::token::QuotedPartKind::Newline,
                        start,
                        end,
                    });
                    start_offset = token.offset.end;
                    start = token.end;
                }
                TokenKind::EscapedNewline => {
                    quote_tokens.push(QuotedValuePart {
                        kind: super::token::QuotedPartKind::EscapedNewline,
                        start,
                        end,
                    });
                    start_offset = token.offset.end;
                    start = token.end;
                }
                k if k == open_quote.kind => {
                    // closing quote
                    self.iter.skip_to_peek();
                    return Ok(AttributeToken {
                        kind: AttributeTokenKind::QuotedPart(QuotedPart {
                            parts: quote_tokens,
                            quote: quote_char,
                        }),
                        start: open_quote.start,
                        end,
                    });
                }
                _ => {}
            }

            end = token.end;
        }

        self.iter.skip_to_peek();
        Ok(AttributeToken {
            kind: AttributeTokenKind::QuotedPart(QuotedPart {
                parts: quote_tokens,
                quote: quote_char,
            }),
            start: open_quote.start,
            end,
        })
    }

    fn parse_semicolon(&mut self, attrb_tokens: &mut Vec<AttributeToken>, len: usize) {
        if len == Comment::keyword_len() {
            let comment = Comment::parse(&mut self.iter)
                .expect("Valid comment start always leads to a comment.");
            attrb_tokens.push(AttributeToken {
                kind: AttributeTokenKind::Semicolon,
                start: comment.start(),
                end: comment.end(),
            })
        } else {
            // Multiple semicolons are combined to one, because CSS syntax uses semicolons as statement ends.
            let semicolon_token = self
                .iter
                .next()
                .expect("Peek was some, so next must return value");

            attrb_tokens.push(AttributeToken {
                kind: AttributeTokenKind::Semicolon,
                start: semicolon_token.start,
                end: semicolon_token.end,
            })
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum IdentSelectorKind {
    Ident,
    Selector,
    NoValue,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum QuoteParsing {
    /// To consider tokens allowed for quoted idents.
    Ident,
    /// To consider tokens allowed for attribute values.
    Value,
}
