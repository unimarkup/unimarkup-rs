use unimarkup_commons::lexer::{position::Position, span::Span};

use self::{
    base::{EscapedNewline, EscapedPlain, EscapedWhitespace, Newline, Plain},
    formatting::{
        Bold, Highlight, Italic, Math, Overline, Quote, Strikethrough, Subscript, Superscript,
        Underline, Verbatim,
    },
    substitution::{named::NamedSubstitution, DirectUri},
    textbox::{hyperlink::Hyperlink, TextBox},
};

mod helper;

pub mod base;
pub mod formatting;
pub mod substitution;
pub mod textbox;

pub trait InlineElement {
    fn to_plain_string(&self) -> String;
    fn start(&self) -> Position;
    fn end(&self) -> Position;
    fn span(&self) -> Span {
        Span {
            start: self.start(),
            end: self.end(),
        }
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

    DirectUri(DirectUri),
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
            Inline::Verbatim(_) => "Verbatim",
            Inline::Newline(_) => "Newline",
            Inline::ImplicitNewline(_) => "ImplicitNewline",
            Inline::EscapedNewline(_) => "EscapedNewline",
            Inline::EscapedWhitespace(_) => "EscapedWhitespace",
            Inline::Plain(_) => "Plain",
            Inline::EscapedPlain(_) => "EscapedPlain",
            Inline::DirectUri(_) => "DirectUri",
            Inline::NamedSubstitution(_) => "NamedSubstitution",
        }
    }
}

impl InlineElement for Inline {
    fn to_plain_string(&self) -> String {
        match self {
            Inline::Bold(inline) => inline.to_plain_string(),
            Inline::Italic(inline) => inline.to_plain_string(),
            Inline::Underline(inline) => inline.to_plain_string(),
            Inline::Subscript(inline) => inline.to_plain_string(),
            Inline::Superscript(inline) => inline.to_plain_string(),
            Inline::Overline(inline) => inline.to_plain_string(),
            Inline::Strikethrough(inline) => inline.to_plain_string(),
            Inline::Highlight(inline) => inline.to_plain_string(),
            Inline::Quote(inline) => inline.to_plain_string(),
            Inline::Math(inline) => inline.to_plain_string(),
            Inline::TextBox(inline) => inline.to_plain_string(),
            Inline::Hyperlink(inline) => inline.to_plain_string(),
            Inline::Verbatim(inline) => inline.to_plain_string(),
            Inline::Newline(inline) => inline.to_plain_string(),
            Inline::ImplicitNewline(inline) => inline.to_plain_string(),
            Inline::EscapedNewline(inline) => inline.to_plain_string(),
            Inline::EscapedWhitespace(inline) => inline.to_plain_string(),
            Inline::Plain(inline) => inline.to_plain_string(),
            Inline::EscapedPlain(inline) => inline.to_plain_string(),
            Inline::DirectUri(inline) => inline.to_plain_string(),

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
            Inline::Verbatim(inline) => inline.start(),
            Inline::Newline(inline) => inline.start(),
            Inline::ImplicitNewline(inline) => inline.start(),
            Inline::EscapedNewline(inline) => inline.start(),
            Inline::EscapedWhitespace(inline) => inline.start(),
            Inline::Plain(inline) => inline.start(),
            Inline::EscapedPlain(inline) => inline.start(),
            Inline::DirectUri(inline) => inline.start(),

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
            Inline::Verbatim(inline) => inline.end(),
            Inline::Newline(inline) => inline.end(),
            Inline::ImplicitNewline(inline) => inline.end(),
            Inline::EscapedNewline(inline) => inline.end(),
            Inline::EscapedWhitespace(inline) => inline.end(),
            Inline::Plain(inline) => inline.end(),
            Inline::EscapedPlain(inline) => inline.end(),
            Inline::DirectUri(inline) => inline.end(),

            Inline::NamedSubstitution(_) => todo!(),
        }
    }
}

impl InlineElement for Vec<Inline> {
    fn to_plain_string(&self) -> String {
        self.iter().fold(String::default(), |mut combined, inline| {
            combined.push_str(&inline.to_plain_string());
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
