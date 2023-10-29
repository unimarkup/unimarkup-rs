use self::{
    formatting::{
        Bold, Highlight, Italic, Math, Overline, Quote, Strikethrough, Subscript, Superscript,
        Underline, Verbatim,
    },
    plain::{EscapedPlain, Plain},
    spaces::{EscapedNewline, EscapedWhitespace, Newline, Whitespace},
    substitution::named::NamedSubstitution,
    textbox::{hyperlink::Hyperlink, TextBox},
};

pub mod formatting;
pub mod plain;
pub mod spaces;
pub mod substitution;
pub mod textbox;

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

    /// Escaped newline.
    EscapedNewline(EscapedNewline),

    /// Regular whitespace
    Whitespace(Whitespace),

    /// Escaped whitespace.
    EscapedWhitespace(EscapedWhitespace),

    /// Plain text without any formatting.
    Plain(Plain),

    /// Escaped plain text without any formatting.
    EscapedPlain(EscapedPlain),
}

impl Inline {
    pub fn is_plain(&self) -> bool {
        matches!(self, Inline::Plain(_))
    }

    pub fn is_whitespace(&self) -> bool {
        matches!(self, Inline::Whitespace(_))
    }
}
