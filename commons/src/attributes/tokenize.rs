use itertools::PeekingNext;

use crate::{
    attributes::rules,
    comments::Comment,
    lexer::{
        position::Position,
        token::{iterator::TokenIterator, Token, TokenKind},
    },
    parsing::{Element, Parser, ParserError},
};

use super::token::{
    AttributeToken, AttributeTokenKind, AttributeTokens, CssFn, QuotedPart, QuotedValuePart,
    ValuePart,
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
        // Start: IdentOrSelectorPart | AtRuleIdent | Logic | Comment | Whitespace | Newline | Semicolon
        // => after IdentMarker: (Comment | Whitespace | Newline)* (SingleValue | Array | Nested | Logic | QuotedValue) (Comment | Whitespace | Newline)*
        //    => after SingleValue: (Logic | QuotedValue | Plain | Bool | Int | Float | Comment | Whitespace | Newline)* Important? Semicolon
        //    => after Nested: (Comment | Whitespace | Newline)* Important? Semicolon
        //    => after Array: (Comment | Whitespace | Newline)* Important? Semicolon
        //    => after Logic: <after Ident> | Important? (Comment | Whitespace | Newline)* Important? Semicolon
        // => implicit SelectorMarker: (Comment | Whitespace | Newline)* Nested
        //    => after Nested: (Comment | Whitespace | Newline)* Semicolon? <Start>
        // => after AtRuleIdent: (Comment | Whitespace | Newline)* (AtRulePreludePart | Nested)
        //    => after AtRulePreludePart: <after AtRuleIdent> | Semicolon
        //    => after Nested: <Start>
        // => after Logic, Comment, Whitespace, Newline: <Start>
        //
        // Nested: `{` <Start>* `}`
        // Array: `[` ( SingleValue | Nested | Array ) `]`
        // QuotedValue: (`"` QuotedValuePartKind* `"`) | (`'` QuotedValuePartKind* `'`)

        // Attribute tokenization must not fail once valid start is detected, but mark invalid tokens.
        // Needed to get better semantic highlighting and LSP diagnostics.

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

        let impl_closed = self.parse_block(&mut attrb_tokens);
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
    fn parse_block(&mut self, attrb_tokens: &mut Vec<AttributeToken>) -> bool {
        while let Some(token) = self.iter.peek() {
            match token.kind {
                TokenKind::At(len) => {
                    if len != 1 {
                        // TODO: set log error for multiple `@`
                        attrb_tokens.push(AttributeToken {
                            kind: AttributeTokenKind::Invalid(token.kind, token.to_string()),
                            start: token.start,
                            end: token.end,
                        });
                    } else {
                        rules::parse_at_rule(self, attrb_tokens);
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
                    return false;
                }
                TokenKind::Whitespace | TokenKind::Blankline => {
                    // not interested in whitespace
                    let token = self
                        .iter
                        .next()
                        .expect("Peek was some, so next must return value.");
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
                    attrb_tokens.push(AttributeToken {
                        kind: AttributeTokenKind::Invalid(token.kind, token.to_string()),
                        start: token.start,
                        end: token.end,
                    });
                }
                _ => {
                    // Note: Quotes may be part of the ident
                    self.parse_single_or_nested(attrb_tokens);
                }
            }
        }

        true
    }

    fn parse_single_or_nested(&mut self, attrb_tokens: &mut Vec<AttributeToken>) {
        let ident_part = self.parse_ident_or_selector(attrb_tokens);

        self.parse_comments(attrb_tokens); // To consume any amount of comments, spaces, newlines between IdentMarker and value

        match ident_part {
            IdentSelectorKind::Ident => {
                if self.iter.peek_kind() == Some(TokenKind::OpenBrace) {
                    self.parse_nested(attrb_tokens)
                } else if self.iter.peek_kind() == Some(TokenKind::OpenBracket) {
                    self.parse_array(attrb_tokens)
                } else {
                    self.parse_single_value(attrb_tokens)
                }
            }
            IdentSelectorKind::Selector => {
                // next token must either be OpenBrace or None, because parse_ident_or_selector() wouldn't have stopped otherwise
                self.parse_nested(attrb_tokens)
            }
            IdentSelectorKind::NoValue => {
                // either reached semicolon or EOI
            }
        }
    }

    fn parse_array(&mut self, attrb_tokens: &mut Vec<AttributeToken>) {
        todo!()
    }

    fn parse_single_value(&mut self, attrb_tokens: &mut Vec<AttributeToken>) {
        let mut value_pushed = false;
        let mut important_pushed = false;

        while let Some(token) = self.iter.peek() {
            match token.kind {
                TokenKind::OpenBrace => {
                    // TODO: check for logic element

                    // Nested not allowed in value context
                    attrb_tokens.push(AttributeToken {
                        kind: AttributeTokenKind::Invalid(token.kind, token.to_string()),
                        start: token.start,
                        end: token.end,
                    });
                }
                TokenKind::CloseBrace => {
                    return;
                }
                TokenKind::Semicolon(len) => {
                    self.parse_semicolon(attrb_tokens, len);

                    if len != Comment::keyword_len() {
                        return;
                    }
                }
                TokenKind::OpenBracket
                | TokenKind::CloseBracket
                | TokenKind::EscapedNewline
                | TokenKind::EscapedWhitespace => attrb_tokens.push(AttributeToken {
                    kind: AttributeTokenKind::Invalid(token.kind, token.to_string()),
                    start: token.start,
                    end: token.end,
                }),
                TokenKind::Newline => {
                    self.iter.next();
                    attrb_tokens.push(AttributeToken {
                        kind: AttributeTokenKind::Newline,
                        start: token.start,
                        end: token.end,
                    })
                }
                TokenKind::Whitespace | TokenKind::Blankline | TokenKind::Comma(1) => {
                    // CSS array separations are considered as one single value
                    self.iter.next();
                }
                TokenKind::ExclamationMark(1) if !important_pushed => {
                    // TODO: might be "!important"
                }
                _ if !important_pushed => {
                    self.parse_value_part(attrb_tokens);
                }
                _ => attrb_tokens.push(AttributeToken {
                    kind: AttributeTokenKind::Invalid(token.kind, token.to_string()),
                    start: token.start,
                    end: token.end,
                }),
            }
        }
    }

    /// Parses a flat attribute value.
    ///
    /// **Note:** Because of CSS functions, a flat value may consist of multiple value parts.
    fn parse_value_part(&mut self, attrb_tokens: &mut Vec<AttributeToken>) {
        let mut part = String::new();
        // Peeked some token before calling "value_part"
        let Some(first_token) = self.iter.peek() else {
            return;
        };
        let mut end = first_token.start;
        let mut possible_int = false;
        let mut possible_float = false;

        while let Some(token) = self.iter.peek() {
            match token.kind {
                TokenKind::CloseBrace
                | TokenKind::OpenBrace
                | TokenKind::OpenBracket
                | TokenKind::CloseBracket
                | TokenKind::CloseParenthesis
                | TokenKind::Semicolon(_)
                | TokenKind::Whitespace
                | TokenKind::Blankline
                | TokenKind::Newline
                | TokenKind::EscapedNewline
                | TokenKind::EscapedWhitespace => {
                    end = token.start;
                    break;
                }
                TokenKind::OpenParenthesis => {
                    if part.is_empty() {
                        // fn call without fn name not allowed
                        // TODO: handle logic context

                        attrb_tokens.push(AttributeToken {
                            kind: AttributeTokenKind::Invalid(token.kind, token.to_string()),
                            start: token.start,
                            end: token.end,
                        });
                    }

                    return self.parse_css_fn(attrb_tokens, first_token, part);
                }
                TokenKind::SingleQuote | TokenKind::DoubleQuote => {
                    attrb_tokens.push(self.parse_quote(QuoteParsing::Value));
                    return;
                }
                _ => {
                    self.iter.next();
                    let might_be_int_part = matches!(
                        token.kind,
                        TokenKind::Digit(_) | TokenKind::Minus(1) | TokenKind::Underline(1)
                    );
                    // "Plain" might lead to "e" or "E" for exponent
                    let might_be_float_part = might_be_int_part
                        || (matches!(token.kind, TokenKind::Plain | TokenKind::Dot(1))
                            && (token.end.col_utf8 - token.start.col_utf8) == 1);

                    possible_int = (possible_int || part.is_empty()) && might_be_int_part;
                    possible_float = (possible_float || part.is_empty()) && might_be_float_part;

                    part.push_str(&token.to_string());
                    end = token.end;
                }
            }
        }

        if !part.is_empty() {
            let combined_part = value_part_to_value(part, possible_int, possible_float);

            attrb_tokens.push(AttributeToken {
                kind: AttributeTokenKind::FlatValue(combined_part),
                start: first_token.start,
                end,
            })
        }
    }

    /// Parses the enclosed content of a CSS function.
    /// e.g. `--custom-var` of `var(--custom-var)`
    fn parse_css_fn(
        &mut self,
        attrb_tokens: &mut Vec<AttributeToken>,
        open_token: &'slice Token<'input>,
        fn_name: String,
    ) {
        let mut inner_attrbs = Vec::new();
        let mut content = String::new();
        let mut start = None;
        let mut end = open_token.end;

        while let Some(token) = self.iter.peek() {
            match token.kind {
                TokenKind::CloseParenthesis => {
                    self.iter.next();

                    attrb_tokens.push(AttributeToken {
                        kind: AttributeTokenKind::FlatValue(ValuePart::CssFn(CssFn {
                            name: fn_name,
                            inner: inner_attrbs,
                            implicit_closed: false,
                        })),
                        start: open_token.start,
                        end: token.end,
                    });
                    return;
                }
                TokenKind::Newline => {
                    if !content.is_empty() {
                        inner_attrbs.push(AttributeToken {
                            kind: AttributeTokenKind::FlatValue(ValuePart::Plain(std::mem::take(
                                &mut content,
                            ))),
                            start: start.unwrap_or(open_token.end),
                            end: token.start,
                        })
                    }

                    self.iter.next();
                    inner_attrbs.push(AttributeToken {
                        kind: AttributeTokenKind::Newline,
                        start: token.start,
                        end: token.end,
                    })
                }
                TokenKind::SingleQuote | TokenKind::DoubleQuote => {
                    if !content.is_empty() {
                        inner_attrbs.push(AttributeToken {
                            kind: AttributeTokenKind::FlatValue(ValuePart::Plain(std::mem::take(
                                &mut content,
                            ))),
                            start: start.unwrap_or(open_token.end),
                            end: token.start,
                        })
                    }

                    inner_attrbs.push(self.parse_quote(QuoteParsing::Value));
                }
                TokenKind::EscapedNewline | TokenKind::EscapedWhitespace => {
                    attrb_tokens.push(AttributeToken {
                        kind: AttributeTokenKind::Invalid(token.kind, token.to_string()),
                        start: token.start,
                        end: token.end,
                    })
                }
                _ => {
                    if start.is_none() {
                        start = Some(token.start);
                    }
                    end = token.end;

                    content.push_str(&String::from(token));
                    self.iter.next();
                }
            }
        }

        attrb_tokens.push(AttributeToken {
            kind: AttributeTokenKind::FlatValue(ValuePart::CssFn(CssFn {
                name: fn_name,
                inner: inner_attrbs,
                implicit_closed: false,
            })),
            start: open_token.start,
            end,
        });
    }

    fn parse_nested(&mut self, attrb_tokens: &mut Vec<AttributeToken>) {
        let Some(open_token) = self.iter.next() else {
            return;
        };
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
                let impl_closed = self.parse_block(&mut inner_tokens);
                let inner_start = inner_tokens
                    .first()
                    .map(|t| t.start)
                    .unwrap_or(open_token.end);
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
    }

    fn parse_comments(&mut self, attrb_tokens: &mut Vec<AttributeToken>) {
        while let Some(token) = self.iter.peek() {
            match token.kind {
                TokenKind::Semicolon(len) => {
                    if len == Comment::keyword_len() {
                        self.parse_semicolon(attrb_tokens, len);
                    } else {
                        return;
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
                TokenKind::Whitespace | TokenKind::Blankline => {
                    self.iter.next();
                }
                _ => {
                    return;
                }
            }
        }
    }

    fn parse_ident_or_selector(
        &mut self,
        attrb_tokens: &mut Vec<AttributeToken>,
    ) -> IdentSelectorKind {
        let mut ident_or_selector = String::new();

        let Some(start_token) = self.iter.peek() else {
            return IdentSelectorKind::NoValue;
        };
        let mut start_pos = Some(start_token.start);
        let mut end = start_token.end;
        let mut quote_parsing = QuoteParsing::Ident;

        while let Some(token) = self.iter.peek() {
            if start_pos.is_none() {
                start_pos = Some(token.start);
            }

            if !ident_or_selector.is_empty()
                && matches!(
                    token.kind,
                    TokenKind::SingleQuote
                        | TokenKind::DoubleQuote
                        | TokenKind::Semicolon(_)
                        | TokenKind::Newline
                        | TokenKind::EscapedNewline
                        | TokenKind::EscapedWhitespace
                )
            {
                attrb_tokens.push(AttributeToken {
                    kind: AttributeTokenKind::IdentOrSelectorPart(
                        super::token::IdentOrSelectorPart::Plain(std::mem::take(
                            &mut ident_or_selector,
                        )),
                    ),
                    start: start_pos.unwrap_or(token.start),
                    end: token.start,
                });

                start_pos = None;
            }

            match token.kind {
                TokenKind::Colon(1) => {
                    self.iter.next();

                    if self
                        .iter
                        .peeking_next(|t| {
                            matches!(t.kind, TokenKind::Whitespace | TokenKind::Newline)
                        })
                        .is_some()
                    {
                        self.iter.next();

                        if !ident_or_selector.is_empty() {
                            attrb_tokens.push(AttributeToken {
                                kind: AttributeTokenKind::IdentOrSelectorPart(
                                    super::token::IdentOrSelectorPart::Plain(std::mem::take(
                                        &mut ident_or_selector,
                                    )),
                                ),
                                start: start_pos.unwrap_or(token.start),
                                end: token.start,
                            });
                        }

                        attrb_tokens.push(AttributeToken {
                            kind: AttributeTokenKind::IdentMarker,
                            start: token.start,
                            end: token.end,
                        });
                        return IdentSelectorKind::Ident;
                    }

                    ident_or_selector.push(':');
                }
                TokenKind::Newline => {
                    self.iter.next();
                    attrb_tokens.push(AttributeToken {
                        kind: AttributeTokenKind::Newline,
                        start: token.start,
                        end: token.end,
                    });
                }
                TokenKind::EscapedNewline => {
                    self.iter.next();
                    attrb_tokens.push(AttributeToken {
                        kind: AttributeTokenKind::Invalid(token.kind, token.to_string()),
                        start: token.start,
                        end: token.end,
                    });
                }
                TokenKind::Semicolon(len) => {
                    if len != Comment::keyword_len() {
                        // attribute without value => will be handled during resolve step
                        // semicolon is parsed in `parse_block()` fn
                        return IdentSelectorKind::NoValue;
                    }

                    self.parse_semicolon(attrb_tokens, len);
                }
                TokenKind::OpenBrace => {
                    break;
                }
                TokenKind::SingleQuote | TokenKind::DoubleQuote => {
                    attrb_tokens.push(self.parse_quote(quote_parsing));
                }
                _ => {
                    // treat quoted content in attribute selectors (`[]`) as values
                    if token.kind == TokenKind::OpenBracket {
                        quote_parsing = QuoteParsing::Value;
                    } else if token.kind == TokenKind::CloseBracket {
                        quote_parsing = QuoteParsing::Ident;
                    }

                    self.iter.next();
                    ident_or_selector.push_str(&token.to_string());
                }
            }

            end = token.end;
        }

        if !ident_or_selector.is_empty() {
            attrb_tokens.push(AttributeToken {
                kind: AttributeTokenKind::IdentOrSelectorPart(
                    super::token::IdentOrSelectorPart::Plain(std::mem::take(
                        &mut ident_or_selector,
                    )),
                ),
                start: start_pos.unwrap_or(start_token.start),
                end,
            });
        }

        // always selector, because a valid ident must end with `: `.
        IdentSelectorKind::Selector
    }

    fn parse_quote(&mut self, variant: QuoteParsing) -> AttributeToken {
        let open_quote = self
            .iter
            .next()
            .expect("Quote parsing is called when next is a quote.");
        let quote_char = if open_quote.kind == TokenKind::DoubleQuote {
            '"'
        } else {
            '\''
        };
        let mut quote_tokens = Vec::new();
        let mut content = String::new();
        let mut start = open_quote.end;
        let mut end = open_quote.end;

        while let Some(token) = self.iter.peeking_next(|_| true) {
            if (variant == QuoteParsing::Value
                && matches!(token.kind, TokenKind::ImplicitSubstitution(_)))
                || matches!(token.kind, TokenKind::Newline | TokenKind::EscapedNewline)
                || token.kind == open_quote.kind
            {
                if !content.is_empty() {
                    quote_tokens.push(QuotedValuePart {
                        kind: super::token::QuotedPartKind::Plain(std::mem::take(&mut content)),
                        start,
                        end,
                    });
                }

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
                    return AttributeToken {
                        kind: AttributeTokenKind::FlatValue(ValuePart::Quoted(QuotedPart {
                            parts: quote_tokens,
                            quote: quote_char,
                            implicit_closed: false,
                        })),
                        start: open_quote.start,
                        end,
                    };
                }
                _ => {
                    content.push_str(&token.to_string());
                }
            }

            end = token.end;
        }

        if !content.is_empty() {
            quote_tokens.push(QuotedValuePart {
                kind: super::token::QuotedPartKind::Plain(std::mem::take(&mut content)),
                start,
                end,
            });
        }

        self.iter.skip_to_peek();
        AttributeToken {
            kind: AttributeTokenKind::FlatValue(ValuePart::Quoted(QuotedPart {
                parts: quote_tokens,
                quote: quote_char,
                implicit_closed: true,
            })),
            start: open_quote.start,
            end,
        }
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

fn value_part_to_value(part: String, possible_int: bool, possible_float: bool) -> ValuePart {
    if possible_int {
        match part.parse() {
            Ok(num) => ValuePart::Int(num),
            Err(_) => ValuePart::Plain(part),
        }
    } else if possible_float {
        match part.parse() {
            Ok(num) => ValuePart::Float(num),
            Err(_) => ValuePart::Plain(part),
        }
    } else if part.to_lowercase() == "true" {
        ValuePart::Bool(true)
    } else if part.to_lowercase() == "false" {
        ValuePart::Bool(false)
    } else {
        ValuePart::Plain(part)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        attributes::token::{
            AttributeTokenKind, AttributeTokens, IdentOrSelectorPart, QuotedPartKind, ValuePart,
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
            4,
            "Ident, ident marker, value part, and semicolon were not correctly parsed."
        );
        assert_eq!(
            tokens.tokens[0].kind,
            AttributeTokenKind::IdentOrSelectorPart(IdentOrSelectorPart::Plain(
                "color".to_string()
            )),
            "'color' ident not correctly parsed."
        );
        assert_eq!(
            tokens.tokens[1].kind,
            AttributeTokenKind::IdentMarker,
            "Ident marker not correctly parsed."
        );
        assert_eq!(
            tokens.tokens[2].kind,
            AttributeTokenKind::FlatValue(crate::attributes::token::ValuePart::Plain(
                "red".to_string()
            )),
            "Color value not correctly parsed."
        );
        assert_eq!(
            tokens.tokens[3].kind,
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
            4,
            "Ident, ident marker, quoted value part, and semicolon were not correctly parsed."
        );
        assert_eq!(
            tokens.tokens[0].kind,
            AttributeTokenKind::IdentOrSelectorPart(IdentOrSelectorPart::Plain("id".to_string())),
            "'id' ident not correctly parsed."
        );

        let value_kind = &tokens.tokens[2].kind;
        let AttributeTokenKind::FlatValue(value) = value_kind else {
            panic!()
        };
        let ValuePart::Quoted(quoted_part) = value else {
            panic!()
        };
        assert_eq!(quoted_part.quote, '\'', "Wrong quote char detected.");
        assert_eq!(
            quoted_part.parts[0].kind,
            QuotedPartKind::Plain("my-id".to_string()),
            "'my-id' not part of the parsed quoted value."
        );

        assert_eq!(
            tokens.tokens[3].kind,
            AttributeTokenKind::Semicolon,
            "Semicolon not correctly parsed."
        );
    }

    #[test]
    fn two_html_attrbs() {
        // TODO: should also accept `id: my-id; class: some-class other-class`
        // 'class' ident directly after ';' to not get a 'Whitespace' token
        let s = "{id: 'my-id';class: 'some-class other-class'}";
        let tokens = attrb_tokens(s).unwrap();

        assert_eq!(
            tokens.tokens.len(),
            7,
            "Ident one, ident marker, quoted value part, semicolon, ident two, ident marker, and second quoted value were not correctly parsed."
        );
        assert_eq!(
            tokens.tokens[0].kind,
            AttributeTokenKind::IdentOrSelectorPart(IdentOrSelectorPart::Plain("id".to_string())),
            "'id' ident not correctly parsed."
        );

        let value_kind = &tokens.tokens[2].kind;
        let AttributeTokenKind::FlatValue(value) = value_kind else {
            panic!()
        };
        let ValuePart::Quoted(quoted_part) = value else {
            panic!()
        };
        assert_eq!(quoted_part.quote, '\'', "Wrong quote char detected.");
        assert_eq!(
            quoted_part.parts[0].kind,
            QuotedPartKind::Plain("my-id".to_string()),
            "'my-id' not part of the parsed quoted value."
        );

        assert_eq!(
            tokens.tokens[3].kind,
            AttributeTokenKind::Semicolon,
            "Semicolon not correctly parsed."
        );

        assert_eq!(
            tokens.tokens[4].kind,
            AttributeTokenKind::IdentOrSelectorPart(IdentOrSelectorPart::Plain(
                "class".to_string()
            )),
            "'class' ident not correctly parsed."
        );

        let value_kind = &tokens.tokens[6].kind;
        let AttributeTokenKind::FlatValue(value) = value_kind else {
            panic!()
        };
        let ValuePart::Quoted(quoted_part) = value else {
            panic!()
        };
        assert_eq!(quoted_part.quote, '\'', "Wrong quote char detected.");
        assert_eq!(
            quoted_part.parts[0].kind,
            QuotedPartKind::Plain("some-class other-class".to_string()),
            "'some-class other-class' not part of the parsed quoted value."
        );
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
            AttributeTokenKind::IdentOrSelectorPart(IdentOrSelectorPart::Plain(
                "#some-id".to_string()
            )),
            "'#some-id' selector not correctly parsed."
        );

        let value_kind = &tokens.tokens[1].kind;
        if let AttributeTokenKind::Nested(nested_tokens) = value_kind {
            assert_eq!(
                nested_tokens.tokens.len(),
                3,
                "Nested ident, ident marker, and value not correctly parsed."
            );
            assert_eq!(
                nested_tokens.tokens[0].kind,
                AttributeTokenKind::IdentOrSelectorPart(IdentOrSelectorPart::Plain(
                    "color".to_string()
                )),
                "'color' ident not parsed in the nested block."
            );
            assert_eq!(
                nested_tokens.tokens[2].kind,
                AttributeTokenKind::FlatValue(crate::attributes::token::ValuePart::Plain(
                    "red".to_string()
                )),
                "'color' ident not parsed in the nested block."
            );
        } else {
            panic!("Detected '{:?}' instead of a nested block.", value_kind);
        }
    }
}
