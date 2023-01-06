use super::*;

mod single {
    mod open {
        use super::super::*;

        #[test]
        fn no_spacing() {
            let input = "a{a";

            let token = input.lex_iter().nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBrace,
                    Spacing::None,
                    (start, end),
                    "{"
            };
        }

        #[test]
        fn spacing_pre() {
            let input = " {a";

            let token = input.lex_iter().nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBrace,
                    Spacing::Pre,
                    (start, end),
                    "{"
            };
        }

        #[test]
        fn spacing_post() {
            let input = "a{ ";

            let token = input.lex_iter().nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBrace,
                    Spacing::Post,
                    (start, end),
                    "{"
            };
        }

        #[test]
        fn spacing_both() {
            let input = " { ";

            let token = input.lex_iter().nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBrace,
                    Spacing::Both,
                    (start, end),
                    "{"
            };
        }
    }

    mod close {
        use super::super::*;

        #[test]
        fn no_spacing() {
            let input = "a}a";

            let token = input.lex_iter().nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBrace,
                    Spacing::None,
                    (start, end),
                    "}"
            };
        }

        #[test]
        fn spacing_pre() {
            let input = " }a";

            let token = input.lex_iter().nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBrace,
                    Spacing::Pre,
                    (start, end),
                    "}"
            };
        }

        #[test]
        fn spacing_post() {
            let input = "a} ";

            let token = input.lex_iter().nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBrace,
                    Spacing::Post,
                    (start, end),
                    "}"
            };
        }

        #[test]
        fn spacing_both() {
            let input = " } ";

            let token = input.lex_iter().nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBrace,
                    Spacing::Both,
                    (start, end),
                    "}"
            };
        }
    }
}

mod multiple {
    mod open {
        use super::super::*;

        #[test]
        fn no_spacing() {
            let input = "a{{a";

            let mut iter = input.lex_iter();

            let token = iter.nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBrace,
                    Spacing::None,
                    (start, end),
                    "{"
            };

            let token = iter.next().unwrap();
            let start = Position::new(1, 3);
            let end = Position::new(1, 3);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBrace,
                    Spacing::None,
                    (start, end),
                    "{"
            };
        }

        #[test]
        fn spacing_pre() {
            let input = " {{a";

            let mut iter = input.lex_iter();

            let token = iter.nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBrace,
                    Spacing::Pre,
                    (start, end),
                    "{"
            };

            let token = iter.next().unwrap();
            let start = Position::new(1, 3);
            let end = Position::new(1, 3);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBrace,
                    Spacing::None,
                    (start, end),
                    "{"
            };
        }

        #[test]
        fn spacing_post() {
            let input = "a{{ ";

            let mut iter = input.lex_iter();

            let token = iter.nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBrace,
                    Spacing::None,
                    (start, end),
                    "{"
            };

            let token = iter.next().unwrap();
            let start = Position::new(1, 3);
            let end = Position::new(1, 3);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBrace,
                    Spacing::Post,
                    (start, end),
                    "{"
            };
        }

        #[test]
        fn spacing_both() {
            let input = " {{ ";

            let mut iter = input.lex_iter();

            let token = iter.nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBrace,
                    Spacing::Pre,
                    (start, end),
                    "{"
            };

            let token = iter.next().unwrap();
            let start = Position::new(1, 3);
            let end = Position::new(1, 3);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBrace,
                    Spacing::Post,
                    (start, end),
                    "{"
            };
        }
    }

    mod close {
        use super::super::*;

        #[test]
        fn no_spacing() {
            let input = "a}}a";

            let mut iter = input.lex_iter();

            let token = iter.nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBrace,
                    Spacing::None,
                    (start, end),
                    "}"
            };

            let token = iter.next().unwrap();
            let start = Position::new(1, 3);
            let end = Position::new(1, 3);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBrace,
                    Spacing::None,
                    (start, end),
                    "}"
            };
        }

        #[test]
        fn spacing_pre() {
            let input = " }}a";

            let mut iter = input.lex_iter();

            let token = iter.nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBrace,
                    Spacing::Pre,
                    (start, end),
                    "}"
            };

            let token = iter.next().unwrap();
            let start = Position::new(1, 3);
            let end = Position::new(1, 3);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBrace,
                    Spacing::None,
                    (start, end),
                    "}"
            };
        }

        #[test]
        fn spacing_post() {
            let input = "a}} ";

            let mut iter = input.lex_iter();

            let token = iter.nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBrace,
                    Spacing::None,
                    (start, end),
                    "}"
            };

            let token = iter.next().unwrap();
            let start = Position::new(1, 3);
            let end = Position::new(1, 3);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBrace,
                    Spacing::Post,
                    (start, end),
                    "}"
            };
        }

        #[test]
        fn spacing_both() {
            let input = " }} ";

            let mut iter = input.lex_iter();

            let token = iter.nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBrace,
                    Spacing::Pre,
                    (start, end),
                    "}"
            };

            let token = iter.next().unwrap();
            let start = Position::new(1, 3);
            let end = Position::new(1, 3);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBrace,
                    Spacing::Post,
                    (start, end),
                    "}"
            };
        }
    }
}
