mod symbol;

use symbol::Symbol;

use crate::elements::Blocks;

/// Parser as function that can parse Unimarkup content
pub type ParserFn = dyn for<'i> Fn(&'i [Symbol<'i>]) -> Option<(Blocks, &'i [Symbol<'i>])>;
