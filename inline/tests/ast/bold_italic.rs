use unimarkup_inline::{parse, InlineKind, FlatInline, Span, Position, NestedInline};

use crate::EXPECTED_MSG;

#[test]
pub fn test_parser__plain_before_italic() {
  let input = "plain text *italic*";
  let expected = [
    InlineKind::Plain(FlatInline{ 
      content: "plain text ".to_string(),
      span: Span {
        start: Position{
          line: 0,
          column: 0
        },
        end: Position{
          line: 0,
          column: 10
        }
      }
    }),
    InlineKind::Italic(NestedInline{
      content: vec![InlineKind::Plain(FlatInline{
        content: "italic".to_string(),
        span: Span {
          start: Position{
            line: 0,
            column: 12
          },
          end: Position{
            line: 0,
            column: 17
          }
        }
      })],
      span: Span {
        start: Position{
          line: 0,
          column: 11
        },
        end: Position{
          line: 0,
          column: 18
        }
      }
    })
  ];

  let actual = parse(input).unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}
