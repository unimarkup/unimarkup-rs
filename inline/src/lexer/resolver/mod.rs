use std::{
    collections::{btree_map::Entry, BTreeMap},
    ops::Range,
    vec,
};

use crate::{TokenIterator, TokenKind};

mod raw_token;

pub(crate) use raw_token::*;

type Scope = usize;
type Indices = Vec<usize>;

#[derive(Debug, Clone)]
#[repr(transparent)]
/// Internal data structure for storing [`Indices`] of [`TokenKind`]s in specific [`Scope`].
struct TokenMap {
    map: BTreeMap<(TokenKind, Scope), Indices>,
}

impl TokenMap {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    fn general_key(kind: TokenKind) -> TokenKind {
        if let Some(amb_variant) = kind.get_ambiguous_variant() {
            amb_variant
        } else {
            kind
        }
    }

    fn update_or_insert(&mut self, kind: TokenKind, index: usize, scope: Scope) {
        self.entry(kind, scope)
            .and_modify(|indices| indices.push(index))
            .or_insert_with(|| vec![index]);
    }

    fn entry(&mut self, kind: TokenKind, scope: Scope) -> Entry<(TokenKind, Scope), Indices> {
        let key = (Self::general_key(kind), scope);
        self.map.entry(key)
    }

    fn get_mut(&mut self, kind: TokenKind, scope: Scope) -> Option<&mut Indices> {
        let key = (Self::general_key(kind), scope);
        self.map.get_mut(&key)
    }
}

/// Resolver of [`RawToken`]s, finds pairs of open/close tokens and marks them as such. If no pairs
/// are found, tokens are marked as plain.
///
/// Invariants of [`TokenResolver`]
///
/// ## During resolving:
/// - [`TokenKind`] of every token that's not resolved is stored into the token map (with current scope).
/// - Compound tokens (i.e. ***) are split, and each part is resolved separately
/// - Compound tokens with resolved parts is sorted based on their state, so that closing token
/// comes before opening token. If both parts have same state, then the second resolved part is
/// stored into token's tail.
/// - Every time a token pair is matched, all non-resolved tokens between them are marked as plain
#[derive(Debug, Clone)]
pub(crate) struct TokenResolver<'token> {
    curr_scope: usize,
    interrupted: Vec<Range<usize>>,
    pub(crate) tokens: Vec<RawToken<'token>>,
}

impl<'token> TokenResolver<'token> {
    pub(crate) fn new(iter: TokenIterator<'token>) -> Self {
        let mut new = Self {
            curr_scope: 0,
            interrupted: Vec::default(),
            tokens: iter.map(RawToken::new).collect(),
        };

        new.resolve();
        new
    }

    fn resolve(&mut self) {
        // map found tokens to their index in tokens vector
        let mut token_map: TokenMap = TokenMap::new();

        for index in 0..self.tokens.len() {
            // on open/close bracket simply increment/decrement scope
            if self.tokens[index].token.kind.is_open_bracket() {
                self.curr_scope += 1;
                continue;
            } else if self.tokens[index].token.kind.is_close_bracket() {
                // scope < 0 is user input error
                // TODO: report this as a warning or an error
                self.curr_scope = self.curr_scope.saturating_sub(1);
                continue;
            }

            // try to resolve token
            if let Some(begin_index) = self.resolve_token(&mut token_map, index) {
                // open tokens from begin_index to index are interrupted
                self.interrupted.push((begin_index + 1)..index);
            }

            if !self.tokens[index].state {
                let kind = self.tokens[index].token.kind;
                // save positions of every unresolved token
                token_map.update_or_insert(kind, index, self.curr_scope);
            }
        }
    }

    fn resolve_token(&mut self, token_map: &mut TokenMap, index: usize) -> Option<usize> {
        // multiple cases for current - unresolved token relationship:
        // 1. current IS ambiguous, there is unresolved one that IS ambiguous (ambiguous, ambiguous)
        // 2. current IS ambiguous, there is unresolved one that IS NOT ambiguous (simple, ambiguous)
        // 3. current NOT ambiguous, there is unresolved one that IS NOT ambiguous: (simple, simple)
        // 4. current NOT ambiguous, there is unresolved one that IS ambiguous (ambiguous, simple)

        if self.tokens[index].token.closes(None) {
            if self.tokens[index].token.is_ambiguous() {
                return self.resolve_compound_token(token_map, index);
            } else {
                return self.resolve_simple_token(token_map, index);
            }
        }

        None
    }

