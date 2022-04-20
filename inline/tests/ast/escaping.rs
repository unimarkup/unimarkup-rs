use unimarkup_inline::{parse, InlineKind, FlatInline, Span, Position};

use crate::EXPECTED_MSG;

#[test]
pub fn test_parser__escape_space() {
  let input = "\\ ";
  let expected = [
    InlineKind::EscapedSpace(FlatInline{ 
      content: " ".to_string(),
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
pub fn test_parser__escape_plain() {
  let input = "\\plain";
  let expected = [
    InlineKind::Plain(FlatInline{ 
      content: "plain".to_string(),
      span: Span {
        start: Position{
          line: 0,
          column: 0
        },
        end: Position{
          line: 0,
          column: 6 // note that the backslash is taken into account
        }
      }
    }),
  ];

  let actual = parse(input).unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}

#[test]
pub fn test_parser__escape_newline_after_plain() {
  let input = "plain\\\n";
  let expected = [
    InlineKind::Plain(FlatInline{ 
      content: "plain".to_string(),
      span: Span {
        start: Position{
          line: 0,
          column: 0
        },
        end: Position{
          line: 0,
          column: 5
        }
      }
    }),
    InlineKind::EscapedNewLine(FlatInline{ 
      content: "\n".to_string(),
      span: Span {
        start: Position{
          line: 0,
          column: 5
        },
        end: Position{
          line: 0,
          column: 7
        }
      }
    }),
  ];

  let actual = parse(input).unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}
