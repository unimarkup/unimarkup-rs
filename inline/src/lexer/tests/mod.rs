use super::*;

macro_rules! assert_token {
    ($token:ident with $kind:expr, $spacing:expr, $span:expr) => {
        assert_eq!($token.kind(), $kind);
        assert_eq!($token.spacing(), $spacing);
        assert_eq!($token.span(), Span::from($span));
        true
    };

    ($token:ident with $kind:expr, $spacing:expr, $span:expr, $content:expr) => {
        assert_token!($token with $kind, $spacing, $span);
        assert_eq!($token.as_str(), $content);
        true
    }
}

mod caret;
mod overline;
mod pipe;
mod star;
mod tick;
mod underline;

#[test]
fn lines() {
    let input = r#"first line

            third line"#;

    assert_eq!(input.lines().count(), 3);
}

#[test]
fn test_lex_with_offs() {
    let pos_offs = Position::new(1, 5);
    let lexer = "Some string".lex_with_offs(pos_offs);
    let mut iter = lexer.into_iter();

    let token = iter.next().unwrap();
    let start = Position::new(1, 5);
    let end = start + (0, 7 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            " string"
    };
}

#[test]
fn test_lex_with_line_offs() {
    let input = "This is some\ntext with multiple lines.";

    let pos_offs = Position::new(2, 20);
    let lexer = input.lex_with_offs(pos_offs);
    let mut iter = lexer.into_iter();

    let token = iter.next().unwrap();
    let start = Position::new(2, 20);
    let end = start + (0, 6 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "lines."
    };
}

#[test]
fn test_iter_with_offs() {
    let pos = Position::new(1, 5);
    let mut iter = "Some string".lex_iter_with_offs(pos);

    let token = iter.next().unwrap();
    let start = Position::new(1, 5);
    let end = start + (0, 7 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            " string"
    };
}

#[test]
fn test_iter_with_line_offs() {
    let input = "This is some\ntext with multiple lines.";

    let pos_offs = Position::new(2, 20);
    let mut iter = input.lex_iter_with_offs(pos_offs);

    let token = iter.next().unwrap();
    let start = Position::new(2, 20);
    let end = start + (0, 6 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "lines."
    };
}

#[test]
fn into_iter() {
    let lexer = "Some string".lex();

    for token in &lexer {
        println!("{:?}", token);
    }

    assert_eq!(lexer.into_iter().count(), 1);

    let lexer = r#"Some string
                with new line"#
        .lex();

    for token in &lexer {
        println!("{token:?}");
    }

    assert_eq!(lexer.into_iter().count(), 2);
}

#[test]
fn lex_plain() {
    let lexer = "Some string".lex();
    let mut iter = lexer.into_iter();

    let token = iter.next().unwrap();
    let start = Position::default();
    let end = start + (0, 11 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "Some string"
    };
}

#[test]
fn lex_parens() {
    let lexer = "(Some string".lex();
    let mut iter = lexer.into_iter();

    let token = iter.next().unwrap();
    let start = Position::default();
    let end = start + (0, 1 - 1);

    assert_token! {
        token with
            TokenKind::OpenParens,
            Spacing::None,
            (start, end),
            "("
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 11 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "Some string"
    };
}

#[test]
fn lex_plain_with_esc() {
    let input = "Some string \\* with escaped character";
    let lexer = input.lex();

    let mut iter = lexer.into_iter();

    assert_eq!(lexer.into_iter().count(), 1);

    let token = iter.next().unwrap();
    let start = Position::default();
    let end = start + (0, 37 - 1);

    println!("Parsed token {token:?}");

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "Some string * with escaped character"
    };
}

#[test]
fn lex_plain_with_esc_begin() {
    let input = "\\*This string has escape sequence at the beginning";
    let lexer = input.lex();

    let mut iter = lexer.into_iter();

    assert_eq!(lexer.into_iter().count(), 1);

    let token = iter.next().unwrap();

    println!("Parsed token {token:?}");

    let start = Position::default();
    let end = start + (0, 50 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "*This string has escape sequence at the beginning"
    };
}

