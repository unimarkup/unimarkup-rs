use unimarkup_core::elements::{blocks::Block, Blocks};

pub(crate) trait AsSnapshot {
    fn as_snapshot(&self) -> String;
}

impl AsSnapshot for Blocks {
    fn as_snapshot(&self) -> String {
        let mut output = String::with_capacity(self.len());

        for block in self {
            output.push_str(&block.as_snapshot());
            output.push('\n');
        }

        output
    }
}

impl AsSnapshot for Block {
    fn as_snapshot(&self) -> String {
        match self {
            Block::Paragraph(b) => b.as_snapshot(),
            // Block::Verbatim(b) => b.as_snapshot(),
            // Block::Heading(b) => b.as_snapshot(),
            block => unimplemented!("Parsing of block {block:?} is not implemented!"),
        }
    }
}
