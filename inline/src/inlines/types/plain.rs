//! Structs and implementations of Inlines that can contain only String as content.

use unimarkup_commons::lexer::span::Span;

macro_rules! impl_plain_inline {
    (
        $(
            $(#[$($attrss:tt)*])*
            $inline:ident
        ),*
    ) => {
    $(
        $(#[$($attrss)*])*
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct $inline {
            pub(crate) content: String,
            pub(crate) span: Span,
        }

        impl $inline {
            /// Returns the length of this inline.
            pub fn content_len(&self) -> usize {
                self.content.len()
            }

            /// Returns immutable reference to inner content.
            pub fn inner(&self) -> &str {
                &self.content
            }

            /// Returns mutable reference to inner content.
            pub fn inner_mut(&mut self) -> &mut str {
                &mut self.content
            }

            #[doc = concat!(r"Appends another ", stringify!($inline), " inline to this one.")]
            pub fn append(&mut self, other: Self) {
                self.content.push_str(&other.content);
                self.span.end = other.span.end;
            }
        }
    )*
    };
}

impl_plain_inline!(
    /// A plain text inline.
    Plain,
    /// A verbatim inline with input preserved.
    Verbatim,
    /// Content inside of pair of parentheses.
    Parentheses,
    /// Newline literal.
    EscapedNewline,
    /// Any whitespace literal except newline.
    EscapedWhitespace,
    /// Inline explicitly marking end of line.
    Newline
);
