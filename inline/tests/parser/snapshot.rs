use std::collections::VecDeque;

use unimarkup_commons::test_runner::as_snapshot::AsSnapshot;
use unimarkup_inline::{ContentRef, Inline};

use crate::Snapshot;

impl AsSnapshot for Snapshot<&[Inline]> {
    fn as_snapshot(&self) -> String {
        self.iter()
            .filter(|inline| !inline.as_string().trim().is_empty())
            .map(Snapshot::snap)
            .collect()
    }
}

impl AsSnapshot for Snapshot<&VecDeque<Inline>> {
    fn as_snapshot(&self) -> String {
        self.iter()
            .filter(|inline| !inline.as_string().trim().is_empty())
            .map(Snapshot::snap)
            .collect()
    }
}

impl AsSnapshot for Snapshot<&Inline> {
    fn as_snapshot(&self) -> String {
        let start = self.variant_str();
        let inner = Snapshot::snap(self.inner());

        let indent = "    ";

        let mut res = String::from(start);
        res.push_str(&Snapshot::snap(&self.span()));
        res.push_str(" (\n");
        for line in inner.lines() {
            let content = match line {
                "\n" => Self::NEWLINE_SYBMOL,
                other => other,
            };

            res.push_str(indent);

            // Escaped tokens cause the span to be longer than the rendered content. Distribute
            // the _excess_ length as prefix/suffix
            let has_prefix_suffix = matches!(self.inner(), ContentRef::Plain(_))
                && self.span().len_utf8().unwrap_or(1) > content.len();

            let prefix_sufix_len = if has_prefix_suffix {
                (self.span().len_utf8().unwrap_or(1) - content.len()) / 2
            } else {
                0
            };

            if has_prefix_suffix {
                res.push_str(&Self::BLANK_SYMBOL.repeat(prefix_sufix_len));
            }

            res.push_str(content);

            if has_prefix_suffix {
                res.push_str(&Self::BLANK_SYMBOL.repeat(prefix_sufix_len));
            }

            res.push('\n');

            if matches!(
                self.0,
                Inline::Plain(_)
                    | Inline::Verbatim(_)
                    | Inline::Parentheses(_)
                    | Inline::EscapedNewline(_)
                    | Inline::EscapedWhitespace(_)
                    | Inline::Newline(_)
            ) {
                res.push_str(indent);
                res.push_str(&"^".repeat(self.span().len_grapheme().unwrap_or(1)));
                res.push('\n');
            }
        }

        res.push_str(")\n");
        res
    }
}

impl AsSnapshot for Snapshot<ContentRef<'_>> {
    fn as_snapshot(&self) -> String {
        match self.0 {
            ContentRef::Plain(plain) => Snapshot(plain).as_snapshot(),
            ContentRef::Nested(nested) => Snapshot(nested).as_snapshot(),
        }
    }
}

impl AsSnapshot for Snapshot<&str> {
    fn as_snapshot(&self) -> String {
        self.trim_end().to_string()
    }
}
