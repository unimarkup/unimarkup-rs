use super::*;

macro_rules! assert_token {
    ($token:ident with $kind:expr, $spacing:expr, $span:expr) => {
        assert_eq!($token.kind(), $kind);
        assert_eq!($token.spacing(), $spacing);
        assert_eq!($token.span(), crate::Span::from($span));
        true
    };

    ($token:ident with $kind:expr, $spacing:expr, $span:expr, $content:expr) => {
        assert_token!($token with $kind, $spacing, $span);
        assert_eq!($token.as_str(), $content);
        true
    }
}

// mod brace;
// mod bracket;
// mod caret;
// mod dollar;
// mod other;
// mod overline;
// mod parenthesis;
// mod pipe;
// mod quote;
// mod star;
// mod substitute;
// mod tick;
// mod tilde;
// mod underline;
//
#[test]
fn tokenization() {
    let input = "**This bold [group**] and close** bold..";

    let tokens = input.tokens();

    println!("Lexing: \n{}", input);

    for _token in tokens {
        print!("{_token}");
        continue;
    }
    println!();
}

#[test]
fn tokenization_2() {
    let input = "**double **nested** bold**";

    let tokens = input.tokens();

    println!("Lexing: \n{}", input);

    for _token in tokens {
        print!("{_token}");
        continue;
    }
    println!();
}
