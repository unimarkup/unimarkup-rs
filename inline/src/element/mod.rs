//! Contains all Unimarkup [`Inline`] elements.

use unimarkup_commons::{
    lexer::{position::Position, span::Span},
    parsing::Element,
};

use self::{
    base::{EscapedNewline, EscapedPlain, EscapedWhitespace, Newline, Plain},
    formatting::{
        Bold, Highlight, Italic, Math, Overline, Quote, Strikethrough, Subscript, Superscript,
        Underline, Verbatim, Cite
    },
    substitution::{named::NamedSubstitution, DirectUri, ImplicitSubstitution},
    textbox::{citation::Citation, hyperlink::Hyperlink, TextBox},
};

mod helper;

pub mod base;
pub mod formatting;
pub mod substitution;
pub mod textbox;

// Needed to implement the [`Element`] trait for Vec<Inline> in this crate
pub trait InlineElement {
    /// Shows the element in its original plain markup form.
    fn as_unimarkup(&self) -> String;
    /// Return the start of the element in the original content.
    fn start(&self) -> Position;
    /// Return the end of the element in the original content.
    fn end(&self) -> Position;
    /// The span of an element in the original content.
    fn span(&self) -> Span {
        Span {
            start: self.start(),
            end: self.end(),
        }
    }
}

impl Element for dyn InlineElement {
    fn as_unimarkup(&self) -> String {
        self.as_unimarkup()
    }

    fn start(&self) -> Position {
        self.start()
    }

    fn end(&self) -> Position {
        self.end()
    }
}

/// Supported Unimarkup inline elements.
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

    /// Mathematical content.
    Math(Math),

    /// Content of a TextBox `[]`.
    TextBox(TextBox),

    /// Hyperlink element `[link text](url)`.
    Hyperlink(Hyperlink),

    /// Citation element `[&&cite-id]`
    Citation(Citation),

    /// Named substitution ( i.e. `::heart::`).
    NamedSubstitution(NamedSubstitution),

    /// Verbatim (monospaced) content.
    Verbatim(Verbatim),

    /// End of line (regular newline)
    Newline(Newline),

    /// Implicit newline in case newlines should be kept, but it was not manually escaped.
    /// e.g. in verbatim blocks
    ImplicitNewline(Newline),

    /// Escaped newline.
    EscapedNewline(EscapedNewline),

    /// Escaped whitespace.
    EscapedWhitespace(EscapedWhitespace),

    /// Plain text without any formatting.
    Plain(Plain),

    /// Escaped plain text without any formatting.
    EscapedPlain(EscapedPlain),

    /// Implicit substitutions like emojis and arrows.
    ImplicitSubstitution(ImplicitSubstitution),

    /// Direct URI
    DirectUri(DirectUri),

    /// Distinct reference
    Cite(Cite)
}

impl Inline {
    pub fn is_plain(&self) -> bool {
        matches!(self, Inline::Plain(_))
    }

    pub fn variant_str(&self) -> &'static str {
        match self {
            Inline::Bold(_) => "Bold",
            Inline::Italic(_) => "Italic",
            Inline::Underline(_) => "Underline",
            Inline::Subscript(_) => "Subscript",
            Inline::Superscript(_) => "Superscript",
            Inline::Overline(_) => "Overline",
            Inline::Strikethrough(_) => "Strikethrough",
            Inline::Highlight(_) => "Highlight",
            Inline::Quote(_) => "Quote",
            Inline::Math(_) => "Math",
            Inline::TextBox(_) => "TextBox",
            Inline::Hyperlink(_) => "Hyperlink",
            Inline::Citation(_) => "Citation",
            Inline::Verbatim(_) => "Verbatim",
            Inline::Newline(_) => "Newline",
            Inline::ImplicitNewline(_) => "ImplicitNewline",
            Inline::EscapedNewline(_) => "EscapedNewline",
            Inline::EscapedWhitespace(_) => "EscapedWhitespace",
            Inline::Plain(_) => "Plain",
            Inline::EscapedPlain(_) => "EscapedPlain",
            Inline::DirectUri(_) => "DirectUri",
            Inline::NamedSubstitution(_) => "NamedSubstitution",
            Inline::ImplicitSubstitution(_) => "ImplicitSubstitution",
            Inline::Cite(_) => "Cite",
        }
    }
}

