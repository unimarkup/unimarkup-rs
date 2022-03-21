//mod lexer;
mod parser;
mod inlines;
mod error;

//pub use lexer::*;

#[derive(Debug, Default, Clone, PartialEq, Copy)]
pub struct Position {
  pub line: usize,
  pub column: usize,
}


#[derive(Debug, Default, Clone, PartialEq, Copy)]
pub struct Span {
  pub start: Position,
  pub end: Position,
}

