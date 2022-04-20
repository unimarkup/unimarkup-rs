use unimarkup_inline::{InlineKind, FlatInline, Span, Position, parse_with_offset};

use crate::EXPECTED_MSG;


#[test]
pub fn test_parser_with_offset__newline_between_plain() {
  let offset = Position{ line: 100, column: 2 };
  let input = "line1\nline2";
  let expected = [
    InlineKind::Plain(FlatInline{ 
      content: "line1".to_string(),
      span: Span {
        start: Position{
          line: offset.line,
          column: offset.column
        },
        end: Position{
          line: offset.line,
          column: offset.column + 5
        }
      }
    }),
    InlineKind::PlainNewLine(FlatInline{ 
      content: " ".to_string(),
      span: Span {
        start: Position{
          line: offset.line,
          column: offset.column + 5
        },
        end: Position{
          line: offset.line,
          column: offset.column + 5
        }
      }
    }),
    InlineKind::Plain(FlatInline{ 
      content: "line2".to_string(),
      span: Span {
        start: Position{
          line: offset.line + 1,
          column: 0
        },
        end: Position{
          line: offset.line + 1,
          column: 5
        }
      }
    }),
  ];

  let actual = parse_with_offset(input, offset).unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}
