use crate::um_elements::types::UnimarkupType;

use super::{parser::CursorPos, syntax_error::UmSyntaxError};

#[allow(dead_code)]
pub struct IrLine {
    id: String,
    flow_count: usize,
    sub_flow_count: usize,
    fallback_text: String,
    text: String,
    attributes: String,
    block_type: UnimarkupType,
}

impl Default for IrLine {
    fn default() -> Self {
        IrLine {
            id: String::from("0"),
            flow_count: 0,
            sub_flow_count: 0,
            fallback_text: String::from(""),
            text: String::from(""),
            attributes: String::from(""),
            block_type: UnimarkupType::Paragraph,
        }
    }
}

impl IrLine {
    pub fn new(
        id: impl Into<String>,
        flow_count: usize,
        sub_flow_count: usize,
        fallback_text: impl Into<String>,
        text: impl Into<String>,
        attributes: impl Into<String>,
        block_type: UnimarkupType,
    ) -> Self {
        IrLine {
            id: id.into(),
            flow_count,
            sub_flow_count,
            fallback_text: fallback_text.into(),
            text: text.into(),
            attributes: attributes.into(),
            block_type,
        }
    }
}

pub struct IrBlock {
    pub lines: Vec<IrLine>,
}

impl IrBlock {
    pub fn new() -> Self {
        let lines = vec![];

        IrBlock { lines }
    }

    pub fn push_line(&mut self, line: IrLine) {
        self.lines.push(line);
    }
}

impl Default for IrBlock {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ParseForIr {
    fn parse_for_ir<'a>(
        content: &'a [&str],
        cursor_pos: &CursorPos,
    ) -> Result<(IrBlock, CursorPos), UmSyntaxError<'a>>;

    fn generate_ir_lines(&self) -> Vec<IrLine>;
}

pub trait WriteToIr {}
