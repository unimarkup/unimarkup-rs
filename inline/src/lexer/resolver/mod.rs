use ribbon::{Enroll, Ribbon, Tape};

use std::{
    collections::{btree_map::Entry, BTreeMap},
    iter::Map,
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct IdxRef<'indices> {
    idx: usize,
    indices: &'indices mut Indices,
}

impl IdxRef<'_> {
    fn delete(self) {
        self.indices.remove(self.idx);
    }
}

type TokenIter<'token> = Map<TokenIterator<'token>, fn(crate::Token) -> RawToken>;

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
    /// Scopes are introduced by brackets (for example text groups).
    curr_scope: usize,

    /// `Token`s with index contained in any of the interrupted ranges are interrupted and should be
    /// treated as plain tokens.
    interrupted: Vec<Range<usize>>,

    /// Tape that enables expanding *visible* context inside of the TokenIterator.
    pub(crate) tape: Tape<TokenIter<'token>>,

    /// The index of the `Token` at head of `Tape`. That is, the number of the `Token` that will be
    /// returned by the resolver, with 0 being the number of the first `Token`.
    tape_idx: usize,

    /// Mapping of `TokenKind` to indices of `Token`s that are not yet resolved.
    unresolved: TokenMap,
}

impl<'token> TokenResolver<'token> {
    pub(crate) fn new(iter: TokenIterator<'token>) -> Self {
        let tape = iter.clone().map(RawToken::new as _).tape();

        Self {
            curr_scope: 0,
            interrupted: Vec::default(),
            tape,
            tape_idx: 0,
            unresolved: TokenMap::new(),
        }
    }

    fn next_token(&mut self) -> Option<RawToken<'token>> {
        // idea:
        //
        // * First token resolved -> Pop and return it
        // * First token not resolved:
        //   - expand tape until matching closing token is found
        //   - try to resolve the first token
        //   - repeat until whole token is resolved (two resolving needed for compound token)
        //   - pop and return it

