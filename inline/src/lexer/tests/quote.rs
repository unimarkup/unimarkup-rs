use super::*;

mod quotation {
    use super::*;

    #[test]
    fn no_spacing() {
        let input = r#""""#;

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 2);

        dbg!(&token);

        assert!(token.opens());
        assert!(token.closes());

        assert_token! {
            token with
                TokenKind::Quote,
                Spacing::None,
                (start, end),
                r#""""#
        };
    }

    #[test]
    fn spacing_pre() {
        let input = r#" """#;

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 3);

        assert!(token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Quote,
                Spacing::Pre,
                (start, end),
                r#""""#
        };
    }

    #[test]
    fn spacing_post() {
        let input = r#""" "#;

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 2);

        assert!(!token.opens());
        assert!(token.closes());

        assert_token! {
            token with
                TokenKind::Quote,
                Spacing::Post,
                (start, end),
                r#""""#
        };
    }

    #[test]
    fn spacing_both() {
        let input = r#" "" "#;

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 3);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Quote,
                Spacing::Both,
                (start, end),
                r#""""#
        };
    }
}

mod plain {
    use super::*;

    #[test]
    fn no_spacing() {
        let input = r#"""""#;

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 3);

        dbg!(&token);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                r#"""""#
        };
    }

    #[test]
    fn spacing_pre() {
        let input = r#" """"#;

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 4);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Pre,
                (start, end),
                r#"""""#
        };
    }

    #[test]
    fn spacing_post() {
        let input = r#"""" "#;

        let token = input.lex_iter().next().unwrap();
        let start = Position::new(1, 1);
        let end = Position::new(1, 3);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Post,
                (start, end),
                r#"""""#
        };
    }

    #[test]
    fn spacing_both() {
        let input = r#" """ "#;

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 4);

        assert!(!token.opens());
        assert!(!token.closes());

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Both,
                (start, end),
                r#"""""#
        };
    }
}
