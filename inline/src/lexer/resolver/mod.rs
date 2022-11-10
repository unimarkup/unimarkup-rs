#![allow(dead_code)]
#![warn(clippy::pedantic)]

use std::collections::{BTreeMap, VecDeque};

use crate::{Spacing, Span, Token, TokenIterator, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct UnresolvedToken {
    token: Token,
    resolved: bool,
    second_part: Option<Box<UnresolvedToken>>,
}

impl UnresolvedToken {
    pub(crate) fn pop(&mut self) -> Option<UnresolvedToken> {
        self.second_part.take().map(|mut token| {
            if self.token.span.start.column < token.token.span.start.column {
                std::mem::swap(&mut self.token, &mut token.token);
                std::mem::swap(&mut self.resolved, &mut token.resolved);
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
            resolved: false,
            second_part: None,
        }));
    }
}

impl From<UnresolvedToken> for Token {
    fn from(unr_token: UnresolvedToken) -> Self {
        let mut token = unr_token.token;

        if !token.kind.is_parenthesis() && token.is_nesting_token() && !unr_token.resolved {
            token.content = Some(format!("|AsPlain({})|", token.as_str()));
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

#[derive(Debug, Clone)]
pub(crate) struct TokenResolver<'a> {
    iter: TokenIterator<'a>,
    curr_scope: usize,
    token_map: BTreeMap<TokenKind, ScopedIndices>,
    pub(crate) tokens: VecDeque<UnresolvedToken>,
}

impl<'a> TokenResolver<'a> {
    pub(crate) fn new(iter: TokenIterator<'a>) -> Self {
        Self {
            iter,
            curr_scope: 0,
            token_map: BTreeMap::default(),
            tokens: VecDeque::default(),
        }
    }

    fn consume_line(&mut self) {
        for token in self.iter.by_ref() {
            let should_stop = matches!(token.kind(), TokenKind::Newline | TokenKind::EndOfLine);

            let unresolved_token = UnresolvedToken {
                token,
                resolved: false,
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
        let mut token_map: BTreeMap<TokenKind, ScopedIndices> = BTreeMap::new();

        for index in 0..self.tokens.len() {
            // on open/close bracket simply increment/decrement scope
            if self.tokens[index].token.kind.is_open_bracket() {
                self.curr_scope += 1;
                continue;
            } else if self.tokens[index].token.kind.is_close_bracket() {
                self.curr_scope -= 1; // panics on underflow, but that is a bug!
                continue;
            }

            // try to resolve token
            self.resolve_token(&mut token_map, index);

            if !self.tokens[index].resolved {
                // save positions of every unresolved token
                token_map
                    .entry(self.tokens[index].token.kind)
                    .and_modify(|indices| indices.indices.push(index))
                    .or_insert_with(|| ScopedIndices {
                        scope: self.curr_scope,
                        indices: vec![index],
                    });
            }
        }
    }

    fn resolve_token(&mut self, token_map: &mut BTreeMap<TokenKind, ScopedIndices>, index: usize) {
        // multiple cases for current - unresolved token relationship:
        // 1. current NOT ambiguous, there is unresolved one that IS NOT ambiguous: (simple, simple)
        // 2. current NOT ambiguous, there is unresolved one that IS ambiguous (ambiguous, simple)
        // 3. current IS ambiguous, there is unresolved one that IS ambiguous (ambiguous, ambiguous)
        // 4. current IS ambiguous, there is unresolved one that IS NOT ambiguous (simple, ambiguous)

        // (1. and 2.) current NOT ambiguous
        if !self.tokens[index].token.is_ambiguous() && self.tokens[index].token.closes() {
            // there is unresolved one that IS NOT ambiguous: (simple, simple)
            if let Some(indices) = token_map.get_mut(&self.tokens[index].token.kind) {
                if let Some((unr_token, i)) = self.find_first_unresolved(indices) {
                    // resolve them both
                    unr_token.resolved = true;
                    self.tokens[index].resolved = true;

                    // remove unresolved token
                    indices.indices.remove(i);
                }
            } else if let Some(ambiguous_variant) =
                self.tokens[index].token.kind.get_ambiguous_variant()
            {
                if let Some(indices) = token_map.get_mut(&ambiguous_variant) {
                    let token_kind = self.tokens[index].token.kind;
                    if let Some((unr_token, i)) = self.find_first_unresolved(indices) {
                        // first token COULD be ambiguous
                        // first is ambiguous, so we have to split it
                        unr_token.split_ambiguous();

                        // easier manipulation on first part of ambiguous unresolved token
                        if unr_token.token.kind != token_kind {
                            unr_token.swap_parts();
                        }

                        // resolve
                        unr_token.resolved = true;

                        if unr_token.token.kind == token_kind {
                            unr_token.swap_parts();
                        }

                        if unr_token.resolved {
                            indices.indices.remove(i);
                        }

                        self.tokens[index].resolved = true;
                    }
                }
            }
        }
        // (3. and 4.) current IS ambiguous
        else if self.tokens[index].token.closes() {
            // there is unresolved one that IS ambiguous (ambiguous, ambiguous)
            if let Some(indices) = token_map.get_mut(&self.tokens[index].token.kind) {
                if let Some((unr_token, i)) = self.find_first_unresolved(indices) {
                    // split them both
                    unr_token.split_ambiguous();

                    // resolve them both
                    unr_token.resolved = true;
                    if let Some(second) = unr_token.second_part.as_mut() {
                        second.resolved = true;
                    }

                    self.tokens[index].split_ambiguous();
                    self.tokens[index].resolved = true;
                    if let Some(second) = self.tokens[index].second_part.as_mut() {
                        second.resolved = true;
                    }

                    indices.indices.remove(i);
                }
            } else if let Some((first_kind, second_kind)) =
                self.tokens[index].token.kind.get_ambiguous_parts()
            {
                // there is unresolved one that IS NOT ambiguous (simple, ambiguous)
                let mut resolved = false;
                if let Some(indices) = token_map.get_mut(&first_kind) {
                    if self.resolve_partial_kind(indices, index, first_kind) {
                        resolved = true;
                        // try to resolve the remaining part
                        // Should fall into case (1. and 2.)
                        self.resolve_token(token_map, index);
                    }
                }

                if let (Some(indices), false) = (token_map.get_mut(&second_kind), resolved) {
                    if self.resolve_partial_kind(indices, index, second_kind) {
                        // try to resolve the remaining part
                        // Should fall into case (1. and 2.)
                        self.resolve_token(token_map, index);
                    }
                }
            }
        }
    }

    fn resolve_partial_kind(
        &mut self,
        indices: &mut ScopedIndices,
        index: usize,
        kind: TokenKind,
    ) -> bool {
        if let Some((unr_token, i)) = self.find_first_unresolved(indices) {
            unr_token.resolved = true;

            let curr_token = &mut self.tokens[index];

            curr_token.split_ambiguous();

            if curr_token.token.kind != kind {
                curr_token.swap_parts();
            }
            curr_token.resolved = true;

            indices.indices.remove(i);

            // move unresolved part, for it to be resolved
            curr_token.swap_parts();

            true
        } else {
            false
        }
    }

    fn find_first_unresolved(
        &mut self,
        indices: &ScopedIndices,
    ) -> Option<(&mut UnresolvedToken, usize)> {
        let indices = if indices.scope == self.curr_scope {
            &indices.indices
        } else {
            return None;
        };

        // find first unresolved token
        for (i, idx) in indices.iter().enumerate() {
            if !self.tokens[*idx].resolved && self.tokens[*idx].token.opens() {
                return Some((&mut self.tokens[*idx], i));
            }
        }

        None
    }
}
