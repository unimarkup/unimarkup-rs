use crate::{
    lexer::{
        position::Position,
        token::{iterator::TokenIterator, TokenKind},
    },
    parsing::Element,
};

mod doc_comment;

pub use doc_comment::DocComment;
use itertools::{Itertools, PeekingNext};

pub const COMMENT_TOKEN_KIND: TokenKind = TokenKind::Semicolon(Comment::keyword_len());

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Comment {
    content: String,
    implicit_closed: bool,
    start: Position,
    end: Position,
}

impl Element for Comment {
    fn as_unimarkup(&self) -> String {
        format!(
            "{}{}{}",
            Comment::keyword(),
            self.content,
            if self.implicit_closed {
                ""
            } else {
                Comment::keyword()
            }
        )
    }

    fn start(&self) -> Position {
        self.start
    }

    fn end(&self) -> Position {
        self.end
    }
}

impl Comment {
    pub const fn keyword() -> &'static str {
        ";;"
    }

    pub const fn keyword_len() -> usize {
        Comment::keyword().len()
    }

    pub fn parse(iter: &mut TokenIterator) -> Option<Self> {
        let peek_index = iter.peek_index();
        let next_token_opt = iter.peeking_next(|_| true);
        let open_token = match next_token_opt {
            Some(token) if token.kind == COMMENT_TOKEN_KIND => token,
            Some(_) | None => {
                iter.set_peek_index(peek_index);
                return None;
            }
        };

        let next_tokens = iter.peeking_take_while(|t| {
            !matches!(
                t.kind,
                COMMENT_TOKEN_KIND
                    | TokenKind::Newline
                    | TokenKind::EscapedNewline
                    | TokenKind::Eoi
            )
        });
        let mut content = String::new();
        let mut end = open_token.end;

        for t in next_tokens {
            content.push_str(&String::from(t));
            end = t.end;
        }

        // Newlines are not part of the comment
        let implicit_closed = if iter.peek_kind() == Some(COMMENT_TOKEN_KIND) {
            let close_token = iter.peeking_next(|_| true).expect("Peeked kind before.");
            end = close_token.end;
            false
        } else {
            true
        };

        iter.skip_to_peek();

        Some(Self {
            content,
            implicit_closed,
            start: open_token.start,
            end,
        })
    }
}
