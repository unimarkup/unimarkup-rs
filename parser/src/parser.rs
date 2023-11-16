//! Module for parsing of Unimarkup elements.

use unimarkup_commons::lexer::{
    span::Span,
    token::{
        iterator::{IteratorEndFn, IteratorPrefixFn, TokenIterator},
        TokenKind,
    },
};
use unimarkup_inline::parser::{InlineContext, InlineContextFlags};

use crate::{
    document::Document,
    elements::{
        atomic::{Heading, Paragraph},
        blocks::Block,
        enclosed::VerbatimBlock,
        indents::BulletList,
        kind::PossibleBlockStart,
        Blocks,
    },
    metadata::{Metadata, MetadataKind},
    security,
};
use unimarkup_commons::config::Config;

/// Parses and returns a Unimarkup document.
pub fn parse_unimarkup(um_content: &str, config: &mut Config) -> Document {
    let tokens = unimarkup_commons::lexer::token::lex_str(um_content);

    //TODO: extract and parse preamble before parsing

    let (_, blocks) = BlockParser::parse(BlockParser::new(
        TokenIterator::from(&*tokens),
        BlockContext::default(),
    ));

    let mut unimarkup = Document {
        config: config.clone(),
        blocks,
        ..Default::default()
    };

    let metadata = Metadata {
        file: config.input.clone(),
        contenthash: security::get_contenthash(um_content),
        preamble: String::new(),
        kind: MetadataKind::Root,
        namespace: ".".to_string(),
    };

    unimarkup.metadata.push(metadata);

    unimarkup
}

/// Function type for functions that parse block elements
pub(crate) type BlockParserFn =
    for<'s, 'i> fn(BlockParser<'s, 'i>) -> (BlockParser<'s, 'i>, Option<Block>);

/// The block parser holding the [`TokenIterator`],
/// and [`BlockContext`] used to parse Unimarkup content.
#[derive(Debug)]
pub(crate) struct BlockParser<'slice, 'input> {
    /// The iterator over [`Token`](unimarkup_commons::lexer::token::Token)s of Unimarkup content.
    pub iter: TokenIterator<'slice, 'input>,
    /// Context for block element parsing.
    pub context: BlockContext,
}

impl<'slice, 'input> BlockParser<'slice, 'input> {
    /// Create a new block parser.
    pub fn new(token_iter: TokenIterator<'slice, 'input>, context: BlockContext) -> Self {
        BlockParser {
            iter: token_iter,
            context,
        }
    }

    /// Parses Unimarkup content and produces Unimarkup blocks.
    pub fn parse(mut parser: Self) -> (Self, Blocks) {
        let mut blocks = Vec::default();

        #[cfg(debug_assertions)]
        let mut curr_len = parser.iter.max_len();

        parser.iter.reset_peek();

        'outer: while let Some(kind) = parser.iter.peek_kind() {
            if matches!(kind, TokenKind::Blankline | TokenKind::Newline) {
                // skip newlines besides blanklines so new blocks can always assume that they start on a new line.
                let next = parser
                    .iter
                    .next()
                    .expect("Must be some token, because peek returned some.");

                // Keep blanklines between blocks
                // Newlines before blocks are not needed, because every block must start at a new line.
                if parser.context.flags.keep_newline && next.kind == TokenKind::Blankline {
                    blocks.push(Block::Blankline(Span {
                        start: next.start,
                        end: next.end,
                    }))
                }
            } else if kind == TokenKind::Eoi {
                break 'outer;
            } else {
                let block_start = PossibleBlockStart::from(kind);
                if block_start == PossibleBlockStart::Paragraph {
                    let (updated_parser, paragraph) = Paragraph::parse(parser);
                    parser = updated_parser;
                    blocks.push(paragraph);
                } else {
                    // Token might be start of a block element
                    for parser_fn in get_parser_fn(block_start, &parser.context) {
                        let checkpoint = parser.iter.checkpoint();
                        let (updated_parser, block_opt) = parser_fn(parser);
                        parser = updated_parser;
                        match block_opt {
                            Some(block) => {
                                blocks.push(block);
                                continue 'outer;
                            }
                            None => {
                                let success = parser.iter.rollback(checkpoint);
                                debug_assert!(
                                    success,
                                    "Rollback was not successful for checkpoint '{:?}'",
                                    checkpoint
                                )
                            }
                        }
                    }

                    // no other block parser created a block for curr token -> take as paragraph
                    let (updated_parser, paragraph) = Paragraph::parse(parser);
                    parser = updated_parser;
                    blocks.push(paragraph);
                }
            }

            #[cfg(debug_assertions)]
            {
                assert!(
                    parser.iter.max_len() < curr_len,
                    "Parser consumed no symbol in iteration."
                );
                curr_len = parser.iter.max_len();
            }
        }

