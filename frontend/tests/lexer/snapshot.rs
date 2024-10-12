use std::fmt::Write;

use unimarkup_commons::test_runner::as_snapshot::AsSnapshot;
use unimarkup_frontend::lexer::{token::Token, token_kind::TokenKind};

use crate::snapshot::Snapshot;

impl AsSnapshot for Snapshot<Token<'_>> {
    fn as_snapshot(&self) -> String {
        let token = self.0;

        let indent_len = get_indent(token.input, token.span.offs as usize);

        let orig_input = token.as_input_str();
        let marker = "^".repeat(token.span.len as usize);
        let indent = " ".repeat(indent_len);
        let kind = Snapshot(token.kind).as_snapshot();

        let mut output = String::new();
        let _ = writeln!(&mut output, "{indent}{orig_input}");
        let _ = write!(
            &mut output,
            "{indent}{marker} - {kind} @ ({} -> {})",
            token.span.offs,
            token.span.offs + token.span.len
        );
        output
    }
}

impl AsSnapshot for Snapshot<TokenKind> {
    fn as_snapshot(&self) -> String {
        match self.0 {
            TokenKind::Star(r) => format!("Star({r})"),
            TokenKind::Hash(r) => format!("Hash({r})"),
            TokenKind::Minus(r) => format!("Minus({r})"),
            TokenKind::Plus(r) => format!("Plus({r})"),
            TokenKind::Underline(r) => format!("Underline({r})"),
            TokenKind::Caret(r) => format!("Caret({r})"),
            TokenKind::Tick(r) => format!("Tick({r})"),
            TokenKind::Pipe(r) => format!("Pipe({r})"),
            TokenKind::Tilde(r) => format!("Tilde({r})"),
            TokenKind::Quote(r) => format!("Quote({r})"),
            TokenKind::Dollar(r) => format!("Dollar({r})"),
            TokenKind::Colon(r) => format!("Colon({r})"),
            TokenKind::Dot(r) => format!("Dot({r})"),
            TokenKind::Ampersand(r) => format!("Ampersand({r})"),
            TokenKind::Comma(r) => format!("Comma({r})"),
            TokenKind::OpenParenthesis => format!("OpenParenthesis"),
            TokenKind::CloseParenthesis => format!("CloseParenthesis"),
            TokenKind::OpenBracket => format!("OpenBracket"),
            TokenKind::CloseBracket => format!("CloseBracket"),
            TokenKind::OpenBrace => format!("OpenBrace"),
            TokenKind::CloseBrace => format!("CloseBrace"),
            TokenKind::Whitespace => format!("Whitespace"),
            TokenKind::Newline => format!("Newline"),
            TokenKind::Blankline => format!("Blankline"),
            TokenKind::Eoi => format!("Eoi"),
            TokenKind::Indentation(r) => format!("Indentation({r})"),
            TokenKind::EscapedPlain => format!("EscapedPlain"),
            TokenKind::EscapedWhitespace => format!("EscapedWhitespace"),
            TokenKind::EscapedNewline => format!("EscapedNewline"),
            TokenKind::Plain => format!("Plain"),
            TokenKind::TerminalPunctuation => format!("TerminalPunctuation"),
            TokenKind::Comment { implicit_close } => {
                if implicit_close {
                    format!("Comment(implicitly closed)")
                } else {
                    format!("Comment")
                }
            }
            TokenKind::DirectUri => format!("DirectUri"),
            TokenKind::Any => format!("Any"),
            TokenKind::Space => format!("Space"),
            TokenKind::EnclosedBlockEnd => format!("EnclosedBlockEnd"),
            TokenKind::PossibleAttributes => format!("PossibleAttributes"),
            TokenKind::PossibleDecorator => format!("PossibleDecorator"),
        }
    }
}

fn get_indent(input: &str, offs: usize) -> usize {
    input[0..offs]
        .bytes()
        .rev()
        .position(|byte| byte == b'\n')
        .unwrap_or(offs)
}
