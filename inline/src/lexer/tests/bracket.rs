use super::*;

mod single {
    mod open {
        use super::super::*;

        #[test]
        fn no_spacing() {
            let input = "[";

            let token = input.lex_iter().next().unwrap();
            let start = Position::new(1, 1);
            let end = Position::new(1, 1);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBracket,
                    Spacing::None,
                    (start, end),
                    "["
            };
        }

        #[test]
        fn spacing_pre() {
            let input = " [";

            let token = input.lex_iter().nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBracket,
                    Spacing::Pre,
                    (start, end),
                    "["
            };
        }

        #[test]
        fn spacing_post() {
            let input = "[ ";

            let token = input.lex_iter().next().unwrap();
            let start = Position::new(1, 1);
            let end = Position::new(1, 1);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBracket,
                    Spacing::Post,
                    (start, end),
                    "["
            };
        }

        #[test]
        fn spacing_both() {
            let input = " [ ";

            let token = input.lex_iter().nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBracket,
                    Spacing::Both,
                    (start, end),
                    "["
            };
        }
    }

    mod close {
        use super::super::*;

        #[test]
        fn no_spacing() {
            let input = "]";

            let token = input.lex_iter().next().unwrap();
            let start = Position::new(1, 1);
            let end = Position::new(1, 1);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBracket,
                    Spacing::None,
                    (start, end),
                    "]"
            };
        }

        #[test]
        fn spacing_pre() {
            let input = " ]";

            let token = input.lex_iter().nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBracket,
                    Spacing::Pre,
                    (start, end),
                    "]"
            };
        }

        #[test]
        fn spacing_post() {
            let input = "] ";

            let token = input.lex_iter().next().unwrap();
            let start = Position::new(1, 1);
            let end = Position::new(1, 1);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBracket,
                    Spacing::Post,
                    (start, end),
                    "]"
            };
        }

        #[test]
        fn spacing_both() {
            let input = " ] ";

            let token = input.lex_iter().nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBracket,
                    Spacing::Both,
                    (start, end),
                    "]"
            };
        }
    }
}

mod multiple {
    mod open {
        use super::super::*;

        #[test]
        fn no_spacing() {
            let input = "[[";

            let mut iter = input.lex_iter();

            let token = iter.next().unwrap();
            let start = Position::new(1, 1);
            let end = Position::new(1, 1);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBracket,
                    Spacing::None,
                    (start, end),
                    "["
            };

            let token = iter.next().unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBracket,
                    Spacing::None,
                    (start, end),
                    "["
            };
        }

        #[test]
        fn spacing_pre() {
            let input = " [[";

            let mut iter = input.lex_iter();

            let token = iter.nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBracket,
                    Spacing::Pre,
                    (start, end),
                    "["
            };

            let token = iter.next().unwrap();
            let start = Position::new(1, 3);
            let end = Position::new(1, 3);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBracket,
                    Spacing::None,
                    (start, end),
                    "["
            };
        }

        #[test]
        fn spacing_post() {
            let input = "[[ ";

            let mut iter = input.lex_iter();

            let token = iter.next().unwrap();
            let start = Position::new(1, 1);
            let end = Position::new(1, 1);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBracket,
                    Spacing::None,
                    (start, end),
                    "["
            };

            let token = iter.next().unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBracket,
                    Spacing::Post,
                    (start, end),
                    "["
            };
        }

        #[test]
        fn spacing_both() {
            let input = " [[ ";

            let mut iter = input.lex_iter();

            let token = iter.nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBracket,
                    Spacing::Pre,
                    (start, end),
                    "["
            };

            let token = iter.next().unwrap();
            let start = Position::new(1, 3);
            let end = Position::new(1, 3);

            assert!(token.opens());
            assert!(!token.closes());

            assert_token! {
                token with
                    TokenKind::OpenBracket,
                    Spacing::Post,
                    (start, end),
                    "["
            };
        }
    }

    mod close {
        use super::super::*;

        #[test]
        fn no_spacing() {
            let input = "]]";

            let mut iter = input.lex_iter();

            let token = iter.next().unwrap();
            let start = Position::new(1, 1);
            let end = Position::new(1, 1);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBracket,
                    Spacing::None,
                    (start, end),
                    "]"
            };

            let token = iter.next().unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBracket,
                    Spacing::None,
                    (start, end),
                    "]"
            };
        }

        #[test]
        fn spacing_pre() {
            let input = " ]]";

            let mut iter = input.lex_iter();

            let token = iter.nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBracket,
                    Spacing::Pre,
                    (start, end),
                    "]"
            };

            let token = iter.next().unwrap();
            let start = Position::new(1, 3);
            let end = Position::new(1, 3);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBracket,
                    Spacing::None,
                    (start, end),
                    "]"
            };
        }

        #[test]
        fn spacing_post() {
            let input = "]] ";

            let mut iter = input.lex_iter();

            let token = iter.next().unwrap();
            let start = Position::new(1, 1);
            let end = Position::new(1, 1);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBracket,
                    Spacing::None,
                    (start, end),
                    "]"
            };

            let token = iter.next().unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBracket,
                    Spacing::Post,
                    (start, end),
                    "]"
            };
        }

        #[test]
        fn spacing_both() {
            let input = " ]] ";

            let mut iter = input.lex_iter();

            let token = iter.nth(1).unwrap();
            let start = Position::new(1, 2);
            let end = Position::new(1, 2);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBracket,
                    Spacing::Pre,
                    (start, end),
                    "]"
            };

            let token = iter.next().unwrap();
            let start = Position::new(1, 3);
            let end = Position::new(1, 3);

            assert!(!token.opens());
            assert!(token.closes());

            assert_token! {
                token with
                    TokenKind::CloseBracket,
                    Spacing::Post,
                    (start, end),
                    "]"
            };
        }
    }
}
