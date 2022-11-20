use std::ops::Not;

use crate::{Spacing, Span, Token, TokenKind};

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
pub(crate) struct RawToken {
    pub(crate) token: Token,
    pub(crate) state: Resolved,
    pub(crate) tail: Option<Box<RawToken>>,
}

impl RawToken {
    fn order(&mut self) {
        if let Some(ref sec_part) = self.tail {
            match (self.state, sec_part.state) {
                (Resolved::Open, Resolved::Close)
                | (Resolved::Neither, Resolved::Close)
                | (Resolved::Open, Resolved::Neither) => {}
                _ => self.swap_parts(),
            }
        }
    }

    pub(crate) fn pop(&mut self) -> Option<RawToken> {
        // moves the next token to `tail` so it can be `take`n
        self.order();

        self.tail.take().map(|mut token| {
            if self.token.span.start.column < token.token.span.start.column {
                let (first, second) = self.token.span.swapped(&token.token.span);
                self.token.span = first;
                token.token.span = second;
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
        let mut token = Token {
            kind: TokenKind::Plain,
            span: Span::default(),
            spacing: Spacing::default(),
            content: None,
        };

        std::mem::swap(&mut self.token, &mut token);

        let (first, second) = token.split_ambiguous();
        self.token = first;
        self.tail = Some(Box::new(RawToken {
            token: second,
            state: Resolved::Neither,
            tail: None,
        }));
    }

    pub(crate) fn set_head_state(&mut self, state: Resolved) {
        self.state = state;
    }

    pub(crate) fn set_tail_state(&mut self, state: Resolved) {
        if let Some(tail) = self.tail.as_mut() {
            tail.state = state;
        }
    }

    pub(crate) fn set_state(&mut self, state: Resolved) {
        self.set_head_state(state);
        self.set_tail_state(state);
    }
}

impl From<RawToken> for Token {
    fn from(unr_token: RawToken) -> Self {
        let mut token = unr_token.token;

        token.spacing = Spacing::from(unr_token.state);
        if !token.kind.is_parenthesis() && token.is_nesting_token() && !unr_token.state {
            token.content = Some(token.as_str().to_string());
            token.kind = TokenKind::Plain;
        }

        token
    }
}
