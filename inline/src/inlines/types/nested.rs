//! Structs and implementations of Inlines that can contain other Inlines.

use std::collections::VecDeque;

use unimarkup_commons::lexer::span::Span;

use crate::Inline;

macro_rules! impl_nested_inline {
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
            pub(crate) content: VecDeque<Inline>,
            pub(crate) span: Span,
        }

        impl $inline {
            /// Returns the length of this inline.
            pub fn content_len(&self) -> usize {
                self.content.iter().map(Inline::content_len).sum()
            }

            /// Returns immutable reference to inner content.
            pub fn inner(&self) -> &VecDeque<Inline> {
                &self.content
            }

            /// Returns mutable reference to inner content.
            pub fn inner_mut(&mut self) -> &mut VecDeque<Inline> {
                &mut self.content
            }

            /// Merges consecutive inlines of same kind contained in this inline.
            pub fn merge(&mut self) {
                if self.content.is_empty() {
                    return;
                }

                let curr_content = std::mem::take(&mut self.content);
                let mut res_vec: VecDeque<Inline> = VecDeque::with_capacity(curr_content.len());

                for inline in curr_content.into_iter() {
                    let matches_prev = res_vec
                        .back()
                        .map_or(false, |prev_inline| prev_inline.matches_kind(&inline));

                    if matches_prev {
                        if let Some(mut prev_inline) = res_vec.pop_back() {
                            prev_inline.append(inline);
                            res_vec.push_back(prev_inline);
                        }
                    } else {
                        res_vec.push_back(inline);
                    }
                }

                self.content = res_vec;
            }

            #[doc = concat!(r"Appends another [`", stringify!($inline), "`] inline to this one.")]
            pub fn append(&mut self, mut other: Self) {
                self.content.append(&mut other.content);
                self.span.end = other.span.end;
            }
        }

        impl From<(VecDeque<Inline>, Span)> for $inline {
            fn from((content, span): (VecDeque<Inline>, Span)) -> Self {
                Self { content, span }
            }
        }
    )*
    }
}

impl_nested_inline!(
    /// Bold formatted content.
    Bold,
    /// Italic formatted content.
    Italic,
    /// Underlined content.
    Underline,
    /// Content in a subscript.   
    Subscript,
    /// Content in a superscript.
    Superscript,
    /// Overlined content.
    Overline,
    /// Content with a strikethrough.
    Strikethrough,
    /// Highlighted content.
    Highlight,
    /// Quoted content.
    Quote,
    /// Mathematical content.
    Math,
    /// Content of a TextGroup `[]`.
    TextGroup,
    /// Unimarkup attributes for some content.
    Attributes,
    /// Alias substitution ( i.e. `::heart::`).
    Substitution,
    /// Wrapper without any special formatting for multiple other [`Inline`]s.
    ///
    /// [`Inline`]: self::Inline
    Multiple
);
