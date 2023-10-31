use unimarkup_commons::test_runner::as_snapshot::AsSnapshot;
use unimarkup_inline::element::{
    formatting::{
        Bold, Highlight, Italic, Math, Overline, Quote, Strikethrough, Subscript, Superscript,
        Underline, Verbatim,
    },
    Inline, InlineElement,
};

use crate::Snapshot;

impl AsSnapshot for Snapshot<&str> {
    fn as_snapshot(&self) -> String {
        self.trim_end().to_string()
    }
}

impl AsSnapshot for Snapshot<&[Inline]> {
    fn as_snapshot(&self) -> String {
        self.iter()
            .filter(|inline| !inline.to_plain_string().trim().is_empty())
            .map(Snapshot::snap)
            .collect()
    }
}

impl AsSnapshot for Snapshot<&Vec<Inline>> {
    fn as_snapshot(&self) -> String {
        self.iter()
            .filter(|inline| !inline.to_plain_string().trim().is_empty())
            .map(Snapshot::snap)
            .collect()
    }
}

impl AsSnapshot for Snapshot<&Inline> {
    fn as_snapshot(&self) -> String {
        let start = self.variant_str();
        let inner = inner_snapshot(self);

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
            let has_prefix_suffix =
                self.is_plain() && self.span().len_utf8().unwrap_or(1) > content.len();

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
                    | Inline::EscapedNewline(_)
                    | Inline::EscapedWhitespace(_)
                    | Inline::Newline(_)
                    | Inline::ImplicitNewline(_)
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

fn inner_snapshot(inline: &Inline) -> String {
    match inline {
        Inline::Bold(inline) => Snapshot::snap(inline),
        Inline::Italic(inline) => Snapshot::snap(inline),
        Inline::Underline(inline) => Snapshot::snap(inline),
        Inline::Subscript(inline) => Snapshot::snap(inline),
        Inline::Superscript(inline) => Snapshot::snap(inline),
        Inline::Overline(inline) => Snapshot::snap(inline),
        Inline::Strikethrough(inline) => Snapshot::snap(inline),
        Inline::Highlight(inline) => Snapshot::snap(inline),
        Inline::Quote(inline) => Snapshot::snap(inline),
        Inline::Math(inline) => Snapshot::snap(inline),
        Inline::TextBox(inline) => Snapshot::snap(inline.inner()),
        Inline::Hyperlink(inline) => Snapshot::snap(inline.inner()),
        Inline::Verbatim(inline) => Snapshot::snap(inline),
        Inline::Newline(inline) => Snapshot::snap(inline.as_str()),
        Inline::ImplicitNewline(inline) => Snapshot::snap(inline.as_str()),
        Inline::EscapedNewline(inline) => Snapshot::snap(inline.as_str()),
        Inline::EscapedWhitespace(inline) => inline.space().clone(),
        Inline::Plain(inline) => inline.content().clone(),
        Inline::EscapedPlain(inline) => inline.content().clone(),
        Inline::DirectUri(inline) => inline.uri().to_string(),

        Inline::NamedSubstitution(_) => todo!(),
    }
}

macro_rules! format_to_inline {
    ($($format:ident),+) => {
        $(
            impl AsSnapshot for Snapshot<&$format> {
                fn as_snapshot(&self) -> String {
                    Snapshot::snap(self.inner())
                }
            }
        )+
    };
}

format_to_inline!(
    Bold,
    Italic,
    Underline,
    Subscript,
    Superscript,
    Strikethrough,
    Highlight,
    Overline,
    Verbatim,
    Quote,
    Math
);
