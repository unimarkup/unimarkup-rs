//! Scanner and helper types and traits for structurization of Unimarkup input.

pub mod position;
pub mod span;
mod symbol;

use icu::segmenter::{GraphemeClusterSegmenter, SegmenterError};
use icu_provider_adapters::fallback::LocaleFallbackProvider;
use position::{Offset, Position};
pub use symbol::{Symbol, SymbolKind};

#[derive(Debug, Clone)]
struct IcuDataProvider;
// Generated using: `icu4x-datagen --keys-for-bin .\target\debug\unimarkup.exe --locales full --format mod --out .\commons\src\scanner\icu_data`
// Note: Run `cargo build` before re-generating the data to ensure the newest binary is inspected by icu4x-datagen.
include!("./icu_data/mod.rs");
impl_data_provider!(IcuDataProvider);

#[derive(Debug)]
pub struct Scanner {
    provider: LocaleFallbackProvider<IcuDataProvider>,
    segmenter: GraphemeClusterSegmenter,
}

impl Clone for Scanner {
    fn clone(&self) -> Self {
        let segmenter = GraphemeClusterSegmenter::try_new_unstable(&self.provider)
            .expect("Provider is valid at this point.");

        Self {
            provider: self.provider.clone(),
            segmenter,
        }
    }
}

impl Scanner {
    pub fn try_new() -> Result<Self, SegmenterError> {
        let icu_data_provider = IcuDataProvider;
        let fallback_provider = LocaleFallbackProvider::try_new_unstable(icu_data_provider)?;
        let segmenter = GraphemeClusterSegmenter::try_new_unstable(&fallback_provider)?;

        Ok(Self {
            provider: fallback_provider,
            segmenter,
        })
    }

    pub fn scan_str<'s>(&self, input: &'s str) -> Vec<Symbol<'s>> {
        let mut symbols: Vec<Symbol> = Vec::new();
        let mut curr_pos: Position = Position::default();
        let mut prev_offset = 0;

        // skip(1) to ignore break at start of input
        for offset in self.segmenter.segment_str(input).skip(1) {
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
}