#[test]
fn lex_simple_space() {
    let input = "\\ ";
    let lexer = input.lex();

    let mut iter = lexer.into_iter();

    let token = iter.next().unwrap();

    let start = Position::default();
    let end = start + (0, 2 - 1);

    assert_token! {
        token with
            TokenKind::Whitespace,
            Spacing::None,
            (start, end),
            " "
    };
}

#[test]
fn lex_tab_whitespace() {
    let input = "\\\t";
    let lexer = input.lex();

    let mut iter = lexer.into_iter();

    let token = iter.next().unwrap();

    let start = Position::default();
    let end = start + (0, 2 - 1);

    assert_token! {
        token with
            TokenKind::Whitespace,
            Spacing::None,
            (start, end),
            "\t"
    };
}

#[test]
fn lex_newline() {
    let input = "\\\n";
    let lexer = input.lex();

    let mut iter = lexer.into_iter();

    let token = iter.next().unwrap();

    let start = Position::default();
    let end = start + (0, 2 - 1);

    assert_token! {
        token with
            TokenKind::Newline,
            Spacing::None,
            (start, end),
            "\n"
    };
}

#[test]
fn lex_whitespace_plain_combined() {
    let input = "This is some text \\ with whitespace token";
    let lexer = input.lex();

    assert_eq!(lexer.iter().count(), 3);

    let mut iter = lexer.iter();

    // PLAIN
    let first = iter.next().unwrap();
    let start = Position::default();
    let end = start + (0, 18 - 1);

    assert_token! {
        first with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "This is some text "
    };

    // WHITESPACE
    let second = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 2 - 1);

    assert_token! {
        second with
            TokenKind::Whitespace,
            Spacing::None,
            (start, end),
            " "
    };

    // PLAIN
    let third = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 21 - 1);

    assert_token! {
        third with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "with whitespace token"
    };
}

#[test]
fn lex_bold() {
    let input = "**This is text in bold** whereas this isn't.";

    let lexer = input.lex();

    let mut iter = lexer.iter();

    assert_eq!(lexer.iter().count(), 4);

    let token = iter.next().unwrap();
    let start = Position::default();
    let end = start + (0, 1);

    assert_token! {
        token with
            TokenKind::Bold,
            Spacing::None,
            (start, end)
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 20 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "This is text in bold"
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 2 - 1);

    assert_token! {
        token with
            TokenKind::Bold,
            Spacing::Post,
            (start, end)
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 20 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            " whereas this isn't."
    };
}

#[test]
fn lex_bold_italic_combined() {
    let input = "****bold and italic***";

    let lexer = input.lex();

    let mut iter = lexer.iter();
    let token = iter.next().unwrap();
    let start = Position::new(1, 1);
    let end = start + (0, 4 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "****"
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 15 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "bold and italic"
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 3 - 1);

    assert_token! {
        token with
            TokenKind::ItalicBold,
            Spacing::None,
            (start, end)
    };
}

#[test]
fn lex_underline() {
    let mut iter = "__This is underlined__".lex().iter();

    let token = iter.next().unwrap();
    let start = Position::default();
    let end = start + (0, 2 - 1);

    assert_token! {
        token with
            TokenKind::Underline,
            Spacing::None,
            (start, end)
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 18 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "This is underlined"
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 2 - 1);

    assert_token! {
        token with
            TokenKind::Underline,
            Spacing::None,
            (start, end)
    };
}

#[test]
fn lex_subscript() {
    let mut iter = "_This is underlined_".lex().iter();

    let token = iter.next().unwrap();
    let start = Position::default();
    let end = start + (0, 1 - 1);

    assert_token! {
        token with
            TokenKind::Subscript,
            Spacing::None,
            (start, end)
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 18 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "This is underlined"
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 1 - 1);

    assert_token! {
        token with
            TokenKind::Subscript,
            Spacing::None,
            (start, end)
    };
}

