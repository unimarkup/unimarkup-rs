use std::fmt::Debug;
use std::rc::Rc;

use unimarkup_commons::lexer::position::Position;
use unimarkup_commons::lexer::token::iterator::EndMatcher;
use unimarkup_inline::element::{Inline, InlineElement};
use unimarkup_inline::inline_parser;

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
    fn to_plain_string(&self) -> String {
        self.content.to_plain_string()
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
        let (iter, inline_context, parsed_inlines) = inline_parser::parse_inlines(
            parser.iter,
            (&parser.context).into(),
            None,
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.consumed_is_blank_line()
            })),
        );
        parser.iter = iter;
        parser.context.update_from(inline_context);
        let inlines = parsed_inlines.to_inlines();

        (parser, Block::Paragraph(Paragraph { content: inlines }))
    }
}

// impl From<Vec<&'_ Symbol<'_>>> for Paragraph {
//     fn from(value: Vec<&'_ Symbol<'_>>) -> Self {
//         let content = value
//             .iter()
//             .map(|&s| *s)
//             .collect::<Vec<Symbol<'_>>>()
//             .parse_inlines()
//             .collect();
//         let line_nr = value.get(0).map(|symbol| symbol.start.line).unwrap_or(0);

//         let id = crate::generate_id::generate_id(&format!(
//             "paragraph{delim}{}",
//             line_nr,
//             delim = types::ELEMENT_TYPE_DELIMITER
//         ))
//         .unwrap();

//         Paragraph {
//             id,
//             content,
//             attributes: None,
//             line_nr,
//         }
//     }
// }

// impl ElementParser for Paragraph {
//     type Token<'a> = &'a Symbol<'a>;

//     fn tokenize<'i>(input: &mut SymbolIterator<'i>) -> Option<TokenizeOutput<Self::Token<'i>>> {
//         let mut content_iter = input.nest(
//             None,
//             Some(Rc::new(|matcher: &mut dyn EndMatcher| {
//                 matcher.consumed_is_empty_line()
//             })),
//         );
//         let content = content_iter.take_to_end();
//         content_iter.update(input);

//         if content.is_empty() {
//             return None;
//         }

//         Some(TokenizeOutput { tokens: content })
//     }

//     fn parse(input: Vec<Self::Token<'_>>) -> Option<Blocks> {
//         let block = Block::Paragraph(Paragraph::from(input));

//         Some(vec![block])
//     }
// }
