use itertools::PeekingNext;

use crate::{
    attributes::rules,
    comments::Comment,
    lexer::{
        position::Position,
        token::{iterator::TokenIterator, TokenKind},
    },
    parsing::{Element, Parser, ParserError},
};

use super::token::{
    AttributeToken, AttributeTokenKind, AttributeTokens, QuotedPart, QuotedValuePart,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AttributeContext {
    in_page_rule: bool,
}

pub struct AttributeTokenizer<'slice, 'input> {
    pub iter: TokenIterator<'slice, 'input>,
    /// Context for attribute tokenization.
    pub context: AttributeContext,
}

impl<'slice, 'input> Parser<'slice, 'input, AttributeTokens, AttributeContext>
    for AttributeTokenizer<'slice, 'input>
{
    fn new(iter: TokenIterator<'slice, 'input>, context: AttributeContext) -> Self {
        Self { iter, context }
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

        match self.parse_block(&mut attrb_tokens) {
            Ok(impl_closed) => {
                if impl_closed {
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
                } else {
                    let close_token = self
                        .iter
                        .next()
                        .expect("Must be CloseBrace, because block is explicitly closed.");
                    debug_assert_eq!(
                        close_token.kind,
                        TokenKind::CloseBrace,
                        "Explicitly closed attribute block has kind '{:?}' instead of 'CloseBrace'",
                        close_token.kind
                    );
                    (
                        self,
                        Ok(AttributeTokens {
                            tokens: attrb_tokens,
                            implicit_closed: false,
                            start: open_token.start,
                            end: close_token.end,
                        }),
                    )
                }
            }
            Err(err) => (self, Err(err)),
        }
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

impl<'slice, 'input> AttributeTokenizer<'slice, 'input> {
    fn parse_block(&mut self, attrb_tokens: &mut Vec<AttributeToken>) -> Result<bool, ParserError> {
        while let Some(next_kind) = self.iter.peek_kind() {
            match next_kind {
                TokenKind::At(len) => {
                    if len != 1 {
                        // TODO: set log error for multiple `@`
                        return Err(ParserError::SyntaxViolation);
                    }
                    let at_rule_parsed = rules::parse_at_rule(self, attrb_tokens);
                    if !at_rule_parsed {
                        return Err(ParserError::SyntaxViolation);
                    }
                }
                TokenKind::Semicolon(len) => {
                    self.parse_semicolon(attrb_tokens, len);
                }
                TokenKind::OpenBrace => {
                    // Only logic allowed at this point
                    // TODO: add logic parser
                    self.iter
                        .next()
                        .expect("Peek was some, so next must return value.");
                }
                TokenKind::CloseBrace => {
                    // Must be closing brace, because inner braces are consumed in inner fn calls.
                    return Ok(false);
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
                    return Err(ParserError::SyntaxViolation);
                }
                _ => {
                    // Note: Quotes may be part of the ident
                    self.parse_single_or_nested(attrb_tokens)?;
                }
            }
        }

        Ok(true)
    }

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
                    // TODO: check for logic element

                    // Nested not allowed in value context
                    return Err(ParserError::SyntaxViolation);
                }
                TokenKind::CloseBrace => {
                    return Ok(());
                }
                TokenKind::Semicolon(1) => {
                    self.iter.next();
                    attrb_tokens.push(AttributeToken {
                        kind: AttributeTokenKind::Semicolon,
                        start: token.start,
                        end: token.end,
                    });
                    return Ok(());
                }
                TokenKind::EscapedNewline | TokenKind::EscapedWhitespace => {
                    return Err(ParserError::SyntaxViolation);
                }
                TokenKind::Newline => attrb_tokens.push(AttributeToken {
                    kind: AttributeTokenKind::Newline,
                    start: token.start,
                    end: token.end,
                }),
                TokenKind::SingleQuote | TokenKind::DoubleQuote => attrb_tokens.push(
                    self.parse_quote(QuoteParsing::Value)
                        .expect("Must be quoted content, because peek is quote."),
                ),
                TokenKind::Whitespace => attrb_tokens.push(AttributeToken {
                    kind: AttributeTokenKind::Whitespace,
                    start: token.start,
                    end: token.end,
                }),
                _ => {
                    self.parse_value_part(attrb_tokens)?;
                }
            }
        }

        Ok(())
    }

    fn parse_value_part(
        &mut self,
        attrb_tokens: &mut Vec<AttributeToken>,
    ) -> Result<(), ParserError> {
        let mut part = String::new();
        // Peeked some token before calling "value_part"
        let open_token = self.iter.peek().ok_or(ParserError::SyntaxViolation)?;
        let mut end = Position::default();
        let mut is_num = false;

        while let Some(token) = self.iter.peek() {
            match token.kind {
                TokenKind::CloseBrace
                | TokenKind::OpenBrace
                | TokenKind::Semicolon(_)
                | TokenKind::Blankline
                | TokenKind::Newline
                | TokenKind::Whitespace
                | TokenKind::EscapedNewline
                | TokenKind::EscapedWhitespace => {
                    end = token.start;
                    break;
                }
                _ => {
                    self.iter.next();

                    if is_num && !matches!(token.kind, TokenKind::Digit(_)) {
                        is_num = false;
                    } else if !is_num
                        && part.is_empty()
                        && matches!(token.kind, TokenKind::Digit(_))
                    {
                        is_num = true;
                    }

                    part.push_str(&token.input[token.offset.start..token.offset.end]);
                    end = token.end;
                }
            }
        }

        if !part.is_empty() {
            let combined_part = if is_num {
                match part.parse() {
                    Ok(num) => super::token::ValuePart::Num(num),
                    Err(_) => super::token::ValuePart::Plain(part),
                }
            } else if part == "!important" {
                super::token::ValuePart::Important
            } else {
                super::token::ValuePart::Plain(part)
            };

            attrb_tokens.push(AttributeToken {
                kind: AttributeTokenKind::ValuePart(combined_part),
                start: open_token.start,
                end,
            })
        }

        Ok(())
    }

    fn parse_nested(&mut self, attrb_tokens: &mut Vec<AttributeToken>) -> Result<(), ParserError> {
        let open_token = self.iter.next().ok_or(ParserError::InvalidStart)?;
        debug_assert_eq!(
            open_token.kind,
            TokenKind::OpenBrace,
            "Nested attribute parser called for token other than 'OpenBrace'."
        );

        match self.iter.peek_kind() {
            Some(TokenKind::Dollar(1)) => {
                // TODO: parse logic
                todo!()
            }
            Some(_) => {
                let mut inner_tokens = Vec::new();
                let impl_closed = self.parse_block(&mut inner_tokens)?;
                let inner_start = inner_tokens
                    .first()
                    .map(|t| t.start)
                    .unwrap_or(open_token.start);
                let inner_end = inner_tokens.last().map(|t| t.end).unwrap_or(open_token.end);

                let outer_end = if impl_closed {
                    inner_end
                } else {
                    let close_token = self
                        .iter
                        .next()
                        .expect("Must be CloseBrace, because block is explicitly closed.");

                    close_token.end
                };

                attrb_tokens.push(AttributeToken {
                    kind: AttributeTokenKind::Nested(AttributeTokens {
                        tokens: inner_tokens,
                        implicit_closed: impl_closed,
                        start: inner_start,
                        end: inner_end,
                    }),
                    start: open_token.start,
                    end: outer_end,
                });
            }
            None => {
                attrb_tokens.push(AttributeToken {
                    kind: AttributeTokenKind::Nested(AttributeTokens {
                        tokens: vec![],
                        implicit_closed: true,
                        start: open_token.start,
                        end: open_token.end,
                    }),
                    start: open_token.start,
                    end: open_token.end,
                });
            }
        }

        Ok(())
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
        let mut quote_parsing = QuoteParsing::Ident;

        while let Some(token) = self.iter.peek() {
            match token.kind {
                TokenKind::Colon(1) => {
                    ident_or_selector.push_str(&token.input[start_offset..token.offset.start]);
                    self.iter.next();

                    if self
                        .iter
                        .peeking_next(|t| {
                            matches!(t.kind, TokenKind::Whitespace | TokenKind::Newline)
                        })
                        .is_some()
                    {
                        self.iter.next();
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
                    } else if token.kind == TokenKind::CloseBracket {
                        quote_parsing = QuoteParsing::Ident;
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

        Ok(IdentSelectorKind::Selector)
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
        let mut end_offset = open_quote.offset.end;
        let mut start = open_quote.start;
        let mut end = open_quote.end;

        while let Some(token) = self.iter.peeking_next(|_| true) {
            if (variant == QuoteParsing::Value
                && matches!(token.kind, TokenKind::ImplicitSubstitution(_)))
                || matches!(token.kind, TokenKind::Newline | TokenKind::EscapedNewline)
                || token.kind == open_quote.kind
            {
                let plain = &token.input[start_offset..token.offset.start];
                if !plain.is_empty() {
                    quote_tokens.push(QuotedValuePart {
                        kind: super::token::QuotedPartKind::Plain(plain.to_string()),
                        start,
                        end,
                    });
                }

                start_offset = token.offset.end;
                start = token.end;
                end = token.end;
            }

            match token.kind {
                TokenKind::ImplicitSubstitution(subst) if variant == QuoteParsing::Value => {
                    quote_tokens.push(QuotedValuePart {
                        kind: super::token::QuotedPartKind::ImplicitSubstitution(subst),
                        start,
                        end,
                    });
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
                }
                TokenKind::EscapedNewline => {
                    quote_tokens.push(QuotedValuePart {
                        kind: super::token::QuotedPartKind::EscapedNewline,
                        start,
                        end,
                    });
                }
                k if k == open_quote.kind => {
                    // closing quote
                    self.iter.skip_to_peek();
                    return Ok(AttributeToken {
                        kind: AttributeTokenKind::QuotedPart(QuotedPart {
                            parts: quote_tokens,
                            quote: quote_char,
                            implicit_closed: false,
                        }),
                        start: open_quote.start,
                        end,
                    });
                }
                _ => {}
            }

            end_offset = token.offset.end;
            end = token.end;
        }

        let plain = &open_quote.input[start_offset..end_offset];
        if !plain.is_empty() {
            quote_tokens.push(QuotedValuePart {
                kind: super::token::QuotedPartKind::Plain(plain.to_string()),
                start,
                end,
            });
        }

        self.iter.skip_to_peek();
        Ok(AttributeToken {
            kind: AttributeTokenKind::QuotedPart(QuotedPart {
                parts: quote_tokens,
                quote: quote_char,
                implicit_closed: true,
            }),
            start: open_quote.start,
            end,
        })
    }

    fn parse_semicolon(&mut self, attrb_tokens: &mut Vec<AttributeToken>, len: usize) {
        if len == Comment::keyword_len() {
            let comment = Comment::parse(&mut self.iter)
                .expect("Valid comment start always leads to a comment.");
            let start = comment.start();
            let end = comment.end();

            attrb_tokens.push(AttributeToken {
                kind: AttributeTokenKind::Comment(comment),
                start,
                end,
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

#[cfg(test)]
mod test {
    use crate::{
        attributes::token::{
            AttributeTokenKind, AttributeTokens, Ident, QuotedPartKind, TokenPart,
        },
        lexer::token::iterator::TokenIterator,
        parsing::{Parser, ParserError},
    };

    use super::{AttributeContext, AttributeTokenizer};

    fn attrb_tokens(s: &str) -> Result<AttributeTokens, ParserError> {
        let tokens = crate::lexer::token::lex_str(s);
        let attrb_tokenizer: AttributeTokenizer<'_, '_> =
            AttributeTokenizer::new(TokenIterator::from(&*tokens), AttributeContext::default());

        let (_, res) = attrb_tokenizer.parse();
        res
    }

    #[test]
    fn single_css_color_attrb() {
        let s = "{color: red;}";
        let tokens = attrb_tokens(s).unwrap();

        assert_eq!(
            tokens.tokens.len(),
            3,
            "Ident, value part, and semicolon were not correctly parsed."
        );
        assert_eq!(
            tokens.tokens[0].kind,
            AttributeTokenKind::Ident(Ident::from("color".to_string())),
            "'color' ident not correctly parsed."
        );
        assert_eq!(
            tokens.tokens[1].kind,
            AttributeTokenKind::ValuePart(crate::attributes::token::ValuePart::Plain(
                "red".to_string()
            )),
            "Color value not correctly parsed."
        );
        assert_eq!(
            tokens.tokens[2].kind,
            AttributeTokenKind::Semicolon,
            "Semicolon not correctly parsed."
        );
    }

    #[test]
    fn single_html_id_attrb() {
        let s = "{id: 'my-id';}";
        let tokens = attrb_tokens(s).unwrap();

        assert_eq!(
            tokens.tokens.len(),
            3,
            "Ident, quoted value part, and semicolon were not correctly parsed."
        );
        assert_eq!(
            tokens.tokens[0].kind,
            AttributeTokenKind::Ident(Ident::from("id".to_string())),
            "'id' ident not correctly parsed."
        );

        let value_kind = &tokens.tokens[1].kind;
        if let AttributeTokenKind::QuotedPart(quoted_part) = value_kind {
            assert_eq!(quoted_part.quote, '\'', "Wrong quote char detected.");
            assert_eq!(
                quoted_part.parts[0].kind,
                QuotedPartKind::Plain("my-id".to_string()),
                "'my-id' not part of the parsed quoted value."
            );
        } else {
            panic!("Detected '{:?}' instead of a quoted part.", value_kind);
        }

        assert_eq!(
            tokens.tokens[2].kind,
            AttributeTokenKind::Semicolon,
            "Semicolon not correctly parsed."
        );
    }

    #[test]
    fn two_html_attrbs() {
        // 'class' ident directly after ';' to not get a 'Whitespace' token
        let s = "{id: 'my-id';class: 'some-class other-class'}";
        let tokens = attrb_tokens(s).unwrap();

        assert_eq!(
            tokens.tokens.len(),
            5,
            "Ident one, quoted value part, semicolon, ident two, and second quoted value were not correctly parsed."
        );
        assert_eq!(
            tokens.tokens[0].kind,
            AttributeTokenKind::Ident(Ident::from("id".to_string())),
            "'id' ident not correctly parsed."
        );

        let value_kind = &tokens.tokens[1].kind;
        if let AttributeTokenKind::QuotedPart(quoted_part) = value_kind {
            assert_eq!(quoted_part.quote, '\'', "Wrong quote char detected.");
            assert_eq!(
                quoted_part.parts[0].kind,
                QuotedPartKind::Plain("my-id".to_string()),
                "'my-id' not part of the parsed quoted value."
            );
        } else {
            panic!("Detected '{:?}' instead of a quoted part.", value_kind);
        }

        assert_eq!(
            tokens.tokens[2].kind,
            AttributeTokenKind::Semicolon,
            "Semicolon not correctly parsed."
        );

        assert_eq!(
            tokens.tokens[3].kind,
            AttributeTokenKind::Ident(Ident::from("class".to_string())),
            "'class' ident not correctly parsed."
        );

        let value_kind = &tokens.tokens[4].kind;
        if let AttributeTokenKind::QuotedPart(quoted_part) = value_kind {
            assert_eq!(quoted_part.quote, '\'', "Wrong quote char detected.");
            assert_eq!(
                quoted_part.parts[0].kind,
                QuotedPartKind::Plain("some-class other-class".to_string()),
                "'some-class other-class' not part of the parsed quoted value."
            );
        } else {
            panic!("Detected '{:?}' instead of a quoted part.", value_kind);
        }
    }

    #[test]
    fn nested_attrb() {
        let s = "{#some-id{color: red}}";
        let tokens = attrb_tokens(s).unwrap();

        assert_eq!(
            tokens.tokens.len(),
            2,
            "Selector and nested block were not correctly parsed."
        );
        assert_eq!(
            tokens.tokens[0].kind,
            AttributeTokenKind::SelectorPart(TokenPart::from("#some-id".to_string())),
            "'#some-id' selector not correctly parsed."
        );

        let value_kind = &tokens.tokens[1].kind;
        if let AttributeTokenKind::Nested(nested_tokens) = value_kind {
            assert_eq!(
                nested_tokens.tokens.len(),
                2,
                "Nested ident and value not correctly parsed."
            );
            assert_eq!(
                nested_tokens.tokens[0].kind,
                AttributeTokenKind::Ident(Ident::from("color".to_string())),
                "'color' ident not parsed in the nested block."
            );
            assert_eq!(
                nested_tokens.tokens[1].kind,
                AttributeTokenKind::ValuePart(crate::attributes::token::ValuePart::Plain(
                    "red".to_string()
                )),
                "'color' ident not parsed in the nested block."
            );
        } else {
            panic!("Detected '{:?}' instead of a nested block.", value_kind);
        }
    }
}
