use std::{iter::Peekable, str::Bytes};

use super::position::Span as SymPos;
use crate::symbol::{Symbol, SymbolKind};

/// Iterator of Unimarkup [`Symbol`]s over a given input.
pub(crate) struct SymbolStream<'input> {
    /// The input from which the Unimarkup symbols are to be scanned.
    input: &'input str,

    /// The bytes representation of the input.
    bytes: Peekable<Bytes<'input>>,

    /// Byte offset into the `self.input`. Input can't be larger than `2^32 B = 4 GB`
    curr_offs: u32,
}

impl<'input> SymbolStream<'input> {
    pub fn scan_str(input: &'input str) -> Self {
        let bytes = input.bytes().peekable();

        // make sure the input does not exceed the maximum size.
        debug_assert!(bytes.len() < (2usize.pow(32)));

        Self {
            input,
            bytes,
            curr_offs: 0,
        }
    }
}

impl<'input> Iterator for SymbolStream<'input> {
    type Item = Symbol<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_byte = self.bytes.next()?;

        let kind = SymbolKind::from(next_byte);

        let byte_len = match (kind, next_byte, self.bytes.peek()) {
            (SymbolKind::Newline, b'\r', Some(b'\n')) => {
                // "\r\n" is split into '\r' and '\n', so we can check if the char was '\r' and if
                // so, we can consume '\n' as well.
                self.bytes.next();
                2
            }
            _ => 1,
        };

        let prev_offs = self.curr_offs;

        self.curr_offs += byte_len;

        Some(Symbol {
            input: self.input,
            kind,
            span: SymPos {
                offs: prev_offs,
                len: byte_len,
            },
        })
    }
}
