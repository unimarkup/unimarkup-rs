use unimarkup_inline::{Token, TokenKind, Position, Tokenizer};

use crate::EXPECTED_MSG;

#[test]
pub fn test_tokenize__plain_before_italic() {
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

  let actual = input.tokenize().unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}

#[test]
pub fn test_tokenize__plain_after_bold() {
  let input = "**bold** plain text";
  let expected = [
    Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 0 } },
    Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 2 } },
    Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 6 } },
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 8 } },
    Token{ kind: TokenKind::Plain, content: "plain".to_string(), position: Position { line: 0, column: 9 } },
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 14 } },
    Token{ kind: TokenKind::Plain, content: "text".to_string(), position: Position { line: 0, column: 15 } },
  ];

  let actual = input.tokenize().unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}

#[test]
pub fn test_tokenize__right_side_nested() {
  let input = "**bold and *italic***";
  let expected = [
    Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 0 } },
    Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 2 } },
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 6 } },
    Token{ kind: TokenKind::Plain, content: "and".to_string(), position: Position { line: 0, column: 7 } },
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 10 } },
    Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 11 } },
    Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 12 } },
    Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 18 } },
    Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 19 } },
  ];

  let actual = input.tokenize().unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}

#[test]
pub fn test_tokenize__bold_with_unopened_italic() {
  let input = "**bold no-italic* bold**";
  let expected = [
    Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 0 } },
    Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 2 } },
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 6 } },
    Token{ kind: TokenKind::Plain, content: "no-italic*".to_string(), position: Position { line: 0, column: 7 } },
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 17 } },
    Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 18 } },
    Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 22 } },
  ];

  let actual = input.tokenize().unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}

#[test]
pub fn test_tokenize__italic_with_unopened_bold() {
  let input = "*italic no-bold** italic*";
  let expected = [
    Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 0 } },
    Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 1 } },
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 7 } },
    Token{ kind: TokenKind::Plain, content: "no-bold**".to_string(), position: Position { line: 0, column: 8 } },
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 17 } },
    Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 18 } },
    Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 24 } },
  ];

  let actual = input.tokenize().unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}

#[test]
pub fn test_tokenize__left_side_nested() {
  let input = "***italic* and bold**";
  let expected = [
    Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 0 } },
    Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 2 } },
    Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 3 } },
    Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 9 } },
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 10 } },
    Token{ kind: TokenKind::Plain, content: "and".to_string(), position: Position { line: 0, column: 11 } },
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 14 } },
    Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 15 } },
    Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 19 } },
  ];

  let actual = input.tokenize().unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}

#[test]
pub fn test_tokenize__left_side_nested_with_plain_ending() {
  let input = "***italic* and bold** plain";
  let expected = [
    Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 0 } },
    Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 2 } },
    Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 3 } },
    Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 9 } },
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 10 } },
    Token{ kind: TokenKind::Plain, content: "and".to_string(), position: Position { line: 0, column: 11 } },
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 14 } },
    Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 15 } },
    Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 19 } },
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 21 } },
    Token{ kind: TokenKind::Plain, content: "plain".to_string(), position: Position { line: 0, column: 22 } },
  ];

  let actual = input.tokenize().unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}

#[test]
  pub fn test_tokenize__bold_directly_after_italic() {
    let input = "*italic***bold**";
    let expected = [
      Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 1 } },
      Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 7 } },
      Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 8 } },
      Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 10 } },
      Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 14 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }
  
  #[test]
  pub fn test_tokenize__split_bold_italic_combined_close_due_to_space() {
    let input = "*before ***after*";
    let expected = [
      Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Plain, content: "before".to_string(), position: Position { line: 0, column: 1 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 7 } },
      Token{ kind: TokenKind::Plain, content: "*".to_string(), position: Position { line: 0, column: 8 } },
      Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 9 } },
      Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 10 } },
      Token{ kind: TokenKind::Plain, content: "after".to_string(), position: Position { line: 0, column: 11 } },
      Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 16 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_tokenize__asterisks_as_plain() {
    let input = "before****after";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "before****after".to_string(), position: Position { line: 0, column: 0 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_tokenize__asterisks_as_plain_surrounded_by_space() {
    let input = "before **** after";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "before".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 6 } },
      Token{ kind: TokenKind::Plain, content: "****".to_string(), position: Position { line: 0, column: 7 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 11 } },
      Token{ kind: TokenKind::Plain, content: "after".to_string(), position: Position { line: 0, column: 12 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }
  
  #[test]
  pub fn test_tokenize__italic_directly_after_bold() {
    let input = "**bold***italic*";
    let expected = [
      Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 2 } },
      Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 6 } },
      Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 8 } },
      Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 9 } },
      Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 15 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }
   
  #[test]
  pub fn test_tokenize__italic_directly_after_combined_bold_italic() {
    let input = "***bold & italic****italic*";
    let expected = [
      Token{ kind: TokenKind::BoldItalicOpen, content: "***".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 3 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 7 } },
      Token{ kind: TokenKind::Plain, content: "&".to_string(), position: Position { line: 0, column: 8 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 9 } },
      Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 10 } },
      Token{ kind: TokenKind::BoldItalicClose, content: "***".to_string(), position: Position { line: 0, column: 16 } },
      Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 19 } },
      Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 20 } },
      Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 26 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_tokenize__italic_directly_after_plain_asterisks() {
    let input = "****italic*";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "***".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 3 } },
      Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 4 } },
      Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 10 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_tokenize__bold_directly_after_plain_asterisks() {
    let input = "*****bold**";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "***".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 3 } },
      Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 5 } },
      Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 9 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_tokenize__combined_directly_after_plain_asterisks() {
    let input = "******bold-italic***";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "***".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::BoldItalicOpen, content: "***".to_string(), position: Position { line: 0, column: 3 } },
      Token{ kind: TokenKind::Plain, content: "bold-italic".to_string(), position: Position { line: 0, column: 6 } },
      Token{ kind: TokenKind::BoldItalicClose, content: "***".to_string(), position: Position { line: 0, column: 17 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_tokenize__plain_asterisks() {
    let input = "*********";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "*********".to_string(), position: Position { line: 0, column: 0 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_tokenize__invalid_italic_open() {
    let input = "* no italic*";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "*".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 1 } },
      Token{ kind: TokenKind::Plain, content: "no".to_string(), position: Position { line: 0, column: 2 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 4 } },
      Token{ kind: TokenKind::Plain, content: "italic*".to_string(), position: Position { line: 0, column: 5 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_tokenize__invalid_bold_open() {
    let input = "plain** still plain**";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "plain**".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 7 } },
      Token{ kind: TokenKind::Plain, content: "still".to_string(), position: Position { line: 0, column: 8 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 13 } },
      Token{ kind: TokenKind::Plain, content: "plain**".to_string(), position: Position { line: 0, column: 14 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
pub fn test_tokenize__escape_open_italic() {
  let input = "\\*not italic*";
  let expected = [
    Token{ kind: TokenKind::EscapedGrapheme, content: "*".to_string(), position: Position { line: 0, column: 0 } },
    Token{ kind: TokenKind::Plain, content: "not".to_string(), position: Position { line: 0, column: 2 } },
    Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 5 } },
    Token{ kind: TokenKind::Plain, content: "italic*".to_string(), position: Position { line: 0, column: 6 } },
  ];

  let actual = input.tokenize().unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}
