use std::{collections::VecDeque, ops::Deref};

use crate::{
    Inline, InlineContent, NestedContent, PlainContent, Span, Token, TokenIterator, TokenKind,
    Tokenize,
};

/// Internal data structure used for parsing of Unimarkup [`Inline`]s.
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

    /// Removes the last element pushed to the stack, if any.
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

/// Parser of Unimarkup inline formatting. Implemented as an [`Iterator`], yields one
/// self-contained Unimarkup [`Inline`] with every iteration.
#[derive(Debug, Clone)]
pub struct Parser<'i> {
    iter: TokenIterator<'i>,
    stack: ParserStack,
    token_cache: Option<Token>,
    stack_cache: Vec<ParserStack>,
    scope_cleared: bool,
    inline_cache: VecDeque<Inline>,
}

impl Parser<'_> {
    /// Returns the next [`Token`] either from [`Lexer`] directly or from internal token cache.
    fn next_token(&mut self) -> Option<Token> {
        if self.token_cache.is_some() {
            self.token_cache.take()
        } else {
            self.iter.next()
        }
    }

    /// Checks whether any given opening [`Token`] is already encountered and not yet closed.
    fn is_token_open(&self, token: &Token) -> bool {
        let res = self.stack.iter().any(|inner_token| {
            inner_token.is_or_contains(token)
                || token.is_or_contains(inner_token)
                || inner_token.matches_pair(token)
        });

        !matches!(token.kind(), TokenKind::OpenBracket) && res
    }

    /// Checks whether the given [`Token`] is the last one encountered.
    fn is_token_latest(&self, token: &Token) -> bool {
        match self.stack().last() {
            Some(last_open_token) => {
                last_open_token.is_or_contains(token) || last_open_token.matches_pair(token)
            }
            None => false,
        }
    }

    /// Returns a mutable reference to the currently active stack - corresponding to current scope.
    fn stack_mut(&mut self) -> &mut ParserStack {
        &mut self.stack
    }

    /// Returns a reference to the currently active stack - corresponding to current scope.
    fn stack(&self) -> &ParserStack {
        &self.stack
    }

    /// Creates a new stack for the scope and sets it as the currently active stack.
    fn enter_scope(&mut self) {
        let new_stack = ParserStack::default();

        let old_stack = std::mem::replace(&mut self.stack, new_stack);

        self.stack_cache.push(old_stack);
    }

    /// Removes the currently active stack and restores the stack of the outer scope.
    fn exit_scope(&mut self) {
        if !self.scope_cleared {
            return;
        }

        match self.stack_cache.pop() {
            Some(old_stack) => self.stack = old_stack,
            None => self.stack = ParserStack::default(),
        }
    }

    /// Pushes a token to the currently active stack.
    fn push_to_stack(&mut self, token: Token) -> usize {
        if matches!(token.kind(), TokenKind::OpenBracket) {
            self.enter_scope();
        }

        self.stack_mut().push(token)
    }

    /// Pops the token last added to the currently active stack.
    fn pop_last(&mut self) -> Option<Token> {
        match self.stack_mut().pop_last() {
            Some(token) => {
                self.scope_cleared = token.kind().is_open_parentheses() && self.stack().is_empty();
                self.exit_scope();

                Some(token)
            }
            None => {
                // stack might be empty for current scope, try to exit scope and try again
                None
            }
        }
    }

    /// Pops the (part of) token that matches the token reference passed to the function.
    ///
    /// In case that token on stack contains the passed token, only the part that matches the
    /// passed token gets removed, and the rest of the token stays on the stack.
    /// This means that even if there is only one token on the stack and `pop()` is called,
    /// there might still be one token left on the stack.
    fn pop(&mut self, token: &Token) -> Option<Token> {
        match self.stack_mut().pop(token) {
            Some(token) => {
                self.scope_cleared = token.kind().is_open_parentheses() && self.stack().is_empty();

                self.exit_scope();

                Some(token)
            }
            None => {
                // stack might be empty for current scope, try to exit scope and try again
                None
            }
        }
    }

    /// Returns the last encountered [`Token`], if any.
    fn last_token(&self) -> Option<&Token> {
        match self.stack().last() {
            Some(token) => Some(token),
            None => match self.stack_cache.last() {
                Some(stack) => stack.last(),
                None => None,
            },
        }
    }

    /// Checks whether the [`Inline`] that's currently being parsed is correctly closed.
    fn inline_closed(&self, kind: TokenKind, span: Span) -> bool {
        if let Some(token) = self.last_token() {
            !(token.kind() == kind && token.span().start() == span.start())
        } else {
            true
        }
    }

    /// Parses one Unimarkup [`Inline`] that contains [`NestedContent`] as it's content. That
    /// corresponds to any [`Inline`] that is enclosed between two delimiters.
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

        let mut kind = token.kind();
        let mut start = token.span().start();
        let mut end = start;
        let mut content: InlineContent<_, _> = NestedContent {
            content: Vec::default(),
            span: (start, end).into(),
        }
        .into();

        self.push_to_stack(token);

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
                        end = next_token.span().end();

                        if let Some(token) = self.pop(&next_token) {
                            start = token.span().start();
                            kind = token.kind();
                        }

                        break;
                    } else {
                        // It might be ambiguous token and part of it is open,
                        // for example ** followed by ***. Such token should be split as **|*,
                        // where first part (**) is being closed, and second part (*) is now in
                        // token_cache for next iteration

                        if next_token.is_ambiguous() {
                            // at this point we know there is at least one token in stack
                            let last_token = self.last_token().unwrap();

                            if next_token.is_or_contains(last_token) {
                                let parsed_token = next_token.remove_partial(last_token);

                                self.pop_last();
                                self.token_cache = Some(next_token);

                                end = parsed_token.span().end();

                                // close this inline
                                break;
                            }
                        } else {
                            // It is closing one, but it was not open last -> Return contents as inline

                            // remove the opening token from the stack
                            let token = self.pop(&next_token).unwrap();

                            // NOTE: when coming from nested, while loop will be continued -> takes
                            // another token from iterator or cache
                            self.token_cache = Some(next_token);

                            // prepend the token to content as plain text
                            content.prepend(InlineContent::from_token_as_plain(token));

                            return Inline::Plain(content.into_plain());
                        }
                    }
                } else {
                    // plain text
                    end = next_token.span().end();

                    // consume plain text
                    content.append(InlineContent::from(next_token));
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
                end = next_token.span().end();

                let inline_content = InlineContent::from(next_token);
                content.append(inline_content);
            }
        }

        let span = Span::from((start, end));

        if !self.inline_closed(kind, span) {
            if let Some(last_token) = self.pop_last() {
                content.prepend(InlineContent::from(last_token));
                kind = TokenKind::Plain;
            }
        }

        // if content contains only plain contents, then merge them and make into one
        content.try_flatten();
        content.set_span(span);

        Inline::new(content, kind)
    }

    /// Parses one single Unimarkup [`Inline`].
    fn parse_inline(&mut self) -> Option<Inline> {
        if !self.inline_cache.is_empty() {
            self.inline_cache.pop_front()
        } else {
            let next_token = self.next_token()?;

            let inline = if next_token.opens() {
                let parsed_inline = self.parse_nested_inline(next_token);

                if !self.stack().is_empty() {
                    // cache parsed inline for next iteration

                    // return remaining tokens as plain inline
                    if let Some(content) = self
                        .stack_mut()
                        .data
                        .drain(..)
                        .map(InlineContent::from_token_as_plain)
                        .reduce(|mut accumulated_content, content| {
                            accumulated_content.append(content);
                            accumulated_content
                        })
                    {
                        self.inline_cache.push_front(parsed_inline);
                        Inline::new(content, TokenKind::Plain)
                    } else {
                        parsed_inline
                    }
                } else {
                    parsed_inline
                }
            } else {
                let kind = next_token.kind();

                let (content, span) = next_token.into_inner();
                let inline_content = InlineContent::Plain(PlainContent { content, span });

                Inline::new(inline_content, kind)
            };

            Some(inline)
        }
    }
}

