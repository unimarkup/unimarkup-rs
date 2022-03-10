use crate::lexer::Lexer;
use crate::{Spacing, Token, TokenKind};

#[test]
fn lexer_plain() {
    let input = r#"This is some text"#;
    let mut lexer = Lexer::new(input);

    let expected = Token::new(TokenKind::Plain(String::from(input)), Spacing::Neither);

    assert_eq!(lexer.next(), Some(expected));
    assert_eq!(lexer.next(), None);
}

#[test]
fn lexer_empty_bold() {
    let input = r#"****"#;
    let mut lexer = Lexer::new(input);

    assert_eq!(
        lexer.next(),
        Some(Token::new(TokenKind::Bold, Spacing::Neither))
    );

    assert_eq!(
        lexer.next(),
        Some(Token::new(TokenKind::Bold, Spacing::Neither))
    );
}

#[test]
fn lexer_bold_plain() {
    let input = r#"**Some text in bold**"#;
    let mut lexer = Lexer::new(input);

    assert_eq!(
        lexer.next(),
        Some(Token::new(TokenKind::Bold, Spacing::Neither))
    );

    assert_eq!(
        lexer.next(),
        Some(Token::new(
            TokenKind::Plain(String::from("Some text in bold")),
            Spacing::Neither
        ))
    );

    assert_eq!(
        lexer.next(),
        Some(Token::new(TokenKind::Bold, Spacing::Neither))
    );
}

#[test]
fn lexer_italic_plain() {
    let input = r#"*Some text in italic*"#;
    let mut lexer = Lexer::new(input);

    assert_eq!(
        lexer.next(),
        Some(Token::new(TokenKind::Italic, Spacing::Neither))
    );

    assert_eq!(
        lexer.next(),
        Some(Token::new(
            TokenKind::Plain(String::from("Some text in italic")),
            Spacing::Neither
        ))
    );

    assert_eq!(
        lexer.next(),
        Some(Token::new(TokenKind::Italic, Spacing::Neither))
    );
}

#[test]
fn lexer_italic_bold_plain() {
    let input = r#"*Some text in italic **with bold** inside*"#;
    let mut lexer = Lexer::new(input);

    assert_eq!(
        lexer.next(),
        Some(Token::new(TokenKind::Italic, Spacing::Neither))
    );

    assert_eq!(
        lexer.next(),
        Some(Token::new(
            TokenKind::Plain(String::from("Some text in italic ")),
            Spacing::Neither
        ))
    );

    assert_eq!(
        lexer.next(),
        Some(Token::new(TokenKind::Bold, Spacing::Pre))
    );

    assert_eq!(
        lexer.next(),
        Some(Token::new(
            TokenKind::Plain(String::from("with bold")),
            Spacing::Neither
        ))
    );

    assert_eq!(
        lexer.next(),
        Some(Token::new(TokenKind::Bold, Spacing::Post))
    );

    assert_eq!(
        lexer.next(),
        Some(Token::new(
            TokenKind::Plain(String::from(" inside")),
            Spacing::Neither
        ))
    );

    assert_eq!(
        lexer.next(),
        Some(Token::new(TokenKind::Italic, Spacing::Neither))
    )
}
