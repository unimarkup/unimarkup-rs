//! Crate for parsing Unimarkup inline elements.

pub mod element;
pub mod parser;

mod tokenize;

pub use tokenize::kind::InlineTokenKind;
