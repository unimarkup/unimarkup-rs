use crate::span::Span;

use super::token_kind::TokenKind;

/// Token lexed from grapheme [`Symbol`]s of the given input.
///
/// # Lifetimes
///
/// * `'input` - lifetime of input the [`Token`] was lexed from.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token<'input> {
    pub input: &'input str,
    pub kind: TokenKind,
    pub span: Span,
}

impl std::fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = self.span.offs as usize;
        let end = self.span.offs as usize + self.span.len as usize;

        f.debug_struct("Token")
            .field("input", &self.input)
            .field("output", &self.input[start..end].to_string())
            .field("kind", &self.kind)
            .field("offs", &self.span.offs)
            .field("len", &self.span.len)
            .finish()
    }
}

impl<'input> Token<'input> {
    pub fn as_input_str(&self) -> &'input str {
        let start = self.span.offs as usize;
        let end = self.span.offs as usize + self.span.len as usize;
        &self.input[start..end]
    }
}
