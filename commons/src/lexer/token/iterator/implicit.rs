// //! Contains the [`TokenIteratorRoot`] that is the root iterator in any [`TokenIterator`](super::TokenIterator).

// use itertools::PeekingNext;

// use crate::lexer::{
//     new::SymbolIterator,
//     token::{
//         iterator::{base::TokenIteratorBase, extension::TokenIteratorExt},
//         Token, TokenKind,
//     },
// };

// pub trait TokenIteratorImplicitExt {
//     fn ignore_implicits(&mut self);
//     fn allow_implicits(&mut self);
//     fn implicits_allowed(&self) -> bool;
// }

// /// The [`TokenIteratorRoot`] is the root iterator in any [`TokenIterator`](super::TokenIterator).
// /// It holds the actual [`Symbol`] slice.
// #[derive(Debug, Clone)]
// pub struct TokenIteratorImplicits<'input> {
//     /// The [`Symbol`] slice the iterator was created for.
//     pub(crate) base_iter: TokenIteratorBase<'input>,
//     prev_token: Option<Token<'input>>,
//     prev_peeked_token: Option<Token<'input>>,
//     allow_implicits: bool,
//     /// Flag to mark if arrow substitutions are allowed,
//     /// without ensuring that it is followed by another arrow, terminal punctuation, or space.
//     ///
//     /// Helps to prevent repeated lookaheads.
//     allow_arrow: bool,
//     /// Flag to mark if arrow substitutions are allowed,
//     /// without ensuring that it is followed by another emoji, terminal punctuation, or space.
//     ///
//     /// Helps to prevent repeated lookaheads.
//     allow_emoji: bool,
// }

// impl<'input> TokenIteratorImplicits<'input> {
//     pub(crate) fn prev_peeked(&self) -> Option<&Token<'input>> {
//         self.prev_peeked_token.as_ref()
//     }
// }

// impl<'input> TokenIteratorImplicitExt for TokenIteratorImplicits<'input> {
//     fn ignore_implicits(&mut self) {
//         self.allow_implicits = false;
//     }

//     fn allow_implicits(&mut self) {
//         self.allow_implicits = true;
//     }

//     fn implicits_allowed(&self) -> bool {
//         self.allow_implicits
//     }
// }

// impl<'input> TokenIteratorExt<'input> for TokenIteratorImplicits<'input> {
//     /// Returns the symbol that is directly before the current index.
//     /// If no previous symbol exists, `None`` is returned.
//     fn prev_token(&self) -> Option<&Token<'input>> {
//         self.prev_token.as_ref()
//     }

//     fn max_len(&self) -> usize {
//         self.base_iter.max_len()
//     }

//     /// Returns `true` if no more [`Symbol`]s are available.
//     fn is_empty(&self) -> bool {
//         self.max_len() == 0
//     }

//     /// Returns the current index this iterator is in the [`Symbol`] slice of the root iterator.
//     fn index(&self) -> usize {
//         self.base_iter.index()
//     }

//     /// Sets the current index of this iterator to the given index.
//     fn set_index(&mut self, index: usize) {
//         self.base_iter.set_index(index);
//     }

//     /// Returns the index used to peek.
//     fn peek_index(&self) -> usize {
//         self.base_iter.peek_index()
//     }

//     /// Sets the peek index of this iterator to the given index.
//     fn set_peek_index(&mut self, index: usize) {
//         self.base_iter.set_peek_index(index);

//         if self.peek_index() != index {
//             // Jumping arround by index invalidates implicits lookahead
//             self.allow_arrow = false;
//             self.allow_emoji = false;
//         }
//     }

//     fn reset_peek(&mut self) {
//         self.set_peek_index(self.index());
//     }

//     fn scope(&self) -> usize {
//         self.base_iter.scope()
//     }

//     fn set_scope(&mut self, scope: usize) {
//         self.base_iter.set_scope(scope);
//     }
// }

