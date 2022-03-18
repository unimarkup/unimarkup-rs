use crate::{inlines::InlineKind, error::InlineError};

use self::{tokens::Token, tokenizer::Tokenizer};

mod substitutions;
mod tokens;
mod tokenizer;

pub type Inline = Vec<InlineKind>;


pub fn parse(content: &str) -> Result<Inline, InlineError> {
  return Ok(content.tokenize()?.collect());
}


trait Parser {
  fn collect(self) -> Inline;
}

impl Parser for Vec<Token> {
  fn collect(self) -> Inline {
    todo!()
  }
}