impl InlineElement for Inline {
    fn as_unimarkup(&self) -> String {
        match self {
            Inline::Bold(inline) => inline.as_unimarkup(),
            Inline::Italic(inline) => inline.as_unimarkup(),
            Inline::Underline(inline) => inline.as_unimarkup(),
            Inline::Subscript(inline) => inline.as_unimarkup(),
            Inline::Superscript(inline) => inline.as_unimarkup(),
            Inline::Overline(inline) => inline.as_unimarkup(),
            Inline::Strikethrough(inline) => inline.as_unimarkup(),
            Inline::Highlight(inline) => inline.as_unimarkup(),
            Inline::Quote(inline) => inline.as_unimarkup(),
            Inline::Math(inline) => inline.as_unimarkup(),
            Inline::TextBox(inline) => inline.as_unimarkup(),
            Inline::Hyperlink(inline) => inline.as_unimarkup(),
            Inline::Citation(inline) => inline.as_unimarkup(),
            Inline::Verbatim(inline) => inline.as_unimarkup(),
            Inline::Newline(inline) => inline.as_unimarkup(),
            Inline::ImplicitNewline(inline) => inline.as_unimarkup(),
            Inline::EscapedNewline(inline) => inline.as_unimarkup(),
            Inline::EscapedWhitespace(inline) => inline.as_unimarkup(),
            Inline::Plain(inline) => inline.as_unimarkup(),
            Inline::EscapedPlain(inline) => inline.as_unimarkup(),
            Inline::DirectUri(inline) => inline.as_unimarkup(),
            Inline::ImplicitSubstitution(inline) => inline.as_unimarkup(),
            Inline::Cite(inline) => inline.as_unimarkup(),

            Inline::NamedSubstitution(_) => todo!(),
        }
    }

    fn start(&self) -> Position {
        match self {
            Inline::Bold(inline) => inline.start(),
            Inline::Italic(inline) => inline.start(),
            Inline::Underline(inline) => inline.start(),
            Inline::Subscript(inline) => inline.start(),
            Inline::Superscript(inline) => inline.start(),
            Inline::Overline(inline) => inline.start(),
            Inline::Strikethrough(inline) => inline.start(),
            Inline::Highlight(inline) => inline.start(),
            Inline::Quote(inline) => inline.start(),
            Inline::Math(inline) => inline.start(),
            Inline::TextBox(inline) => inline.start(),
            Inline::Hyperlink(inline) => inline.start(),
            Inline::Citation(inline) => inline.start(),
            Inline::Verbatim(inline) => inline.start(),
            Inline::Newline(inline) => inline.start(),
            Inline::ImplicitNewline(inline) => inline.start(),
            Inline::EscapedNewline(inline) => inline.start(),
            Inline::EscapedWhitespace(inline) => inline.start(),
            Inline::Plain(inline) => inline.start(),
            Inline::EscapedPlain(inline) => inline.start(),
            Inline::DirectUri(inline) => inline.start(),
            Inline::ImplicitSubstitution(inline) => inline.start(),
            Inline::Cite(inline) => inline.start(),

            Inline::NamedSubstitution(_) => todo!(),
        }
    }

    fn end(&self) -> Position {
        match self {
            Inline::Bold(inline) => inline.end(),
            Inline::Italic(inline) => inline.end(),
            Inline::Underline(inline) => inline.end(),
            Inline::Subscript(inline) => inline.end(),
            Inline::Superscript(inline) => inline.end(),
            Inline::Overline(inline) => inline.end(),
            Inline::Strikethrough(inline) => inline.end(),
            Inline::Highlight(inline) => inline.end(),
            Inline::Quote(inline) => inline.end(),
            Inline::Math(inline) => inline.end(),
            Inline::TextBox(inline) => inline.end(),
            Inline::Hyperlink(inline) => inline.end(),
            Inline::Citation(inline) => inline.end(),
            Inline::Verbatim(inline) => inline.end(),
            Inline::Newline(inline) => inline.end(),
            Inline::ImplicitNewline(inline) => inline.end(),
            Inline::EscapedNewline(inline) => inline.end(),
            Inline::EscapedWhitespace(inline) => inline.end(),
            Inline::Plain(inline) => inline.end(),
            Inline::EscapedPlain(inline) => inline.end(),
            Inline::DirectUri(inline) => inline.end(),
            Inline::ImplicitSubstitution(inline) => inline.end(),
            Inline::Cite(inline) => inline.end(),

            Inline::NamedSubstitution(_) => todo!(),
        }
    }
}

impl InlineElement for Vec<Inline> {
    fn as_unimarkup(&self) -> String {
        self.iter().fold(String::default(), |mut combined, inline| {
            combined.push_str(&inline.as_unimarkup());
            combined
        })
    }

    fn start(&self) -> Position {
        match self.first() {
            Some(first) => first.start(),
            None => Position::default(),
        }
    }

    fn end(&self) -> Position {
        match self.last() {
            Some(last) => last.end(),
            None => Position::default(),
        }
    }
}
