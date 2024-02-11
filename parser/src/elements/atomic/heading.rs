//! Contains the structs and parsers to parse heading elements.

use std::rc::Rc;

use strum_macros::*;
use unimarkup_commons::attributes::token::AttributeTokens;
use unimarkup_commons::attributes::tokenize::{AttributeContext, AttributeTokenizer};
use unimarkup_commons::lexer::token::iterator::{EndMatcher, Itertools, PrefixMatcher};
use unimarkup_commons::lexer::token::TokenKind;
use unimarkup_commons::parsing::Parser;
use unimarkup_inline::element::{Inline, InlineElement};
use unimarkup_inline::parser;

use crate::elements::BlockElement;
use crate::{elements::blocks::Block, BlockParser};
use unimarkup_commons::lexer::position::Position;

use super::log_id::AtomicError;

/// Enum of possible heading levels for unimarkup headings
#[derive(Eq, PartialEq, Debug, strum_macros::Display, EnumString, Clone, Copy)]
#[strum(serialize_all = "kebab-case")]
pub enum HeadingLevel {
    /// Heading level 1, corresponds to `# ` in Unimarkup.
    #[strum(serialize = "level-1")]
    Level1 = 1, // start counting from 0

    /// Heading level 2, corresponds to `## ` in Unimarkup.
    #[strum(serialize = "level-2")]
    Level2,

    /// Heading level 3, corresponds to `### ` in Unimarkup.
    #[strum(serialize = "level-3")]
    Level3,

    /// Heading level 4, corresponds to `#### ` in Unimarkup.
    #[strum(serialize = "level-4")]
    Level4,

    /// Heading level 5, corresponds to `##### ` in Unimarkup.
    #[strum(serialize = "level-5")]
    Level5,

    /// Heading level 6, corresponds to `###### ` in Unimarkup.
    #[strum(serialize = "level-6")]
    Level6,
}

impl From<HeadingLevel> for u8 {
    fn from(level: HeadingLevel) -> Self {
        match level {
            HeadingLevel::Level1 => 1,
            HeadingLevel::Level2 => 2,
            HeadingLevel::Level3 => 3,
            HeadingLevel::Level4 => 4,
            HeadingLevel::Level5 => 5,
            HeadingLevel::Level6 => 6,
        }
    }
}

impl HeadingLevel {
    /// The `str` representation of the [`HeadingLevel`].
    pub fn as_str(&self) -> &'static str {
        match self {
            HeadingLevel::Level1 => "#",
            HeadingLevel::Level2 => "##",
            HeadingLevel::Level3 => "###",
            HeadingLevel::Level4 => "####",
            HeadingLevel::Level5 => "#####",
            HeadingLevel::Level6 => "######",
        }
    }
}

impl TryFrom<&str> for HeadingLevel {
    type Error = AtomicError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let level = match input {
            "1" | "#" => HeadingLevel::Level1,
            "2" | "##" => HeadingLevel::Level2,
            "3" | "###" => HeadingLevel::Level3,
            "4" | "####" => HeadingLevel::Level4,
            "5" | "#####" => HeadingLevel::Level5,
            "6" | "######" => HeadingLevel::Level6,
            _ => return Err(AtomicError::InvalidHeadingLvl),
        };

        Ok(level)
    }
}

impl TryFrom<usize> for HeadingLevel {
    type Error = AtomicError;

    fn try_from(level_depth: usize) -> Result<Self, Self::Error> {
        let level = match level_depth {
            1 => Self::Level1,
            2 => Self::Level2,
            3 => Self::Level3,
            4 => Self::Level4,
            5 => Self::Level5,
            6 => Self::Level6,
            _ => return Err(AtomicError::InvalidHeadingLvl),
        };

        Ok(level)
    }
}

/// Structure of a Unimarkup heading element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    /// Unique identifier for a heading.
    pub id: String,

    /// Heading level.
    pub level: HeadingLevel,

    /// The content of the heading line.
    pub content: Vec<Inline>,

    /// Attributes of the heading.
    pub attributes: Option<AttributeTokens>,

    /// The start of this block in the original content.
    pub start: Position,
    /// The end of this block in the original content.
    pub end: Position,
}

impl BlockElement for Heading {
    fn as_unimarkup(&self) -> String {
        let prefix = self.level.as_str();
        let content = self
            .content
            .as_unimarkup()
            .lines()
            .join(&" ".repeat(prefix.len()));
        format!("{prefix}{content}")
    }

