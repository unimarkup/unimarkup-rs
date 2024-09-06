use icu_segmenter::{GraphemeClusterBreakIteratorUtf8, GraphemeClusterSegmenter};

use super::position::{Offset, Position as SymPos};
use crate::symbol::{Symbol, SymbolKind};

pub(crate) struct SymbolStream<'segm, 'input> {
    input: &'input str,
    graph_iter: GraphemeClusterBreakIteratorUtf8<'segm, 'input>,
    curr_pos: SymPos,
    prev_offset: usize,
}

pub fn scan_str<'segm, 'input>(
    input: &'input str,
    segmenter: &'segm GraphemeClusterSegmenter,
) -> SymbolStream<'segm, 'input> {
    let mut graph_iter = segmenter.segment_str(input);
    let curr_pos: SymPos = SymPos::default();
    let prev_offset: usize = 0;

    // skip 1 to ignore break at start of input
    graph_iter.next();

    SymbolStream {
        input,
        graph_iter,
        curr_pos,
        prev_offset,
    }
}

impl<'segm, 'input> Iterator for SymbolStream<'segm, 'input> {
    type Item = Symbol<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.graph_iter.next()?;

        let grapheme = self.input.get(self.prev_offset..offset)?;

        let kind = SymbolKind::from(grapheme);

        let end_pos = if kind == SymbolKind::Newline {
            SymPos {
                line: (self.curr_pos.line + 1),
                ..Default::default()
            }
        } else {
            SymPos {
                line: self.curr_pos.line,
                col_utf8: (self.curr_pos.col_utf8 + grapheme.len()),
                col_utf16: (self.curr_pos.col_utf16 + grapheme.encode_utf16().count()),
                col_grapheme: (self.curr_pos.col_grapheme + 1),
            }
        };

        let prev_offset = self.prev_offset;
        let curr_pos = self.curr_pos;

        self.curr_pos = end_pos;
        self.prev_offset = offset;

        Some(Symbol {
            input: self.input,
            kind,
            offset: Offset {
                start: prev_offset,
                end: offset,
            },
            start: curr_pos,
            end: end_pos,
        })
    }
}
