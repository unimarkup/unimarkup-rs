//! Functionality, iterators, helper types and traits to get [`Symbol`]s from `&str`.
//! These [`Symbol`]s and iterators are used to convert the input into a Unimarkup document.

use icu_segmenter::GraphemeClusterSegmenter;

pub mod position;
pub mod span;
mod symbol;

use position::{Offset, Position as SymPos};
pub use symbol::{iterator::*, Symbol, SymbolKind};

/// Scans given input and returns vector of [`Symbol`]s needed to convert the input to Unimarkup content.
pub fn scan_str(input: &str) -> Vec<Symbol<'_>> {
    let segmenter = GraphemeClusterSegmenter::new();

    let mut symbols: Vec<Symbol> = Vec::new();
    let mut curr_pos: SymPos = SymPos::default();
    let mut prev_offset = 0;

    // skip(1) to ignore break at start of input
    for offset in segmenter.segment_str(input).skip(1) {
        if let Some(grapheme) = input.get(prev_offset..offset) {
            let kind = SymbolKind::from(grapheme);

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