    fn resolve_simple_token(&mut self, token_map: &mut TokenMap, index: usize) -> Option<usize> {
        let token_kind = self.tokens[index].token.kind;

        let indices = token_map.get_mut(token_kind, self.curr_scope)?;
        let (unr_token, i, token_index) = self.find_first_matching(indices, index)?;

        if unr_token.token.is_ambiguous() {
            // opening token IS ambiguous (ambiguous, simple)

            unr_token.split_ambiguous();
            if unr_token.token.kind != token_kind {
                unr_token.set_tail_state(Resolved::Open);
            } else {
                unr_token.set_head_state(Resolved::Open);
                unr_token.swap_parts();
            }

            self.tokens[index].set_head_state(Resolved::Close);
            Some(token_index)
        } else {
            // opening token IS NOT ambiguous, (simple, simple) case
            unr_token.set_head_state(Resolved::Open);
            self.tokens[index].set_head_state(Resolved::Close);

            if let Some(RawToken {
                state: Resolved::Close,
                ..
            }) = self.tokens[index].tail.as_deref()
            {
                self.tokens[index].swap_parts();
            }

            indices.remove(i);
            Some(token_index)
        }
    }

    fn resolve_compound_token(&mut self, token_map: &mut TokenMap, index: usize) -> Option<usize> {
        let token_kind = self.tokens[index].token.kind;
        let indices = token_map.get_mut(token_kind, self.curr_scope)?;
        let (unr_token, i, token_index) = self.find_first_matching(indices, index)?;

        if unr_token.token.is_ambiguous() {
            // there is unresolved one that IS ambiguous (ambiguous, ambiguous)
            unr_token.split_ambiguous();

            let unr_kind = unr_token.token.kind;
            unr_token.set_state(Resolved::Open);

            self.tokens[index].split_ambiguous();
            self.tokens[index].set_state(Resolved::Close);

            // make sure the parts are symmetric
            if self.tokens[index].token.kind == unr_kind {
                self.tokens[index].swap_parts();
            }

            indices.remove(i);
            return Some(token_index);
        } else {
            // there is unresolved one that IS NOT ambiguous (simple, ambiguous)
            let kind = unr_token.token.kind;
            if let Some(token_index) = self.resolve_partial(indices, index, kind) {
                // try to resolve the remaining part
                self.resolve_token(token_map, index);
                return Some(token_index);
            }
        }

        None
    }

    fn resolve_partial(
        &mut self,
        indices: &mut Indices,
        index: usize,
        kind: TokenKind,
    ) -> Option<usize> {
        if let Some((unr_token, i, token_index)) = self.find_first_matching(indices, index) {
            unr_token.set_head_state(Resolved::Open);

            let curr_token = &mut self.tokens[index];

            curr_token.split_ambiguous();

            if curr_token.token.kind != kind {
                curr_token.set_tail_state(Resolved::Close);
            } else {
                curr_token.set_head_state(Resolved::Close);
                curr_token.swap_parts();
            }

            indices.remove(i);

            Some(token_index)
        } else {
            None
        }
    }

    fn find_first_matching(
        &mut self,
        indices: &Indices,
        curr_idx: usize,
    ) -> Option<(&mut RawToken<'token>, usize, usize)> {
        // find first unresolved token
        for (i, idx) in indices.iter().enumerate() {
            let curr_token = &self.tokens[curr_idx];
            let token = &self.tokens[*idx];

            if !token.state && token.token.overlaps(&curr_token.token) && token.token.opens() {
                if self.interrupted.iter().any(|range| range.contains(idx)) {
                    return None;
                }

                return Some((&mut self.tokens[*idx], i, *idx));
            }
        }

        None
    }

    pub(crate) fn into_iter(self) -> IntoIter<'token> {
        IntoIter {
            iter: self.tokens.into_iter(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct IntoIter<'token> {
    iter: vec::IntoIter<RawToken<'token>>,
}

impl<'token> Iterator for IntoIter<'token> {
    type Item = RawToken<'token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
