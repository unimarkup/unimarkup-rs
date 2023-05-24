use unimarkup_commons::test_runner::as_snapshot::AsSnapshot;
use unimarkup_inline::{Token, TokenKind, Tokens};

use crate::Snapshot;

impl AsSnapshot for Snapshot<(&str, Tokens)> {
    fn as_snapshot(&self) -> String {
        let input = (self.0).0;
        let mut lines = input.lines();

        let tokens = (self.0).1.clone();

        let mut curr_line_nr = 0;
        let mut content = String::new();

        for token in tokens {
            if token.span().start.line != curr_line_nr {
                let line = lines.next().unwrap();

                if curr_line_nr > 0 {
                    content.push('\n');
                }

                content.push_str(line);
                content.push('\n');
                curr_line_nr += 1;
            }

            {};

            let token_as_snap = Snapshot(token).as_snapshot();
            content.push_str(&token_as_snap);
        }

        content
    }
}

impl AsSnapshot for Snapshot<Token> {
    fn as_snapshot(&self) -> String {
        let span = self.0.span();

        let indent = " ".repeat(span.start.col_utf8.saturating_sub(1));
        let mut content = String::from(&indent);
        // only newline token spans 2 lines, should not be the case for others!

        let inner = match self.as_str() {
            "\n" => "\u{23CE}",
            other => other,
        };
        content.push_str(inner);
        content.push('\n');
        content.push_str(&indent);

        let underline = "^".repeat(span.len_utf8().unwrap_or(1));
        content.push_str(&underline);
        content.push_str(" -> ");

        let kind = Snapshot(self.kind()).as_snapshot();

        let start = span.start;
        let end = span.end;
        content.push_str(&format!(
            "{} @ ({}:{})->({}:{})\n",
            kind, start.line, start.col_utf8, end.line, end.col_utf8
        ));

        content
    }
}

impl AsSnapshot for Snapshot<TokenKind> {
    fn as_snapshot(&self) -> String {
        let string = match self.0 {
            TokenKind::Bold => "Bold",
            TokenKind::Italic => "Italic",
            TokenKind::ItalicBold => "ItalicBold",
            TokenKind::Underline => "Underline",
            TokenKind::Subscript => "Subscript",
            TokenKind::UnderlineSubscript => "UnderlineSubscript",
            TokenKind::Superscript => "Superscript",
            TokenKind::Overline => "Overline",
            TokenKind::Strikethrough => "Strikethrough",
            TokenKind::Highlight => "Highlight",
            TokenKind::Verbatim => "Verbatim",
            TokenKind::Quote => "Quote",
            TokenKind::Math => "Math",
            TokenKind::OpenParens => "OpenParens",
            TokenKind::CloseParens => "CloseParens",
            TokenKind::OpenBracket => "OpenBracket",
            TokenKind::CloseBracket => "CloseBracket",
            TokenKind::OpenBrace => "OpenBrace",
            TokenKind::CloseBrace => "CloseBrace",
            TokenKind::Substitution => "Substitution",
            TokenKind::Newline => "Newline",
            TokenKind::EndOfLine => "EndOfLine",
            TokenKind::Whitespace => "Whitespace",
            TokenKind::Plain => "Plain",
        };

        string.into()
    }
}
