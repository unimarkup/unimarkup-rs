#![allow(dead_code)]

use std::{
    collections::{btree_map::Entry, BTreeMap, VecDeque},
    ops::{Not, Range},
    vec,
};

use crate::{Spacing, Span, Token, TokenIterator, TokenKind};

// Token can either be opening one, closing one, or neither
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Resolved {
    Open,
    Close,
    Neither,
}

impl Not for Resolved {
    type Output = bool;

    fn not(self) -> Self::Output {
        matches!(self, Self::Neither)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct UnresolvedToken {
    token: Token,
    resolved: Resolved,
    second_part: Option<Box<UnresolvedToken>>,
}

impl UnresolvedToken {
    fn order(&mut self) {
        if let Some(ref sec_part) = self.second_part {
            match (self.resolved, sec_part.resolved) {
                (Resolved::Open, Resolved::Close)
                | (Resolved::Neither, Resolved::Close)
                | (Resolved::Open, Resolved::Neither) => {}
                _ => self.swap_parts(),
            }
        }
    }

    pub(crate) fn pop(&mut self) -> Option<UnresolvedToken> {
        // sets the next token in order to `second_part` so it can be `take`n
        self.order();

        self.second_part.take().map(|mut token| {
            if self.token.span.start.column < token.token.span.start.column {
                let (first, second) = self.token.span.swapped(&token.token.span);
                self.token.span = first;
                token.token.span = second;
            }

            *token
        })
    }

    pub(crate) fn pop_second_part(&mut self) -> Option<UnresolvedToken> {
        self.second_part.take().map(|boxed| *boxed)
    }

    pub(crate) fn swap_parts(&mut self) {
        if let Some(second_token) = self.second_part.as_mut() {
            std::mem::swap(&mut self.token, &mut second_token.token);
            std::mem::swap(&mut self.resolved, &mut second_token.resolved);
        }
    }

    fn split_ambiguous(&mut self) {
        let mut token = Token {
            kind: TokenKind::Plain,
            span: Span::default(),
            spacing: Spacing::default(),
            content: None,
        };

        std::mem::swap(&mut self.token, &mut token);

        let (first, second) = token.split_ambiguous();
        self.token = first;
        self.second_part = Some(Box::new(UnresolvedToken {
            token: second,
            resolved: Resolved::Neither,
            second_part: None,
        }));
    }
}

impl From<UnresolvedToken> for Token {
    fn from(unr_token: UnresolvedToken) -> Self {
        let mut token = unr_token.token;

        token.spacing = Spacing::from(unr_token.resolved);
        if !token.kind.is_parenthesis() && token.is_nesting_token() && !unr_token.resolved {
            token.content = Some(token.as_str().into());
            token.kind = TokenKind::Plain;
        }

        token
    }
}

#[derive(Debug, Clone)]
struct ScopedIndices {
    scope: usize,
    indices: Vec<usize>,
}

impl ScopedIndices {
    fn push(&mut self, index: usize) {
        self.indices.push(index);
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
struct TokenMap {
    map: BTreeMap<TokenKind, ScopedIndices>,
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

    fn entry(&mut self, kind: TokenKind) -> Entry<TokenKind, ScopedIndices> {
        self.map.entry(Self::general_key(kind))
    }

    fn get(&self, kind: TokenKind) -> Option<&ScopedIndices> {
        self.map.get(&Self::general_key(kind))
    }

    fn get_mut(&mut self, kind: TokenKind) -> Option<&mut ScopedIndices> {
        self.map.get_mut(&Self::general_key(kind))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TokenResolver<'a> {
    iter: TokenIterator<'a>,
    curr_scope: usize,
    interrupted: Vec<Range<usize>>,
    pub(crate) tokens: VecDeque<UnresolvedToken>,
}

impl<'a> TokenResolver<'a> {
    pub(crate) fn new(iter: TokenIterator<'a>) -> Self {
        Self {
            iter,
            curr_scope: 0,
            interrupted: Vec::default(),
            tokens: VecDeque::default(),
        }
    }

    fn consume_line(&mut self) {
        for token in self.iter.by_ref() {
            let should_stop = matches!(token.kind(), TokenKind::Newline | TokenKind::EndOfLine);

            let unresolved_token = UnresolvedToken {
                token,
                resolved: Resolved::Neither,
                second_part: None,
            };

            self.tokens.push_back(unresolved_token);

            if should_stop {
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
                token_map
                    .entry(kind)
                    .and_modify(|indices| indices.push(index))
                    .or_insert_with(|| ScopedIndices {
                        scope: self.curr_scope,
                        indices: vec![index],
                    });
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

        let indices = token_map.get_mut(token_kind)?;
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

            if let Some(UnresolvedToken {
                resolved: Resolved::Close,
                ..
            }) = self.tokens[index].second_part.as_deref()
            {
                // second part was resolved sooner, so it should be first part
                self.tokens[index].swap_parts();
            }

            // remove unresolved token
            indices.indices.remove(i);
            Some(token_index)
        }
    }

    fn resolve_compound_token(&mut self, token_map: &mut TokenMap, index: usize) -> Option<usize> {
        let token_kind = self.tokens[index].token.kind;
        let indices = token_map.get_mut(token_kind)?;
        let (unr_token, i, token_index) = self.find_first_matching(indices, index)?;

        if unr_token.token.is_ambiguous() {
            // there is unresolved one that IS ambiguous (ambiguous, ambiguous)
            // split them both
            unr_token.split_ambiguous();

            // resolve them both
            unr_token.resolved = Resolved::Open;
            let unr_kind = unr_token.token.kind;

            if let Some(second) = unr_token.second_part.as_mut() {
                second.resolved = Resolved::Open;
            }

            self.tokens[index].split_ambiguous();
            self.tokens[index].resolved = Resolved::Close;

            if let Some(second) = self.tokens[index].second_part.as_mut() {
                second.resolved = Resolved::Close;
            }

            // make sure the parts are symmetric
            // if self.tokens[index].token.kind == unr_kind {
            if token_kind != unr_kind {
                dbg!(&self.tokens[index].token.kind);
                self.tokens[index].swap_parts();
            }

            indices.indices.remove(i);
            return Some(token_index);
        } else {
            // there is unresolved one that IS NOT ambiguous (simple, ambiguous)
            let kind = unr_token.token.kind;
            if let Some(token_index) = self.resolve_partial_kind(indices, index, kind) {
                // try to resolve the remaining part
                self.resolve_token(token_map, index);
                return Some(token_index);
            }
        }

        None
    }

    fn resolve_partial_kind(
        &mut self,
        indices: &mut ScopedIndices,
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
            indices.indices.remove(i);

            // move unresolved part, for it to be resolved
            curr_token.swap_parts();

            Some(token_index)
        } else {
            None
        }
    }

    fn find_first_matching(
        &mut self,
        indices: &ScopedIndices,
        curr_idx: usize,
    ) -> Option<(&mut UnresolvedToken, usize, usize)> {
        let indices = if indices.scope == self.curr_scope {
            &indices.indices
        } else {
            return None;
        };

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
    iter: vec::IntoIter<UnresolvedToken>,
    index: usize,
}

impl IntoIter<'_> {
    fn next_token(&mut self) -> Option<UnresolvedToken> {
        if let Some(token) = self.iter.next() {
            return Some(token);
        }

        self.resolver.resolve();
        self.iter = Vec::from(std::mem::take(&mut self.resolver.tokens)).into_iter();
        self.index = 0;
        self.iter.next()
    }
}

impl Iterator for IntoIter<'_> {
    type Item = UnresolvedToken;

    fn next(&mut self) -> Option<Self::Item> {
        let next_token = self.next_token()?;

        self.index += 1;
        Some(next_token)
    }
}
