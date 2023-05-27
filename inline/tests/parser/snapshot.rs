use unimarkup_commons::test_runner::as_snapshot::AsSnapshot;
use unimarkup_inline::{Inline, InlineContent, NestedContent, PlainContent};

use crate::Snapshot;

impl AsSnapshot for Snapshot<&[Inline]> {
    fn as_snapshot(&self) -> String {
        self.iter()
            .filter(|inline| !inline.as_string().trim().is_empty())
            .map(Snapshot::snap)
            .collect()
    }
}

impl AsSnapshot for Snapshot<&Inline> {
    fn as_snapshot(&self) -> String {
        dbg!(self.0);

        let start = match self.0 {
            Inline::Bold(_) => "Bold",
            Inline::Italic(_) => "Italic",
            Inline::Underline(_) => "Underline",
            Inline::Subscript(_) => "Subscript",
            Inline::Superscript(_) => "Superscript",
            Inline::Overline(_) => "Overline",
            Inline::Strikethrough(_) => "Strikethrough",
            Inline::Highlight(_) => "Highlight",
            Inline::Verbatim(_) => "Verbatim",
            Inline::Quote(_) => "Quote",
            Inline::Math(_) => "Math",
            Inline::Parentheses(_) => "Parentheses",
            Inline::TextGroup(_) => "TextGroup",
            Inline::Attributes(_) => "Attributes",
            Inline::Substitution(_) => "Substitution",
            Inline::Newline(_) => "Newline",
            Inline::Whitespace(_) => "Whitespace",
            Inline::EndOfLine(_) => "EndOfLine",
            Inline::Plain(_) => "Plain",
            Inline::Multiple(_) => "Multiple",
        };

        let inner = Snapshot::snap(self.as_ref());

        let mut res = String::from(start);
        res.push_str(&Snapshot::snap(&self.span()));
        res.push_str(" (\n");
        for line in inner.lines() {
            let content = match line {
                "\n" => "    \u{23CE}",
                other => other,
            };

            res.push_str("    ");
            res.push_str(content);
            res.push('\n');
        }

        res.push_str(")\n");
        res
    }
}

impl AsSnapshot for Snapshot<InlineContent<&PlainContent, &NestedContent>> {
    fn as_snapshot(&self) -> String {
        match self.0 {
            InlineContent::Plain(plain_content) => Snapshot(plain_content).as_snapshot(),
            InlineContent::Nested(nested_content) => Snapshot(nested_content).as_snapshot(),
        }
    }
}

impl AsSnapshot for Snapshot<&PlainContent> {
    fn as_snapshot(&self) -> String {
        self.as_str().trim_end().into()
    }
}

impl AsSnapshot for Snapshot<&NestedContent> {
    fn as_snapshot(&self) -> String {
        self.iter().map(Snapshot::snap).collect()
    }
}
