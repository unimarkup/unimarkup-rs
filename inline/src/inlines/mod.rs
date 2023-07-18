use std::collections::VecDeque;

use crate::{TokenDelimiters, TokenKind};

mod content;
mod substitute;

pub mod types;

use types::*;
use unimarkup_commons::scanner::span::Span;

pub use content::*;
pub use substitute::*;

/// Representation of Unimarkup inline-formatted text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inline {
    /// Bold formatted content.
    Bold(Bold),

    /// Italic formatted content.
    Italic(Italic),

    /// Underlined content.
    Underline(Underline),

    /// Content in a subscript.   
    Subscript(Subscript),

    /// Content in a superscript.
    Superscript(Superscript),

    /// Overlined content.
    Overline(Overline),

    /// Content with a strikethrough.
    Strikethrough(Strikethrough),

    /// Highlighted content.
    Highlight(Highlight),

    /// Quoted content.
    Quote(Quote),

    /// LaTeX-like math content.
    Math(Math),

    /// Content of a TextGroup `[]`.
    TextGroup(TextGroup),

    /// Unimarkup attributes for some content.
    Attributes(Attributes),

    /// Alias substitution ( i.e. `::heart::`).
    Substitution(Substitution),

    /// Wrapper without any special formatting for multiple other [`Inline`]s.
    ///
    /// [`Inline`]: self::Inline
    Multiple(Multiple),

    /// Content inside a pair of parenthesis `()`.
    Parentheses(Parentheses),

    /// Verbatim (monospaced) content.
    Verbatim(Verbatim),

    /// Explicit newline.
    Newline(Newline),

    /// Explicit whitespace.
    Whitespace(Whitespace),

    /// End of line (regular newline)
    EndOfLine(EndOfLine),

    /// Plain text without any formatting.
    Plain(Plain),
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
    // pub fn new(content: InlineContent<PlainContent, NestedContent>, kind: TokenKind) -> Self {
    //     let consume_as_plain = |content: InlineContent<PlainContent, NestedContent>| match content {
    //         InlineContent::Plain(plain_content) => Self::Plain(Plain {
    //             content: plain_content.content,
    //             span: plain_content.span,
    //         }),
    //         InlineContent::Nested(nested_content) => Self::Multiple(Multiple {
    //             content: nested_content.content,
    //             span: nested_content.span,
    //         }),
    //     };
    //
    //     match kind {
    //         TokenKind::Bold => Self::Bold(content.into()),
    //         TokenKind::Italic => Self::Italic(content.into()),
    //         TokenKind::Underline => Self::Underline(content.into()),
    //         TokenKind::Subscript => Self::Subscript(content.into()),
    //         TokenKind::Superscript => Self::Superscript(content.into()),
    //         TokenKind::Overline => Self::Overline(content.into()),
    //         TokenKind::Strikethrough => Self::Strikethrough(content.into()),
    //         TokenKind::Highlight => Self::Highlight(content.into()),
    //         TokenKind::Quote => Self::Quote(content.into()),
    //         TokenKind::Math => Self::Math(content.into()),
    //         TokenKind::OpenParens => Self::Parentheses(content.into()),
    //         TokenKind::OpenBracket => Self::TextGroup(content.into()),
    //         TokenKind::OpenBrace => Self::Attributes(content.into()),
    //         TokenKind::Substitution => Self::Substitution(content.into()),
    //
    //         TokenKind::Verbatim => Self::Verbatim(content.into()),
    //         TokenKind::Newline => Self::Newline(content.into()),
    //         TokenKind::EndOfLine => Self::EndOfLine(content.into()),
    //         TokenKind::Whitespace => Self::Whitespace(content.into()),
    //         TokenKind::Plain => consume_as_plain(content),
    //
    //         // These cases should never be reached
    //         TokenKind::UnderlineSubscript
    //         | TokenKind::ItalicBold
    //         | TokenKind::CloseParens
    //         | TokenKind::CloseBracket
    //         | TokenKind::CloseBrace => unreachable!(
    //             "Inlines parser encountered TokenKind that should have been resolved by lexer."
    //         ),
    //     }
    // }

    /// Create new [`Inline::Plain`], [`Inline::Multiple`] [`Inline::EndOfLine`] from given content
    /// depending on the given kind.
    ///
    /// # Arguments
    ///
    /// * `content` - the [`InlineContent`] put inside the created [`Inline`]
    /// * `span` - [`Span`] that is occupied by the given content
    /// * `kind` - the [`TokenKind`] used to choose one of the three options
    ///
    /// [`Inline`]: self::Inline
    /// [`Inline::Plain`]: self::Inline::Plain
    /// [`Inline::Multiple`]: self::Inline::Multiple
    /// [`Inline::EndOfLine`]: self::Inline::EndOfLine
    /// [`TokenKind`]: crate::TokenKind
    /// [`InlineContent`]: self::content::InlineContent
    pub fn plain_or_eol(content: String, span: Span, kind: TokenKind) -> Self {
        match kind {
            TokenKind::EndOfLine => Self::EndOfLine(EndOfLine { content, span }),
            _ => Self::Plain(Plain { content, span }),
        }
    }

    /// Creates a new [`Inline::Plain`] from the given content with the given [`Span`].
    pub fn plain(content: String, _kind: TokenKind, span: Span) -> Self {
        // TODO: Do we need to check the kind here?
        Self::Plain(Plain { content, span })
    }

    /// Creates a nested [`Inline`]. The [`Inline`] variant is chosen based on the [`TokenKind`]
    /// that's passed.
    pub fn nested(content: VecDeque<Inline>, kind: TokenKind) -> Self {
        let start = content
            .front()
            .map(|i| i.span().start())
            .unwrap_or_default();

        let end = content.back().map(|i| i.span().start()).unwrap_or_default();
        let span = Span::from((start, end));
        match kind {
            TokenKind::Bold => Self::Bold((content, span).into()),
            TokenKind::Italic => Self::Italic((content, span).into()),
            TokenKind::Underline => Self::Underline((content, span).into()),
            TokenKind::Subscript => Self::Subscript((content, span).into()),
            TokenKind::Superscript => Self::Superscript((content, span).into()),
            TokenKind::Overline => Self::Overline((content, span).into()),
            TokenKind::Strikethrough => Self::Strikethrough((content, span).into()),
            TokenKind::Highlight => Self::Highlight((content, span).into()),
            TokenKind::Quote => Self::Quote((content, span).into()),
            TokenKind::Math => Self::Math((content, span).into()),
            TokenKind::OpenBracket => Self::TextGroup((content, span).into()),
            TokenKind::OpenBrace => Self::Attributes((content, span).into()),
            TokenKind::Substitution => Self::Substitution((content, span).into()),

            // These cases should never be reached
            TokenKind::OpenParens
            | TokenKind::Verbatim
            | TokenKind::Newline
            | TokenKind::EndOfLine
            | TokenKind::Whitespace
            | TokenKind::Plain
            | TokenKind::UnderlineSubscript
            | TokenKind::ItalicBold
            | TokenKind::CloseParens
            | TokenKind::CloseBracket
            | TokenKind::CloseBrace => unreachable!(
                "Tried to construct nested Inline from non-nesting Token with TokenKind '{:?}'",
                kind
            ),
        }
    }

    /// Same as [`Inline::nested`] but with additional [`Span`] parameter that will be used for the
    /// [`Inline`].
    pub fn nested_with_span(content: VecDeque<Inline>, kind: TokenKind, span: Span) -> Self {
        let mut inline = Self::nested(content, kind);
        inline.set_span(span);
        inline
    }

    /// Sets the [`Span`] for this [`Inline`].
    pub fn set_span(&mut self, span: Span) {
        match self {
            Inline::Bold(inline) => inline.span = span,
            Inline::Italic(inline) => inline.span = span,
            Inline::Underline(inline) => inline.span = span,
            Inline::Subscript(inline) => inline.span = span,
            Inline::Superscript(inline) => inline.span = span,
            Inline::Overline(inline) => inline.span = span,
            Inline::Strikethrough(inline) => inline.span = span,
            Inline::Highlight(inline) => inline.span = span,
            Inline::Quote(inline) => inline.span = span,
            Inline::Math(inline) => inline.span = span,
            Inline::TextGroup(inline) => inline.span = span,
            Inline::Attributes(inline) => inline.span = span,
            Inline::Substitution(inline) => inline.span = span,
            Inline::Multiple(inline) => inline.span = span,
            Inline::Parentheses(inline) => inline.span = span,
            Inline::Verbatim(inline) => inline.span = span,
            Inline::Newline(inline) => inline.span = span,
            Inline::Whitespace(inline) => inline.span = span,
            Inline::EndOfLine(inline) => inline.span = span,
            Inline::Plain(inline) => inline.span = span,
        }
    }

    /// Checks whether this [`Inline`] and `other` are of the same kind.
    ///
    /// [`Inline`]: self::Inline
    pub fn matches_kind(&self, other: &Inline) -> bool {
        use Inline::*;

        matches!(
            (self, other),
            (Bold(_), Bold(_))
                | (Italic(_), Italic(_))
                | (Underline(_), Underline(_))
                | (Subscript(_), Subscript(_))
                | (Superscript(_), Superscript(_))
                | (Overline(_), Overline(_))
                | (Strikethrough(_), Strikethrough(_))
                | (Highlight(_), Highlight(_))
                | (Verbatim(_), Verbatim(_))
                | (Quote(_), Quote(_))
                | (Math(_), Math(_))
                | (Parentheses(_), Parentheses(_))
                | (TextGroup(_), TextGroup(_))
                | (Attributes(_), Attributes(_))
                | (Substitution(_), Substitution(_))
                | (Newline(_), Newline(_))
                | (Whitespace(_), Whitespace(_))
                | (EndOfLine(_), EndOfLine(_))
                | (Plain(_), Plain(_))
                | (Multiple(_), Multiple(_))
        )
    }

    /// Merges consecutive Inlines of same kind in a nested Inline.
    pub fn try_merge(&mut self) {
        match self {
            // Inlines containing list of inlines as contetn can merge.
            Inline::Bold(inline) => inline.try_merge(),
            Inline::Italic(inline) => inline.try_merge(),
            Inline::Underline(inline) => inline.try_merge(),
            Inline::Subscript(inline) => inline.try_merge(),
            Inline::Superscript(inline) => inline.try_merge(),
            Inline::Overline(inline) => inline.try_merge(),
            Inline::Strikethrough(inline) => inline.try_merge(),
            Inline::Highlight(inline) => inline.try_merge(),
            Inline::Quote(inline) => inline.try_merge(),
            Inline::Math(inline) => inline.try_merge(),
            Inline::TextGroup(inline) => inline.try_merge(),
            Inline::Attributes(inline) => inline.try_merge(),
            Inline::Substitution(inline) => inline.try_merge(),
            Inline::Multiple(inline) => inline.try_merge(),

            // String inlines can't merge.
            Inline::Parentheses(_)
            | Inline::Verbatim(_)
            | Inline::Newline(_)
            | Inline::Whitespace(_)
            | Inline::EndOfLine(_)
            | Inline::Plain(_) => {}
        }
    }

    /// Merges inlines of same kind.
    ///
    /// # Panics
    /// If the kinds don't match, the function will panic.
    pub(crate) fn append(&mut self, other: Inline) {
        use Inline::*;

        match (self, other) {
            (Bold(inline), Bold(other)) => inline.append(other),
            (Italic(inline), Italic(other)) => inline.append(other),
            (Underline(inline), Underline(other)) => inline.append(other),
            (Subscript(inline), Subscript(other)) => inline.append(other),
            (Superscript(inline), Superscript(other)) => inline.append(other),
            (Overline(inline), Overline(other)) => inline.append(other),
            (Strikethrough(inline), Strikethrough(other)) => inline.append(other),
            (Highlight(inline), Highlight(other)) => inline.append(other),
            (Verbatim(inline), Verbatim(other)) => inline.append(other),
            (Quote(inline), Quote(other)) => inline.append(other),
            (Math(inline), Math(other)) => inline.append(other),
            (Parentheses(inline), Parentheses(other)) => inline.append(other),
            (TextGroup(inline), TextGroup(other)) => inline.append(other),
            (Attributes(inline), Attributes(other)) => inline.append(other),
            (Substitution(inline), Substitution(other)) => inline.append(other),
            (Newline(inline), Newline(other)) => inline.append(other),
            (Whitespace(inline), Whitespace(other)) => inline.append(other),
            (EndOfLine(inline), EndOfLine(other)) => inline.append(other),
            (Plain(inline), Plain(other)) => inline.append(other),
            (Multiple(inline), Multiple(other)) => inline.append(other),
            _ => panic!("Cannot merge inlines with different kinds."),
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
        res.push_str(&String::from(self.inner()));
        res.push_str(end_delim.unwrap_or_default());

        res
    }

    /// Returns immutable reference to inner content.
    pub fn inner(&self) -> ContentRef {
        match self {
            Inline::Bold(inline) => ContentRef::Nested(inline.inner()),
            Inline::Italic(inline) => ContentRef::Nested(inline.inner()),
            Inline::Underline(inline) => ContentRef::Nested(inline.inner()),
            Inline::Subscript(inline) => ContentRef::Nested(inline.inner()),
            Inline::Superscript(inline) => ContentRef::Nested(inline.inner()),
            Inline::Overline(inline) => ContentRef::Nested(inline.inner()),
            Inline::Strikethrough(inline) => ContentRef::Nested(inline.inner()),
            Inline::Highlight(inline) => ContentRef::Nested(inline.inner()),
            Inline::Quote(inline) => ContentRef::Nested(inline.inner()),
            Inline::Math(inline) => ContentRef::Nested(inline.inner()),
            Inline::TextGroup(inline) => ContentRef::Nested(inline.inner()),
            Inline::Attributes(inline) => ContentRef::Nested(inline.inner()),
            Inline::Substitution(inline) => ContentRef::Nested(inline.inner()),
            Inline::Multiple(inline) => ContentRef::Nested(inline.inner()),

            Inline::Parentheses(inline) => ContentRef::Plain(inline.inner()),
            Inline::Verbatim(inline) => ContentRef::Plain(inline.inner()),
            Inline::Newline(inline) => ContentRef::Plain(inline.inner()),
            Inline::Whitespace(inline) => ContentRef::Plain(inline.inner()),
            Inline::EndOfLine(inline) => ContentRef::Plain(inline.inner()),
            Inline::Plain(inline) => ContentRef::Plain(inline.inner()),
        }
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
            Inline::Bold(inline) => inline.content.len(),
            Inline::Italic(inline) => inline.content.len(),
            Inline::Underline(inline) => inline.content.len(),
            Inline::Subscript(inline) => inline.content.len(),
            Inline::Superscript(inline) => inline.content.len(),
            Inline::Overline(inline) => inline.content.len(),
            Inline::Strikethrough(inline) => inline.content.len(),
            Inline::Highlight(inline) => inline.content.len(),
            Inline::Quote(inline) => inline.content.len(),
            Inline::Math(inline) => inline.content.len(),
            Inline::TextGroup(inline) => inline.content.len(),
            Inline::Attributes(inline) => inline.content.len(),
            Inline::Substitution(inline) => inline.content.len(),
            Inline::Multiple(inline) => inline.content.len(),
            Inline::Parentheses(inline) => inline.content.len(),
            Inline::Verbatim(inline) => inline.content.len(),
            Inline::Newline(inline) => inline.content.len(),
            Inline::Whitespace(inline) => inline.content.len(),
            Inline::EndOfLine(inline) => inline.content.len(),
            Inline::Plain(inline) => inline.content.len(),
        }
    }

    /// Returns the [`Span`] that this [`Inline`] occupies.
    ///
    /// [`Inline`]: self::Inline
    /// [`Span`]: unimarkup_commons::scanner::span::Span
    pub fn span(&self) -> Span {
        match self {
            Inline::Bold(inline) => inline.span,
            Inline::Italic(inline) => inline.span,
            Inline::Underline(inline) => inline.span,
            Inline::Subscript(inline) => inline.span,
            Inline::Superscript(inline) => inline.span,
            Inline::Overline(inline) => inline.span,
            Inline::Strikethrough(inline) => inline.span,
            Inline::Highlight(inline) => inline.span,
            Inline::Quote(inline) => inline.span,
            Inline::Math(inline) => inline.span,
            Inline::TextGroup(inline) => inline.span,
            Inline::Attributes(inline) => inline.span,
            Inline::Substitution(inline) => inline.span,
            Inline::Multiple(inline) => inline.span,
            Inline::Parentheses(inline) => inline.span,
            Inline::Verbatim(inline) => inline.span,
            Inline::Newline(inline) => inline.span,
            Inline::Whitespace(inline) => inline.span,
            Inline::EndOfLine(inline) => inline.span,
            Inline::Plain(inline) => inline.span,
        }
    }
}

// impl From<PlainContent> for Inline {
//     fn from(content: PlainContent) -> Self {
//         Self::Plain(Plain {
//             content: content.content,
//             span: content.span,
//         })
//     }
// }
