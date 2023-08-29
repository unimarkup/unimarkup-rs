use unimarkup_commons::test_runner::as_snapshot::AsSnapshot;
use unimarkup_inline::{Token, TokenKind, Tokens};

use crate::snapshot::Snapshot;

impl AsSnapshot for Snapshot<(&str, Tokens<'_>)> {
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

            let token_as_snap = Snapshot(token).as_snapshot();
            content.push_str(&token_as_snap);
        }

        content
    }
}

impl AsSnapshot for Snapshot<Token<'_>> {
    fn as_snapshot(&self) -> String {
        let span = self.0.span();

        let indent = " ".repeat(span.start.col_grapheme.saturating_sub(1));
        let mut content = String::from(&indent);
        // only newline token spans 2 lines, should not be the case for others!

        let inner = match self.as_str() {
            "\n" => Self::NEWLINE_SYBMOL,
            other => other,
        };

        if span.len_utf8().unwrap_or(1).saturating_sub(inner.len()) == 1 {
            // Some tokens occupy more characters in text (e.g.the backslash and symbol) than what's
            // being rendered in the output. In such cases, span is longer than the actual content
            // by a single character.
            // e.g. content like "\*" will be rendered as "␢*" in snapshots to indicate the
            // backslash escape.
            content.push_str(Self::BLANK_SYMBOL);
        }

        content.push_str(inner);
        content.push('\n');
        content.push_str(&indent);

        let underline = "^".repeat(span.len_grapheme().unwrap_or(1));
        content.push_str(&underline);
        content.push_str(" -> ");

        let kind = Snapshot(self.kind()).as_snapshot();
        content.push_str(&format!("{}{}", kind, Snapshot(&span).as_snapshot()));

        content.push('\n');

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
            TokenKind::EscapedNewline => "EscapedNewline",
            TokenKind::Newline => "Newline",
            TokenKind::EscapedWhitespace => "EscapedWhitespace",
            TokenKind::Plain => "Plain",
            TokenKind::Whitespace => "Whitespace",
        };

        string.into()
    }
}
