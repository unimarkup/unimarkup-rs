use std::ops::Deref;

use unimarkup_commons::lexer::token::TokenKind;
use unimarkup_commons::test_runner::as_snapshot::AsSnapshot;
use unimarkup_parser::elements::blocks::Block;
use unimarkup_parser::elements::Blocks;

mod bullet_list;
mod heading;
mod paragraph;
mod verbatim_block;

#[derive(Debug)]
pub struct Snapshot<T>(pub T);

impl<T> Deref for Snapshot<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsSnapshot for Snapshot<Blocks> {
    fn as_snapshot(&self) -> String {
        if self.len() > 1 {
            let content: String = self.0.iter().fold(String::new(), |s, block| {
                format!("{}{}\n", s, Snapshot(block).as_snapshot())
            });

            content.rsplit_once('\n').unwrap().0.to_string()
        } else {
            self.0
                .iter()
                .map(|block| Snapshot(block).as_snapshot())
                .collect()
        }
    }
}

impl AsSnapshot for Snapshot<&Block> {
    fn as_snapshot(&self) -> String {
        match **self {
            Block::Paragraph(block) => Snapshot(block).as_snapshot(),
            Block::BulletList(block) => Snapshot(block).as_snapshot(),
            Block::VerbatimBlock(block) => Snapshot(block).as_snapshot(),
            Block::Heading(block) => Snapshot(block).as_snapshot(),
            Block::Blankline(_) => String::from(TokenKind::Blankline),
            _ => unimplemented!("TODO: Implement snapshot for {:?}", self),
        }
    }
}
