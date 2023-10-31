use std::collections::VecDeque;

use itertools::PeekingNext;

use crate::lexer::{new::SymbolIterator, token::Token};

use super::{
    extension::TokenIteratorExt,
    implicit::{TokenIteratorImplicitExt, TokenIteratorImplicits},
};

#[derive(Debug, Clone)]
pub struct CachedTokenIterator<'input> {
    iter: TokenIteratorImplicits<'input>,
    prev_token: Option<Token<'input>>,
    prev_peeked_token: Option<Token<'input>>,
    cache: VecDeque<CacheEntry<'input>>,
}

impl<'input> CachedTokenIterator<'input> {
    pub(crate) fn prev_peeked(&self) -> Option<&Token<'input>> {
        self.prev_peeked_token.as_ref()
    }

    /// Returns and pops the first cached token from the cache.
    fn next_cached_token(&mut self) -> Option<Token<'input>> {
        while !self.cache.is_empty() {
            // Increase index of base iterator to "cover" same distance
            self.iter.set_index(self.iter.index() + 1);

            if let Some(CacheEntry::Token(token)) = self.cache.pop_front() {
                return Some(token);
            }
        }

        debug_assert!(false, "No token found in cache.");

        None
    }

    fn cached_token_at(&mut self, peek_index: usize) -> Option<Token<'input>> {
        let base_index = self.index();
        if base_index > peek_index {
            return None;
        }

        let rel_cache_index = peek_index - base_index;
        if rel_cache_index >= self.cache.len() {
            return None;
        }

        let contiguous_cache = &self.cache.make_contiguous()[rel_cache_index..];
        let mut cached_token = None;
        let mut index_skips = 0;

        for cache_entry in contiguous_cache {
            index_skips += 1;

            if let CacheEntry::Token(token) = cache_entry {
                cached_token = Some(*token);
                break;
            }
        }

        if cached_token.is_some() {
            self.set_peek_index(peek_index + index_skips);
        }

        cached_token
    }
}

impl<'input> From<SymbolIterator<'input>> for CachedTokenIterator<'input> {
    fn from(value: SymbolIterator<'input>) -> Self {
        CachedTokenIterator {
            iter: value.into(),
            prev_token: None,
            prev_peeked_token: None,
            cache: VecDeque::new(),
        }
    }
}

impl<'input> TokenIteratorExt<'input> for CachedTokenIterator<'input> {
    fn prev_token(&self) -> Option<&Token<'input>> {
        self.prev_token.as_ref()
    }

    fn max_len(&self) -> usize {
        self.iter.max_len()
    }

    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }

    fn index(&self) -> usize {
        self.iter.index()
    }

    fn set_index(&mut self, index: usize) {
        if self.index() <= index {
            // pop tokens from cache up until new index position
            self.cache
                .drain(0..self.cache.len().min(index - self.index()));
            self.set_index(index);
        }
    }

    fn peek_index(&self) -> usize {
        self.iter.peek_index()
    }

    fn set_peek_index(&mut self, index: usize) {
        self.iter.set_peek_index(index);
    }

    fn reset_peek(&mut self) {
        self.iter.reset_peek();
    }

    fn scope(&self) -> usize {
        self.iter.scope()
    }

    fn set_scope(&mut self, scope: usize) {
        self.iter.set_scope(scope);
    }
}

impl<'input> TokenIteratorImplicitExt for CachedTokenIterator<'input> {
    fn ignore_implicits(&mut self) {
        if self.implicits_allowed() {
            // invalidate cache, because cached implicits might become invalid
            self.cache = VecDeque::default();
        }

        self.iter.ignore_implicits();
    }

    fn allow_implicits(&mut self) {
        self.iter.allow_implicits();
    }

    fn implicits_allowed(&self) -> bool {
        self.iter.implicits_allowed()
    }
}

impl<'input> Iterator for CachedTokenIterator<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = if self.cache.is_empty() {
            self.iter.next()
        } else {
            self.next_cached_token()
        };

        if next.is_some() {
            self.prev_token = next;
        }

        next
    }
}

