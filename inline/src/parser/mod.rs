use std::ops::Deref;

use crate::{Inline, InlineContent, PlainInline, Span, Token, TokenIterator, TokenKind, Tokenize};

#[derive(Debug, Default, Clone)]
struct ParserStack {
    data: Vec<Token>,
}

impl Deref for ParserStack {
    type Target = Vec<Token>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl ParserStack {
    /// Pushes the element onto the stack and returns the index of the element
    pub fn push(&mut self, token: Token) -> usize {
        self.data.push(token);
        self.data.len() - 1
    }

    pub fn pop_last(&mut self) -> Option<Token> {
        self.data.pop()
    }

    /// Removes and returns the last item on stack
    pub fn pop(&mut self, token: &Token) -> Option<Token> {
        if self.data.is_empty() {
            None
        } else {
            let last_open_token = self.data.last_mut().unwrap();

            if last_open_token.is_ambiguous() {
                // remove the ambiguous part...
                let removed_token = last_open_token.remove_partial(token);

                Some(removed_token)
            } else {
                self.data.pop()
            }
        }
    }
}

pub struct Parser<'i> {
    iter: TokenIterator<'i>,
    stack: ParserStack,
    token_cache: Option<Token>,
}

impl Parser<'_> {
    fn next_token(&mut self) -> Option<Token> {
        if self.token_cache.is_some() {
            self.token_cache.take()
        } else {
            self.iter.next()
        }
    }

    fn is_token_open(&self, token: &Token) -> bool {
        self.stack.iter().any(|inner_token| {
            inner_token.is_or_contains(token) || token.is_or_contains(inner_token)
        })
    }

    fn is_token_latest(&self, token: &Token) -> bool {
        match self.stack.last() {
            Some(last_open_token) => last_open_token.is_or_contains(token),
            None => false,
        }
    }

    fn parse_nested_inline(&mut self, token: Token) -> Inline {
        // Push token kind to stack
        // Open corresponding inline
        // If nesting of inline occurs, parse inner inline -> PROBLEM: Ambiguous tokens?
        // Parse until closing token is found
        // Close inline and return it

        // PROBLEM: AmbiguousToken that comes as next token
        // example: **Bold Text***Italic text*
        //            ^^^^^^^^^   ^^^^^^^^^^^
        //              BOLD        ITALIC
        //  So the ambiguous token AFTER bold content (***) should be split into
        //  bold close token and italic open. That means, that the ambiguous token should be split,
        //  first part taken (based on what part was open) and the second part left for the next
        //  iteration
        let kind = token.kind();
        let mut content: InlineContent = InlineContent::Nested(Vec::default());
        let start = token.span().start();
        let mut end = start;

        self.stack.push(token);

        while let Some(mut next_token) = self.next_token() {
            // Multiple cases:
            // 1. token is (nesting one and) already open
            //      - Is it closing one and it was open last? Close Inline
            //      - Is it closing one, but it was not open last? Return inline and merge into outer one
            //      - If not closing one, then it's plain text
            //      - If no more tokens available, then:
            //          -> First token (opening one) should be treated as plain text
            //          -> All inlines found inside should be given as such
            //          -> That means that the inline becomes: (PlainInline, Inline, Inline...)
            // 2. token is not already open
            //      - content until token is plain text

            if next_token.closes() {
                if self.is_token_open(&next_token) {
                    if self.is_token_latest(&next_token) {
                        // It is closing one and it was open last -> Close Inline
                        self.stack.pop(&next_token);
                        break;
                    } else {
                        // It might be ambiguous token and part of it is open,
                        // for example ** followed by ***. Such token should be split as **|*,
                        // where first part (**) is being closed, and second part (*) is now in
                        // token_cache for next iteration

                        if next_token.is_ambiguous() {
                            // at this point we know there is at least one token in stack
                            let last_token = self.stack.last().unwrap();

                            if next_token.is_or_contains(last_token) {
                                let _parsed_token = next_token.remove_partial(last_token);

                                self.stack.pop_last();

                                self.token_cache = Some(next_token);

                                // close this inline
                                break;
                            }
                        } else {
                            // It is closing one, but it was not open last -> Return contents as inline

                            // remove the opening token from the stack
                            let token = self.stack.pop(&next_token).unwrap();

                            // prepend the token to content as plain text
                            content.prepend(InlineContent::from(token));

                            return Inline {
                                inner: content,
                                span: Span::from((start, end)),
                                kind: TokenKind::Plain,
                            };
                        }
                    }
                }
            } else if next_token.opens() {
                if self.is_token_open(&next_token) {
                    // plain text

                    // update end position
                    end = next_token.span().end();

                    // consume plain text
                    content.append(InlineContent::from(next_token));
                } else {
                    // parse open and merge into upper one
                    let nested = self.parse_nested_inline(next_token);

                    end = nested.span().end();

                    content.append_inline(nested);
                }
            } else {
                // neither opens nor closes - is plain text
                let inline_content = InlineContent::from(next_token);
                content.append(inline_content);
            }
        }

        // if content contains only plain contents, then merge them and make into one
        content.try_flatten();

        Inline {
            span: Span::from((start, end)),
            inner: content,
            kind,
        }
    }
}

