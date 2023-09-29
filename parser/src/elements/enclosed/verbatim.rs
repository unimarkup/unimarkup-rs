use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::elements::blocks::Block;
use crate::elements::Blocks;
use crate::parser::{ElementParser, TokenizeOutput};
use unimarkup_commons::scanner::{EndMatcher, Itertools, Symbol, SymbolIterator, SymbolKind};

/// Structure of a Unimarkup verbatim block element.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Verbatim {
    /// Unique identifier for a verbatim block.
    pub id: String,

    /// The content of the verbatim block.
    pub content: String,

    /// The language used to highlight the content.
    pub data_lang: Option<String>,

    /// Attributes of the verbatim block.
    // TODO: make attributes data structure
    pub attributes: Option<String>,

    /// Line number, where the verbatim block occurs in
    /// the Unimarkup document.
    pub line_nr: usize,
}

pub(crate) enum Token<'a> {
    StartDelim(Vec<&'a Symbol<'a>>),
    DataLang(Vec<&'a Symbol<'a>>),
    Content(Vec<&'a Symbol<'a>>),
}

impl ElementParser for Verbatim {
    type Token<'a> = self::Token<'a>;

    fn tokenize<'i>(input: &mut SymbolIterator<'i>) -> Option<TokenizeOutput<Self::Token<'i>>> {
        let start_delim_len = input
            .by_ref()
            .peeking_take_while(|symbol| matches!(symbol.kind, SymbolKind::Tick))
            .count();

        if start_delim_len < 3 {
            return None;
        };

        let start_delim = input.by_ref().take(start_delim_len).collect();
        // Note: Consuming `Newline` is intended, because it is not part of the content, but also not of data-lang
        let data_lang = input
            .take_while(|s| s.kind != SymbolKind::Newline)
            .collect::<Vec<_>>();

        let end_sequence = std::iter::once(SymbolKind::Newline)
            .chain(std::iter::repeat(SymbolKind::Tick).take(start_delim_len))
            .collect::<Vec<SymbolKind>>();
        let mut longer_delim_sequence = end_sequence.clone();
        longer_delim_sequence.push(SymbolKind::Tick);

        let end_fn = Rc::new(move |matcher: &mut dyn EndMatcher| {
            if !matcher.matches(&longer_delim_sequence) {
                matcher.consumed_matches(&end_sequence)
            } else {
                false
            }
        });

        let mut content_iter = input.nest(None, Some(end_fn));
        let content = content_iter.take_to_end();

        if !content_iter.end_reached() {
            return None;
        }

        content_iter.update(input);

        // TODO: handle language attribute

        // ensures empty line after block
        if !input.consumed_is_empty_line() {
            return None;
        }

        let output = TokenizeOutput {
            tokens: vec![
                Token::StartDelim(start_delim),
                Token::DataLang(data_lang),
                Token::Content(content),
            ],
        };

        Some(output)
    }

    fn parse(input: Vec<Self::Token<'_>>) -> Option<Blocks> {
        let Token::StartDelim(start) = input.get(0)? else {
            return None;
        };
        let line_nr = start.get(0)?.start.line;

        let Token::DataLang(lang_symbols) = input.get(1)? else {
            return None;
        };
        let data_lang = if lang_symbols.is_empty() {
            None
        } else {
            Some(Symbol::flatten_iter(lang_symbols.iter().copied())?.to_string())
        };

        let Token::Content(symbols) = input.get(2)? else {
            return None;
        };
        let content = Symbol::flatten_iter(symbols.iter().copied())?;

        let block = Self {
            id: String::default(),
            content: String::from(content),
            data_lang,
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
