use super::*;

mod lex_offset {
    use super::*;

    #[test]
    fn lex_with_offs() {
        let input = "Some text\nthat spans\nmultiple lines of text.";

        let offset_pos = Position::new(3, 10);
        let lexer = input.lex_with_offs(offset_pos);

        for token in &lexer {
            let start = Position::new(3, 10);
            let end = Position::new(3, 23);
            assert_token! {
                token with
                    TokenKind::Plain,
                    Spacing::Both,
                    (start, end),
                    "lines of text."
            };
        }
    }

    #[test]
    fn lex_iter_with_offs() {
        let input = "Some text\nthat spans\nmultiple lines of text.";

        let offset_pos = Position::new(3, 10);
        let iter = input.lex_iter_with_offs(offset_pos);

        for token in iter {
            let start = Position::new(3, 10);
            let end = Position::new(3, 23);
            assert_token! {
                token with
                    TokenKind::Plain,
                    Spacing::Both,
                    (start, end),
                    "lines of text."
            };
        }
    }
}

mod plain {
    use super::*;

    #[test]
    fn simple_text() {
        let input = "This is some text";

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 17);

        assert!(!token.opens());
        assert!(!token.closes());
        assert!(!token.is_nesting_token());
        assert!(!token.is_ambiguous());

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Both,
                (start, end),
                input
        };
    }

    #[test]
    fn multi_line() {
        let input = "This is first line\nAnd this second \nAnd third.";

        let mut iter = input.lex_iter();

        let token = iter.next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 18);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Both,
                (start, end),
                "This is first line"
        };

        let token = iter.next().unwrap();
        let start = Position::new(1, 19);
        let end = Position::new(1, 19);

        assert_token! {
            token with
                TokenKind::EndOfLine,
                Spacing::None,
                (start, end)
        };

        let token = iter.next().unwrap();
        let start = Position::new(2, 1);
        let end = Position::new(2, 16);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Both,
                (start, end),
                "And this second "
        };

        let token = iter.next().unwrap();
        let start = Position::new(2, 17);
        let end = Position::new(2, 17);

        assert_token! {
            token with
                TokenKind::EndOfLine,
                Spacing::Pre,
                (start, end)
        };

        let token = iter.next().unwrap();
        let start = Position::new(3, 1);
        let end = Position::new(3, 10);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Both,
                (start, end),
                "And third."
        };

        let token = iter.next();
        assert_eq!(token, None);
    }
}

mod escape {
    mod whitespace {
        use super::super::*;

        #[test]
        fn newline() {
            let input = "Escaped\\\nnewline";

            let token = input.lex_iter().nth(1).unwrap();
            let start = Position::new(1, 8);
            let end = Position::new(1, 9);

            assert_token! {
                token with
                    TokenKind::Newline,
                    Spacing::None,
                    (start, end),
                    "\n"
            };
        }

        #[test]
        fn space() {
            let input = "Escaped\\  space";

            let token = input.lex_iter().nth(1).unwrap();
            let start = Position::new(1, 8);
            let end = Position::new(1, 9);

            assert_token! {
                token with
                    TokenKind::Whitespace,
                    Spacing::None,
                    (start, end),
                    " "
            };
        }

        #[test]
        fn tab() {
            let input = "Escaped\\\t tab";

            let token = input.lex_iter().nth(1).unwrap();
            let start = Position::new(1, 8);
            let end = Position::new(1, 9);

            assert_token! {
                token with
                    TokenKind::Whitespace,
                    Spacing::None,
                    (start, end),
                    "\t"
            };
        }
    }

    mod keyword {
        use super::super::*;

        #[test]
        fn star() {
            let input = "This is \\* text with escaped star.";

            let token = input.lex_iter().next().unwrap();
            let start = Position::new(1, 1);
            let end = Position::new(1, 34);

            dbg!(&token);

            assert_token! {
                token with
                    TokenKind::Plain,
                    Spacing::Both,
                    (start, end),
                    "This is * text with escaped star."
            };
        }

        #[test]
        fn start_begin_of_line() {
            let input = "\\*This text escapes a start at the beginning of line.";

            let token = input.lex_iter().next().unwrap();
            let start = Position::new(1, 1);
            let end = Position::new(1, 53);

            dbg!(&token);

            assert_token! {
                token with
                    TokenKind::Plain,
                    Spacing::Both,
                    (start, end),
                    "*This text escapes a start at the beginning of line."
            };
        }
    }
}