impl Iterator for Parser<'_> {
    type Item = Inline;

    fn next(&mut self) -> Option<Self::Item> {
        let content: InlineContent;
        let mut _kind: TokenKind;

        if let Some(token) = self.next_token() {
            if token.opens() {
                return Some(self.parse_nested_inline(token));
            } else {
                _kind = token.kind();

                let (token_content, token_span) = token.into_inner();
                content = InlineContent::Plain(PlainInline {
                    content: token_content,
                    span: token_span,
                });

                return Some(Inline {
                    inner: content,
                    span: token_span,
                    kind: TokenKind::Plain,
                });
            }
        }

        None
    }
}

pub trait ParseUnimarkupInlines<'p, 'i>
where
    'i: 'p,
{
    fn parse_unimarkup_inlines(&'i self) -> Parser<'p>;
}

impl<'p, 'i> ParseUnimarkupInlines<'p, 'i> for &str
where
    'i: 'p,
{
    fn parse_unimarkup_inlines(&'i self) -> Parser<'p> {
        Parser {
            iter: self.lex_iter(),
            stack: ParserStack::default(),
            token_cache: None,
        }
    }
}

// IMPLEMENTATION IDEA
//
// consume (stackable) tokens from TokenIterator and push them to the stack.
// Plain tokens are to be used as content
// Each time some token is closed, remove it's corresponding part from stack (resolve ambiguity)
// Once the stack is empty, that is one Inline parsed and should be returned (if we want to have
// iterator-like API)

// impl<'i> From<&'i str> for Parser<'i> {
//     fn from(input: &'i str) -> Self {
//         let lexer: Lexer<'i> = input.lex();
//
//         Self { lexer }
//     }
// }

// So let's think about the API

#[cfg(test)]
mod tests {
    use crate::Position;

    use super::*;

    #[test]
    fn parse_simple_plain() {
        let parser = "Some text".parse_unimarkup_inlines();

        assert_eq!(parser.count(), 1);
    }

    #[test]
    fn parse_simple_bold() {
        let mut parser = "**Bold text**".parse_unimarkup_inlines();

        let inline = parser.next().unwrap();
        let start = Position { line: 1, column: 3 };
        let end = start + (0, 9 - 1);

        // no remaining inlines
        assert_eq!(parser.count(), 0);
        assert_eq!(inline.kind, TokenKind::Bold);
        assert_eq!(
            inline.inner,
            InlineContent::Plain(PlainInline {
                content: String::from("Bold text"),
                span: Span::from((start, end))
            })
        );
    }
}
