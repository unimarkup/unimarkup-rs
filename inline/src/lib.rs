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
