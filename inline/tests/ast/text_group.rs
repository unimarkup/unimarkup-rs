use unimarkup_inline::{parse, InlineKind, FlatInline, Span, Position, NestedInline, TextGroupAttributes};

use crate::EXPECTED_MSG;

#[test]
pub fn test_parser__plain_text_group() {
  let input = "[group]";
  let expected = [
    InlineKind::TextGroup(NestedInline{
      content: vec![InlineKind::Plain(FlatInline{
        content: "group".to_string(),
        span: Span {
          start: Position{
            line: 0,
            column: 1
          },
          end: Position{
            line: 0,
            column: 6
          }
        }
      })],
      span: Span {
        start: Position{
          line: 0,
          column: 0
        },
        end: Position{
          line: 0,
          column: 7
        }
      }
    },
    TextGroupAttributes{ ..Default::default() })
  ];

  let actual = parse(input).unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}
