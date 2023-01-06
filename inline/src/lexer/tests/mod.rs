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
fn tokenization_1() {
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

#[test]
fn tokenization_3() {
    let input = "**double ***nested* bold**";

    let tokens = input.tokens();

    println!("Lexing: \n{}", input);

    for _token in tokens {
        print!("{_token}");
        continue;
    }
    println!();
}

#[test]
fn tokenization_4() {
    let input = "**double *nested*** bold**";

    let tokens = input.tokens();

    println!("Lexing: \n{}", input);

    for _token in tokens {
        print!("{_token}");
        continue;
    }
    println!();
}

#[test]
fn tokenization_5() {
    let input = "***This bold* haha** with some more** text!";

    let tokens = input.tokens();

    println!("Lexing: \n{}", input);

    for _token in tokens {
        print!("{_token}");
        continue;
    }
    println!();
}

#[test]
fn tokenization_6() {
    let input = "**Bold that *should interrupt** italic*";

    let tokens = input.tokens();

    println!("Lexing: \n{}", input);

    for _token in tokens {
        print!("{_token}");
        continue;
    }
    println!();
}

#[test]
fn tokenization_tests() {
    let texts: &[&str] = &[
        "Some text",
        "**Bold text**",
        "*Italic text*",
        "*Italic text***Bold text**",
        "**This is bold *with* italic inside.**",
        "This is text [with text group] as part of it.",
        "This is **text [with text** group] as part of it.",
        "***This is input**",
        "[This text group [has another one inside] of it.]",
        "This huhuu [***This is input**]",
        "**This *is input**",
        "This is\ninput with\nmulti-line content.",
        "This is text::with_alias::substitution inside.",
        "`verbatim\\`escaped`",
        "******no bold**",
        "*open *2nd closing*",
        "**This bold [group**] and close** bold..",
        "***This bold* haha**",
        "**Bold that *should interrupt** italic*",
        "**bold **inside** of a bold**",
        "**bold **inside **of** bold** inside of a bold**",
        "**bold ^sup inside **opens bold^ close sup**",
    ];

    for input in texts {
        let tokens = input.tokens();

        println!("Lexing: \n{}\n", input);

        println!("Output: ");
        for token in tokens {
            println!("{token:?}");
        }
        println!("\n==================================================================\n");
    }
}

#[test]
fn tokenization_7() {
    let input = "***This bold* haha**";

    let tokens = input.tokens();

    println!("Lexing: \n{}", input);

    for _token in tokens {
        println!("{_token:?}");
        continue;
    }
    println!();
}

#[test]
fn tokenization_8() {
    let input = "*Italic text***Bold text**";

    let tokens = input.tokens();

    println!("Lexing: \n{}", input);

    for _token in tokens {
        println!("{_token:?}");
        continue;
    }
    println!();
}

#[test]
fn tokenization_9() {
    let input = "***This is input**";

    let tokens = input.tokens();

    println!("Lexing: \n{}", input);

    for _token in tokens {
        println!("{_token:?}");
        continue;
    }
    println!();
}

#[test]
fn tokenization_dash() {
    let input = "multi-word";

    let tokens = input.tokens();

    println!("Lexing: \n{}", input);

    for _token in tokens {
        println!("{_token:?}");
        continue;
    }
    println!();
}

#[test]
fn tokenization_underscore() {
    let input = "This is text::with_alias::substitution inside.";

    let tokens = input.tokens();

    println!("Lexing: \n{}", input);

    for _token in tokens {
        println!("{_token:?}");
        continue;
    }
    println!();
}

#[test]
fn tokenization_amb_open_close() {
    let input = "***italic bold***";

    let tokens = input.tokens();

    println!("Lexing: \n{}", input);

    for _token in tokens {
        println!("{_token:?}");
        continue;
    }
    println!();
}

#[test]
fn tokenization_repeated() {
    let input = "*italic* and *italic* again";
    let tokens = input.tokens();

    println!("Lexing: \n{}", input);

    for _token in tokens {
        println!("{_token:?}");
        continue;
    }
    println!();
}

#[test]
fn tokenization_bold_around_tg() {
    let input = "**This bold [group**] and close** bold..";

    println!("Lexing: \n{}", input);
    for _token in input.tokens() {
        print!("{_token}");
        continue;
    }
    println!();
}

#[test]
fn tokenization_scope_problem() {
    let input = "**bold [**in tg**] close**";

    println!("Lexing: \n{}", input);
    for _token in input.tokens() {
        print!("{_token}");
        continue;
    }
    println!();
}