#[test]
fn lex_underline_subscript() {
    let mut iter = "___Bla some__ text_ hehe _bla".lex().iter();

    let token = iter.next().unwrap();
    let start = Position::default();
    let end = start + (0, 3 - 1);

    assert_token! {
        token with
            TokenKind::UnderlineSubscript,
            Spacing::None,
            (start, end)
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 8 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "Bla some"
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 2 - 1);

    assert_token! {
        token with
            TokenKind::Underline,
            Spacing::Post,
            (start, end)
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 5 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            " text"
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 1 - 1);

    assert_token! {
        token with
            TokenKind::Subscript,
            Spacing::Post,
            (start, end)
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 6 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            " hehe "
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 1 - 1);

    assert_token! {
        token with
            TokenKind::Subscript,
            Spacing::Pre,
            (start, end)
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 3 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "bla"
    };
}

#[test]
fn lex_overline() {
    let input = "‾This is overlined‾";

    let mut iter = input.lex().iter();

    let token = iter.next().unwrap();
    let start = Position::default();
    let end = start + (0, 1 - 1);

    assert_token! {
        token with
            TokenKind::Overline,
            Spacing::None,
            (start, end)
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 17 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "This is overlined"
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 1 - 1);

    assert_token! {
        token with
            TokenKind::Overline,
            Spacing::None,
            (start, end)
    };
}

#[test]
fn lex_overline_too_long() {
    let input = "‾‾This is overlined‾‾‾";

    let mut iter = input.lex().iter();

    let token = iter.next().unwrap();
    let start = Position::default();
    let end = start + (0, 2 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "‾‾"
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 17 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "This is overlined"
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 3 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "‾‾‾"
    };
}

#[test]
fn lex_superscript() {
    let input = "^This is superscript^";

    let mut iter = input.lex().iter();

    let token = iter.next().unwrap();
    let start = Position::default();
    let end = start + (0, 1 - 1);

    assert_token! {
        token with
            TokenKind::Superscript,
            Spacing::None,
            (start, end)
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 19 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "This is superscript"
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 1 - 1);

    assert_token! {
        token with
            TokenKind::Superscript,
            Spacing::None,
            (start, end)
    };
}

#[test]
fn lex_superscript_too_long() {
    let input = "^^This is superscript^^^";

    let mut iter = input.lex().iter();

    let token = iter.next().unwrap();
    let start = Position::default();
    let end = start + (0, 2 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "^^"
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 19 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "This is superscript"
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 3 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "^^^"
    };
}

#[test]
fn lex_strikethrough() {
    let input = "~~This is strikethrough~~";

    let mut iter = input.lex().iter();

    let token = iter.next().unwrap();
    let start = Position::default();
    let end = start + (0, 2 - 1);

    assert_token! {
        token with
            TokenKind::Strikethrough,
            Spacing::None,
            (start, end)
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 21 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "This is strikethrough"
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 2 - 1);

    assert_token! {
        token with
            TokenKind::Strikethrough,
            Spacing::None,
            (start, end)
    };
}

#[test]
fn lex_strikethrough_too_long() {
    let input = "~This is not strikethrough~~~";

    let mut iter = input.lex().iter();

    let token = iter.next().unwrap();
    let start = Position::default();
    let end = start + (0, 1 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "~"
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 25 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "This is not strikethrough"
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 3 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end)
    };
}

#[test]
fn lex_late_highlight() {
    let input = "|||||Some text";

    let mut iter = input.lex().iter();

    let token = iter.next().unwrap();
    let start = Position::default();
    let end = start + (0, 5 - 1);

    dbg!(&token);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "|||||"
    };

    let token = iter.next().unwrap();
    let start = end + (0, 1);
    let end = start + (0, 9 - 1);

    assert_token! {
        token with
            TokenKind::Plain,
            Spacing::None,
            (start, end),
            "Some text"
    };
}