        // To consume tokens in end matching of peek_kind(), or consume EOI
        let _ = parser.iter.next();

        (parser, blocks)
    }

    /// Create a new block parser that takes this parser as parent.
    /// The new parser creates a new scope.
    pub fn nest_scoped(
        mut self,
        prefix_match: Option<IteratorPrefixFn>,
        end_match: Option<IteratorEndFn>,
    ) -> Self {
        self.iter = self.iter.nest_scoped(prefix_match, end_match);
        self
    }

    /// Creates a new block parser that takes this parser as parent.
    pub fn nest(
        mut self,
        prefix_match: Option<IteratorPrefixFn>,
        end_match: Option<IteratorEndFn>,
    ) -> Self {
        self.iter = self.iter.nest(prefix_match, end_match);
        self
    }

    /// Unfolds this parser, returning the parent parser if a parent exists.
    /// If no parent exists, it leaves this parser unchanged.
    ///
    /// The block context is kept as is.
    pub fn unfold(mut self) -> Self {
        self.iter = self.iter.into_inner();
        self
    }
}

/// Gets possible matching parser functions depending on the peeked token.
fn get_parser_fn(start: PossibleBlockStart, context: &BlockContext) -> &'static [BlockParserFn] {
    if context.flags.logic_only {
        // if start == PossibleBlockStart::OpenBrace {
        //     // TODO: return macro parser
        //     &[] // &[block_macro_parser]
        // } else {
        //     &[]
        // }
        &[]
    } else {
        match start {
            PossibleBlockStart::Heading(_) => &[Heading::parse],
            PossibleBlockStart::ColumnBlock => &[], //&[implicit_column_parser, explicit_column_parser],
            PossibleBlockStart::MathBlock => &[],
            PossibleBlockStart::RenderBlock => &[],
            PossibleBlockStart::VerbatimBlock => &[VerbatimBlock::parse],
            PossibleBlockStart::Table => &[],
            PossibleBlockStart::BulletList => &[BulletList::parse],
            PossibleBlockStart::Digit => &[],
            PossibleBlockStart::QuotationBlock => &[],
            PossibleBlockStart::LineBlock => &[],
            PossibleBlockStart::MediaInsert => &[],
            PossibleBlockStart::RenderInsert => &[],
            PossibleBlockStart::VerbatimInsert => &[],
            PossibleBlockStart::HorizontalLine => &[],
            PossibleBlockStart::LineBreak => &[],
            PossibleBlockStart::Decoration | PossibleBlockStart::Paragraph => &[],
            PossibleBlockStart::OpenBracket => &[],
            PossibleBlockStart::OpenBrace => &[], //&[attribute_block_parser, block_macro_parser],
        }
    }
}

/// Block context that helps to provide additional context to parser functions of Unimarkup block elements.
#[derive(Debug, Default, Clone)]
pub struct BlockContext {
    /// Flags used to define parser behavior of block element parsing.
    pub flags: BlockContextFlags,
}

/// Block context flags used to define parser behavior of block element parsing.
#[derive(Debug, Default, Clone, Copy)]
pub struct BlockContextFlags {
    /// Flag to indicate that only escaped symbols and logic elements are allowed besides plain content.
    pub logic_only: bool,
    /// Flag to indicate that multiple contiguous whitespaces must not be combined.
    pub keep_whitespaces: bool,
    /// Flag to indicate that a newline must be explicitly kept, and not converted to one space.
    pub keep_newline: bool,
}

impl From<&BlockContext> for InlineContext {
    fn from(value: &BlockContext) -> Self {
        InlineContext {
            flags: InlineContextFlags {
                logic_only: value.flags.logic_only,
                keep_whitespaces: value.flags.keep_whitespaces,
                keep_newline: value.flags.keep_newline,
                allow_implicits: value.flags.logic_only,
            },
        }
    }
}

impl BlockContext {
    /// Updates the block context using the given [`InlineContext`].
    pub fn update_from(&mut self, _inline_context: InlineContext) {
        //TODO: update block context

        // Flags are not updated, because they only "propagate" block->inline, but not the other way.
    }
}

#[cfg(test)]
mod test {
    use unimarkup_commons::lexer::token::iterator::TokenIterator;

    use crate::{BlockContext, BlockParser};

    #[test]
    fn debugging_dummy() {
        let tokens = unimarkup_commons::lexer::token::lex_str("## Heading **with** inlines.");
        let parser = BlockParser {
            iter: TokenIterator::from(&*tokens),
            context: BlockContext::default(),
        };

        let (_, blocks) = BlockParser::parse(parser);

        dbg!(blocks);
    }
}
