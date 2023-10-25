use unimarkup_commons::parser::Parser;

use self::{
    formatting::{
        bold_italic::{Bold, Italic},
        highlight::Highlight,
        math::Math,
        overline::Overline,
        quote::Quote,
        strikethrough::Strikethrough,
        superscript::Superscript,
        underline_subscript::{Subscript, Underline},
        verbatim::Verbatim,
    },
    multiple::Multiple,
    plain::{EscapedPlain, Plain},
    spaces::{EscapedNewline, EscapedWhitespace, Newline, Whitespace},
    substitution::named::NamedSubstitution,
    textbox::{hyperlink::Hyperlink, TextBox},
};

pub mod formatting;
pub mod multiple;
pub mod plain;
pub mod spaces;
pub mod substitution;
pub mod textbox;

pub trait InlineElement: Parser<Inline> + Into<Inline> + TryFrom<Inline> {}

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

    /// Wrapper without any special formatting for multiple other [`Inline`]s.
    ///
    /// [`Inline`]: self::Inline
    Multiple(Multiple),

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
        match self {
            Inline::Plain(_) => true,
            Inline::Multiple(multiple) => {
                multiple.inner.last().map_or(false, |last| last.is_plain())
            }
            _ => false,
        }
    }

    pub fn merge_plain(&mut self, plain: Plain) -> Result<(), InlineError> {
        match self {
            Inline::Plain(self_plain) => {
                self_plain.content.push_str(&plain.content);
                Ok(())
            }
            Inline::Multiple(multiple) => match multiple.inner.last_mut() {
                Some(last) => last.merge_plain(plain),
                None => Err(InlineError::MergeMismatch),
            },
            _ => Err(InlineError::MergeMismatch),
        }
    }

    pub fn is_whitespace(&self) -> bool {
        match self {
            Inline::Whitespace(_) => true,
            Inline::Multiple(multiple) => multiple
                .inner
                .last()
                .map_or(false, |last| last.is_whitespace()),
            _ => false,
        }
    }

    pub fn merge_whitespace(&mut self, _whitespace: Whitespace) -> Result<(), InlineError> {
        match self {
            // TODO: update spans here
            Inline::Whitespace(_) => Ok(()),
            Inline::Multiple(multiple) => match multiple.inner.last_mut() {
                Some(last) => last.merge_whitespace(_whitespace),
                None => Err(InlineError::MergeMismatch),
            },
            _ => Err(InlineError::MergeMismatch),
        }
    }
}

#[derive(Debug)]
pub enum InlineError {
    MergeMismatch,
    ConversionMismatch,
}
