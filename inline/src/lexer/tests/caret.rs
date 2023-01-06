use super::*;

mod superscript {
    use super::*;

    #[test]
    fn no_spacing() {
        let input = "a^a";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 2);

        assert!(token.opens());
        assert!(token.closes());

        assert_token! {
            token with
                TokenKind::Superscript,
                Spacing::None,
                (start, end),
                "^"
        };
    }

    #[test]
    fn spacing_pre() {
        let input = " ^a";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 2);

        assert!(token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Superscript,
                Spacing::Pre,
                (start, end),
                "^"
        };
    }

    #[test]
    fn spacing_post() {
        let input = "a^ ";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 2);

        assert!(!token.opens());
        assert!(token.closes());

        assert_token! {
            token with
                TokenKind::Superscript,
                Spacing::Post,
                (start, end),
                "^"
        };
    }

    #[test]
    fn spacing_both() {
        let input = " ^ ";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 2);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Superscript,
                Spacing::Both,
                (start, end),
                "^"
        };
    }
}

mod plain {
    use super::*;

    #[test]
    fn no_spacing() {
        let input = "a^^a";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 3);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "^^"
        };
    }

    #[test]
    fn spacing_pre() {
        let input = " ^^a";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 3);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Pre,
                (start, end),
                "^^"
        };
    }

    #[test]
    fn spacing_post() {
        let input = "a^^ ";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 3);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Post,
                (start, end),
                "^^"
        };
    }

    #[test]
    fn spacing_both() {
        let input = " ^^ ";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 3);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Both,
                (start, end),
                "\u{1F604}"
        };
    }
}