// impl<'input> From<SymbolIterator<'input>> for TokenIteratorImplicits<'input> {
//     fn from(value: SymbolIterator<'input>) -> Self {
//         TokenIteratorImplicits {
//             base_iter: TokenIteratorBase::from(value),
//             prev_token: None,
//             prev_peeked_token: None,
//             allow_implicits: true,
//             allow_arrow: false,
//             allow_emoji: false,
//         }
//     }
// }

// impl<'input> Iterator for TokenIteratorImplicits<'input> {
//     type Item = Token<'input>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let next = self.next_base_token(NextVariant::Next);

//         if next.is_some() {
//             self.prev_token = next;
//         }

//         next
//     }

//     fn size_hint(&self) -> (usize, Option<usize>) {
//         self.base_iter.size_hint()
//     }
// }

// impl<'input> PeekingNext for TokenIteratorImplicits<'input> {
//     fn peeking_next<F>(&mut self, accept: F) -> Option<Self::Item>
//     where
//         Self: Sized,
//         F: FnOnce(&Self::Item) -> bool,
//     {
//         let peek_index = self.base_iter.peek_index();
//         let token = self.next_base_token(NextVariant::Peek)?;

//         if accept(&token) {
//             self.prev_peeked_token = Some(token);
//             Some(token)
//         } else {
//             // reset peek to also reset peek of base iterator, because base peeking_next was without condition.
//             self.set_peek_index(peek_index);
//             None
//         }
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum NextVariant {
//     Next,
//     Peek,
// }

// impl<'input> TokenIteratorImplicits<'input> {
//     fn next_base_token(&mut self, variant: NextVariant) -> Option<Token<'input>> {
//         let kind = self.base_iter.peek_kind()?;

//         if self.allow_implicits {
//             if kind == TokenKind::TerminalPunctuation || kind.is_space() {
//                 // reached end of arrow/emoji sequence
//                 // => next arrow/emoji must ensure it is followed by another arrow/emoji, terminal punctuation, or space.
//                 self.allow_arrow = false;
//                 self.allow_emoji = false;
//             }

//             // let mut implicit_iter = self.clone();
//             // if let Some(implicit_token) =
//             //     crate::lexer::token::implicit::get_implicit(&mut implicit_iter)
//             // {
//             //     let token = implicit_token;
//             //     if variant == NextVariant::Peek {
//             //         self.set_peek_index(implicit_iter.peek_index());
//             //         // Must be set after peek index update, because setting peek index invalidates flags
//             //         self.allow_arrow = implicit_iter.allow_arrow;
//             //         self.allow_emoji = implicit_iter.allow_emoji;

//             //         // Implicit check is done on the base iterator, so pev tokens of self must remain unchanged
//             //         debug_assert_eq!(
//             //             self.prev_peeked_token, implicit_iter.prev_peeked_token,
//             //             "Previous peeked token differs in implicit iter from self."
//             //         );
//             //         debug_assert_eq!(
//             //             self.prev_token, implicit_iter.prev_token,
//             //             "Previous token differs in implicit iter from self."
//             //         );
//             //     } else {
//             //         *self = implicit_iter;
//             //     }

//             //     return Some(token);
//             // }
//         } else {
//             self.allow_arrow = false;
//             self.allow_emoji = false;
//         }

//         if variant == NextVariant::Peek {
//             self.base_iter.peeking_next(|_| true)
//         } else {
//             self.base_iter.next()
//         }
//     }
// }

// // #[cfg(test)]
// // mod test {
// //     use crate::lexer::token::{
// //         implicit::ImplicitSubstitutionKind, iterator::TokenIterator, TokenKind,
// //     };

// //     #[test]
// //     fn trademark_substitution() {
// //         let symbols = crate::lexer::scan_str("(TM)");
// //         let mut token_iter = TokenIterator::from(&*symbols);

// //         let token = token_iter.next().unwrap();

// //         assert_eq!(
// //             token.kind,
// //             TokenKind::ImplicitSubstitution(ImplicitSubstitutionKind::Trademark),
// //             "Trademark token was not detected."
// //         );
// //     }
// // }
