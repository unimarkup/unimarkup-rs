use std::collections::VecDeque;

use crate::{Span, TokenDelimiters, TokenKind};

mod content;
mod substitute;

pub use content::*;
pub use substitute::*;

/// Representation of Unimarkup inline-formatted text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inline {
    /// Bold formatted content.
    Bold(NestedContent),

    /// Italic formatted content.
    Italic(NestedContent),

    /// Underlined content.
    Underline(NestedContent),

    /// Content in a subscript.   
    Subscript(NestedContent),

    /// Content in a superscript.
    Superscript(NestedContent),

    /// Overlined content.
    Overline(NestedContent),

    /// Content with a strikethrough.
    Strikethrough(NestedContent),

    /// Highlighted content.
    Highlight(NestedContent),

    /// Verbatim (monospaced) content.
    Verbatim(PlainContent),

    /// Quoted content.
    Quote(NestedContent),

    /// LaTeX-like math content.
    Math(NestedContent),

    /// Content inside a pair of parenthesis `()`.
    Parentheses(PlainContent),

    /// Content of a TextGroup `[]`.
    TextGroup(NestedContent),

    /// Unimarkup attributes for some content.
    Attributes(NestedContent),

    /// Alias substitution ( i.e. `::heart::`).
    Substitution(NestedContent),

    /// Explicit newline.
    Newline(PlainContent),

    /// Explicit whitespace.
    Whitespace(PlainContent),

    /// End of line (regular newline)
    EndOfLine(PlainContent),

    /// Plain text without any formatting.
    Plain(PlainContent),

    /// Wrapper without any special formatting for multiple other [`Inline`]s.
    ///
    /// [`Inline`]: self::Inline
    Multiple(NestedContent),
}

impl Inline {
    /// Create new [`Inline`] from given content depending on the given kind.
    ///
    /// # Arguments
    ///
    /// * `content` - the [`InlineContent`] put inside the created [`Inline`]
    /// * `kind` - the [`TokenKind`] used to define the kind of [`Inline`] that should be created
    ///
    /// [`Inline`]: self::Inline
    /// [`TokenKind`]: crate::TokenKind
    /// [`InlineContent`]: self::content::InlineContent
    pub fn new(content: InlineContent<PlainContent, NestedContent>, kind: TokenKind) -> Self {
        let consume_as_plain = |content| match content {
            InlineContent::Plain(plain_content) => Self::Plain(plain_content),
            InlineContent::Nested(nested_content) => Self::Multiple(nested_content),
        };

        match kind {
            TokenKind::Bold => Self::Bold(content.into()),
            TokenKind::Italic => Self::Italic(content.into()),
            TokenKind::Underline => Self::Underline(content.into()),
            TokenKind::Subscript => Self::Subscript(content.into()),
            TokenKind::Superscript => Self::Superscript(content.into()),
            TokenKind::Overline => Self::Overline(content.into()),
            TokenKind::Strikethrough => Self::Strikethrough(content.into()),
            TokenKind::Highlight => Self::Highlight(content.into()),
            TokenKind::Quote => Self::Quote(content.into()),
            TokenKind::Math => Self::Math(content.into()),
            TokenKind::OpenParens => Self::Parentheses(content.into()),
            TokenKind::OpenBracket => Self::TextGroup(content.into()),
            TokenKind::OpenBrace => Self::Attributes(content.into()),
            TokenKind::Substitution => Self::Substitution(content.into()),

            TokenKind::Verbatim => Self::Verbatim(content.into()),
            TokenKind::Newline => Self::Newline(content.into()),
            TokenKind::EndOfLine => Self::EndOfLine(content.into()),
            TokenKind::Whitespace => Self::Whitespace(content.into()),
            TokenKind::Plain => consume_as_plain(content),

            // These cases should never be reached
            TokenKind::UnderlineSubscript
            | TokenKind::ItalicBold
            | TokenKind::CloseParens
            | TokenKind::CloseBracket
            | TokenKind::CloseBrace => consume_as_plain(content),
        }
    }

    /// Create new [`Inline::Plain`], [`Inline::Multiple`] [`Inline::EndOfLine`] from given content
    /// depending on the given kind.
    ///
    /// # Arguments
    ///
    /// * `content` - the [`InlineContent`] put inside the created [`Inline`]
    /// * `kind` - the [`TokenKind`] used to choose one of the three options
    ///
    /// [`Inline`]: self::Inline
    /// [`Inline::Plain`]: self::Inline::Plain
    /// [`Inline::Multiple`]: self::Inline::Multiple
    /// [`Inline::EndOfLine`]: self::Inline::EndOfLine
    /// [`TokenKind`]: crate::TokenKind
    /// [`InlineContent`]: self::content::InlineContent
    pub fn as_plain_or_eol(
        content: InlineContent<PlainContent, NestedContent>,
        kind: TokenKind,
    ) -> Self {
        let consume_as_plain = |content| match content {
            InlineContent::Plain(plain_content) => Self::Plain(plain_content),
            InlineContent::Nested(nested_content) => Self::Multiple(nested_content),
        };

        match kind {
            TokenKind::EndOfLine => Self::EndOfLine(content.into()),
            _ => consume_as_plain(content),
        }
    }

