#![deny(missing_docs)]
//! Crate for lexing and parsing of Unimarkup inline formatted text.

mod inlines;
mod lexer;
mod parser;

pub use inlines::*;
pub use lexer::*;
pub use parser::*;