        let mut looped = false;
        let mut expanded = false;
        loop {
            match self.tape.peek_front() {
                Some(t) => {
                    if t.token.kind.is_parenthesis() {
                        if t.token.kind.is_open_bracket() {
                            self.curr_scope += 1;
                        } else {
                            // TODO: syntax error, report to user
                            let _ = self.curr_scope.saturating_sub(1);
                        }

                        self.tape.pop_front();
                        self.tape_idx += 1;

                        continue;
                    } else if t.is_resolved() {
                        self.tape_idx += 1;
                        return self.tape.pop_front();
                    } else if (t.token.closes(None) && !t.token.opens()) || !expanded {
                        // token not resolved, but it's either:
                        // * closing token with no tokens before it, and is not an opening token
                        //   -> is plain token
                        //
                        // * or token not resolved, but there are no more tokens to check,
                        //   -> it can't be resolved, so it's a plain token
                        return self.tape.pop_front();
                    }
                }
                None => {
                    // no new tokens available, token at the head of tape cannot be resolved
                    if looped && !expanded {
                        return None;
                    }
                }
            }

            expanded = self.try_resolve();
            looped = true;
        }
    }

    fn try_resolve(&mut self) -> bool {
        // multiple cases for current - unresolved token relationship:
        // 1. current IS ambiguous, there is unresolved one that IS ambiguous (ambiguous, ambiguous)
        // 2. current IS ambiguous, there is unresolved one that IS NOT ambiguous (simple, ambiguous)
        // 3. current NOT ambiguous, there is unresolved one that IS NOT ambiguous: (simple, simple)
        // 4. current NOT ambiguous, there is unresolved one that IS ambiguous (ambiguous, simple)

        let expanded = self.tape.expand();

        let resolved_idx = match self.tape.peek_back() {
            None => return false,

            Some(end) => {
                if !end.token.closes(None) {
                    // if it's not closing, it cannot resolve opened token
                    None
                } else if end.token.is_ambiguous() {
                    self.resolve_compound_token()
                } else {
                    self.resolve_simple_token()
                }
            }
        };

        match resolved_idx {
            Some(idx) => {
                let tail_idx = self.tape_idx + self.tape.len() - 1;
                self.interrupted.push((idx + 1)..tail_idx);
            }
            None => {
                if let Some(end) = self.tape.peek_back() {
                    if end.token.opens() && !end.token.kind.is_parenthesis() {
                        self.unresolved.update_or_insert(
                            end.token.kind,
                            self.tape_idx + self.tape.len() - 1,
                            self.curr_scope,
                        );
                    }
                }
            }
        }

        expanded
    }

    fn resolve_simple_token(&mut self) -> Option<usize> {
        let token_kind = self.tape.peek_back()?.token.kind;

        let (unr_token, idx_ref, token_index) = self.find_first_matching(token_kind)?;

        if unr_token.token.is_ambiguous() {
            // opening token IS ambiguous (ambiguous, simple)

            unr_token.split_ambiguous();
            if unr_token.token.kind == token_kind {
                // make sure resolved part is in tail
                unr_token.swap_parts();
            }

            unr_token.set_tail_state(State::Open);

            self.tape.peek_back_mut()?.set_head_state(State::Close);

            Some(token_index)
        } else {
            idx_ref.delete();

            // opening token IS NOT ambiguous, only head available so mark it appropriately
            unr_token.set_head_state(State::Open);

            let curr_token = self.tape.peek_back_mut()?;

            curr_token.set_head_state(State::Close);

            if let Some(RawToken {
                state: State::Close,
                ..
            }) = curr_token.tail.as_deref()
            {
                curr_token.swap_parts();
            }

            Some(token_index)
        }
    }

    fn resolve_compound_token(&mut self) -> Option<usize> {
        let token_kind = self.tape.peek_back()?.token.kind;
        let (unr_token, idx_ref, token_index) = self.find_first_matching(token_kind)?;

        if unr_token.token.is_ambiguous() {
            idx_ref.delete();
            // there is unresolved one that IS ambiguous (ambiguous, ambiguous)
            unr_token.split_ambiguous();

            let unr_kind = unr_token.token.kind;
            unr_token.set_state(State::Open);

            let curr_token = self.tape.peek_back_mut()?;

            curr_token.split_ambiguous();
            curr_token.set_state(State::Close);

            // make sure the parts are symmetric
            if curr_token.token.kind == unr_kind {
                curr_token.swap_parts();
            }

            return Some(token_index);
        } else {
            // there is unresolved one that IS NOT ambiguous (simple, ambiguous)
            let kind = unr_token.token.kind;
            if let Some(token_index) = self.resolve_partial(kind) {
                // try to resolve the remaining part
                self.try_resolve();
                return Some(token_index);
            }
        }

        None
    }

    fn resolve_partial(&mut self, kind: TokenKind) -> Option<usize> {
        let (unr_token, idx_ref, token_index) = self.find_first_matching(kind)?;

        idx_ref.delete();

        unr_token.set_head_state(State::Open);

        let curr_token = self.tape.peek_back_mut()?;

        curr_token.split_ambiguous();

        if curr_token.token.kind != kind {
            curr_token.set_tail_state(State::Close);
        } else {
            curr_token.set_head_state(State::Close);
            curr_token.swap_parts();
        }

        Some(token_index)
    }

    fn find_first_matching(
        &mut self,
        token_kind: TokenKind,
    ) -> Option<(&mut RawToken<'token>, IdxRef, usize)> {
        let indices = self.unresolved.get_mut(token_kind, self.curr_scope)?;

        // find first unresolved token
        for (i, idx) in indices.iter().enumerate() {
            if *idx < self.tape_idx {
                // token already resolved
                indices.remove(i);
                return None;
            }

            let idx = *idx - self.tape_idx; // offset the index as tape progressed

            let curr_token = &self.tape.peek_back()?;
            let token = self.tape.peek_at(idx)?;

            if !token.state && token.token.overlaps(&curr_token.token) && token.token.opens() {
                if self
                    .interrupted
                    .iter()
                    .any(|range| range.contains(&(idx + self.tape_idx)))
                {
                    return None;
                }

                let idx_ref = IdxRef { idx: i, indices };
                return Some((self.tape.peek_at_mut(idx)?, idx_ref, idx + self.tape_idx));
            }
        }

        None
    }

    pub(crate) fn into_iter(self) -> IntoIter<'token> {
        IntoIter { inner: self }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct IntoIter<'token> {
    inner: TokenResolver<'token>,
}

impl<'token> Iterator for IntoIter<'token> {
    type Item = RawToken<'token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next_token()
    }
}
