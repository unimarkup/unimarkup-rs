use unimarkup_inline::{parse, InlineKind, FlatInline, Span, Position, NestedInline};

use crate::EXPECTED_MSG;

#[test]
pub fn test_parser__verbatim_with_escaped_words_and_spaces() {
  let input = "`es*ca*ping\\ in\\ner`";
  let expected = [
    InlineKind::Verbatim(NestedInline{
      content: vec![
        InlineKind::Plain(FlatInline{
          content: "es*ca*ping".to_string(),
          span: Span {
            start: Position{
              line: 0,
              column: 1
            },
            end: Position{
              line: 0,
              column: 11
            }
          }
        }),
        InlineKind::EscapedSpace(FlatInline{
          content: " ".to_string(),
          span: Span {
            start: Position{
              line: 0,
              column: 11
            },
            end: Position{
              line: 0,
              column: 13
            }
          }
        }),
        InlineKind::Plain(FlatInline{
          content: "inner".to_string(),
          span: Span {
            start: Position{
              line: 0,
              column: 13
            },
            end: Position{
              line: 0,
              column: 19
            }
          }
        })
      ],
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
    })
  ];

  let actual = parse(input).unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}

#[test]
pub fn test_parser__verbatim_with_plain_newline() {
  let input = "`plain\nnewline`";
  let expected = [
    InlineKind::Verbatim(NestedInline{
      content: vec![
        InlineKind::Plain(FlatInline{
          content: "plain".to_string(),
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
        }),
        InlineKind::PlainNewLine(FlatInline{
          content: " ".to_string(),
          span: Span {
            start: Position{
              line: 0,
              column: 6
            },
            end: Position{
              line: 0,
              column: 6
            }
          }
        }),
        InlineKind::Plain(FlatInline{
          content: "newline".to_string(),
          span: Span {
            start: Position{
              line: 1,
              column: 0
            },
            end: Position{
              line: 1,
              column: 7
            }
          }
        })
      ],
      span: Span {
        start: Position{
          line: 0,
          column: 0
        },
        end: Position{
          line: 1,
          column: 8
        }
      }
    })
  ];

  let actual = parse(input).unwrap();

  assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}
