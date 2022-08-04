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
    pub fn pop(&mut self) -> Option<Token> {
        self.data.pop()
    }

    /// Removes and returns the last item on stack. If the last item on stack is an ambiguous
    /// [`Token`], the [`Token`] passed as parameter is partially removed from it and the resulting
    /// [`Token`] is returned.
    ///
    /// [`Token`]: crate::Token
    pub fn pop_or_remove_partial(&mut self, token: &Token) -> Option<Token> {
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

    fn drain_as_plain(&mut self) -> Option<InlineContent<PlainContent, NestedContent>> {
        self.data
            .drain(..)
            .map(InlineContent::from_token_as_plain)
            .reduce(|mut accumulated_content, content| {
                accumulated_content.append(content);
                accumulated_content
            })
    }
}

/// Parser of Unimarkup inline formatting. Implemented as an [`Iterator`], yields one
/// self-contained Unimarkup [`Inline`] with every iteration.
///
/// [`Iterator`]: Iterator
/// [`Inline`]: crate::Inline
#[derive(Debug, Clone)]
pub struct Parser<'i> {
    /// Iterator over [`Token`]s found in Unimarkup input.
    ///
    /// [`Token`]: crate::Token
    iter: TokenIterator<'i>,

    /// Stack used for parsing.
    stack: ParserStack,

    /// Storage of [`Token`] already yielded from [`TokenIterator`] but not consumed in current
    /// iteration of parsing.
    ///
    /// [`Token`]: crate::Token
    /// [`TokenIterator`]: crate::TokenIterator
    /// [`Inline`]: crate::Inline
    token_cache: Option<Token>,

    /// Storage of stacks used for scopes.
    stack_cache: Vec<ParserStack>,

    /// Flag to know if the current scope is fully cleared.
    scope_cleared: bool,

    /// Storage of parsed [`Inline`]s that should be returned before parsing next [`Inline`].
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
        let matches_with_open_token = self.stack.iter().any(|inner_token| {
            inner_token.is_or_contains(token)
                || token.is_or_contains(inner_token)
                || inner_token.matches_pair(token)
        });

        !token.kind().is_open_bracket() && matches_with_open_token
    }

    /// Checks if the cached [`Token`] is a closing [`Token`] and if it's opening pair is already
    /// encountered and stored in parser stack.
    ///
    /// [`Token`]: crate::Token
    fn cached_token_open(&self) -> bool {
        self.token_cache
            .as_ref()
            .map_or(false, |token| token.closes() && self.is_token_open(token))
    }

    /// Checks whether the given [`Token`] matches with the last encountered [`Token`].
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
    /// Scope is only exited if it's completely cleared.
    fn exit_scope(&mut self) {
        if !self.scope_cleared {
            return;
        }

        match self.stack_cache.pop() {
            Some(old_stack) => self.stack = old_stack,
            None => self.stack = ParserStack::default(),
        }
    }

    /// Pushes a token to the currently active stack, or enters a new scope if [`Token`] with
    /// [`TokenKind::OpenBracket`] is encountered, since that starts an [`Inline::TextGroup`].
    ///
    /// [`Token`]: crate::Token
    /// [`TokenKind::OpenBracket`]: crate::TokenKind::OpenBracket
    /// [`Inline::TextGroup`]: crate::Inline::TextGroup
    fn push_to_stack(&mut self, token: Token) -> usize {
        if matches!(token.kind(), TokenKind::OpenBracket) {
            self.enter_scope();
        }

        self.stack_mut().push(token)
    }

    /// Pops the token last pushed to the currently active stack.
    fn pop(&mut self) -> Option<Token> {
        match self.stack_mut().pop() {
            Some(token) => {
                self.scope_cleared = token.kind().is_open_bracket() && self.stack().is_empty();
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
    fn pop_or_remove_partial(&mut self, token: &Token) -> Option<Token> {
        match self.stack_mut().pop_or_remove_partial(token) {
            Some(token) => {
                self.scope_cleared = token.kind().is_open_bracket() && self.stack().is_empty();

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

    /// Returns **all** [`Token`]s pushed to **any** stack (scope) by the `Parser`.
    ///
    /// [`Token`]: crate::Token
    fn all_tokens(&self) -> impl DoubleEndedIterator<Item = &Token> {
        self.stack_cache
            .iter()
            .flat_map(|stack| stack.iter())
            .chain(self.stack.iter())
    }

    /// Checks whether the [`Inline`] that's currently being parsed is correctly closed.
    ///
    /// [`Inline`]: crate::Inline
    fn is_inline_closed(&self, token: &Token) -> bool {
        self.all_tokens()
            .rev()
            .all(|token_in_stack| token != token_in_stack)
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
        start_token: Token,
        mut content: InlineContent<PlainContent, NestedContent>,
    ) -> Inline {
        let mut start = content.span().start();
        let end = content.span().end();

        // next_token is a closing one, but it was not open last
        // -> Return parsed content as plain text backwards up to the corresponding opening token
        while !self.is_token_latest(&start_token) {
            match self.pop() {
                Some(token) => {
                    start = token.span().start();
                    content.prepend(InlineContent::from_token_as_plain(token));
                }
                None => break,
            }
        }

        if let Some(start_token) = self.pop_or_remove_partial(&start_token) {
            start = start_token.span().start();
            content.prepend(InlineContent::from_token_as_plain(start_token));
        }

        content.try_flatten();

        Inline::with_span(content, TokenKind::Plain, Span::from((start, end)))
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
    ///    remaining part of `next_token` will be stored into the token cache.
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
                    self.pop();
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

        // keyword tokens don't have content, clone is cheap
        let mut start_token = token.clone();

        self.push_to_stack(token);

        while let Some(next_token) = self.next_token() {
            if next_token.closes() && self.is_token_open(&next_token) {
                if self.is_token_latest(&next_token) {
                    let closing_token = self.resolve_closing_token(next_token);

                    end = closing_token.span().end();

                    if let Some(token) = self.pop_or_remove_partial(&closing_token) {
                        start = token.span().start();
                        kind = token.kind();
                    }

                    if self.cached_token_open() || start_token.is_ambiguous() {
                        if start_token.is_ambiguous() {
                            start_token.remove_partial(&closing_token);
                        }

                        let inner_inline = Inline::with_span(content, kind, (start, end).into());

                        content = NestedContent::from(inner_inline).into();
                    } else {
                        break;
                    }
                } else {
                    self.token_cache = Some(next_token);
                    return self.nested_inline_as_plain(start_token, content);
                }
            } else if next_token.opens() && !self.is_token_open(&next_token) {
                let nested = self.parse_nested_inline(next_token);

                content.append_inline(nested);
            } else {
                end = Self::consume_as_plain(next_token, &mut content);
            }
        }

        if !self.is_inline_closed(&start_token) {
            if start_token.span().start() != start {
                let inner_inline = Inline::with_span(content, kind, (start, end).into());
                content = NestedContent::from(inner_inline).into();
            }

            if let Some(last_token) = self.pop() {
                content.prepend(InlineContent::from(last_token));
                kind = TokenKind::Plain;
            }
        }

        let span = Span::from((start, end));
        content.try_flatten();

        Inline::with_span(content, kind, span)
    }

    /// Parses one single Unimarkup [`Inline`].
    ///
    /// [`Inline`]: crate::Inline
    fn parse_inline(&mut self) -> Option<Inline> {
        if !self.inline_cache.is_empty() {
            return self.inline_cache.pop_front();
        }

        let next_token = self.next_token()?;

        let mut inline = if next_token.opens() {
            let parsed_inline = self.parse_nested_inline(next_token);

            if !self.stack().is_empty() {
                // return remaining tokens as plain inline
                if let Some(content) = self.stack_mut().drain_as_plain() {
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
            let inline_content = InlineContent::Plain(PlainContent::new(content, span));

            Inline::as_plain_or_eol(inline_content, kind)
        };

        if let Inline::Multiple(ref mut nested_content) = inline {
            let next_inline = nested_content.content.pop_front()?;

            while let Some(inline) = nested_content.content.pop_back() {
                self.inline_cache.push_front(inline);
            }

            Some(next_inline)
        } else {
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
                self.inline_cache.push_front(next_inline);
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
mod tests;
