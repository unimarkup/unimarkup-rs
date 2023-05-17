//! Symbol and helper types and traits for structurization of Unimarkup input.

pub mod position;

use icu::segmenter::GraphemeClusterSegmenter;
use position::{Offset, Position};
use std::fmt;

/// Possible kinds of Symbol found in Unimarkup document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SymbolKind {
    /// Hash symbol (#) used for headings
    Hash,
    /// Regular text with no semantic meaning
    Plain,
    /// Any non-linebreaking whitespace
    Whitespace,
    /// A line break literal (for example `\n` or '\r\n')
    Newline,
    /// Empty line, can be separator between blocks
    Blankline,
    /// End of Unimarkup document
    EOI,

    /// The backslash (`\`) is used for escaping other symbols.
    Backslash,
    /// The start (`*`) literal is used for bold and/or italic formatting.
    Star,
    /// The underline (`_`) literal is used for underline and/or subscript formatting.
    Underline,
    /// The caret (`^`) literal is used for superscript formatting.
    Caret,
    /// The tick (```) literal is used for verbatim blocks and formatting.
    Tick,
    /// The overline (`‾`) literal is used for overline formatting.
    Overline,
    /// The pipe (`|`) literal is used for highlight formatting.
    Pipe,
    /// The tilde (`~`) literal is used for strikethrough formatting.
    Tilde,
    /// The quote (`"`) literal is used for quotation formatting.
    Quote,
    /// The dollar (`$`) literal is used for math mode formatting.
    Dollar,
    /// The open parentheses (`(`) literal is used for additional data to text group elements (e.g.
    /// image insert).
    OpenParenthesis,
    /// The close parentheses (`)`) literal is used to close the additional data to text group.
    CloseParenthesis,
    /// The open bracket (`[`) literal is used for text group elements.
    OpenBracket,
    /// The close bracket (`]`) literal is used for text group elements.
    CloseBracket,
    /// The open brace (`{`) literal is used for inline attributes.
    OpenBrace,
    /// The close brace (`}`) literal is used for inline attributes.
    CloseBrace,
    /// A colon literal used for alias substitutions (`::heart::`).
    Colon,
}

impl Default for SymbolKind {
    fn default() -> Self {
        Self::Plain
    }
}

/// Symbol representation of literals found in Unimarkup document.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol<'a> {
    /// Original input the symbol is found in.
    pub input: &'a str,
    pub(crate) offset: Offset,
    /// Kind of the symbol, e.g. a hash (#)
    pub kind: SymbolKind,
    /// Start position of the symbol in input.
    pub start: Position,
    /// End position of the symbol in input.
    pub end: Position,
}

impl fmt::Debug for Symbol<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let input = if self.input.len() < 100 {
            String::from(self.input)
        } else {
            format!("{}...", &self.input[0..100])
        };

        let output = {
            let start = self.offset.start;
            let end = self.offset.end;
            &self.input[start..end]
        };

        f.debug_struct("Symbol")
            .field("input", &input)
            .field("output", &output)
            .field("offset", &self.offset)
            .field("kind", &self.kind)
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

impl Symbol<'_> {
    // TODO: extension trait in core?
    pub fn is_not_keyword(&self) -> bool {
        matches!(
            self.kind,
            SymbolKind::Newline
                | SymbolKind::Whitespace
                | SymbolKind::Plain
                | SymbolKind::Blankline
                | SymbolKind::EOI
        )
    }

    /// Returns the original string representation of the symbol.
    pub fn as_str(&self) -> &str {
        match self.kind {
            SymbolKind::Hash => "#",
            SymbolKind::Plain => &self.input[self.offset.start..self.offset.end],
            SymbolKind::Tick => "`",
            SymbolKind::Whitespace => &self.input[self.offset.start..self.offset.end],
            SymbolKind::Newline | SymbolKind::Blankline => "\n",
            SymbolKind::EOI => "",
            SymbolKind::Backslash => "\\",
            SymbolKind::Star => "*",
            SymbolKind::Underline => "_",
            SymbolKind::Caret => "^",
            SymbolKind::Overline => "‾",
            SymbolKind::Pipe => "|",
            SymbolKind::Tilde => "~",
            SymbolKind::Quote => "\"",
            SymbolKind::Dollar => "$",
            SymbolKind::OpenParenthesis => "(",
            SymbolKind::CloseParenthesis => ")",
            SymbolKind::OpenBracket => "[",
            SymbolKind::CloseBracket => "]",
            SymbolKind::OpenBrace => "{",
            SymbolKind::CloseBrace => "}",
            SymbolKind::Colon => ":",
        }
    }

    /// Flattens the input of consecutive symbols. Returns the slice of input starting from start
    /// position of first symbol until the end of last symbol.
    ///
    /// Note: The input must be same in all symbols!
    pub fn flatten(symbols: &[Self]) -> &str {
        debug_assert!(symbols
            .windows(2)
            .all(|window| window[0].input == window[1].input));

        if symbols.is_empty() {
            return "";
        }

        let first = symbols.first().unwrap();
        let last = symbols.last().unwrap();
        let input = first.input;

        let start = first.offset.start;
        let end = last.offset.end;

        &input[start..end]
    }
}

