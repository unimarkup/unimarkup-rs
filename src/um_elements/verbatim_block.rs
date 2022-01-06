use crate::{
    middleend::{AsIrLines, ContentIrLine},
    um_elements::types::UnimarkupType,
};

/// Structure of a Unimarkup verbatim block element.
pub struct VerbatimBlock {
    /// Unique identifier for a verbatim block.
    pub id: String,

    /// The content of the verbatim block.
    pub content: String,

    /// Attributes of the verbatim block.
    pub attributes: String,

    /// Line number, where the verbatim block occurs in
    /// the Unimarkup document.
    pub line_nr: usize,
}

impl AsIrLines for VerbatimBlock {
    fn as_ir_lines(&self) -> Vec<ContentIrLine> {
        let line = ContentIrLine::new(
            &self.id,
            self.line_nr,
            UnimarkupType::VerbatimBlock.to_string(),
            &self.content,
            "",
            &self.attributes,
            "",
        );

        vec![line]
    }
}
