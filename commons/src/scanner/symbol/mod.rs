//! Symbol and helper types for structurization of Unimarkup input.

use core::fmt;

use super::position::{Offset, Position};

pub mod iterator;

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
    /// End of Unimarkup document
    EOI,
    /// The backslash (`\`) is used for escaping other symbols.
    Backslash,
    /// The star (`*`) literal is used for various elements.
    Star,
    /// The minus (`-`) literal is used for various elements.
    Minus,
    /// The plus (`+`) literal is used for various elements.
    Plus,
    /// The underline (`_`) literal is used for underline and/or subscript formatting.
    Underline,
    /// The caret (`^`) literal is used for superscript formatting.
    Caret,
    /// The tick (`` ` ``) literal is used for verbatim blocks and formatting.
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
    /// A colon literal (`:`) is used as marker (e.g. for alias substitutions `::heart::`).
    Colon,
}

impl Default for SymbolKind {
    fn default() -> Self {
        Self::Plain
    }
}

impl SymbolKind {
    pub fn is_not_keyword(&self) -> bool {
        matches!(
            self,
            SymbolKind::Newline | SymbolKind::Whitespace | SymbolKind::Plain | SymbolKind::EOI
        )
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
        self.kind.is_not_keyword()
    }

    /// Returns the original string representation of the symbol.
    pub fn as_str(&self) -> &str {
        match self.kind {
            SymbolKind::Plain => &self.input[self.offset.start..self.offset.end],
            SymbolKind::Whitespace => &self.input[self.offset.start..self.offset.end],
            _ => self.kind.as_str(),
        }
    }

    /// Flattens the input of consecutive symbols. Returns the slice of input starting from start
    /// position of first symbol until the end of last symbol. Returns [`None`] if slice is empty.
    ///
    /// # Panics
    ///
    /// It's assumed that all [`Symbol`]s in slice reference the same input. If not, the function
    /// might panic (guaranteed in debug) if inputs are not the same and last [`Symbol`] in slice
    /// references input that is longer than the one referenced in the first [`Symbol`].
    ///
    /// # Examples
    ///
    /// ```
    /// use unimarkup_commons::scanner::{scan_str, Symbol};
    ///
    /// let input = "This is a string";
    /// let symbols: Vec<_> = scan_str(input);
    ///
    /// assert_eq!(input, Symbol::flatten(&symbols).unwrap());
    /// ```
    pub fn flatten(symbols: &[Self]) -> Option<&str> {
        let (first, last) = (symbols.first()?, symbols.last()?);

        debug_assert_eq!(first.input, last.input);

        let input = first.input;

        let start = first.offset.start;
        let end = last.offset.end;

        Some(&input[start..end])
    }

    /// Flattens the iterator of consecutive symbols. Returns the slice of input starting from start
    /// position of first symbol until the end of last symbol.
    ///
    /// It is assumed (and checked in debug release) that the symbols are in contiguous order.
    ///
    /// Returns `None` if the referenced input is not same in all symbols.
    pub fn flatten_iter<'s>(mut iter: impl Iterator<Item = &'s Symbol<'s>>) -> Option<&'s str> {
        let first = iter.next()?;

        #[cfg(debug_assertions)]
        let last = std::iter::once(first).chain(iter).reduce(|prev, curr| {
            debug_assert!(prev.end.col_grapheme == curr.start.col_grapheme);
            curr
        })?;

        #[cfg(not(debug_assertions))]
        let last = iter.last().unwrap_or(first);

        let input = first.input;

        let start = first.offset.start;
        let end = last.offset.end;

        Some(&input[start..end])
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
            "-" => SymbolKind::Minus,
            "+" => SymbolKind::Plus,
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

impl SymbolKind {
    pub fn as_str(&self) -> &str {
        match self {
            SymbolKind::Hash => "#",
            SymbolKind::Plain => "",
            SymbolKind::Tick => "`",
            SymbolKind::Whitespace => " ",
            SymbolKind::Newline => "\n",
            SymbolKind::EOI => "",
            SymbolKind::Backslash => "\\",
            SymbolKind::Star => "*",
            SymbolKind::Minus => "-",
            SymbolKind::Plus => "+",
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
}
