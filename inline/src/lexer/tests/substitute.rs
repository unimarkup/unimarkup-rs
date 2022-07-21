use super::*;

mod emoji_or_plain {
    use super::*;

    #[test]
    fn no_spacing() {
        let input = "a:Da";

        let mut iter = input.lex_iter();

        let token = iter.nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 2);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                ":"
        };

        let token = iter.next().unwrap();
        let start = Position::new(1, 3);
        let end = Position::new(1, 4);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Post,
                (start, end),
                "Da"
        };
    }

    #[test]
    fn spacing_pre() {
        let input = " :Da";

        let mut iter = input.lex_iter();

        let token = iter.nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 2);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Pre,
                (start, end),
                ":"
        };

        let token = iter.next().unwrap();
        let start = Position::new(1, 3);
        let end = Position::new(1, 4);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Post,
                (start, end),
                "Da"
        };
    }

    #[test]
    fn spacing_post() {
        let input = "a:D ";

        let mut iter = input.lex_iter();

        let token = iter.nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 2);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                ":"
        };

        let token = iter.next().unwrap();
        let start = Position::new(1, 3);
        let end = Position::new(1, 4);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Post,
                (start, end),
                "D "
        };
    }

    #[test]
    fn spacing_both() {
        let input = " :D ";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 3);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Both,
                (start, end),
                "\u{1F603}"
        };
    }
}

mod emoji_or_keyword {
    use super::*;

    #[test]
    fn no_spacing() {
        let input = "a(Y)a";

        let mut iter = input.lex_iter();

        let token = iter.nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 2);

        assert_token! {
            token with
                TokenKind::OpenParens,
                Spacing::None,
                (start, end)
        };

        let token = iter.next().unwrap();
        let start = Position::new(1, 3);
        let end = Position::new(1, 3);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "Y"
        };

        let token = iter.next().unwrap();
        let start = Position::new(1, 4);
        let end = Position::new(1, 4);

        assert_token! {
            token with
                TokenKind::CloseParens,
                Spacing::None,
                (start, end),
                ")"
        };
    }

    #[test]
    fn spacing_pre() {
        let input = " (Y)a";

        let mut iter = input.lex_iter();

        let token = iter.nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 2);

        assert_token! {
            token with
                TokenKind::OpenParens,
                Spacing::Pre,
                (start, end)
        };

        let token = iter.next().unwrap();
        let start = Position::new(1, 3);
        let end = Position::new(1, 3);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "Y"
        };

        let token = iter.next().unwrap();
        let start = Position::new(1, 4);
        let end = Position::new(1, 4);

        assert_token! {
            token with
                TokenKind::CloseParens,
                Spacing::None,
                (start, end)
        };
    }

    #[test]
    fn spacing_post() {
        let input = "a(Y) ";

        let mut iter = input.lex_iter();

        let token = iter.nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 2);

        assert_token! {
            token with
                TokenKind::OpenParens,
                Spacing::None,
                (start, end)
        };

        let token = iter.next().unwrap();
        let start = Position::new(1, 3);
        let end = Position::new(1, 3);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::None,
                (start, end),
                "Y"
        };

        let token = iter.next().unwrap();
        let start = Position::new(1, 4);
        let end = Position::new(1, 4);

        assert_token! {
            token with
                TokenKind::CloseParens,
                Spacing::Post,
                (start, end)
        };
    }

    #[test]
    fn spacing_both() {
        let input = " (Y) ";

        let token = input.lex_iter().nth(1).unwrap();
        let start = Position::new(1, 2);
        let end = Position::new(1, 4);

        assert_token! {
            token with
                TokenKind::Plain,
                Spacing::Both,
                (start, end),
                "\u{1F44D}"
        };
    }
}
