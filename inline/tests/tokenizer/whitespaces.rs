use unimarkup_inline::{Position, Token, TokenKind, Tokenizer};

use crate::EXPECTED_MSG;

#[test]
pub fn test_tokenize__two_plain_lines() {
    let input = "line1\nline2";
    let expected = [
        Token {
            kind: TokenKind::Plain,
            content: "line1".to_string(),
            position: Position { line: 0, column: 0 },
        },
        Token {
            kind: TokenKind::NewLine,
            content: "\n".to_string(),
            position: Position { line: 0, column: 5 },
        },
        Token {
            kind: TokenKind::Plain,
            content: "line2".to_string(),
            position: Position { line: 1, column: 0 },
        },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}