    /// Create new [`Inline`] from given content depending on the given kind.
    ///
    /// # Arguments
    ///
    /// * `content` - the [`InlineContent`] put inside the created [`Inline`]
    /// * `kind` - the [`TokenKind`] used to define the kind of [`Inline`] that should be created
    /// * `span` - given [`Span`] is added to the given content
    ///
    /// [`Inline`]: self::Inline
    /// [`InlineContent`]: self::content::InlineContent
    /// [`TokenKind`]: crate::TokenKind
    /// [`Span`]: crate::TokenKind
    pub fn with_span(
        mut content: InlineContent<PlainContent, NestedContent>,
        kind: TokenKind,
        span: Span,
    ) -> Self {
        content.set_span(span);
        Self::new(content, kind)
    }

    /// Checks whether this [`Inline`] and `other` are of the same kind.
    ///
    /// [`Inline`]: self::Inline
    pub fn matches_kind(&self, other: &Inline) -> bool {
        match self {
            Inline::Bold(_) => matches!(other, Self::Bold(_)),
            Inline::Italic(_) => matches!(other, Self::Italic(_)),
            Inline::Underline(_) => matches!(other, Self::Underline(_)),
            Inline::Subscript(_) => matches!(other, Self::Subscript(_)),
            Inline::Superscript(_) => matches!(other, Self::Superscript(_)),
            Inline::Overline(_) => matches!(other, Self::Overline(_)),
            Inline::Strikethrough(_) => matches!(other, Self::Strikethrough(_)),
            Inline::Highlight(_) => matches!(other, Self::Highlight(_)),
            Inline::Verbatim(_) => matches!(other, Self::Verbatim(_)),
            Inline::Quote(_) => matches!(other, Self::Quote(_)),
            Inline::Math(_) => matches!(other, Self::Math(_)),
            Inline::Parentheses(_) => matches!(other, Self::Parentheses(_)),
            Inline::TextGroup(_) => matches!(other, Self::TextGroup(_)),
            Inline::Attributes(_) => matches!(other, Self::Attributes(_)),
            Inline::Substitution(_) => matches!(other, Self::Substitution(_)),
            Inline::Newline(_) => matches!(other, Self::Newline(_)),
            Inline::Whitespace(_) => matches!(other, Self::Whitespace(_)),
            Inline::EndOfLine(_) => matches!(other, Self::EndOfLine(_)),
            Inline::Plain(_) => matches!(other, Self::Plain(_) | Self::Multiple(_)),
            Inline::Multiple(_) => matches!(other, Self::Multiple(_) | Self::Plain(_)),
        }
    }

    /// Checks whether this [`Inline`] is a `Plain` text constructed from multiple other [`Inline`]s.
    ///
    /// [`Inline`]: self::Inline
    fn is_multiple(&self) -> bool {
        matches!(self, Inline::Multiple(_))
    }

    /// Consumes this [`Inline`] and returns the inner [`InlineContent`] of it.
    ///
    /// [`Inline`]: self::Inline
    /// [`InlineContent`]: self::InlineContent
    pub fn into_inner(self) -> InlineContent<PlainContent, NestedContent> {
        match self {
            Inline::Verbatim(plain_content)
            | Inline::Parentheses(plain_content)
            | Inline::Newline(plain_content)
            | Inline::Whitespace(plain_content)
            | Inline::EndOfLine(plain_content)
            | Inline::Plain(plain_content) => InlineContent::Plain(plain_content),

            Inline::Bold(nested_content)
            | Inline::Italic(nested_content)
            | Inline::Underline(nested_content)
            | Inline::Subscript(nested_content)
            | Inline::Superscript(nested_content)
            | Inline::Overline(nested_content)
            | Inline::Strikethrough(nested_content)
            | Inline::Highlight(nested_content)
            | Inline::Quote(nested_content)
            | Inline::Math(nested_content)
            | Inline::TextGroup(nested_content)
            | Inline::Multiple(nested_content)
            | Inline::Attributes(nested_content)
            | Inline::Substitution(nested_content) => InlineContent::Nested(nested_content),
        }
    }

