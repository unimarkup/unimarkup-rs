use std::{collections::VecDeque, ops::Deref};

use crate::{
    Inline, InlineContent, NestedContent, PlainContent, Position, Span, Token, TokenIterator,
    TokenKind, Tokenize,
};

/// Internal data structure used for parsing of Unimarkup [`Inline`]s.
///
/// [`Inline`]: crate::Inline
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
            match self.data.last_mut() {
                Some(last_open) if last_open.is_ambiguous() && last_open.kind() != token.kind() => {
                    // remove the ambiguous part...
                    let removed_token = last_open.remove_partial(token);

                    Some(removed_token)
                }
                _ => self.data.pop(),
            }
        }
    }
}

/// Parser of Unimarkup inline formatting. Implemented as an [`Iterator`], yields one
/// self-contained Unimarkup [`Inline`] with every iteration.
///
/// [`Iterator`]: Iterator
/// [`Inline`]: crate::Inline
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
    ///
    /// [`Token`]: crate::Token
    /// [`Lexer`]: crate::Lexer
    fn next_token(&mut self) -> Option<Token> {
        if self.token_cache.is_some() {
            self.token_cache.take()
        } else {
            self.iter.next()
        }
    }

    /// Checks whether any given opening [`Token`] is already encountered and not yet closed.
    ///
    /// [`Token`]: crate::Token
    fn is_token_open(&self, token: &Token) -> bool {
        let res = self.stack.iter().any(|inner_token| {
            inner_token.is_or_contains(token)
                || token.is_or_contains(inner_token)
                || inner_token.matches_pair(token)
        });

        !token.kind().is_open_parentheses() && res
    }

    /// Checks whether there is a [`Token`] stored in token cache that is a closing one and is
    /// already open.
    ///
    /// [`Token`]: crate::Token
    fn cached_token_open(&self) -> bool {
        self.token_cache
            .as_ref()
            .map_or(false, |token| token.closes() && self.is_token_open(token))
    }

    /// Checks whether the given [`Token`] is the last one encountered.
    ///
    /// [`Token`]: crate::Token
    fn is_token_latest(&self, token: &Token) -> bool {
        match self.stack().last() {
            Some(last_open_token) => {
                last_open_token.is_or_contains(token)
                    || token.is_or_contains(last_open_token)
                    || last_open_token.matches_pair(token)
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
            None => None,
        }
    }

    /// Returns the last encountered [`Token`], if any.
    ///
    /// [`Token`]: crate::Token
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
    ///
    /// [`Inline`]: crate::Inline
    fn inline_closed(&self, kind: TokenKind, span: Span) -> bool {
        if let Some(token) = self.last_token() {
            !(token.kind() == kind && token.span().start() == span.start())
        } else {
            true
        }
    }

    /// Constructs an [`Inline::Plain`] from [`Inline`] that was parsed up to the `next_token`.
    ///
    /// This is used when parsing of some inner [`Inline`] is started, but before it's being
    /// closed the outer [`Inline`] is closed.
    ///
    /// [`Inline`]: crate::Inline
    /// [`Inline::Plain`]: crate::Inline::Plain
    fn nested_inline_as_plain(
        &mut self,
        next_token: Token,
        mut content: InlineContent<PlainContent, NestedContent>,
    ) -> Inline {
        // It is closing one, but it was not open last -> Return contents as inline

        // remove the opening token from the stack
        let token = self.pop(&next_token).unwrap();

        // NOTE: when coming from nested, while loop will be continued -> takes
        // another token from iterator or cache
        self.token_cache = Some(next_token);

        // prepend the token to content as plain text
        content.prepend(InlineContent::from_token_as_plain(token));

        Inline::Plain(content.into_plain())
    }

    /// Consumes the [`Token`] as [`Inline::Plain`] and appends it to the current
    /// [`InlineContent`].
    ///
    /// [`Token`]: crate::Token
    /// [`Inline::Plain`]: crate::Inline::Plain
    /// [`InlineContent`]: crate::InlineContent
    fn consume_as_plain(
        next_token: Token,
        content: &mut InlineContent<PlainContent, NestedContent>,
    ) -> Position {
        // plain text
        let end = next_token.span().end();

        // consume plain text
        content.append(InlineContent::from(next_token));

        end
    }

    /// Resolves the [`Token`] that's assumed to be the closing one. If the [`Token`] is ambiguous
    /// it will be split into two non-ambiguous tokens.
    ///
    /// There are three cases:
    /// 1. Opening [`Token`] is not ambiguous, but `next_token` is. In this case, the opening
    ///    [`Token`] will be removed from `next_token`, removed token will be returned and the
    ///    remaining pairt of `next_token` will be stored into the token cache.
    /// 2. Both opening and `next_token` are ambiguous. They will be split into their non-ambiguous
    ///    parts and one part will be returned, and other stored into the token cache.
    /// 3. `next_token` is not ambiguous, so the opening [`Token`] is not relevant. The
    ///    `next_token` will be simply returned.
    ///
    /// # Panics
    ///
    /// In the first case the opened [`Token`] will be removed from `next_token`. It's up to the
    /// caller to make sure that these two [`Token`]s are compatible for partial removal. They're
    /// compatible `next_token` contains the opened [`Token`].
    ///
    /// [`Token`]: crate::Token
    fn resolve_closing_token(&mut self, mut next_token: Token) -> Token {
        match self.last_token() {
            Some(last_token) if next_token.is_ambiguous() => {
                // ambiguous token must be split into non-ambiguous tokens
                let (closing_token, next_token) = if !last_token.is_ambiguous() {
                    let closing_token = next_token.remove_partial(last_token);
                    self.pop_last();
                    (closing_token, next_token)
                } else {
                    next_token.split_ambiguous()
                };

                self.token_cache = Some(next_token);
                closing_token
            }
            _ => next_token,
        }
    }

    /// Parses one Unimarkup [`Inline`] that contains [`NestedContent`] as it's content. That
    /// corresponds to any [`Inline`] that is enclosed between two delimiters.
    ///
    /// [`Inline`]: crate::Inline
    /// [`NestedContent`]: crate::NestedContent
    fn parse_nested_inline(&mut self, token: Token) -> Inline {
        let mut kind = token.kind();
        let mut start = token.span().start();
        let mut end = start;
        let mut content: InlineContent<_, _> = NestedContent::default().into();

        self.push_to_stack(token);

        while let Some(next_token) = self.next_token() {
            if next_token.closes() && self.is_token_open(&next_token) {
                if self.is_token_latest(&next_token) {
                    let closing_token = self.resolve_closing_token(next_token);

                    end = closing_token.span().end();

                    if let Some(token) = self.pop(&closing_token) {
                        start = token.span().start();
                        kind = token.kind();
                    }

                    if self.cached_token_open() {
                        content.try_flatten();

                        let inner_inline = Inline::with_span(content, kind, (start, end).into());

                        content = NestedContent::from(inner_inline).into();
                    } else {
                        break;
                    }
                } else {
                    return self.nested_inline_as_plain(next_token, content);
                }
            } else if next_token.opens() && !self.is_token_open(&next_token) {
                let nested = self.parse_nested_inline(next_token);
                end = nested.span().end();

                content.append_inline(nested);
            } else {
                end = Self::consume_as_plain(next_token, &mut content);
            }
        }

        let span = Span::from((start, end));

        if !self.inline_closed(kind, span) {
            if let Some(last_token) = self.pop_last() {
                content.prepend(InlineContent::from(last_token));
                kind = TokenKind::Plain;
            }
        }

        content.try_flatten();
        Inline::with_span(content, kind, span)
    }

    /// Parses one single Unimarkup [`Inline`].
    ///
    /// [`Inline`]: crate::Inline
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
/// [`Parser`]: self::Parser
/// [`Tokenize`]: crate::Tokenize
pub trait ParseUnimarkupInlines<'p> {
    /// Returns a parser over this type.
    fn parse_unimarkup_inlines(&'p self) -> Parser<'p>;
}

impl<T> ParseUnimarkupInlines<'_> for T
where
    T: Tokenize,
{
    fn parse_unimarkup_inlines(&self) -> Parser<'_> {
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
                })]
                .into(),
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
                })]
                .into(),
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
                })]
                .into(),
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
                })]
                .into(),
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

            let inline = &inner_content.content.get(0).unwrap();

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
                    })]
                    .into(),
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
                })]
                .into(),
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
                })]
                .into(),
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
                })]
                .into(),
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
                })]
                .into(),
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

    #[test]
    fn parse_open_italic_closed_bold_in_tg() {
        let input = "This huhuu [***This is input**]";
        let mut parser = input.parse_unimarkup_inlines();

        let inline = parser.next().unwrap();

        let start = Position::new(1, 1);
        let end = start + (0, 11 - 1);

        assert!(matches!(inline, Inline::Plain(_)));
        assert_eq!(
            inline.as_ref(),
            InlineContent::Plain(&PlainContent {
                content: String::from("This huhuu "),
                span: Span::from((start, end))
            })
        );

        let inline = parser.next().unwrap();

        let start = end + (0, 1);
        let end = start + (0, 20 - 1);

        assert!(matches!(inline, Inline::TextGroup(_)));
        assert_eq!(inline.span(), Span::from((start, end)));

        let mut inner = inline.into_inner().into_nested();
        let mut inner = inner.content.drain(..);

        let inline = inner.next().unwrap();
        let start = start + (0, 1);
        let end = start;

        assert!(matches!(inline, Inline::Plain(_)));
        assert_eq!(
            inline.as_ref(),
            InlineContent::Plain(&PlainContent {
                content: String::from("*"),
                span: Span::from((start, end))
            })
        );

        let inline = inner.next().unwrap();
        let start = start + (0, 1);
        let end = start + (0, 17 - 1);

        assert!(matches!(inline, Inline::Bold(_)));
        assert!(matches!(inline.as_ref(), InlineContent::Nested(_)));
        assert_eq!(inline.span(), Span::from((start, end)));

        let mut inner = inline.into_inner().into_nested();
        let mut inner = inner.content.drain(..);

        let inline = inner.next().unwrap();
        let start = start + (0, 2);
        let end = start + (0, 13 - 1);

        assert!(matches!(inline, Inline::Plain(_)));
        assert_eq!(
            inline.as_ref(),
            InlineContent::Plain(&PlainContent {
                content: String::from("This is input"),
                span: Span::from((start, end))
            })
        )
    }
}