    fn start(&self) -> Position {
        self.start
    }

    fn end(&self) -> Position {
        self.end
    }
}

impl Heading {
    pub(crate) fn parse<'s, 'i>(
        mut parser: BlockParser<'s, 'i>,
    ) -> (BlockParser<'s, 'i>, Option<Block>) {
        let hashes_opt = parser.iter.next();
        if hashes_opt.is_none() {
            return (parser, None);
        }
        let hashes = hashes_opt.expect("Ensured above to be Some here.");
        let hashes_len;

        if let TokenKind::Hash(len) = hashes.kind {
            // len == 0 is impossible, because TokenKind would not be of kind Hash in that case.
            if len > 6 {
                return (parser, None);
            }

            hashes_len = len;
        } else {
            return (parser, None);
        }

        if !parser.iter.consumed_matches(&[TokenKind::Space]) {
            return (parser, None);
        }

        let (hashes_prefix, spaces_prefix) = heading_prefix_sequences(hashes_len);
        let sub_heading_prefix = sub_heading_start(hashes_len);

        let (iter, inline_context, parsed_inlines) = parser::parse_inlines(
            parser.iter,
            (&parser.context).into(),
            Some(Rc::new(|matcher: &mut dyn PrefixMatcher| {
                matcher.consumed_prefix(hashes_prefix) || matcher.consumed_prefix(spaces_prefix)
            })),
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                matcher.consumed_is_blank_line()
                    || matcher.matches(sub_heading_prefix)
                    || matcher.outer_end()
            })),
        );
        parser.iter = iter;
        parser.context.update_from(inline_context);

        let prefix_mismatch = parsed_inlines.prefix_mismatch();
        let might_be_attribute_start = parser.iter.peek_kind() == Some(TokenKind::OpenBrace);

        let content = parsed_inlines.to_inlines();
        let heading_end = parser
            .iter
            .prev()
            .expect("At least space after hash must be in prev if inlines was empty")
            .end;

        let attributes = {
            if prefix_mismatch {
                if might_be_attribute_start {
                    let (attrb_iter, attrb_token_res) = AttributeTokenizer::new(
                        parser.iter.nest_scoped(None, None),
                        AttributeContext::default(),
                    )
                    .parse();
                    parser.iter = attrb_iter.iter;
                    let mut attrb = attrb_token_res.ok();
                    if let Some(a) = attrb.as_mut() {
                        if a.id.is_none() {
                            a.id = Some(as_id(&content));
                        }
                    } else {
                        return (parser, None);
                    }
                    attrb
                } else {
                    return (parser, None);
                }
            } else {
                None
            }
        }
        .or_else(|| {
            let mut attrbs = AttributeTokens::default();
            attrbs.id = Some(as_id(&content));
            Some(attrbs)
        });

        (
            parser,
            Some(Block::Heading(Heading {
                id: attributes
                    .as_ref()
                    .expect("Heading always as at least 'id' attribute.")
                    .id
                    .as_ref()
                    .expect("Heading always has an 'id' attribute.")
                    .clone(),
                level: HeadingLevel::try_from(hashes_len)
                    .expect("Correct heading level ensured above."),
                content,
                attributes,
                start: hashes.start,
                end: heading_end,
            })),
        )
    }
}

/// Converts the heading content into a valid ID.
///
/// Whitespaces are replaced with `-`, quotes and backslash are removed,
/// and all other content is lowercased.
fn as_id(content: &Vec<Inline>) -> String {
    let mut s = content.as_unimarkup().to_lowercase();
    s = s.replace(char::is_whitespace, "-");
    s = s.replace('\\', ""); // backslash removed to prevent html escapes
    s.replace(['\'', '"'], "") // quotes removed to prevent early attribute closing
}

// Below consts allow matching without dynamic allocations.

const HEADING_LVL_1_HASH_PREFIX: [TokenKind; 2] = [TokenKind::Hash(1), TokenKind::Space];
const HEADING_LVL_1_SPACE_PREFIX: [TokenKind; 2] = [TokenKind::Space, TokenKind::Space];

const HEADING_LVL_2_HASH_PREFIX: [TokenKind; 2] = [TokenKind::Hash(2), TokenKind::Space];
const SUB_HEADING_LVL_2_HASH_PREFIX: [TokenKind; 3] =
    [TokenKind::Newline, TokenKind::Hash(2), TokenKind::Space];
