use unimarkup_inline::{parse, InlineKind, FlatInline, Span, Position};

use crate::EXPECTED_MSG;

#[test]
pub fn test_parser__arrow_substitution() {
  let input = "-->";
  let expected = [
    InlineKind::Plain(FlatInline{ 
      content: "🠖".to_string(),
      span: Span {
        start: Position{
          line: 0,
          column: 0
        },
        end: Position{
          line: 0,
          column: 2
        }
      }
    }),
  ];

  let actual = parse(input).unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}

#[test]
pub fn test_parser__emoji_substitution_inside_text() {
  let input = "substituted :D smiley";
  let expected = [
    InlineKind::Plain(FlatInline{ 
      content: "substituted 😃 smiley".to_string(),
      span: Span {
        start: Position{
          line: 0,
          column: 0
        },
        end: Position{
          line: 0,
          column: 20
        }
      }
    }),
  ];

  let actual = parse(input).unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}

#[test]
pub fn test_parser__smile_emoji_substitution() {
  let input = "substituted ^^ smile";
  let expected = [
    InlineKind::Plain(FlatInline{ 
      content: "substituted 😄 smile".to_string(),
      span: Span {
        start: Position{
          line: 0,
          column: 0
        },
        end: Position{
          line: 0,
          column: 19
        }
      }
    }),
  ];

  let actual = parse(input).unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}

#[test]
pub fn test_parser__expressionless_emoji_substitution() {
  let input = "substituted -- expressionless";
  let expected = [
    InlineKind::Plain(FlatInline{ 
      content: "substituted 😑 expressionless".to_string(),
      span: Span {
        start: Position{
          line: 0,
          column: 0
        },
        end: Position{
          line: 0,
          column: 28
        }
      }
    }),
  ];

  let actual = parse(input).unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}
