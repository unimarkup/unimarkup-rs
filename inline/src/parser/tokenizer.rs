use std::collections::{BinaryHeap, HashMap};

use crate::Position;

use super::tokens::{Token, TokenKind};



pub(crate) trait Tokenizer {
  fn tokenize(self) -> Vec<Token>;
}

// usize refers to the index of an open token in the tokens vector
type OpenTokenQueue = BinaryHeap<usize>;

impl Tokenizer for &str {
  fn tokenize(self) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let mut open_tokens = HashMap::<TokenKind, OpenTokenQueue>::new();
    let mut cur_pos = Position::default();
    
    for c in self.chars() {
      // track char position
      // update tokens list
      // handle backslash escapes, newlines and explicit newlines
      update_tokens(&mut tokens, &mut open_tokens, &mut cur_pos, c);
    }

    cleanup_loose_open_tokens(&mut tokens, open_tokens);
    tokens
  }
}

fn update_tokens(tokens: &mut Vec::<Token>, open_tokens: &mut HashMap::<TokenKind, OpenTokenQueue>, cur_pos: &mut Position, c: char) {

}

fn cleanup_loose_open_tokens(tokens: &mut Vec::<Token>, open_tokens: HashMap::<TokenKind, OpenTokenQueue>) {

}


#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
  use super::*;

  pub const EXPECTED_MSG: &str = "actual(left) != expected(right)";

  #[test]
  pub fn test_formatting__plain_before_italic() {
    let input = "plain text *italic*";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "plain".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 5 } },
      Token{ kind: TokenKind::Plain, content: "text".to_string(), position: Position { line: 0, column: 6 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 10 } },
      Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 11 } },
      Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 12 } },
      Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 18 } },
    ];

    let actual = input.tokenize();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

}

