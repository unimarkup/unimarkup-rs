use std::{collections::VecDeque, ops::Deref};

use unimarkup_commons::scanner::span::Span;

use crate::{types::*, Inline, Token, TokenKind, Tokenize, Tokens};

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

/// Parser of Unimarkup inline formatting. Implemented as an [`Iterator`], yields one
/// self-contained Unimarkup [`Inline`] with every iteration.
///
/// [`Iterator`]: Iterator
/// [`Inline`]: crate::Inline
#[derive(Debug, Clone)]
pub struct Parser {
    /// Iterator over [`Token`]s found in Unimarkup input.
    ///
    /// [`Token`]: crate::Token
    iter: Tokens,

    /// Storage of [`Token`] already yielded from [`TokenIterator`] but not consumed in current
    /// iteration of parsing.
    ///
    /// [`Token`]: crate::Token
    /// [`TokenIterator`]: crate::TokenIterator
    /// [`Inline`]: crate::Inline
    token_cache: Option<Token>,

    /// Storage of parsed [`Inline`]s that should be returned before parsing next [`Inline`].
    inline_cache: VecDeque<Inline>,
}

impl Parser {
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

    fn parse_plain(&mut self, start_token: Token) -> Inline {
        let kind = start_token.kind;
        let (mut content, mut span) = start_token.into_inner();

        while let Some(next_token) = self.next_token() {
            if next_token.kind == kind {
                let (next_content, next_span) = next_token.into_inner();
                content.push_str(&next_content);
                span.end = next_span.end;
            } else {
                self.token_cache = Some(next_token);
                break;
            }
        }

        Inline::plain(content, kind, span)
    }

    fn parse_nested(&mut self, start_token: Token) -> Inline {
        let start = start_token.span.start;
        let mut end = start_token.span.end;
        let mut content = VecDeque::new();

        while let Some(next_token) = self.next_token() {
            if next_token.closes() {
                end = next_token.span.end;
                break;
            } else if next_token.opens() {
                // lexer resolved tokens, if token opens, it is guaranteed that closing exists too.
                // If not, it's bug in implementation
                let nested = self.parse_inline(next_token);
                content.push_back(nested);
            } else {
                // opening token not yet closed, next token does not start a new inline -> parse as
                // plain text

                end = next_token.span.end;
                let (inner, span) = next_token.into_inner();
                let inline = Inline::Plain(Plain {
                    content: inner,
                    span,
                });
                content.push_back(inline);
            }
        }

        let span = Span::from((start, end));
        Inline::nested_with_span(content, start_token.kind, span)
    }

    fn parse_inline(&mut self, token: Token) -> Inline {
        // opening/closing of tokens is resolved at lexing stage
        // at this point we can simply parse
        let kind = token.kind;

        let mut inline = if token.opens() {
            self.parse_nested(token)
        } else if kind != TokenKind::Plain && !token.opens() {
            let (content, span) = token.into_inner();
            Inline::plain_or_eol(content, span, kind)
        } else {
            self.parse_plain(token)
        };

        inline.try_merge();
        inline
    }
}

impl Iterator for Parser {
    type Item = Inline;

    fn next(&mut self) -> Option<Self::Item> {
        let mut inline = match self.inline_cache.pop_front() {
            Some(inline) => inline,
            _ => {
                let token = self.next_token()?;
                self.parse_inline(token)
            }
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

/// Extension trait for adding [`Parser`] implementation for any type that implements
/// [`Tokenize`] trait.
///
/// [`Parser`]: self::Parser
/// [`Tokenize`]: crate::Tokenize
pub trait ParseInlines {
    /// Returns a parser over this type.
    fn parse_inlines(&self) -> Parser;
}

impl<T> ParseInlines for T
where
    T: Tokenize,
{
    fn parse_inlines(&self) -> Parser {
        let iter = self.tokens();

        Parser {
            iter,
            token_cache: None,
            inline_cache: VecDeque::default(),
        }
    }
}

#[cfg(test)]
mod tests;
