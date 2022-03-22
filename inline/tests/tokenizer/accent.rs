use unimarkup_inline::{Token, TokenKind, Position, Tokenizer};

use crate::EXPECTED_MSG;

#[test]
pub fn test_tokenize__verbatim() {
  let input = "`verbatim`";
  let expected = [
    Token{ kind: TokenKind::VerbatimOpen, content: "`".to_string(), position: Position { line: 0, column: 0 } },
    Token{ kind: TokenKind::Plain, content: "verbatim".to_string(), position: Position { line: 0, column: 1 } },
    Token{ kind: TokenKind::VerbatimClose, content: "`".to_string(), position: Position { line: 0, column: 9 } },
  ];

  let actual = input.tokenize().unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}

#[test]
  pub fn test_tokenize__verbatim_escaped_close() {
    let input = "`verbatim\\`still verbatim`";
    let expected = [
      Token{ kind: TokenKind::VerbatimOpen, content: "`".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Plain, content: "verbatim".to_string(), position: Position { line: 0, column: 1 } },
      Token{ kind: TokenKind::EscapedGrapheme, content: "`".to_string(), position: Position { line: 0, column: 9 } },
      Token{ kind: TokenKind::Plain, content: "still".to_string(), position: Position { line: 0, column: 11 } },     
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 16 } },
      Token{ kind: TokenKind::Plain, content: "verbatim".to_string(), position: Position { line: 0, column: 17 } },
      Token{ kind: TokenKind::VerbatimClose, content: "`".to_string(), position: Position { line: 0, column: 25 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }
