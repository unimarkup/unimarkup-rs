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
                    Spacing::None,
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
                    Spacing::None,
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
                Spacing::None,
                (start, end),
                input
        };
    }

    #[test]
    fn multi_line() {
        let input = "This is first line\nAnd this second\nAnd third.";

        let mut iter = input.lex_iter();

        let token = iter.next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 18);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "This is first line"
        };

        let token = iter.next().unwrap();
        let start = Position::new(2, 1);
        let end = Position::new(2, 15);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "And this second"
        };

        let token = iter.next().unwrap();
        let start = Position::new(3, 1);
        let end = Position::new(3, 10);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "And third."
        };
    }
}

mod escape {
    mod whitespace {
        use super::super::*;

        #[test]
        fn newline() {
            let input = "Escaped\\\n newline";

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

            assert_token! {
                token with
                    TokenKind::Plain,
                    Spacing::None,
                    (start, end),
                    "This is * text with escaped star."
            };
        }
    }
}