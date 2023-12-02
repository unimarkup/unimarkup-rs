use crate::lexer::position::Position;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DocComment {
    content: String,
    impl_closed: bool,
    start: Position,
    end: Position,
}
