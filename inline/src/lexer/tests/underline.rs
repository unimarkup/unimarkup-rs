use super::*;

mod subscript {
    use super::*;

    #[test]
    fn no_spacing() {
        let input = "_";

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 1);

        assert!(token.opens());
        assert!(token.closes());

        assert_token! {
            token with
                TokenKind::Subscript,
                Spacing::None,
                (start, end),
                "_"
        };
    }

    #[test]
    fn spacing_pre() {
        let input = " _";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 2);

        assert!(token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Subscript,
                Spacing::Pre,
                (start, end),
                "_"
        };
    }

    #[test]
    fn spacing_post() {
        let input = "_ ";

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 1);

        assert!(!token.opens());
        assert!(token.closes());

        assert_token! {
            token with
                TokenKind::Subscript,
                Spacing::Post,
                (start, end),
                "_"
        };
    }

    #[test]
    fn spacing_both() {
        let input = " _ ";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 2);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Subscript,
                Spacing::Both,
                (start, end),
                "_"
        };
    }
}

mod underline_token {
    use super::*;

    #[test]
    fn no_spacing() {
        let input = "__";

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 2);

        assert!(token.opens());
        assert!(token.closes());

        assert_token! {
            token with
                TokenKind::Underline,
                Spacing::None,
                (start, end),
                "__"
        };
    }

    #[test]
    fn spacing_pre() {
        let input = " __";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 3);

        assert!(token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Underline,
                Spacing::Pre,
                (start, end),
                "__"
        };
    }

    #[test]
    fn spacing_post() {
        let input = "__ ";

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 2);

        assert!(!token.opens());
        assert!(token.closes());

        assert_token! {
            token with
                TokenKind::Underline,
                Spacing::Post,
                (start, end),
                "__"
        };
    }

    #[test]
    fn spacing_both() {
        let input = " __ ";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 3);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Underline,
                Spacing::Both,
                (start, end),
                "__"
        };
    }
}

mod underline_subscript {
    use super::*;

    #[test]
    fn no_spacing() {
        let input = "___";

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 3);

        assert!(token.opens());
        assert!(token.closes());

        assert_token! {
            token with
                TokenKind::UnderlineSubscript,
                Spacing::None,
                (start, end),
                "___"
        };
    }

    #[test]
    fn spacing_pre() {
        let input = " ___";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 4);

        assert!(token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::UnderlineSubscript,
                Spacing::Pre,
                (start, end),
                "___"
        };
    }

    #[test]
    fn spacing_post() {
        let input = "___ ";

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 3);

        assert!(!token.opens());
        assert!(token.closes());

        assert_token! {
            token with
                TokenKind::UnderlineSubscript,
                Spacing::Post,
                (start, end),
                "___"
        };
    }

    #[test]
    fn spacing_both() {
        let input = " ___ ";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 4);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::UnderlineSubscript,
                Spacing::Both,
                (start, end),
                "___"
        };
    }
}

mod plain {
    use super::*;

    #[test]
    fn no_spacing() {
        let input = "____";

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
                "____"
        };
    }

    #[test]
    fn spacing_pre() {
        let input = " ____";

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
                "____"
        };
    }

    #[test]
    fn spacing_post() {
        let input = "____ ";

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
                "____"
        };
    }

    #[test]
    fn spacing_both() {
        let input = " ____ ";

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
                "____"
        };
    }
}
