use std::fmt;

use icu::segmenter::{GraphemeClusterSegmenter, WordSegmenter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SymbolKind {
    Hash,
    Plain,
    Newline,
    Blankline,
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
            SymbolKind::Plain => &self.input[self.offset.start..self.offset.end],
            SymbolKind::Newline | SymbolKind::Blankline => "\n",
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

pub trait IntoSymbols<'s> {
    fn into_symbols(self) -> Vec<Symbol<'s>>;
}

impl<'s> IntoSymbols<'s> for &'s str {
    fn into_symbols(self) -> Vec<Symbol<'s>> {
        word_split(self)
    }
}

impl<'s> IntoSymbols<'s> for Vec<Symbol<'s>> {
    fn into_symbols(self) -> Vec<Symbol<'s>> {
        self
    }
}

pub fn word_split(input: &str) -> Vec<Symbol> {
    let segmenter =
        WordSegmenter::try_new_unstable(&icu_testdata::unstable()).expect("Data exists");
    let grapheme_segmenter =
        GraphemeClusterSegmenter::try_new_unstable(&icu_testdata::unstable()).expect("Data exists");

    let mut words: Vec<Symbol> = Vec::new();
    let mut curr_pos: Position = Position::default();
    let mut prev_offset = 0;

    // skip(1) to ignore break at start of input
    for offset in segmenter.segment_str(input).skip(1) {
        if let Some(word) = input.get(prev_offset..offset) {
            let kind = SymbolKind::from(word);
            let utf8_len = word.len();
            // only words > 1 byte may have different byte to grapheme count
            let grapheme_len = if utf8_len == 1 {
                1
            } else {
                // grapheme counting has huge performance impact (10x increase)
                // -1 because start of input is always a grapheme breakpoint
                grapheme_segmenter
                    .segment_str(word)
                    .collect::<Vec<usize>>()
                    .len()
                    - 1
                // 2
            };
            let end_pos = if kind == SymbolKind::Newline {
                Position {
                    line: (curr_pos.line + 1),
                    col_utf8: 0,
                    col_utf16: 0,
                    col_grapheme: 0,
                }
            } else {
                Position {
                    line: curr_pos.line,
                    col_utf8: (curr_pos.col_utf8 + utf8_len),
                    col_utf16: (curr_pos.col_utf16 + word.encode_utf16().count()),
                    col_grapheme: (curr_pos.col_grapheme + grapheme_len),
                }
            };

            let mut kind = SymbolKind::from(word);

            if curr_pos.col_grapheme == 0 && kind == SymbolKind::Newline {
                // newline at the start of line -> Blankline
                kind = SymbolKind::Blankline;
            }

            words.push(Symbol {
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
    words
}
