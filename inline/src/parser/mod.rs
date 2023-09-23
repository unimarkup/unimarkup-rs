use std::{collections::VecDeque, ops::Deref};

use unimarkup_commons::scanner::span::Span;

use crate::{Inline, Token, TokenKind, Tokenize, Tokens};

/// Internal data structure used for parsing of Unimarkup [`Inline`]s.
///
/// # Lifetimes
///
/// `'input` - lifetime of the input the [`Token`]s are lexed from.
///
/// [`Inline`]: crate::Inline
#[derive(Debug, Default, Clone)]
struct ParserStack<'input> {
    data: Vec<Token<'input>>,
}

impl<'input> Deref for ParserStack<'input> {
    type Target = Vec<Token<'input>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/// Context passed to [`Parser::parse_inline`] function. This context is used to provide more
/// information for the parser, so that the content can be parsed and stored correctly.
struct PlainContext {
    /// Whether the currently parsed plain inline is enclosed by some symbols. One such inline is
    /// [`Inline::Verbatim`]
    enclosed: bool,

    /// Whether the currently parsed plain inline merges tokens that can be merged (such as
    /// whitespace tokens) or preserves them as is.
    merge_tokens: bool,
}

/// Parser of Unimarkup inline formatting. Implemented as an [`Iterator`], yields one
/// self-contained Unimarkup [`Inline`] with every iteration.
///
/// # Lifetimes
///
/// `'input` - lifetime of the input the [`Token`]s are lexed from.
///
/// [`Iterator`]: Iterator
/// [`Inline`]: crate::Inline
#[derive(Debug, Clone)]
pub struct Parser<'input> {
    /// Iterator over [`Token`]s found in Unimarkup input.
    ///
    /// [`Token`]: crate::Token
    iter: Tokens<'input>,

    /// Storage of [`Token`] already yielded from [`TokenIterator`] but not consumed in current
    /// iteration of parsing.
    ///
    /// [`Token`]: crate::Token
    /// [`TokenIterator`]: crate::TokenIterator
    /// [`Inline`]: crate::Inline
    token_cache: Option<Token<'input>>,

    /// Storage of parsed [`Inline`]s that should be returned before parsing next [`Inline`].
    inline_cache: VecDeque<Inline>,
}

impl<'input> Parser<'input> {
    /// Returns the next [`Token`] either from [`Lexer`] directly or from internal token cache.
    ///
    /// # Lifetimes
    ///
    /// [`Token`]: crate::Token
    /// [`Lexer`]: crate::Lexer
    fn next_token(&mut self) -> Option<Token<'input>> {
        if self.token_cache.is_some() {
            self.token_cache.take()
        } else {
            self.iter.next()
        }
    }

    /// Returns the next [`Token`] for which the passed function evaluates to true either from
    /// [`Lexer`] directly or from internal token cache. This function progresses the tokens, so if
    /// used it is no longer possible to access tokens skiped by this function.
    ///
    /// [`Token`]: crate::Token
    /// [`Lexer`]: crate::Lexer
    fn find_token<F>(&mut self, mut f: F) -> Option<Token<'input>>
    where
        F: FnMut(&Token) -> bool,
    {
        if let Some(token) = self.token_cache.take() {
            if f(&token) {
                return Some(token);
            }
        }

        self.iter.find(f)
    }

    fn parse_plain(&mut self, start_token: Token, ctxt: PlainContext) -> Inline {
        // convert kind into plain, if first token is neither plain nor plain-enclosed
        // (e.g. Whitespace)
        let kind = if start_token.consumable_by_plain() {
            TokenKind::Plain
        } else {
            start_token.kind
        };

        let (tkn_str, mut span) = start_token.parts();

        let mut content = if ctxt.enclosed {
            // skip first (the enclosing) token
            String::new()
        } else if start_token.kind == TokenKind::Whitespace {
            // edge case: plain can start with a whitespace
            String::from(TokenKind::Whitespace.as_str())
        } else {
            String::from(tkn_str)
        };

        while let Some(next_token) = self.next_token() {
            let enclosed_and_closes = ctxt.enclosed && next_token.closes(Some(&start_token));
            let not_enclosed_and_interrupted = !ctxt.enclosed && next_token.kind != kind;

            if enclosed_and_closes {
                span.end = next_token.span.end;
                break;
            } else if not_enclosed_and_interrupted {
                if !matches!(kind, TokenKind::Newline | TokenKind::EscapedNewline)
                    && next_token.consumable_by_plain()
                {
                    // consume the token
                    let (next_content, next_span) = next_token.parts();

                    match next_token.kind {
                        TokenKind::Whitespace => content.push_str(TokenKind::Whitespace.as_str()),
                        _ => content.push_str(next_content),
                    };

                    span.end = next_span.end;

                    if ctxt.merge_tokens {
                        // skip other tokens so that only one token is consumed, effectively
                        // "merging" them into one.
                        self.token_cache = self.find_token(|tkn| !next_token.can_merge_with(tkn));
                    }
                } else {
                    // cache popped token and break
                    self.token_cache = Some(next_token);
                    break;
                }
            } else {
                let (next_content, next_span) = next_token.parts();
                content.push_str(next_content);
                span.end = next_span.end;
            }
        }

        Inline::plain_or_eol(content, span, kind)
    }

    fn parse_nested(&mut self, start_token: Token) -> Inline {
        let start = start_token.span.start;
        let mut end = start_token.span.end;
        let mut content = VecDeque::new();

        while let Some(next_token) = self.next_token() {
            if next_token.closes(Some(&start_token)) {
                end = next_token.span.end;
                break;
            } else {
                let inner = self.parse_inline(next_token);
                content.push_back(inner);
            }
        }

        let span = Span::from((start, end));
        Inline::nested_with_span(content, start_token.kind, span)
    }

    fn parse_inline(&mut self, token: Token) -> Inline {
        // opening/closing of tokens is resolved at lexing stage
        // at this point we can simply parse
        let kind = token.kind;

        let mut inline = match (kind, token.opens()) {
            (TokenKind::Verbatim, _) => self.parse_plain(
                token,
                PlainContext {
                    enclosed: true,
                    merge_tokens: false,
                },
            ),
            (_, true) => self.parse_nested(token),
            _ => self.parse_plain(
                token,
                PlainContext {
                    enclosed: false,
                    merge_tokens: true,
                },
            ),
        };

        inline.merge();
        inline
    }
}

impl Iterator for Parser<'_> {
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
/// # Lifetimes
///
/// - `'input` is the lifetime of the input being parsed.
///
/// [`Parser`]: self::Parser
/// [`Tokenize`]: crate::Tokenize
pub trait ParseInlines<'input> {
    /// Returns a parser over this type.
    fn parse_inlines(&'input self) -> Parser<'input>;
}

impl<'input, T> ParseInlines<'input> for T
where
    T: Tokenize<'input>,
{
    fn parse_inlines(&'input self) -> Parser<'input> {
        let iter = self.tokens();

        Parser {
            iter,
            token_cache: None,
            inline_cache: VecDeque::default(),
        }
    }
}
