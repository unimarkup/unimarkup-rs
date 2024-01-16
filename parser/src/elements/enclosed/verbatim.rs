//! Contains structs and parsers to create verbatim blocks.

use std::rc::Rc;

use unimarkup_commons::attributes::token::AttributeTokens;
use unimarkup_commons::attributes::tokenize::{AttributeContext, AttributeTokenizer};
use unimarkup_commons::lexer::position::Position;
use unimarkup_commons::lexer::token::iterator::EndMatcher;
use unimarkup_commons::lexer::token::TokenKind;
use unimarkup_commons::parsing::Parser;

use crate::elements::BlockElement;
use crate::{elements::blocks::Block, BlockParser};
use unimarkup_commons::lexer::symbol::SymbolKind;

/// Structure of a Unimarkup verbatim block element.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VerbatimBlock {
    /// The content of the verbatim block.
    pub content: String,
    /// The language used to highlight the content.
    pub data_lang: Option<String>,
    /// Attributes of the verbatim block.
    // TODO: make attributes data structure
    pub attributes: Option<AttributeTokens>,
    /// Marks that this verbatim block was implicitly closed.
    pub implicit_closed: bool,
    /// The number of backticks this verbatim block was created with.
    pub tick_len: usize,
    /// The start of this block in the original content.
    pub start: Position,
    /// The end of this block in the original content.
    pub end: Position,
}

impl BlockElement for VerbatimBlock {
    fn as_unimarkup(&self) -> String {
        let ticks = SymbolKind::Tick.as_str().repeat(self.tick_len);
        let lang = self.data_lang.clone().unwrap_or_default();
        format!(
            "{}{}\n{}\n{}",
            &ticks,
            lang,
            self.content,
            if self.implicit_closed { "" } else { &ticks }
        )
    }

    fn start(&self) -> unimarkup_commons::lexer::position::Position {
        self.start
    }

    fn end(&self) -> unimarkup_commons::lexer::position::Position {
        self.end
    }
}

impl VerbatimBlock {
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
                    TokenKind::PossibleDecorator,
                    TokenKind::PossibleAttributes,
                    TokenKind::EnclosedBlockEnd,
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

        // block end before attrb parsing for correct syntax highlighting
        let prev = parser
            .iter
            .prev()
            .expect("Must be some token, because at least start tokens came before.");
        let block_end = if implicit_closed {
            prev.end
        } else {
            prev.start // Start position, because previous was either blankline, attribute start, or decorator start
        };

        let mut attrb_tokens = None;
        let next_kind = parser.iter.peek_kind();
        if next_kind == Some(TokenKind::OpenBrace) {
            // scoped attribute parsing
            let (attrb_iter, attrb_token_res) = AttributeTokenizer::new(
                parser.iter.nest_scoped(None, None),
                AttributeContext::default(),
            )
            .parse();
            parser.iter = attrb_iter.iter;
            attrb_tokens = attrb_token_res.ok();
        } else if let Some(TokenKind::Plus(nr_plus)) = next_kind {
            // scoped decorator parsing
        }

        (
            parser,
            Some(Block::VerbatimBlock(VerbatimBlock {
                content: content.as_unimarkup(),
                data_lang,
                attributes: attrb_tokens,
                implicit_closed,
                tick_len,
                start: open_token.start,
                end: block_end,
            })),
        )
    }
}