impl Iterator for Parser<'_> {
    type Item = Inline;

    fn next(&mut self) -> Option<Self::Item> {
        let mut curr_inline = self.parse_inline()?;

        while let Some(next_inline) = self.parse_inline() {
            let is_kind_same = curr_inline.matches_kind(&next_inline);
            if is_kind_same {
                let (combined_inline, mut rest_of_inlines) = curr_inline.merge(next_inline);

                curr_inline = combined_inline;

                if rest_of_inlines.is_empty() {
                    continue;
                } else {
                    self.inline_cache.append(&mut rest_of_inlines);
                    break;
                }
            } else {
                self.inline_cache.push_back(next_inline);
                break;
            }
        }

        Some(curr_inline)
    }
}

/// Extension trait for adding [`Parser`] implementation for any type that implements
/// [`Tokenize`] trait.
///
/// [`Parser`]: crate::Parser
/// [`Tokenize`]: crate::Tokenize
pub trait ParseUnimarkupInlines<'p, 'i>
where
    'i: 'p,
{
    /// Returns a parser over this type.
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
            stack_cache: Vec::default(),
            scope_cleared: true,
            inline_cache: VecDeque::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{PlainContent, Position};

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
        assert!(matches!(inline, Inline::Bold(_)));
        // assert_eq!(inline.kind, InlineKind::Bold);
        assert_eq!(
            inline.as_ref(),
            InlineContent::Nested(&NestedContent {
                content: vec![Inline::Plain(PlainContent {
                    content: String::from("Bold text"),
                    span: (start, end).into()
                })],
                span: (start - (0, 2), end + (0, 2)).into(),
            })
        );
    }

    #[test]
    fn parse_simple_italic() {
        let mut parser = "*Italic text*".parse_unimarkup_inlines();

        let inline = parser.next().unwrap();
        let start = Position { line: 1, column: 2 };
        let end = start + (0, 11 - 1);

        // no remaining inlines
        assert_eq!(parser.count(), 0);
        assert!(matches!(inline, Inline::Italic(_)));
        assert_eq!(
            inline.as_ref(),
            InlineContent::Nested(&NestedContent {
                content: vec![Inline::Plain(PlainContent {
                    content: String::from("Italic text"),
                    span: Span::from((start, end))
                })],
                span: (start - (0, 1), end + (0, 1)).into()
            })
        );
    }

    #[test]
    fn parse_italic_bold() {
        let mut parser = "*Italic text***Bold text**".parse_unimarkup_inlines();

        let inline = parser.next().unwrap();
        let start = Position { line: 1, column: 2 };
        let end = start + (0, 11 - 1);

        assert!(matches!(inline, Inline::Italic(_)));
        assert_eq!(
            inline.as_ref(),
            InlineContent::Nested(&NestedContent {
                content: vec![Inline::Plain(PlainContent {
                    content: String::from("Italic text"),
                    span: (start, end).into()
                })],
                span: (start - (0, 1), end + (0, 1)).into()
            })
        );

        let inline = parser.next().unwrap();
        let start = end + (0, 5 - 1);
        let end = start + (0, 9 - 1);

        assert!(matches!(inline, Inline::Bold(_)));
        assert_eq!(
            inline.as_ref(),
            InlineContent::Nested(&NestedContent {
                content: vec![Inline::Plain(PlainContent {
                    content: String::from("Bold text"),
                    span: Span::from((start, end))
                })],
                span: (start - (0, 2), end + (0, 2)).into()
            })
        );
    }

    #[test]
    fn parse_bold_italic_nested() {
        let mut parser = "**This is bold *with* italic inside.**".parse_unimarkup_inlines();

        let inline = parser.next().unwrap();
        let start = Position { line: 1, column: 1 };
        let end = start + (0, 38 - 1);

        // no remaining inlines
        assert_eq!(parser.count(), 0);

        assert!(matches!(inline, Inline::Bold(_)));
        assert_eq!(inline.span(), Span::from((start, end)));
        assert!(matches!(inline.as_ref(), InlineContent::Nested(_)));

        if let InlineContent::Nested(inner_content) = inline.into_inner() {
            assert_eq!(inner_content.count(), 3);

            let inline = &inner_content[0];

            let start = Position { line: 1, column: 3 };
            let end = start + (0, 13 - 1);

            assert!(matches!(inline, Inline::Plain(_)));
            assert_eq!(
                inline.as_ref(),
                InlineContent::Plain(&PlainContent {
                    content: String::from("This is bold "),
                    span: Span::from((start, end))
                })
            );

            let inline = &inner_content[1];

            let start = end + (0, 1);
            let end = start + (0, 6 - 1);

            let inner_start = start + (0, 1);
            let inner_end = end - (0, 1);

            assert!(matches!(inline, Inline::Italic(_)));
            assert_eq!(
                inline.as_ref(),
                InlineContent::Nested(&NestedContent {
                    content: vec![Inline::Plain(PlainContent {
                        content: String::from("with"),
                        span: Span::from((inner_start, inner_end))
                    })],
                    span: (inner_start - (0, 1), inner_end + (0, 1)).into()
                })
            );
            assert_eq!(inline.span(), Span::from((start, end)));

            let inline = &inner_content[2];

            let start = end + (0, 1);
            let end = start + (0, 15 - 1);

            assert!(matches!(inline, Inline::Plain(_)));
            assert_eq!(
                inline.as_ref(),
                InlineContent::Plain(&PlainContent {
                    content: String::from(" italic inside."),
                    span: Span::from((start, end))
                })
            );
        } else {
            panic!("Inner content not nested");
        }
    }

    #[test]
    fn parse_text_group_simple() {
        let mut parser = "This is text [with text group] as part of it.".parse_unimarkup_inlines();

        let inline = parser.next().unwrap();
        let start = Position { line: 1, column: 1 };
        let end = start + (0, 13 - 1);

        assert!(matches!(inline, Inline::Plain(_)));
        assert_eq!(inline.span(), Span::from((start, end)));
        assert!(matches!(inline.as_ref(), InlineContent::Plain(_)));

        let inline = parser.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 17 - 1);

        assert!(matches!(inline, Inline::TextGroup(_)));
        assert_eq!(inline.span(), Span::from((start, end)));
        assert_eq!(
            inline.as_ref(),
            InlineContent::Nested(&NestedContent {
                content: vec![Inline::Plain(PlainContent {
                    content: String::from("with text group"),
                    span: (start + (0, 1), end - (0, 1)).into()
                })],
                span: (start, end).into()
            })
        );

        let inline = parser.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 15 - 1);

        assert!(matches!(inline, Inline::Plain(_)));
        assert_eq!(inline.span(), Span::from((start, end)));
        assert!(matches!(inline.as_ref(), InlineContent::Plain(_)));
    }

    #[test]
    fn parse_text_group_interrupt_bold() {
        let input = "This is **text [with text** group] as part of it.";
        let mut parser = input.parse_unimarkup_inlines();

        for inline in parser.clone() {
            println!("{inline:#?}");
        }

        println!("\n\nParsing following text: {input}\n\n");

        let inline = parser.next().unwrap();
        println!("{inline:#?}\n");

        let start = Position { line: 1, column: 1 };
        let end = start + (0, 15 - 1);

        assert!(matches!(inline, Inline::Plain(_)));
        assert_eq!(inline.span(), Span::from((start, end)));
        assert_eq!(
            inline.as_ref(),
            InlineContent::Plain(&PlainContent {
                content: String::from("This is **text "),
                span: Span::from((start, end))
            })
        );

        let inline = parser.next().unwrap();
        println!("{inline:#?}\n");

        let start = end + (0, 1);
        let end = start + (0, 19 - 1);

        assert!(matches!(inline, Inline::TextGroup(_)));
        assert_eq!(inline.span(), Span::from((start, end)));
        assert_eq!(
            inline.as_ref(),
            InlineContent::Nested(&NestedContent {
                content: vec![Inline::Plain(PlainContent {
                    content: String::from("with text** group"),
                    span: Span::from((start + (0, 1), end - (0, 1)))
                })],
                span: Span::from((start, end))
            })
        );

        let inline = parser.next().unwrap();
        println!("{inline:#?}\n");

        let start = end + (0, 1);
        let end = start + (0, 15 - 1);

        assert!(matches!(inline, Inline::Plain(_)));
        assert_eq!(inline.span(), Span::from((start, end)));
        assert_eq!(
            inline.as_ref(),
            InlineContent::Plain(&PlainContent {
                content: String::from(" as part of it."),
                span: Span::from((start, end))
            })
        );
    }

    #[test]
    fn parse_open_italic_closed_bold() {
        let input = "***This is input**";
        let mut parser = input.parse_unimarkup_inlines();

        let inline = parser.next().unwrap();
        let start = Position::new(1, 1);
        let end = start;

        assert!(matches!(inline, Inline::Plain(_)));
        assert_eq!(
            inline.as_ref(),
            InlineContent::Plain(&PlainContent {
                content: String::from("*"),
                span: Span::from((start, end))
            })
        );

        let inline = parser.next().unwrap();
        let start = end + (0, 1);
        let end = start + (0, 17 - 1);

        assert!(matches!(inline, Inline::Bold(_)));
        assert_eq!(
            inline.as_ref(),
            InlineContent::Nested(&NestedContent {
                content: vec![Inline::Plain(PlainContent {
                    content: String::from("This is input"),
                    span: Span::from((start + (0, 2), end - (0, 2)))
                })],
                span: Span::from((start, end))
            })
        );
    }

    #[test]
    fn parse_nested_text_group() {
        let input = "[This text group [has another one inside] of it.]";
        let mut parser = input.parse_unimarkup_inlines();

        println!("\n\nParsing following text: {input}\n\n");

        let inline = parser.next().unwrap();
        println!("{inline:#?}\n");

        let start = Position { line: 1, column: 1 };
        let end = start + (0, 49 - 1);

        assert!(matches!(inline, Inline::TextGroup(_)));
        assert!(matches!(inline.as_ref(), InlineContent::Nested(_)));
        assert_eq!(inline.span(), Span::from((start, end)));

        let inline_content = inline.into_inner().into_nested();
        let mut inner_inlines = inline_content.content.iter();

        let inline = inner_inlines.next().unwrap();

        let start = Position::new(1, 2);
        let end = start + (0, 16 - 1);

        assert!(matches!(inline, Inline::Plain(_)));
        assert_eq!(
            inline.as_ref(),
            InlineContent::Plain(&PlainContent {
                content: String::from("This text group "),
                span: Span::from((start, end))
            })
        );

        let inline = inner_inlines.next().unwrap();

        let start = end + (0, 1);
        let end = start + (0, 24 - 1);

        assert!(matches!(inline, Inline::TextGroup(_)));
        assert_eq!(
            inline.as_ref(),
            InlineContent::Nested(&NestedContent {
                content: vec![Inline::Plain(PlainContent {
                    content: String::from("has another one inside"),
                    span: Span::from((start + (0, 1), end - (0, 1)))
                })],
                span: Span::from((start, end))
            })
        );

        let inline = inner_inlines.next().unwrap();

        let start = end + (0, 1);
        let end = start + (0, 7 - 1);

        assert!(matches!(inline, Inline::Plain(_)));
        assert_eq!(
            inline.as_ref(),
            InlineContent::Plain(&PlainContent {
                content: String::from(" of it."),
                span: Span::from((start, end))
            })
        );
    }
}
