use std::ops::Not;

use crate::{Spacing, SpanExt, Token, TokenKind};

// Token can either be opening one, closing one, or neither
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum State {
    Open,
    Close,
    Unresolved,
    Plain,
}

impl From<TokenKind> for State {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::OpenParens
            | TokenKind::CloseParens
            | TokenKind::OpenBracket
            | TokenKind::CloseBracket
            | TokenKind::OpenBrace
            | TokenKind::CloseBrace
            | TokenKind::Substitution
            | TokenKind::Newline
            | TokenKind::EscapedNewline
            | TokenKind::Whitespace
            | TokenKind::EscapedWhitespace
            | TokenKind::Plain => State::Plain,

            _ => State::Unresolved,
        }
    }
}

impl Not for State {
    type Output = bool;

    fn not(self) -> Self::Output {
        matches!(self, Self::Unresolved)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct RawToken<'input> {
    pub(crate) token: Token<'input>,
    pub(crate) state: State,
    pub(crate) tail: Option<Box<RawToken<'input>>>,
}

impl<'input> RawToken<'input> {
    pub(crate) fn new(token: Token) -> RawToken {
        let state = State::from(token.kind);

        RawToken {
            token,
            state,
            tail: None,
        }
    }

    fn order(&mut self) {
        if let Some(ref sec_part) = self.tail {
            match (self.state, sec_part.state) {
                (State::Open, State::Close)
                | (State::Unresolved, State::Close)
                | (State::Open, State::Unresolved) => {}
                _ => self.swap_parts(),
            }
        }
    }

    pub(crate) fn pop(&mut self) -> Option<RawToken<'input>> {
        // moves the next token to `tail` so it can be taken
        self.order();

        self.tail.take().map(|mut token| {
            if self.token.span.start.col_utf8 < token.token.span.start.col_utf8 {
                let (first, second) = self.token.span.swap(&token.token.span);
                self.token.span = second;
                token.token.span = first;
            }

            *token
        })
    }

    pub(crate) fn swap_parts(&mut self) {
        if let Some(tail) = self.tail.as_mut() {
            std::mem::swap(&mut self.token, &mut tail.token);
            std::mem::swap(&mut self.state, &mut tail.state);
        }
    }

    pub(crate) fn split_ambiguous(&mut self) {
        let token = std::mem::take(&mut self.token);

        let (first, second) = token.split_ambiguous();
        self.token = first;
        self.tail = Some(Box::new(RawToken {
            token: second,
            state: State::Unresolved,
            tail: None,
        }));
    }

    pub(crate) fn set_head_state(&mut self, state: State) {
        self.state = state;
    }

    pub(crate) fn set_tail_state(&mut self, state: State) {
        if let Some(tail) = self.tail.as_mut() {
            tail.state = state;
        }
    }

    pub(crate) fn set_state(&mut self, state: State) {
        self.set_head_state(state);
        self.set_tail_state(state);
    }

    pub(crate) fn is_resolved(&self) -> bool {
        let self_resolved = self.state != State::Unresolved;

        match self.tail.as_ref() {
            Some(tail) => tail.is_resolved() && self_resolved,
            None => self_resolved,
        }
    }
}

impl<'token> From<RawToken<'token>> for Token<'token> {
    fn from(unr_token: RawToken<'token>) -> Self {
        let mut token = unr_token.token;

        token.spacing = Spacing::from(unr_token.state);
        if !token.kind.is_parenthesis() && token.is_nesting_token() && !unr_token.state {
            let content_str = token.as_str();
            token.content = Some(content_str);
            token.kind = TokenKind::Plain;
        }

        token
    }
}
