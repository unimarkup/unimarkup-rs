//! Scanner and helper types and traits for structurization of Unimarkup input.

use icu_segmenter::GraphemeClusterSegmenter;

pub mod position;
pub mod span;
mod symbol;

use position::{Offset, Position as SymPos};
pub use symbol::{iterator::*, Symbol, SymbolKind};

#[derive(Debug)]
pub struct Scanner {
    segmenter: GraphemeClusterSegmenter,
}

impl Clone for Scanner {
    fn clone(&self) -> Self {
        Scanner::new()
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}

impl Scanner {
    pub fn new() -> Self {
        let segmenter = GraphemeClusterSegmenter::new();

        Self { segmenter }
    }

    pub fn scan_str<'s>(&self, input: &'s str) -> Vec<Symbol<'s>> {
        let mut symbols: Vec<Symbol> = Vec::new();
        let mut curr_pos: SymPos = SymPos::default();
        let mut prev_offset = 0;

        // skip(1) to ignore break at start of input
        for offset in self.segmenter.segment_str(input).skip(1) {
            if let Some(grapheme) = input.get(prev_offset..offset) {
                let mut kind = SymbolKind::from(grapheme);

                let end_pos = if kind == SymbolKind::Newline {
                    SymPos {
                        line: (curr_pos.line + 1),
                        ..Default::default()
                    }
                } else {
                    SymPos {
                        line: curr_pos.line,
                        col_utf8: (curr_pos.col_utf8 + grapheme.len()),
                        col_utf16: (curr_pos.col_utf16 + grapheme.encode_utf16().count()),
                        col_grapheme: (curr_pos.col_grapheme + 1),
                    }
                };

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

        symbols.push(Symbol {
            input,
            kind: SymbolKind::EOI,
            offset: Offset {
                start: prev_offset,
                end: prev_offset,
            },
            start: curr_pos,
            end: curr_pos,
        });

        // last offset not needed, because break at EOI is always available
        symbols
    }
}
