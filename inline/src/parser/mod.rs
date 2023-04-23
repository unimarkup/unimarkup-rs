use std::{collections::VecDeque, ops::Deref};

use crate::{
    Inline, InlineContent, NestedContent, PlainContent, Position, Span, Token, TokenKind, Tokenize,
    Tokens,
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

    fn parse_inline(&mut self, token: Token) -> Inline {
        // opening/closing of tokens is resolved at lexing stage
        // at this point we can simply parse
        let kind = token.kind;
        let start = token.span.start;
        let mut end = token.span.end;

        if kind != TokenKind::Plain && !token.opens() {
            let (content, span) = token.into_inner();
            let content = InlineContent::Plain(PlainContent::new(content, span));
            return Inline::as_plain_or_eol(content, kind);
        }

        let mut content: InlineContent<_, _> = NestedContent::default().into();

        if kind == TokenKind::Plain {
            content.append(InlineContent::from(token));
        }

        while let Some(next_token) = self.next_token() {
            end = next_token.span.end;

            if next_token.closes() {
                break;
            } else if next_token.opens() {
                // lexer resolved tokens, if token opens, it is guaranteed that closing exists too.
                // If not, it's bug in implementation
                let nested = self.parse_inline(next_token);
                content.append_inline(nested);
            } else {
                if kind == TokenKind::Plain && next_token.kind != TokenKind::Plain {
                    self.token_cache = Some(next_token);
                    break;
                }

                end = next_token.span.end;
                content.append(InlineContent::from(next_token));
            }
        }

        let span = Span::from((start, end));
        content.try_flatten();
        Inline::with_span(content, kind, span)
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
    fn parse_inlines(&self, pos: Option<Position>) -> Parser;
}

impl<T> ParseInlines for T
where
    T: Tokenize,
{
    fn parse_inlines(&self, pos: Option<Position>) -> Parser {
        let iter = if let Some(pos) = pos {
            self.tokens_with_offs(pos)
        } else {
            self.tokens()
        };

        Parser {
            iter,
            token_cache: None,
            inline_cache: VecDeque::default(),
        }
    }
}

#[cfg(test)]
mod tests;
