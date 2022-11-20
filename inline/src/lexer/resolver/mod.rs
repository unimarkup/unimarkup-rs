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

#[derive(Debug, Clone)]
pub(crate) struct TokenResolver<'a> {
    iter: TokenIterator<'a>,
    curr_scope: usize,
    interrupted: Vec<Range<usize>>,
    pub(crate) tokens: Vec<RawToken>,
}

impl<'a> TokenResolver<'a> {
    pub(crate) fn new(iter: TokenIterator<'a>) -> Self {
        Self {
            iter,
            curr_scope: 0,
            interrupted: Vec::default(),
            tokens: Vec::default(),
        }
    }

    fn consume_line(&mut self) {
        for token in self.iter.by_ref() {
            let should_break = matches!(token.kind, TokenKind::EndOfLine | TokenKind::Newline);

            self.tokens.push(RawToken {
                token,
                resolved: Resolved::Neither,
                tail: None,
            });

            if should_break {
                break;
            }
        }
    }

    pub(crate) fn resolve(&mut self) {
        if self.tokens.is_empty() {
            self.consume_line();
        }

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

            if !self.tokens[index].resolved {
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

        if self.tokens[index].token.closes() {
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
            // first is ambiguous, so we have to split it
            unr_token.split_ambiguous();

            // easier manipulation on first part of ambiguous unresolved token
            if unr_token.token.kind != token_kind {
                unr_token.swap_parts();
            }

            unr_token.resolved = Resolved::Open;

            unr_token.swap_parts();

            self.tokens[index].resolved = Resolved::Close;
            Some(token_index)
        } else {
            // opening token IS NOT ambiguous, (simple, simple) case
            // resolve them both
            unr_token.resolved = Resolved::Open;

            self.tokens[index].resolved = Resolved::Close;

            if let Some(RawToken {
                resolved: Resolved::Close,
                ..
            }) = self.tokens[index].tail.as_deref()
            {
                // tail was resolved sooner, so it should be first part
                self.tokens[index].swap_parts();
            }

            // remove unresolved token
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
            // split them both
            unr_token.split_ambiguous();

            // resolve them both
            unr_token.resolved = Resolved::Open;
            let unr_kind = unr_token.token.kind;

            if let Some(tail) = unr_token.tail.as_mut() {
                tail.resolved = Resolved::Open;
            }

            self.tokens[index].split_ambiguous();
            self.tokens[index].resolved = Resolved::Close;

            if let Some(tail) = self.tokens[index].tail.as_mut() {
                tail.resolved = Resolved::Close;
            }

            // make sure the parts are symmetric
            if self.tokens[index].token.kind == unr_kind {
                dbg!(&self.tokens[index].token.kind);
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
            unr_token.resolved = Resolved::Open;

            let curr_token = &mut self.tokens[index];

            curr_token.split_ambiguous();

            if curr_token.token.kind != kind {
                curr_token.swap_parts();
            }

            curr_token.resolved = Resolved::Close;
            indices.remove(i);

            // move unresolved part, for it to be resolved
            curr_token.swap_parts();

            Some(token_index)
        } else {
            None
        }
    }

    fn find_first_matching(
        &mut self,
        indices: &Indices,
        curr_idx: usize,
    ) -> Option<(&mut RawToken, usize, usize)> {
        // find first unresolved token
        for (i, idx) in indices.iter().enumerate() {
            let curr_token = &self.tokens[curr_idx];
            let token = &self.tokens[*idx];

            if !token.resolved && token.token.overlaps(&curr_token.token) && token.token.opens() {
                if self.interrupted.iter().any(|range| range.contains(idx)) {
                    return None;
                }

                return Some((&mut self.tokens[*idx], i, *idx));
            }
        }

        None
    }

    pub(crate) fn into_iter(self) -> IntoIter<'a> {
        IntoIter {
            resolver: self,
            iter: Vec::new().into_iter(),
            index: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct IntoIter<'a> {
    resolver: TokenResolver<'a>,
    iter: vec::IntoIter<RawToken>,
    index: usize,
}

impl IntoIter<'_> {
    fn next_token(&mut self) -> Option<RawToken> {
        if let Some(token) = self.iter.next() {
            return Some(token);
        }

        self.resolver.resolve();
        self.iter = std::mem::take(&mut self.resolver.tokens).into_iter();
        self.index = 0;
        self.iter.next()
    }
}

impl Iterator for IntoIter<'_> {
    type Item = RawToken;

    fn next(&mut self) -> Option<Self::Item> {
        let next_token = self.next_token()?;

        self.index += 1;
        Some(next_token)
    }
}