impl From<&str> for SymbolKind {
    fn from(value: &str) -> Self {
        match value {
            "#" => SymbolKind::Hash,
            "\n" | "\r\n" => SymbolKind::Newline,
            "`" => SymbolKind::Tick,
            "\\" => SymbolKind::Backslash,
            "*" => SymbolKind::Star,
            "_" => SymbolKind::Underline,
            "^" => SymbolKind::Caret,
            "‾" => SymbolKind::Overline,
            "|" => SymbolKind::Pipe,
            "~" => SymbolKind::Tilde,
            "\"" => SymbolKind::Quote,
            "$" => SymbolKind::Dollar,
            "(" => SymbolKind::OpenParenthesis,
            ")" => SymbolKind::CloseParenthesis,
            "[" => SymbolKind::OpenBracket,
            "]" => SymbolKind::CloseBracket,
            "{" => SymbolKind::OpenBrace,
            "}" => SymbolKind::CloseBrace,
            ":" => SymbolKind::Colon,
            symbol
                if symbol != "\n"
                    && symbol != "\r\n"
                    && symbol.starts_with(char::is_whitespace) =>
            {
                SymbolKind::Whitespace
            }
            _ => SymbolKind::Plain,
        }
    }
}

/// Trait for conversion of input into Unimarkup symbols.
pub trait IntoSymbols<'s> {
    type Output;

    /// Converts input into collection of Unimarkup symbols.
    fn into_symbols(self) -> Self::Output;
}

impl<'s> IntoSymbols<'s> for &'s str {
    type Output = Vec<Symbol<'s>>;

    fn into_symbols(self) -> Self::Output {
        grapheme_split(self)
    }
}

impl<'s> IntoSymbols<'s> for Vec<Symbol<'s>> {
    type Output = Vec<Symbol<'s>>;

    fn into_symbols(self) -> Self::Output {
        self
    }
}

impl<'s> IntoSymbols<'s> for &'s Vec<Symbol<'s>> {
    type Output = &'s [Symbol<'s>];

    fn into_symbols(self) -> Self::Output {
        self
    }
}

impl<'s> IntoSymbols<'s> for &'s [Symbol<'s>] {
    type Output = &'s [Symbol<'s>];

    fn into_symbols(self) -> Self::Output {
        self
    }
}

// TODO: pass locale from Config to this function.
fn grapheme_split(input: &str) -> Vec<Symbol> {
    let segmenter =
        GraphemeClusterSegmenter::try_new_unstable(&icu_testdata::unstable()).expect("Data exists");

    let mut symbols: Vec<Symbol> = Vec::new();
    let mut curr_pos: Position = Position::default();
    let mut prev_offset = 0;

    // skip(1) to ignore break at start of input
    for offset in segmenter.segment_str(input).skip(1) {
        if let Some(grapheme) = input.get(prev_offset..offset) {
            let kind = SymbolKind::from(grapheme);
            let grapheme_len = 1;

            let end_pos = if kind == SymbolKind::Newline {
                Position {
                    line: (curr_pos.line + 1),
                    ..Default::default()
                }
            } else {
                Position {
                    line: curr_pos.line,
                    col_utf8: (curr_pos.col_utf8 + grapheme.len()),
                    col_utf16: (curr_pos.col_utf16 + grapheme.encode_utf16().count()),
                    col_grapheme: (curr_pos.col_grapheme + grapheme_len),
                }
            };

            let mut kind = SymbolKind::from(grapheme);

            if curr_pos.col_utf8 == 1 && kind == SymbolKind::Newline {
                // newline at the start of line -> Blankline
                kind = SymbolKind::Blankline;
            }

            symbols.push(Symbol {
                input,
                kind,
                offset: Offset {
                    start: prev_offset,
                    end: offset,
                },
                start: curr_pos,
                end: end_pos,
            });

            curr_pos = end_pos;
        }
        prev_offset = offset;
    }

    // last offset not needed, because break at EOI is always available
    symbols
}
