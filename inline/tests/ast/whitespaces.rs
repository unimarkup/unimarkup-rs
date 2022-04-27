use unimarkup_inline::{parse, FlatInline, InlineKind, Position, Span};

use crate::EXPECTED_MSG;

#[test]
pub fn test_parser__newline_between_plain() {
    let input = "line1\nline2";
    let expected = [
        InlineKind::Plain(FlatInline {
            content: "line1".to_string(),
            span: Span {
                start: Position { line: 0, column: 0 },
                end: Position { line: 0, column: 5 },
            },
        }),
        InlineKind::PlainNewLine(FlatInline {
            content: " ".to_string(),
            span: Span {
                start: Position { line: 0, column: 5 },
                end: Position { line: 0, column: 5 },
            },
        }),
        InlineKind::Plain(FlatInline {
            content: "line2".to_string(),
            span: Span {
                start: Position { line: 1, column: 0 },
                end: Position { line: 1, column: 5 },
            },
        }),
    ];

    let actual = parse(input).unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
}
