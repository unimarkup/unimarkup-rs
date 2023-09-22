use serde::{Deserialize, Serialize};

use crate::elements::blocks::Block;
use crate::elements::Blocks;
use crate::parser::{ElementParser, TokenizeOutput};
use unimarkup_commons::scanner::{Symbol, SymbolIterator, SymbolKind};

/// Structure of a Unimarkup verbatim block element.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Verbatim {
    /// Unique identifier for a verbatim block.
    pub id: String,

    /// The content of the verbatim block.
    pub content: String,

    /// Attributes of the verbatim block.
    // TODO: make attributes data structure
    pub attributes: Option<String>,

    /// Line number, where the verbatim block occurs in
    /// the Unimarkup document.
    pub line_nr: usize,
}

pub(crate) enum Token<'a> {
    StartDelim(Vec<&'a Symbol<'a>>),
    Content(Vec<&'a Symbol<'a>>),
}

impl ElementParser for Verbatim {
    type Token<'a> = self::Token<'a>;

    fn tokenize<'i>(input: &mut SymbolIterator<'i>) -> Option<TokenizeOutput<'i, Self::Token<'i>>> {
        let start_delim: Vec<_> = input
            .by_ref()
            .take_while(|symbol| matches!(symbol.kind, SymbolKind::Tick))
            .collect();
        let start_delim_len = start_delim.len();

        if start_delim_len < 3 {
            return None;
        };

        let end_sequence = std::iter::repeat(SymbolKind::Tick)
            .take(start_delim_len)
            .collect::<Vec<_>>();
        let _end_fn = Box::new(|sequence: &[Symbol<'i>]| {
            sequence[..start_delim_len]
                .iter()
                .map(|s| s.kind)
                .collect::<Vec<_>>()
                .starts_with(&end_sequence)
        });

        // let mut content_iter = input.nest(&[], Some(end_fn));

        // let content = content_iter.take_to_end();
        // if !content_iter.end_reached() {
        //     return None;
        // }

        // input = content_iter.parent()?;
        match input
            .by_ref()
            .take(start_delim_len)
            .map(|s| s.kind)
            .collect::<Vec<_>>()
        {
            end if end == end_sequence => {
                if input.peek_kind() == Some(SymbolKind::Tick) {
                    return None;
                }
            }
            _ => return None,
        }

        // TODO: handle language attribute

        let output = TokenizeOutput {
            tokens: vec![Token::StartDelim(start_delim), Token::Content(vec![])], //content)],
            rest_of_input: input.remaining_symbols(),
        };

        Some(output)
    }

    fn parse(input: Vec<Self::Token<'_>>) -> Option<Blocks> {
        let Token::StartDelim(start) = input.get(0)? else {
            return None;
        };
        let line_nr = start.get(0)?.start.line;

        let Token::Content(symbols) = input.get(1)? else {
            return None;
        };
        let content = Symbol::flatten_iter(symbols.iter().copied())?;

        let block = Self {
            id: String::default(),
            content: String::from(content),
            attributes: None,
            line_nr,
        };

        Some(vec![Block::Verbatim(block)])
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct VerbatimAttributes {
    language: Option<String>,
}
