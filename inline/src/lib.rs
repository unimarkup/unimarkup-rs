//! This library provides functionality to get a Unimarkup inline AST from a given string 

use ast::collect::InlineAst;
use error::InlineError;

pub mod error;

mod ast;
mod tokenizer;

pub use ast::*;
pub use tokenizer::*;

/// Function to transform a given string into an AST of Unimarkup inline elements.
/// 
/// **Note:** The string must not contain blank lines! It is not checked, but will probably lead to false results.
/// 
/// Returns `InlineError`, if inline constraints are violated by the given string.
pub fn parse(content: &str) -> Result<Inline, InlineError> {
  Ok(content.tokenize()?.collect())
}

/// Function to transform a given string into an AST of Unimarkup inline elements.
/// The additional offset is used to set the start position of the first inline element.
/// This function is useful to get correct element positions inside a Unimarkup document.
/// 
/// **Note:** The string must not contain blank lines! It is not checked, but will probably lead to false results.
/// 
/// Returns `InlineError`, if inline constraints are violated by the given string.
pub fn parse_with_offset(content: &str, offset: Position) -> Result<Inline, InlineError> {
  Ok(content.tokenize_with_offset(offset)?.collect())
}
