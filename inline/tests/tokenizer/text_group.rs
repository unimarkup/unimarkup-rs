use unimarkup_inline::{Token, TokenKind, Position, Tokenizer};

use crate::EXPECTED_MSG;

#[test]
pub fn test_tokenize__simple_text_group() {
  let input = "[valid text group]";
  let expected = [
    Token{ kind: TokenKind::TextGroupOpen, content: "[".to_string(), position: Position { line: 0, column: 0 } },
    Token{ kind: TokenKind::Plain, content: "valid".to_string(), position: Position { line: 0, column: 1 } },
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 6 } },
    Token{ kind: TokenKind::Plain, content: "text".to_string(), position: Position { line: 0, column: 7 } },     
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 11 } },
    Token{ kind: TokenKind::Plain, content: "group".to_string(), position: Position { line: 0, column: 12 } },
    Token{ kind: TokenKind::TextGroupClose, content: "]".to_string(), position: Position { line: 0, column: 17 } },
  ];

  let actual = input.tokenize().unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}

#[test]
pub fn test_tokenize__plain_before_text_group() {
  let input = "plain[valid text group]";
  let expected = [
    Token{ kind: TokenKind::Plain, content: "plain".to_string(), position: Position { line: 0, column: 0 } },
    Token{ kind: TokenKind::TextGroupOpen, content: "[".to_string(), position: Position { line: 0, column: 5 } },
    Token{ kind: TokenKind::Plain, content: "valid".to_string(), position: Position { line: 0, column: 6 } },
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 11 } },
    Token{ kind: TokenKind::Plain, content: "text".to_string(), position: Position { line: 0, column: 12 } },     
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 16 } },
    Token{ kind: TokenKind::Plain, content: "group".to_string(), position: Position { line: 0, column: 17 } },
    Token{ kind: TokenKind::TextGroupClose, content: "]".to_string(), position: Position { line: 0, column: 22 } },
  ];

  let actual = input.tokenize().unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}

#[test]
pub fn test_tokenize__plain_after_text_group() {
  let input = "[valid text group]plain";
  let expected = [
    Token{ kind: TokenKind::TextGroupOpen, content: "[".to_string(), position: Position { line: 0, column: 0 } },
    Token{ kind: TokenKind::Plain, content: "valid".to_string(), position: Position { line: 0, column: 1 } },
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 6 } },
    Token{ kind: TokenKind::Plain, content: "text".to_string(), position: Position { line: 0, column: 7 } },     
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 11 } },
    Token{ kind: TokenKind::Plain, content: "group".to_string(), position: Position { line: 0, column: 12 } },
    Token{ kind: TokenKind::TextGroupClose, content: "]".to_string(), position: Position { line: 0, column: 17 } },
    Token{ kind: TokenKind::Plain, content: "plain".to_string(), position: Position { line: 0, column: 18 } },
  ];

  let actual = input.tokenize().unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}

#[test]
pub fn test_tokenize__formatting_inside_text_group() {
  let input = "[**bold**]";
  let expected = [
    Token{ kind: TokenKind::TextGroupOpen, content: "[".to_string(), position: Position { line: 0, column: 0 } },
    Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 1 } },
    Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 3 } },     
    Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 7 } },
    Token{ kind: TokenKind::TextGroupClose, content: "]".to_string(), position: Position { line: 0, column: 9 } },
  ];

  let actual = input.tokenize().unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}

#[test]
pub fn test_tokenize__invalid_formatting_over_text_group_borders() {
  let input = "[**bold]**";
  let expected = [
    Token{ kind: TokenKind::TextGroupOpen, content: "[".to_string(), position: Position { line: 0, column: 0 } },
    Token{ kind: TokenKind::Plain, content: "**bold".to_string(), position: Position { line: 0, column: 1 } },
    Token{ kind: TokenKind::TextGroupClose, content: "]".to_string(), position: Position { line: 0, column: 7 } },
    Token{ kind: TokenKind::Plain, content: "**".to_string(), position: Position { line: 0, column: 8 } },
  ];

  let actual = input.tokenize().unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}

#[test]
pub fn test_tokenize__formatting_outside_text_group() {
  let input = "**[bold]**";
  let expected = [
    Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 0 } },
    Token{ kind: TokenKind::TextGroupOpen, content: "[".to_string(), position: Position { line: 0, column: 2 } },
    Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 3 } },
    Token{ kind: TokenKind::TextGroupClose, content: "]".to_string(), position: Position { line: 0, column: 7 } },
    Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 8 } },
  ];

  let actual = input.tokenize().unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}
