//mod lexer;
mod parser;
mod inlines;

//pub use lexer::*;

#[derive(Debug, Default, Clone, PartialEq, Copy)]
pub struct Position {
  pub line: usize,
  pub column: usize,
}

