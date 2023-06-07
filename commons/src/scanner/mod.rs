//! Scanner and helper types and traits for structurization of Unimarkup input.

pub mod position;
pub mod span;
mod symbol;

use icu::segmenter::GraphemeClusterSegmenter;
use position::{Offset, Position};
pub use symbol::{Symbol, SymbolKind};

/// Trait for conversion of input into Unimarkup symbols.
pub trait IntoSymbols<'s> {
    type Output: AsRef<[Symbol<'s>]>;

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
            let mut kind = SymbolKind::from(grapheme);

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

    // last offset not needed, because break at EOI is always available
    symbols
}
