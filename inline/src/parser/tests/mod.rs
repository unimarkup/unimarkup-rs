use crate::{PlainContent, Position};

use super::*;

#[test]
fn parse_simple_plain() {
    let parser = "Some text".parse_inlines(None);

    assert_eq!(parser.count(), 1);
}

#[test]
fn parse_simple_bold() {
    let mut parser = "**Bold text**".parse_inlines(None);

    let inline = parser.next().unwrap();
    let start = Position { line: 1, column: 3 };
    let end = start + (0, 9 - 1);

    // no remaining inlines
    assert_eq!(parser.count(), 0);
    assert!(matches!(inline, Inline::Bold(_)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Nested(&NestedContent {
            content: vec![Inline::Plain(PlainContent {
                content: String::from("Bold text"),
                span: (start, end).into()
            })]
            .into(),
            span: (start - (0, 2), end + (0, 2)).into(),
        })
    );
}

#[test]
fn parse_simple_italic() {
    let mut parser = "*Italic text*".parse_inlines(None);

    let inline = parser.next().unwrap();
    let start = Position { line: 1, column: 2 };
    let end = start + (0, 11 - 1);

    // no remaining inlines
    assert_eq!(parser.count(), 0);
    assert!(matches!(inline, Inline::Italic(_)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Nested(&NestedContent {
            content: vec![Inline::Plain(PlainContent {
                content: String::from("Italic text"),
                span: Span::from((start, end))
            })]
            .into(),
            span: (start - (0, 1), end + (0, 1)).into()
        })
    );
}

#[test]
fn parse_italic_bold() {
    let mut parser = "*Italic text***Bold text**".parse_inlines(None);

    let inline = parser.next().unwrap();
    let start = Position { line: 1, column: 2 };
    let end = start + (0, 11 - 1);

    assert!(matches!(inline, Inline::Italic(_)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Nested(&NestedContent {
            content: vec![Inline::Plain(PlainContent {
                content: String::from("Italic text"),
                span: (start, end).into()
            })]
            .into(),
            span: (start - (0, 1), end + (0, 1)).into()
        })
    );

    let inline = parser.next().unwrap();
    let start = end + (0, 5 - 1);
    let end = start + (0, 9 - 1);

    assert!(matches!(inline, Inline::Bold(_)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Nested(&NestedContent {
            content: vec![Inline::Plain(PlainContent {
                content: String::from("Bold text"),
                span: Span::from((start, end))
            })]
            .into(),
            span: (start - (0, 2), end + (0, 2)).into()
        })
    );
}

#[test]
fn parse_bold_italic_nested() {
    let mut parser = "**This is bold *with* italic inside.**".parse_inlines(None);

    let inline = parser.next().unwrap();
    let start = Position { line: 1, column: 1 };
    let end = start + (0, 38 - 1);

    // no remaining inlines
    assert_eq!(parser.count(), 0);

    assert!(matches!(inline, Inline::Bold(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
    assert!(matches!(inline.as_ref(), InlineContent::Nested(_)));

    if let InlineContent::Nested(inner_content) = inline.into_inner() {
        assert_eq!(inner_content.count(), 3);

        let inline = &inner_content.content.get(0).unwrap();

        let start = Position { line: 1, column: 3 };
        let end = start + (0, 13 - 1);

        assert!(matches!(inline, Inline::Plain(_)));
        assert_eq!(
            inline.as_ref(),
            InlineContent::Plain(&PlainContent {
                content: String::from("This is bold "),
                span: Span::from((start, end))
            })
        );

        let inline = &inner_content[1];

        let start = end + (0, 1);
        let end = start + (0, 6 - 1);

        let inner_start = start + (0, 1);
        let inner_end = end - (0, 1);

        assert!(matches!(inline, Inline::Italic(_)));
        assert_eq!(
            inline.as_ref(),
            InlineContent::Nested(&NestedContent {
                content: vec![Inline::Plain(PlainContent {
                    content: String::from("with"),
                    span: Span::from((inner_start, inner_end))
                })]
                .into(),
                span: (inner_start - (0, 1), inner_end + (0, 1)).into()
            })
        );
        assert_eq!(inline.span(), Span::from((start, end)));

        let inline = &inner_content[2];

        let start = end + (0, 1);
        let end = start + (0, 15 - 1);

        assert!(matches!(inline, Inline::Plain(_)));
        assert_eq!(
            inline.as_ref(),
            InlineContent::Plain(&PlainContent {
                content: String::from(" italic inside."),
                span: Span::from((start, end))
            })
        );
    } else {
        panic!("Inner content not nested");
    }
}

#[test]
fn parse_text_group_simple() {
    let mut parser = "This is text [with text group] as part of it.".parse_inlines(None);

    let inline = parser.next().unwrap();
    let start = Position { line: 1, column: 1 };
    let end = start + (0, 13 - 1);

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
    assert!(matches!(inline.as_ref(), InlineContent::Plain(_)));

    let inline = parser.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 17 - 1);

    assert!(matches!(inline, Inline::TextGroup(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Nested(&NestedContent {
            content: vec![Inline::Plain(PlainContent {
                content: String::from("with text group"),
                span: (start + (0, 1), end - (0, 1)).into()
            })]
            .into(),
            span: (start, end).into()
        })
    );

    let inline = parser.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 15 - 1);

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
    assert!(matches!(inline.as_ref(), InlineContent::Plain(_)));
}

#[test]
fn parse_text_group_interrupt_bold() {
    let input = "This is **text [with text** group] as part of it.";
    let mut parser = input.parse_inlines(None);

    let inline = parser.next().unwrap();

    let start = Position { line: 1, column: 1 };
    let end = start + (0, 15 - 1);

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Plain(&PlainContent {
            content: String::from("This is **text "),
            span: Span::from((start, end))
        })
    );

    let inline = parser.next().unwrap();

    let start = end + (0, 1);
    let end = start + (0, 19 - 1);

    assert!(matches!(inline, Inline::TextGroup(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Nested(&NestedContent {
            content: vec![Inline::Plain(PlainContent {
                content: String::from("with text** group"),
                span: Span::from((start + (0, 1), end - (0, 1)))
            })]
            .into(),
            span: Span::from((start, end))
        })
    );

    let inline = parser.next().unwrap();

    let start = end + (0, 1);
    let end = start + (0, 15 - 1);

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Plain(&PlainContent {
            content: String::from(" as part of it."),
            span: Span::from((start, end))
        })
    );
}

#[test]
fn parse_open_italic_closed_bold_hehe() {
    let input = "***This is input**";
    let mut parser = input.parse_inlines(None);

    let inline = parser.next().unwrap();
    let start = Position::new(1, 1);
    let end = start;

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Plain(&PlainContent {
            content: String::from("*"),
            span: Span::from((start, end))
        })
    );

    let inline = parser.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 17 - 1);

    assert!(matches!(inline, Inline::Bold(_)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Nested(&NestedContent {
            content: vec![Inline::Plain(PlainContent {
                content: String::from("This is input"),
                span: Span::from((start + (0, 2), end - (0, 2)))
            })]
            .into(),
            span: Span::from((start, end))
        })
    );
}

#[test]
fn parse_nested_text_group() {
    let input = "[This text group [has another one inside] of it.]";
    let mut parser = input.parse_inlines(None);

    let inline = parser.next().unwrap();

    let start = Position { line: 1, column: 1 };
    let end = start + (0, 49 - 1);

    assert!(matches!(inline, Inline::TextGroup(_)));
    assert!(matches!(inline.as_ref(), InlineContent::Nested(_)));
    assert_eq!(inline.span(), Span::from((start, end)));

    let inline_content = inline.into_inner().into_nested();
    let mut inner_inlines = inline_content.content.iter();

    let inline = inner_inlines.next().unwrap();

    let start = Position::new(1, 2);
    let end = start + (0, 16 - 1);

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Plain(&PlainContent {
            content: String::from("This text group "),
            span: Span::from((start, end))
        })
    );

    let inline = inner_inlines.next().unwrap();

    let start = end + (0, 1);
    let end = start + (0, 24 - 1);

    assert!(matches!(inline, Inline::TextGroup(_)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Nested(&NestedContent {
            content: vec![Inline::Plain(PlainContent {
                content: String::from("has another one inside"),
                span: Span::from((start + (0, 1), end - (0, 1)))
            })]
            .into(),
            span: Span::from((start, end))
        })
    );

    let inline = inner_inlines.next().unwrap();

    let start = end + (0, 1);
    let end = start + (0, 7 - 1);

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Plain(&PlainContent {
            content: String::from(" of it."),
            span: Span::from((start, end))
        })
    );
}

#[test]
fn parse_open_italic_closed_bold_in_tg() {
    let input = "This huhuu [***This is input**]";
    let mut parser = input.parse_inlines(None);

    let inline = parser.next().unwrap();

    let start = Position::new(1, 1);
    let end = start + (0, 11 - 1);

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Plain(&PlainContent {
            content: String::from("This huhuu "),
            span: Span::from((start, end))
        })
    );

    let inline = parser.next().unwrap();

    let start = end + (0, 1);
    let end = start + (0, 20 - 1);

    assert!(matches!(inline, Inline::TextGroup(_)));
    assert_eq!(inline.span(), Span::from((start, end)));

    let mut inner = inline.into_inner().into_nested();
    let mut inner = inner.content.drain(..);

    let inline = inner.next().unwrap();
    let start = start + (0, 1);
    let end = start;

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Plain(&PlainContent {
            content: String::from("*"),
            span: Span::from((start, end))
        })
    );

    let inline = inner.next().unwrap();
    let start = start + (0, 1);
    let end = start + (0, 17 - 1);

    assert!(matches!(inline, Inline::Bold(_)));
    assert!(matches!(inline.as_ref(), InlineContent::Nested(_)));
    assert_eq!(inline.span(), Span::from((start, end)));

    let mut inner = inline.into_inner().into_nested();
    let mut inner = inner.content.drain(..);

    let inline = inner.next().unwrap();
    let start = start + (0, 2);
    let end = start + (0, 13 - 1);

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Plain(&PlainContent {
            content: String::from("This is input"),
            span: Span::from((start, end))
        })
    )
}

#[test]
fn interrupt_italic() {
    let input = "**This *is input**";
    let mut parser = input.parse_inlines(None);

    let inline = parser.next().unwrap();
    let start = Position::new(1, 1);
    let end = start + (0, 18 - 1);

    assert!(matches!(inline, Inline::Bold(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
    assert!(matches!(inline.as_ref(), InlineContent::Nested(_)));

    let mut inner = inline.into_inner().into_nested();
    let mut inner = inner.content.drain(..);

    let inline = inner.next().unwrap();
    let start = start + (0, 2);
    let end = start + (0, 14 - 1);

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Plain(&PlainContent {
            content: String::from("This *is input"),
            span: Span::from((start, end))
        })
    );
}

#[test]
fn parse_multi_line() {
    let input = "This is\ninput with\nmulti-line content.";

    let mut parser = input.parse_inlines(None);

    for inline in parser.clone() {
        dbg!(inline);
    }

    let inline = parser.next().unwrap();
    let start = Position::new(1, 1);
    let end = Position::new(1, 7);

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Plain(&PlainContent {
            content: String::from("This is"),
            span: Span::from((start, end))
        })
    );

    let inline = parser.next().unwrap();
    let start = Position::new(1, 8);
    let end = Position::new(1, 8);

    assert!(matches!(inline, Inline::EndOfLine(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Plain(&PlainContent {
            content: String::from("\n"),
            span: Span::from((start, end))
        })
    );

    let inline = parser.next().unwrap();
    let start = Position::new(2, 1);
    let end = Position::new(2, 10);

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Plain(&PlainContent {
            content: String::from("input with"),
            span: Span::from((start, end))
        })
    );

    let inline = parser.next().unwrap();
    let start = Position::new(2, 11);
    let end = Position::new(2, 11);

    assert!(matches!(inline, Inline::EndOfLine(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Plain(&PlainContent {
            content: String::from("\n"),
            span: Span::from((start, end))
        })
    );

    let inline = parser.next().unwrap();
    let start = Position::new(3, 1);
    let end = Position::new(3, 19);

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Plain(&PlainContent {
            content: String::from("multi-line content."),
            span: Span::from((start, end))
        })
    );
}

#[test]
fn parse_subst_alias() {
    let input = "This is text::with_alias::substitution inside.";

    let mut parser = input.parse_inlines(None);

    for inline in parser.clone() {
        dbg!(inline);
    }

    let inline = parser.next().unwrap();
    let start = Position::new(1, 1);
    let end = Position::new(1, 12);

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(inline.span(), Span::from((start, end)));

    assert_eq!(
        inline.as_ref(),
        InlineContent::Plain(&PlainContent {
            content: String::from("This is text"),
            span: Span::from((start, end))
        })
    );

    let inline = parser.next().unwrap();
    let start = Position::new(1, 13);
    let end = Position::new(1, 26);

    assert!(matches!(inline, Inline::Substitution(_)));
    assert_eq!(inline.span(), Span::from((start, end)));

    assert!(matches!(inline.as_ref(), InlineContent::Nested(_)));

    let inner_inline = &inline.into_inner().into_nested().content[0];
    let inner_start = Position::new(1, 15);
    let inner_end = Position::new(1, 24);
    let span = Span::from((inner_start, inner_end));

    assert!(matches!(inner_inline, Inline::Plain(_)));
    assert_eq!(
        inner_inline.as_ref(),
        InlineContent::Plain(&PlainContent {
            content: String::from("with_alias"),
            span,
        })
    );

    let inline = parser.next().unwrap();
    let start = Position::new(1, 27);
    let end = Position::new(1, 46);

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(inline.span(), Span::from((start, end)));

    assert_eq!(
        inline.as_ref(),
        InlineContent::Plain(&PlainContent {
            content: String::from("substitution inside."),
            span: Span::from((start, end))
        })
    );
}

#[test]
fn verbatim_with_escaped_accent() {
    let input = "`verbatim\\`escaped`";
    let mut parser = input.parse_inlines(None);

    let inline = parser.next().unwrap();
    let start = Position::new(1, 1);
    let end = start + (0, 19 - 1); // 19 because one backslash used for backslash escape in Rust

    assert!(matches!(inline, Inline::Verbatim(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
    assert!(matches!(inline.as_ref(), InlineContent::Plain(_)));

    let inner = inline.into_inner().into_plain();

    assert_eq!(
        inner,
        PlainContent {
            content: String::from("verbatim`escaped"),
            span: Span::from((start, end))
        }
    );
}

#[test]
fn plain_asterisk_no_bold() {
    let input = "******no bold**";
    let mut parser = input.parse_inlines(None);

    let inline = parser.next().unwrap();
    let start = Position::new(1, 1);
    let end = start + (0, input.chars().count() - 1);

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
    assert!(matches!(inline.as_ref(), InlineContent::Plain(_)));

    let inner = inline.into_inner().into_plain();

    assert_eq!(
        inner,
        PlainContent {
            content: String::from("******no bold**"),
            span: Span::from((start, end))
        }
    );
}

#[test]
fn two_open_italic_closing_at_end() {
    let input = "*open *2nd closing*";
    let mut parser = input.parse_inlines(None);

    let inline = parser.next().unwrap();
    let start = Position::new(1, 1);
    let end = start + (0, input.chars().count() - 1);

    assert!(matches!(inline, Inline::Italic(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
    assert!(matches!(inline.as_ref(), InlineContent::Nested(_)));

    let mut inner = inline.into_inner().into_nested();
    let mut inner = inner.content.drain(..);

    let inline = inner.next().unwrap();
    let start = start + (0, 1);
    let end = start + (0, input.chars().count() - (2 + 1));

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(
        inline.as_ref(),
        InlineContent::Plain(&PlainContent {
            content: String::from("open *2nd closing"),
            span: Span::from((start, end))
        })
    );
}

#[test]
fn bold_around_text_group() {
    let input = "**This bold [group**] and close** bold..";
    let mut parser = input.parse_inlines(None);

    let inline = parser.next().unwrap();
    let start = Position::new(1, 1);
    let end = Position::new(1, 33);

    assert!(matches!(inline, Inline::Bold(_)));
    assert_eq!(inline.span(), Span::from((start, end)));

    let mut inner = inline.into_inner().into_nested();
    let mut inner = inner.content.drain(..);

    let inline = inner.next().unwrap();
    let start = Position::new(1, 3);
    let end = Position::new(1, 12);

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(inline.span(), Span::from((start, end)));

    let inline = inner.next().unwrap();
    let start = Position::new(1, 13);
    let end = Position::new(1, 21);

    assert!(matches!(inline, Inline::TextGroup(_)));
    assert_eq!(inline.span(), Span::from((start, end)));

    let inline = inner.next().unwrap();
    let start = Position::new(1, 22);
    let end = Position::new(1, 31);

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(inline.span(), Span::from((start, end)));

    let inline = parser.next().unwrap();
    let start = Position::new(1, 34);
    let end = Position::new(1, 40);

    assert!(matches!(inline, Inline::Plain(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
}

#[test]
fn ambiguous_close_italic_then_bold() {
    let input = "***This bold* haha**";
    let mut parser = input.parse_inlines(None);

    let inline = parser.next().unwrap();
    let start = Position::new(1, 1);
    let end = Position::new(1, 20);

    assert!(matches!(inline, Inline::Bold(_)));
    assert_eq!(inline.span(), Span::from((start, end)));

    let mut inner = inline.into_inner().into_nested();
    let mut inner = inner.content.drain(..);

    let inline = inner.next().unwrap();
    let start = Position::new(1, 3);
    let end = Position::new(1, 13);

    assert!(matches!(inline, Inline::Italic(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
}

#[test]
fn italicbold() {
    let input = "***bold italic***";
    let mut parser = input.parse_inlines(None);

    // for inline in parser.clone() {
    //     dbg!(inline);
    // }

    let inline = parser.next().unwrap();
    let start = Position::new(1, 1);
    let end = Position::new(1, 17);

    let first_token_len = match inline {
        Inline::Bold(_) => 2,
        Inline::Italic(_) => 1,
        _ => 0,
    };

    assert!(matches!(inline, Inline::Bold(_) | Inline::Italic(_)));
    assert_eq!(inline.span(), Span::from((start, end)));

    let mut inner = inline.into_inner().into_nested();
    let mut inner = inner.content.drain(..);

    let inline = inner.next().unwrap();
    let start = Position::new(1, 1 + first_token_len);
    let end = end - (0, first_token_len);

    assert!(matches!(inline, Inline::Bold(_) | Inline::Italic(_)));
    assert_eq!(inline.span(), Span::from((start, end)));
}
