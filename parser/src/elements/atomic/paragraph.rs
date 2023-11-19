//! Contains the structs and parsers to parse paragraph elements.

use std::fmt::Debug;
use std::rc::Rc;

use unimarkup_commons::lexer::token::iterator::EndMatcher;
use unimarkup_inline::element::{Inline, InlineElement};
use unimarkup_inline::parser;

use crate::elements::blocks::Block;
use crate::elements::BlockElement;
use crate::BlockParser;

/// Structure of a Unimarkup paragraph element.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Paragraph {
    /// The content of the paragraph.
    pub content: Vec<Inline>,
}

impl BlockElement for Paragraph {
    fn as_unimarkup(&self) -> String {
        self.content.as_unimarkup()
    }

    fn start(&self) -> unimarkup_commons::lexer::position::Position {
        self.content.start()
    }

    fn end(&self) -> unimarkup_commons::lexer::position::Position {
        self.content.end()
    }
}

impl Paragraph {
    pub(crate) fn parse<'s, 'i>(mut parser: BlockParser<'s, 'i>) -> (BlockParser<'s, 'i>, Block) {
        let (iter, inline_context, parsed_inlines) = parser::parse_inlines(
            parser.iter,
            (&parser.context).into(),
            None,
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.consumed_is_blank_line() || matcher.outer_end()
            })),
        );
        parser.iter = iter;
        parser.context.update_from(inline_context);
        let inlines = parsed_inlines.to_inlines();

        (parser, Block::Paragraph(Paragraph { content: inlines }))
    }
}
