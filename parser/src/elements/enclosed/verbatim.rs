use std::rc::Rc;

use serde::{Deserialize, Serialize};
use unimarkup_commons::lexer::position::Position;
use unimarkup_commons::lexer::token::iterator::EndMatcher;
use unimarkup_commons::lexer::token::TokenKind;

use crate::elements::{BlockElement, Blocks};
use crate::{elements::blocks::Block, BlockParser};
use unimarkup_commons::lexer::{Itertools, SymbolKind};

/// Structure of a Unimarkup verbatim block element.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Verbatim {
    /// The content of the verbatim block.
    pub content: String,

    /// The language used to highlight the content.
    pub data_lang: Option<String>,

    /// Attributes of the verbatim block.
    // TODO: make attributes data structure
    pub attributes: Option<String>,

    pub implicit_closed: bool,
    pub tick_len: usize,
    pub start: Position,
    pub end: Position,
}

impl BlockElement for Verbatim {
    fn to_plain_string(&self) -> String {
        let ticks = SymbolKind::Tick.as_str().repeat(self.tick_len);
        let lang = self.data_lang.clone().unwrap_or_default();
        format!("{}{}\n{}\n{}", ticks, lang, self.content, ticks)
    }

    fn start(&self) -> unimarkup_commons::lexer::position::Position {
        self.start
    }

    fn end(&self) -> unimarkup_commons::lexer::position::Position {
        self.end
    }
}

impl Verbatim {
    pub(crate) fn parse<'s, 'i>(
        mut parser: BlockParser<'s, 'i>,
    ) -> (BlockParser<'s, 'i>, Option<Block>) {
        let tick_opt = parser.iter.next();
        if tick_opt.is_none() {
            return (parser, None);
        }
        let open_token = tick_opt.expect("Ensured above to be Some here.");
        let tick_len;

        if let TokenKind::Tick(len) = open_token.kind {
            if len < 3 {
                return (parser, None);
            }

            tick_len = len;
        } else {
            return (parser, None);
        }

        let mut data_lang_part = parser.iter.by_ref().take_while(|s| !s.kind.is_space());
        let data_lang = if let Some(mut token) = data_lang_part.next().copied() {
            token.end = data_lang_part.last().map_or(token.end, |l| l.end);
            token.kind = TokenKind::Plain;
            Some(String::from(token))
        } else {
            None
        };

        // exit if non-space content is given after data lang ended
        // => invalid verbatim block, take as paragraph
        if !matches!(
            parser.iter.prev_kind(),
            Some(TokenKind::Blankline) | Some(TokenKind::Newline)
        ) && parser
            .iter
            .by_ref()
            .take_while(|t| !matches!(t.kind, TokenKind::Blankline | TokenKind::Newline))
            .any(|t| !t.kind.is_space())
        {
            return (parser, None);
        }

        let prev_context_flags = parser.context.flags;
        let mut content_parser = parser.nest_scoped(
            None,
            Some(Rc::new(move |matcher: &mut dyn EndMatcher| {
                matcher.consumed_matches(&[
                    TokenKind::Newline,
                    TokenKind::Tick(tick_len),
                    TokenKind::Blankline, //TODO: add PossibleAttributes & Possible Decorators besides blanklines
                ])
            })),
        );
        content_parser.context.flags.logic_only = true;
        content_parser.context.flags.keep_whitespaces = true;
        content_parser.context.flags.keep_newline = true;

        let (updated_content_parser, content) = BlockParser::parse(content_parser);
        content_parser = updated_content_parser;
        let implicit_closed = !content_parser.iter.end_reached();

        parser = content_parser.unfold();
        parser.context.flags = prev_context_flags;

        let prev = parser
            .iter
            .prev()
            .expect("Must be some token, because at least start tokens came before.");
        let block_end = if implicit_closed {
            prev.end
        } else {
            prev.start // Start position, because previous was either blankline, attribute start, or decorator start
        };

        (
            parser,
            Some(Block::Verbatim(Verbatim {
                content: content.to_plain_string(),
                data_lang,
                attributes: None,
                implicit_closed,
                tick_len,
                start: open_token.start,
                end: block_end,
            })),
        )
    }
}

// pub(crate) enum Token<'a> {
//     StartDelim(Vec<&'a Symbol<'a>>),
//     DataLang(Vec<&'a Symbol<'a>>),
//     Content(Vec<&'a Symbol<'a>>),
// }

// impl ElementParser for Verbatim {
//     type Token<'a> = self::Token<'a>;

//     fn tokenize<'i>(input: &mut SymbolIterator<'i>) -> Option<TokenizeOutput<Self::Token<'i>>> {
//         let start_delim_len = input
//             .by_ref()
//             .peeking_take_while(|symbol| matches!(symbol.kind, SymbolKind::Tick))
//             .count();

//         if start_delim_len < 3 {
//             return None;
//         };

//         let start_delim = input.by_ref().take(start_delim_len).collect();
//         // Note: Consuming `Newline` is intended, because it is not part of the content, but also not of data-lang
//         let data_lang = input
//             .take_while(|s| s.kind != SymbolKind::Newline)
//             .collect::<Vec<_>>();

//         let end_sequence = std::iter::once(SymbolKind::Newline)
//             .chain(std::iter::repeat(SymbolKind::Tick).take(start_delim_len))
//             .collect::<Vec<SymbolKind>>();
//         let mut longer_delim_sequence = end_sequence.clone();
//         longer_delim_sequence.push(SymbolKind::Tick);

//         let end_fn = Rc::new(move |matcher: &mut dyn EndMatcher| {
//             if !matcher.matches(&longer_delim_sequence) {
//                 matcher.consumed_matches(&end_sequence)
//             } else {
//                 false
//             }
//         });

//         let mut content_iter = input.nest(None, Some(end_fn));
//         let content = content_iter.take_to_end();

//         if !content_iter.end_reached() {
//             return None;
//         }

//         content_iter.update(input);

//         // TODO: handle language attribute

//         // ensures empty line after block
//         if !input.consumed_is_empty_line() {
//             return None;
//         }

//         let output = TokenizeOutput {
//             tokens: vec![
//                 Token::StartDelim(start_delim),
//                 Token::DataLang(data_lang),
//                 Token::Content(content),
//             ],
//         };

//         Some(output)
//     }

//     fn parse(input: Vec<Self::Token<'_>>) -> Option<Blocks> {
//         let Token::StartDelim(start) = input.get(0)? else {
//             return None;
//         };
//         let line_nr = start.get(0)?.start.line;

//         let Token::DataLang(lang_symbols) = input.get(1)? else {
//             return None;
//         };
//         let data_lang = if lang_symbols.is_empty() {
//             None
//         } else {
//             Some(Symbol::flatten_iter(lang_symbols.iter().copied())?.to_string())
//         };

//         let Token::Content(symbols) = input.get(2)? else {
//             return None;
//         };
//         let content = Symbol::flatten_iter(symbols.iter().copied())?;

//         let block = Self {
//             id: String::default(),
//             content: String::from(content),
//             data_lang,
//             attributes: None,
//             line_nr,
//         };

//         Some(vec![Block::Verbatim(block)])
//     }
// }

// #[derive(Serialize, Deserialize, Default, Debug)]
// struct VerbatimAttributes {
//     language: Option<String>,
// }
