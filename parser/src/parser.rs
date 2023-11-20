//! Module for parsing of Unimarkup elements.

use unimarkup_commons::{
    config::ConfigFns,
    lexer::{
        span::Span,
        token::{
            iterator::{IteratorEndFn, IteratorPrefixFn, TokenIterator},
            TokenKind,
        },
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
        preamble::parse_preamble,
        Blocks,
    },
    metadata::{Metadata, MetadataKind},
    security,
};
use unimarkup_commons::config::Config;

/// Parses and returns a Unimarkup document.
pub fn parse_unimarkup(um_content: &str, mut config: Config) -> Document {
    let tokens = unimarkup_commons::lexer::token::lex_str(um_content);

    let mut parser = BlockParser::new(TokenIterator::from(&*tokens), BlockContext::default());

    let checkpoint = parser.iter.checkpoint();
    let (updated_parser, preamble) = parse_preamble(parser);
    parser = updated_parser;

    match preamble.clone() {
        Some(preamble) => config.preamble.merge(preamble),
        None => {
            parser.iter.rollback(checkpoint);
        }
    }

    let (parser, blocks) = BlockParser::parse(parser);

    let input = config.input.clone();
    Document {
        config,
        blocks,
        citations: parser.context.citations,
        metadata: vec![Metadata {
            file: input,
            contenthash: security::get_contenthash(um_content),
            preamble,
            kind: MetadataKind::Root,
            namespace: ".".to_string(),
        }],
        ..Default::default()
    }
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

                if block_start != PossibleBlockStart::Paragraph {
                    // Token might be start of a block element
                    for parser_fn in get_parser_fn(block_start, &parser.context) {
                        let checkpoint = parser.iter.checkpoint();
                        let (updated_parser, block_opt) = parser_fn(parser);
                        parser = updated_parser;
                        match block_opt {
                            Some(block) => {
                                blocks.push(block);

                                if let Some(prev) = parser.iter.prev() {
                                    // To keep possibly consumed blank lines
                                    if parser.context.flags.keep_newline
                                        && prev.kind == TokenKind::Blankline
                                    {
                                        blocks.push(Block::Blankline(Span {
                                            start: prev.start,
                                            end: prev.end,
                                        }))
                                    }
                                }

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
                }

                // no other block parser created a block for curr token -> take as paragraph
                let (updated_parser, paragraph) = Paragraph::parse(parser);
                parser = updated_parser;
                blocks.push(paragraph);

                if let Some(prev) = parser.iter.prev() {
                    // To keep possibly consumed blank lines
                    if parser.context.flags.keep_newline && prev.kind == TokenKind::Blankline {
                        blocks.push(Block::Blankline(Span {
                            start: prev.start,
                            end: prev.end,
                        }))
                    }
                }
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
    pub fn into_inner(mut self) -> Self {
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
    /// Citations used in the Unimarkup content.
    /// The citations are added in document flow.
    /// Every citation may contain one or more citation entry IDs.
    pub citations: Vec<Vec<String>>,
}

/// Block context flags used to define parser behavior of block element parsing.
#[derive(Debug, Default, Clone, Copy)]
pub struct BlockContextFlags {
    /// Flag to indicate that only escaped graphemes and logic elements are allowed besides plain content.
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
            citations: Vec::new(),
        }
    }
}

impl BlockContext {
    /// Updates the block context using the given [`InlineContext`].
    pub fn update_from(&mut self, mut inline_context: InlineContext) {
        // Flags are not updated, because they only "propagate" block->inline, but not the other way.

        self.citations.append(&mut inline_context.citations);
    }
}

#[cfg(test)]
mod test {
    use unimarkup_commons::lexer::token::iterator::TokenIterator;

    use crate::{parse_unimarkup, BlockContext, BlockParser};

    #[test]
    fn debugging_dummy() {
        let tokens = unimarkup_commons::lexer::token::lex_str(
            "```
Verbatim block


Two blank lines before.
```",
        );
        let parser = BlockParser {
            iter: TokenIterator::from(&*tokens),
            context: BlockContext::default(),
        };

        let (_, blocks) = BlockParser::parse(parser);

        assert!(!blocks.is_empty());

        // dbg!(blocks);
    }

    #[test]
    fn debugging_preamble_dummy() {
        let content = "+++
lang: \"de-AT\"
+++

Funktioniert preamble parsing?
        ";
        let doc = parse_unimarkup(content, unimarkup_commons::config::Config::default());

        assert!(!doc.metadata.is_empty());
    }
}