const HEADING_LVL_2_SPACE_PREFIX: [TokenKind; 3] =
    [TokenKind::Space, TokenKind::Space, TokenKind::Space];

const HEADING_LVL_3_HASH_PREFIX: [TokenKind; 2] = [TokenKind::Hash(3), TokenKind::Space];
const SUB_HEADING_LVL_3_HASH_PREFIX: [TokenKind; 3] =
    [TokenKind::Newline, TokenKind::Hash(3), TokenKind::Space];
const HEADING_LVL_3_SPACE_PREFIX: [TokenKind; 4] = [
    TokenKind::Space,
    TokenKind::Space,
    TokenKind::Space,
    TokenKind::Space,
];

const HEADING_LVL_4_HASH_PREFIX: [TokenKind; 2] = [TokenKind::Hash(4), TokenKind::Space];
const SUB_HEADING_LVL_4_HASH_PREFIX: [TokenKind; 3] =
    [TokenKind::Newline, TokenKind::Hash(4), TokenKind::Space];
const HEADING_LVL_4_SPACE_PREFIX: [TokenKind; 5] = [
    TokenKind::Space,
    TokenKind::Space,
    TokenKind::Space,
    TokenKind::Space,
    TokenKind::Space,
];

const HEADING_LVL_5_HASH_PREFIX: [TokenKind; 2] = [TokenKind::Hash(5), TokenKind::Space];
const SUB_HEADING_LVL_5_HASH_PREFIX: [TokenKind; 3] =
    [TokenKind::Newline, TokenKind::Hash(5), TokenKind::Space];
const HEADING_LVL_5_SPACE_PREFIX: [TokenKind; 6] = [
    TokenKind::Space,
    TokenKind::Space,
    TokenKind::Space,
    TokenKind::Space,
    TokenKind::Space,
    TokenKind::Space,
];

const HEADING_LVL_6_HASH_PREFIX: [TokenKind; 2] = [TokenKind::Hash(6), TokenKind::Space];
const SUB_HEADING_LVL_6_HASH_PREFIX: [TokenKind; 3] =
    [TokenKind::Newline, TokenKind::Hash(6), TokenKind::Space];
const HEADING_LVL_6_SPACE_PREFIX: [TokenKind; 7] = [
    TokenKind::Space,
    TokenKind::Space,
    TokenKind::Space,
    TokenKind::Space,
    TokenKind::Space,
    TokenKind::Space,
    TokenKind::Space,
];

/// Returns the correct matching sequence depending on the parsed heading level.
const fn heading_prefix_sequences(
    hashes_len: usize,
) -> (&'static [TokenKind], &'static [TokenKind]) {
    if hashes_len == 1 {
        (&HEADING_LVL_1_HASH_PREFIX, &HEADING_LVL_1_SPACE_PREFIX)
    } else if hashes_len == 2 {
        (&HEADING_LVL_2_HASH_PREFIX, &HEADING_LVL_2_SPACE_PREFIX)
    } else if hashes_len == 3 {
        (&HEADING_LVL_3_HASH_PREFIX, &HEADING_LVL_3_SPACE_PREFIX)
    } else if hashes_len == 4 {
        (&HEADING_LVL_4_HASH_PREFIX, &HEADING_LVL_4_SPACE_PREFIX)
    } else if hashes_len == 5 {
        (&HEADING_LVL_5_HASH_PREFIX, &HEADING_LVL_5_SPACE_PREFIX)
    } else {
        (&HEADING_LVL_6_HASH_PREFIX, &HEADING_LVL_6_SPACE_PREFIX)
    }
}

const fn sub_heading_start(hashes_len: usize) -> &'static [TokenKind; 3] {
    if hashes_len == 1 {
        &SUB_HEADING_LVL_2_HASH_PREFIX
    } else if hashes_len == 2 {
        &SUB_HEADING_LVL_3_HASH_PREFIX
    } else if hashes_len == 3 {
        &SUB_HEADING_LVL_4_HASH_PREFIX
    } else if hashes_len == 4 {
        &SUB_HEADING_LVL_5_HASH_PREFIX
    } else {
        &SUB_HEADING_LVL_6_HASH_PREFIX
    }
}
