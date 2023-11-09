//! Module for parsing of Unimarkup elements.

use unimarkup_commons::lexer::{
    span::Span,
    token::{
        iterator::{IteratorEndFn, IteratorPrefixFn, TokenIterator},
        TokenKind,
    },
};
use unimarkup_inline::inline_parser::{InlineContext, InlineContextFlags};

use crate::{
    document::Document,
    elements::{
        atomic::{Heading, Paragraph},
        blocks::Block,
        enclosed::Verbatim,
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

pub(crate) type BlockParserFn =
    for<'s, 'i> fn(BlockParser<'s, 'i>) -> (BlockParser<'s, 'i>, Option<Block>);

#[derive(Debug)]
pub(crate) struct BlockParser<'slice, 'input> {
    pub iter: TokenIterator<'slice, 'input>,
    pub context: BlockContext,
}

impl<'slice, 'input> BlockParser<'slice, 'input> {
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

    pub fn nest_scoped(
        mut self,
        prefix_match: Option<IteratorPrefixFn>,
        end_match: Option<IteratorEndFn>,
    ) -> Self {
        self.iter = self.iter.nest_with_scope(prefix_match, end_match);
        self
    }

    pub fn nest(
        mut self,
        prefix_match: Option<IteratorPrefixFn>,
        end_match: Option<IteratorEndFn>,
    ) -> Self {
        self.iter = self.iter.nest(prefix_match, end_match);
        self
    }

    pub fn unfold(mut self) -> Self {
        self.iter = self.iter.unfold();
        self
    }
}

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
            PossibleBlockStart::ColumnBlock => todo!(), //&[implicit_column_parser, explicit_column_parser],
            PossibleBlockStart::MathBlock => todo!(),
            PossibleBlockStart::RenderBlock => todo!(),
            PossibleBlockStart::VerbatimBlock => &[Verbatim::parse],
            PossibleBlockStart::Table => todo!(),
            PossibleBlockStart::BulletList => &[BulletList::parse],
            PossibleBlockStart::Digit => todo!(),
            PossibleBlockStart::QuotationBlock => todo!(),
            PossibleBlockStart::LineBlock => todo!(),
            PossibleBlockStart::MediaInsert => todo!(),
            PossibleBlockStart::RenderInsert => todo!(),
            PossibleBlockStart::VerbatimInsert => todo!(),
            PossibleBlockStart::HorizontalLine => todo!(),
            PossibleBlockStart::LineBreak => todo!(),
            PossibleBlockStart::Decoration | PossibleBlockStart::Paragraph => &[],
            PossibleBlockStart::OpenBracket => todo!(),
            PossibleBlockStart::OpenBrace => todo!(), //&[attribute_block_parser, block_macro_parser],
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct BlockContext {
    pub flags: BlockContextFlags,
}

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
    pub fn update_from(&mut self, inline_context: InlineContext) {
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
        let tokens =
            unimarkup_commons::lexer::token::lex_str("- first entry\n\n  - nested list entry");
        let parser = BlockParser {
            iter: TokenIterator::from(&*tokens),
            context: BlockContext::default(),
        };

        let (_, blocks) = BlockParser::parse(parser);

        dbg!(blocks);
    }
}

// /// Output of symbol tokenization by a parser of a block.
// pub(crate) struct TokenizeOutput<T> {
//     pub(crate) tokens: Vec<T>,
// }

// /// Trait implemented by a parser for each Unimarkup element.
// pub(crate) trait ElementParser {
//     /// Token type produced by tokenization.
//     type Token<'a>;

//     /// Function that converts input symbols into tokens specific for the given element.
//     fn tokenize<'i>(input: &mut SymbolIterator<'i>) -> Option<TokenizeOutput<Self::Token<'i>>>;

//     /// Function that parses tokenization output and produces one or more Unimarkup elements.
//     fn parse(input: Vec<Self::Token<'_>>) -> Option<Blocks>;
// }

// // Makes it impossible to implement `ParserGenerator` trait outside of this module,
// // but still makes it possible to name `ParserGenerator` and use it as a bound.
// mod private {
//     pub trait Sealed {}
//     impl<'a, T> Sealed for T where T: super::ElementParser + 'a + 'static {}
// }

// /// Trait implemented by all Unimarkup elements that can generate parser function for their
// /// content.
// pub trait ParserGenerator: private::Sealed {
//     /// Generates parser function for the given Unimarkup element.
//     fn generate_parser() -> ParserFn;
// }

// impl<'a, T> ParserGenerator for T
// where
//     T: ElementParser + 'a + 'static,
// {
//     // NOTE: we might need some context information for parsers. An option could be to pass
//     // some kind of Context struct into generate_parser and use that for whatever we need to.
//     fn generate_parser() -> ParserFn {
//         |input| {
//             let tokenize_output = T::tokenize(input)?;
//             let blocks = T::parse(tokenize_output.tokens)?;

//             Some(blocks)
//         }
//     }
// }

// /// Parser of unimarkup content.
// #[derive(Clone)]
// pub struct MainParser {
//     parsers: Vec<ParserFn>,
//     default_parser: ParserFn,
// }

// impl Default for MainParser {
//     fn default() -> Self {
//         log!(MainParserInfo::StartInitializing);

//         let default = Paragraph::generate_parser();

//         let mut parser = Self {
//             parsers: Vec::with_capacity(3),
//             default_parser: default,
//         };

//         // TODO: how to handle preamble parser?
//         parser.register_parser(Heading::generate_parser());
//         parser.register_parser(Verbatim::generate_parser());
//         parser.register_parser(BulletList::generate_parser());

//         log!(MainParserInfo::Initialized);
//         parser
//     }
// }

// impl MainParser {
//     fn register_parser(&mut self, parser: ParserFn) {
//         self.parsers.push(parser);
//     }

//     /// Parses Unimarkup content and produces Unimarkup blocks.
//     pub fn parse(&self, input: &mut SymbolIterator) -> Blocks {
//         let mut blocks = Vec::default();

//         #[cfg(debug_assertions)]
//         let mut curr_len = input.max_len();

//         input.reset_peek();

//         'outer: while let Some(kind) = input.peek_kind() {
//             match kind {
//                 // skip newlines and empty lines before elements
//                 SymbolKind::Newline => {
//                     while input.consumed_is_empty_line() {
//                         // consume contiguous empty lines
//                     }
//                     // Consume newline before next block element
//                     input.next();
//                 }

//                 // stop parsing when end of input is reached
//                 SymbolKind::Eoi => break 'outer,

//                 // no parser will match, parse with default parser
//                 _ if kind.is_not_keyword() => {
//                     blocks.append(
//                         &mut (self.default_parser)(input)
//                             .expect("Default parser failed parsing non-keyword."),
//                     );
//                 }

//                 // symbol is start of a block, some parser should match
//                 _ => {
//                     for parser_fn in &self.parsers {
//                         let mut iter = input.clone();
//                         if let Some(mut res_blocks) = parser_fn(&mut iter) {
//                             blocks.append(&mut res_blocks);
//                             *input = iter;
//                             continue 'outer; // start from first parser on next input
//                         }
//                     }

//                     // no registered parser matched -> use default parser
//                     blocks.append(
//                         &mut (self.default_parser)(input).expect(
//                             "Default parser failed parsing content no other parser matched.",
//                         ),
//                     );
//                 }
//             }

//             #[cfg(debug_assertions)]
//             {
//                 assert!(
//                     input.max_len() < curr_len,
//                     "Parser consumed no symbol in iteration."
//                 );
//                 curr_len = input.max_len();
//             }
//         }

//         // To consume symbols in end matching
//         let _ = input.next();

//         blocks
//     }
// }