    /// Returns a textual representation of this [`Inline`] as found in original input.
    ///
    /// [`Inline`]: self::Inline
    pub fn as_string(&self) -> String {
        let token_kind = TokenKind::from(self);
        let delimiters = token_kind.delimiters();

        let (begin_delim, end_delim) = delimiters.as_str();

        let delim_len = begin_delim.len() + end_delim.map(str::len).unwrap_or(0);

        let mut res = String::with_capacity(self.content_len() + delim_len);

        res.push_str(begin_delim);
        res.push_str(&self.as_ref().as_string());
        res.push_str(end_delim.unwrap_or(""));

        res
    }

    /// Returns the opening and, if available, closing [`TokenKind`] for the given [`Inline`].
    ///
    /// [`Inline`]: self::Inline
    /// [`TokenKind`]: crate::TokenKind
    pub fn delimiters(&self) -> TokenDelimiters {
        let kind = TokenKind::from(self);
        TokenDelimiters::from(&kind)
    }

    /// Returns the length of content of this [`Inline`].
    ///
    /// [`Inline`]: self::Inline
    pub fn content_len(&self) -> usize {
        match self {
            Inline::Verbatim(plain_content)
            | Inline::Parentheses(plain_content)
            | Inline::Newline(plain_content)
            | Inline::Whitespace(plain_content)
            | Inline::EndOfLine(plain_content)
            | Inline::Plain(plain_content) => plain_content.content_len(),

            Inline::Bold(nested_content)
            | Inline::Italic(nested_content)
            | Inline::Underline(nested_content)
            | Inline::Subscript(nested_content)
            | Inline::Superscript(nested_content)
            | Inline::Overline(nested_content)
            | Inline::Strikethrough(nested_content)
            | Inline::Highlight(nested_content)
            | Inline::Quote(nested_content)
            | Inline::Math(nested_content)
            | Inline::TextGroup(nested_content)
            | Inline::Multiple(nested_content)
            | Inline::Attributes(nested_content)
            | Inline::Substitution(nested_content) => nested_content.content_len(),
        }
    }

    /// Returns the [`Span`] that this [`Inline`] occupies.
    ///
    /// [`Inline`]: self::Inline
    /// [`Span`]: crate::Span
    pub fn span(&self) -> Span {
        self.as_ref().span()
    }

    /// Returns the inner content as an immutable reference.
    pub fn as_ref(&self) -> InlineContent<&PlainContent, &NestedContent> {
        match self {
            Inline::Verbatim(content)
            | Inline::Parentheses(content)
            | Inline::Newline(content)
            | Inline::Whitespace(content)
            | Inline::EndOfLine(content)
            | Inline::Plain(content) => InlineContent::Plain(content),

            Inline::Bold(content)
            | Inline::Italic(content)
            | Inline::Underline(content)
            | Inline::Subscript(content)
            | Inline::Superscript(content)
            | Inline::Overline(content)
            | Inline::Strikethrough(content)
            | Inline::Highlight(content)
            | Inline::Quote(content)
            | Inline::Math(content)
            | Inline::TextGroup(content)
            | Inline::Multiple(content)
            | Inline::Attributes(content)
            | Inline::Substitution(content) => InlineContent::Nested(content),
        }
    }

    /// Merges this [`Inline`] with another into one combined [`Inline`]. Since the other [`Inline`] might
    /// contain multiple inlines inside, some of which aren't compatible with this one, the remaining [`Inline`]s
    /// are returned in a [`VecDeque`].
    ///
    /// [`Inline`]: self::Inline
    /// [`VecDeque`]: std::collections::VecDeque
    pub(crate) fn merge(self, next_inline: Inline) -> (Inline, VecDeque<Inline>) {
        let own_kind = TokenKind::from(&self);
        let is_multiple = next_inline.is_multiple();

        let mut current_content = self.into_inner();
        let next_content = next_inline.into_inner();

        let rest_of_inlines = match next_content {
            InlineContent::Plain(plain_content) => {
                // merge plains trivially
                current_content.append(plain_content.into());
                VecDeque::default()
            }
            InlineContent::Nested(nested_inlines) => {
                let mut content = nested_inlines.content;

                while let Some(inline) = content.front() {
                    let token_kind = TokenKind::from(inline);
                    let should_append = !is_multiple || token_kind == own_kind;

                    if should_append {
                        current_content.append_inline(content.pop_front().unwrap());
                    } else {
                        break;
                    }
                }

                content
            }
        };

        let result_inline = Self::new(current_content, own_kind);
        (result_inline, rest_of_inlines)
    }
}

impl From<PlainContent> for Inline {
    fn from(content: PlainContent) -> Self {
        Self::Plain(content)
    }
}
