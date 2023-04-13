use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SymbolKind {
    Hash,
    Plain,
    Newline,
}

impl Default for SymbolKind {
    fn default() -> Self {
        Self::Plain
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub line: usize,
    pub col_utf8: usize,
    pub col_utf16: usize,
    pub col_grapheme: usize,
}

// Note: start inclusive, end exclusive
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Offset {
    pub start: usize,
    pub end: usize,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol<'a> {
    pub input: &'a str,
    pub offset: Offset,
    pub kind: SymbolKind,
    pub start: Position,
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
    pub fn as_str(&self) -> &str {
        match self.kind {
            SymbolKind::Hash => "#",
            SymbolKind::Newline => "\n",
            SymbolKind::Plain => &self.input[self.offset.start..self.offset.end],
        }
    }

    pub fn flatten(symbols: &[Self]) -> &str {
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
            _ => SymbolKind::Plain,
        }
    }
}
