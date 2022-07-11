use super::*;

mod italic {
    use super::*;

    #[test]
    fn no_spacing() {
        let input = "*";

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 1);

        assert!(token.opens());
        assert!(token.closes());

        assert_token! {
            token with
                TokenKind::Italic,
                Spacing::None,
                (start, end),
                "*"
        };
    }

    #[test]
    fn spacing_pre() {
        let input = " *";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 2);

        assert!(token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Italic,
                Spacing::Pre,
                (start, end),
                "*"
        };
    }

    #[test]
    fn spacing_post() {
        let input = "* ";

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 1);

        assert!(!token.opens());
        assert!(token.closes());

        assert_token! {
            token with
                TokenKind::Italic,
                Spacing::Post,
                (start, end),
                "*"
        };
    }

    #[test]
    fn spacing_both() {
        let input = " * ";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 2);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Italic,
                Spacing::Both,
                (start, end),
                "*"
        };
    }
}

mod bold {
    use super::*;

    #[test]
    fn no_spacing() {
        let input = "**";

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 2);

        assert!(token.opens());
        assert!(token.closes());

        assert_token! {
            token with
                TokenKind::Bold,
                Spacing::None,
                (start, end),
                "**"
        };
    }

    #[test]
    fn spacing_pre() {
        let input = " **";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 3);

        assert!(token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Bold,
                Spacing::Pre,
                (start, end),
                "**"
        };
    }

    #[test]
    fn spacing_post() {
        let input = "** ";

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 2);

        assert!(!token.opens());
        assert!(token.closes());

        assert_token! {
            token with
                TokenKind::Bold,
                Spacing::Post,
                (start, end),
                "**"
        };
    }

    #[test]
    fn spacing_both() {
        let input = " ** ";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 3);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Bold,
                Spacing::Both,
                (start, end),
                "**"
        };
    }
}

mod italic_bold {
    use super::*;

    #[test]
    fn no_spacing() {
        let input = "***";

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 3);

        assert!(token.opens());
        assert!(token.closes());

        assert_token! {
            token with
                TokenKind::ItalicBold,
                Spacing::None,
                (start, end),
                "***"
        };
    }

    #[test]
    fn spacing_pre() {
        let input = " ***";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 4);

        assert!(token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::ItalicBold,
                Spacing::Pre,
                (start, end),
                "***"
        };
    }

    #[test]
    fn spacing_post() {
        let input = "*** ";

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 3);

        assert!(!token.opens());
        assert!(token.closes());

        assert_token! {
            token with
                TokenKind::ItalicBold,
                Spacing::Post,
                (start, end),
                "***"
        };
    }

    #[test]
    fn spacing_both() {
        let input = " *** ";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 4);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::ItalicBold,
                Spacing::Both,
                (start, end),
                "***"
        };
    }
}

mod plain {
    use super::*;

    #[test]
    fn no_spacing() {
        let input = "****";

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 4);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "****"
        };
    }

    #[test]
    fn spacing_pre() {
        let input = " ****";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 5);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Pre,
                (start, end),
                "****"
        };
    }

    #[test]
    fn spacing_post() {
        let input = "**** ";

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 4);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Post,
                (start, end),
                "****"
        };
    }

    #[test]
    fn spacing_both() {
        let input = " **** ";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 5);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Both,
                (start, end),
                "****"
        };
    }
}
