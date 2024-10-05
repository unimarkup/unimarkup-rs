use super::position::Span as SymPos;
use crate::symbol::{Symbol, SymbolKind};

/// Iterator of Unimarkup [`Symbol`]s over a given input.
pub(crate) struct SymbolStream<'input> {
    /// The input from which the Unimarkup symbols are to be scanned.
    input: &'input str,

    /// Byte offset into the `self.input`. Input can't be larger than `2^32 B = 4 GB`
    curr_offs: u32,

    /// Code point offset
    cp_offs: u32,
}

impl<'input> SymbolStream<'input> {
    pub fn scan_str(input: &'input str) -> Self {
        Self {
            input,
            curr_offs: 0,
            cp_offs: 0,
        }
    }
}

impl<'input> Iterator for SymbolStream<'input> {
    type Item = Symbol<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        debug_assert!(self.input.len() < (2usize.pow(32)));

        let not_scanned = self.input.get((self.curr_offs as _)..self.input.len())?;
        let mut chars = not_scanned.split("").skip(1);

        let next_char = chars.next()?;

        if next_char.is_empty() {
            // split with empty string returns iterator with first and last element being empty
            // strings
            return None;
        }

        let kind = SymbolKind::from(next_char);

        let (byte_len, cp_count) = match (kind, next_char, chars.next()) {
            // NOTE: \r\n is split into "\r" "\n", so we can check if the char was \r and if so,
            // we can consume \n as well.
            (SymbolKind::Newline, "\r", Some("\n")) => ("\n".len() + next_char.len(), 2),
            _ => (next_char.len(), 1),
        };

        let prev_offs = self.curr_offs;
        let prev_cp_offs = self.cp_offs;

        self.curr_offs += byte_len as u32;
        self.cp_offs += cp_count as u32;

        Some(Symbol {
            input: self.input,
            kind,
            span: SymPos {
                offs: prev_offs,
                len: byte_len as u16,
                cp_offs: prev_cp_offs,
                cp_count,
            },
        })
    }
}
