use std::fmt::Display;

use crate::Spacing;

/// Token that can be found in Unimarkup inline formats.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    kind: TokenKind,
    spacing: Spacing,
}

impl Token {
    /// Creates a new token with the given [`TokenKind`] and [`Spacing`].
    pub fn new(kind: TokenKind, spacing: Spacing) -> Self {
        Self { kind, spacing }
    }
}

/// Kinds of tokens which can be identified in Unimarkup inline formats.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TokenKind {
    /// Bold delimiter (**).
    Bold,

    /// Italic delimiter (*).
    Italic,

    /// Undeline delimiter (__).
    Underline,

    /// Subscript delimiter (_).
    Subscript,

    /// Superscript delimiter (^).
    Superscript,

    /// Verbatim delimiter (~).
    Verbatim,

    /// Token representing simple text
    Plain(String),

    /// Token which might consist of multiple token kinds.
    /// e.g. `***` might be `Bold` + `Italic`, or `Bold` + `Plain` etc.
    Ambiguous(AmbiguousToken),
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Bold => f.write_str("**"),
            TokenKind::Italic => f.write_str("*"),
            TokenKind::Underline => f.write_str("__"),
            TokenKind::Subscript => f.write_str("_"),
            TokenKind::Superscript => f.write_str("^"),
            TokenKind::Verbatim => f.write_str("~"),
            TokenKind::Plain(content) => f.write_str(content),
            TokenKind::Ambiguous(token) => {
                for token_kind in &token.tokens {
                    f.write_fmt(format_args!("{token_kind}"))?;
                }

                Ok(())
            }
        }
    }
}

/// Token which might consist of multiple token kinds.
/// e.g. `***` might be `Bold` + `Italic`, or `Bold` + `Plain` etc.
///
/// Each potential token kind is contained only once in the `AmbiguousToken`.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct AmbiguousToken {
    /// Potentially contained tokens.
    tokens: Vec<TokenKind>,
}

impl<const N: usize> From<[TokenKind; N]> for AmbiguousToken {
    fn from(tokens: [TokenKind; N]) -> Self {
        let mut inner_tokens = Vec::with_capacity(tokens.len());

        for token in tokens {
            if !inner_tokens.contains(&token) {
                inner_tokens.push(token);
            }
        }

        Self {
            tokens: inner_tokens,
        }
    }
}

impl AmbiguousToken {
    /// Creates new [`AmbiguousToken`] from the given tokens. Each given token is contained only
    /// once in the `AmbiguousToken`.
    pub fn new(first: TokenKind, second: TokenKind) -> Self {
        if first != second {
            Self {
                tokens: vec![first, second],
            }
        } else {
            Self {
                tokens: vec![first],
            }
        }
    }

    /// Convert the [`AmbiguousToken`] with remaining tokens into [`TokenKind::Plain(content)`],
    /// with `content` as concatenated string representation of inner tokens.
    pub fn to_plain(self) -> TokenKind {
        // at least 3 characters
        let mut content = String::with_capacity(3);

        for token_kind in self.tokens {
            content.push_str(&token_kind.to_string());
        }

        TokenKind::Plain(content)
    }
}