impl<'input> PeekingNext for CachedTokenIterator<'input> {
    fn peeking_next<F>(&mut self, accept: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnOnce(&Self::Item) -> bool,
    {
        let mut start_peek_index = self.peek_index();

        // Is peek index in cache range?
        let peeked_token = if let Some(cached_token) = self.cached_token_at(start_peek_index) {
            if accept(&cached_token) {
                Some(cached_token)
            } else {
                None
            }
        } else {
            // peek not in cache => get new token and cache it
            let token = self.iter.peeking_next(accept)?;
            let new_peek_index = self.iter.peek_index();

            // +1 because peeking_next increases index at least by 1
            while start_peek_index + 1 < new_peek_index {
                self.cache.push_back(CacheEntry::Redirect);
                start_peek_index += 1;
            }

            self.cache.push_back(CacheEntry::Token(token));
            Some(token)
        };

        if peeked_token.is_some() {
            self.prev_peeked_token = peeked_token;
        }

        peeked_token
    }
}

#[derive(Debug, Clone)]
enum CacheEntry<'input> {
    Token(Token<'input>),
    /// Indicates that the actual token is found at higher index in the cache.
    Redirect,
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use crate::lexer::{
        new::SymbolIterator,
        token::{iterator::extension::TokenIteratorExt, TokenKind},
    };

    use super::CachedTokenIterator;

    // #[test]
    // fn peek_while_cached() {
    //     let symbols = crate::lexer::scan_str("*+ # - ~");
    //     let mut cached_iter = CachedTokenIterator::from(SymbolIterator::from(&*symbols));

    //     let peeked_cnt = cached_iter
    //         .peeking_take_while(|t| t.kind != TokenKind::Tilde(1))
    //         .count();

    //     assert_eq!(
    //         cached_iter.cache.len(),
    //         cached_iter.peek_index(),
    //         "Iterator did not cache tokens."
    //     );

    //     let tokens = cached_iter
    //         .take_while(|t| t.kind != TokenKind::Tilde(1))
    //         .map(|t| t.kind)
    //         .collect_vec();

    //     assert_eq!(
    //         tokens.len(),
    //         peeked_cnt,
    //         "Peek and take while did not return the same number of tokens."
    //     );

    //     assert_eq!(
    //         tokens,
    //         vec![
    //             TokenKind::Star(1),
    //             TokenKind::Plus(1),
    //             TokenKind::Whitespace,
    //             TokenKind::Hash(1),
    //             TokenKind::Whitespace,
    //             TokenKind::Minus(1),
    //             TokenKind::Whitespace
    //         ],
    //         "Take while returned wrong tokens."
    //     )
    // }

    // #[test]
    // fn cached_tokens_spanning_multiple_symbols() {
    //     let symbols = crate::lexer::scan_str("**bold** plain");
    //     let mut cached_iter = CachedTokenIterator::from(SymbolIterator::from(&*symbols));

    //     let peeked_cnt = cached_iter.peeking_take_while(|_| true).count();

    //     assert_eq!(
    //         cached_iter.cache.len(),
    //         cached_iter.peek_index(),
    //         "Iterator did not cache tokens + redirects."
    //     );

    //     let tokens = cached_iter
    //         .take_while(|t| t.kind != TokenKind::Tilde(1))
    //         .map(|t| t.kind)
    //         .collect_vec();

    //     assert_eq!(
    //         tokens.len(),
    //         peeked_cnt,
    //         "Peek and take while did not return the same number of tokens."
    //     );

    //     assert_eq!(
    //         tokens,
    //         vec![
    //             TokenKind::Star(2),
    //             TokenKind::Plain,
    //             TokenKind::Star(2),
    //             TokenKind::Whitespace,
    //             TokenKind::Plain,
    //             TokenKind::Eoi,
    //         ],
    //         "Take while returned wrong tokens."
    //     )
    // }

    // #[test]
    // fn cached_tokens_peeked() {
    //     let symbols = crate::lexer::scan_str("**bold** plain");
    //     let mut cached_iter = CachedTokenIterator::from(SymbolIterator::from(&*symbols));

    //     let first_peeked_tokens = cached_iter
    //         .peeking_take_while(|_| true)
    //         .map(|t| t.kind)
    //         .collect_vec();
    //     let peek_index = cached_iter.peek_index();

    //     cached_iter.reset_peek();

    //     assert_eq!(
    //         cached_iter.cache.len(),
    //         peek_index,
    //         "Iterator removed cached tokens + redirects on peek reset."
    //     );

    //     let second_peeked_tokens = cached_iter
    //         .peeking_take_while(|_| true)
    //         .map(|t| t.kind)
    //         .collect_vec();

    //     assert_eq!(
    //         first_peeked_tokens, second_peeked_tokens,
    //         "Second peek while did not return same tokens as first."
    //     )
    // }

    // #[test]
    // fn take_next_from_cache() {
    //     let symbols = crate::lexer::scan_str("**bold** plain");
    //     let mut cached_iter = CachedTokenIterator::from(SymbolIterator::from(&*symbols));

    //     let _peeked_tokens = cached_iter
    //         .peeking_take_while(|_| true)
    //         .map(|t| t.kind)
    //         .collect_vec();

    //     let cached_token = cached_iter.next().unwrap();

    //     assert_eq!(
    //         cached_token.kind,
    //         TokenKind::Star(2),
    //         "First cached token was incorrect."
    //     );
    //     assert_eq!(
    //         cached_iter.cache.len(),
    //         ,
    //         "First cached token was incorrect."
    //     );
    // }
}
