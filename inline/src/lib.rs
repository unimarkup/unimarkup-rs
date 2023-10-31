//!#![deny(missing_docs)]
//! Crate for lexing and parsing of Unimarkup inline formatted text.

pub mod element;
mod inlines;
mod lexer;
mod parser;
mod tokenize;

pub use inlines::*;
pub use lexer::*;
pub use parser::*;

pub mod inline_parser;
