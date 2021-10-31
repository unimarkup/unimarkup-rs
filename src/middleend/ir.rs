use crate::frontend::{parser::CursorPos, syntax_error::UmSyntaxError};

#[allow(dead_code)]
#[derive(Debug)]
pub struct IrLine {
    id: String,
    flow_count: usize,
    sub_flow_count: usize,
    fallback_text: String,
    text: String,
    attributes: String,
    block_type: String,
}

impl Default for IrLine {
    fn default() -> Self {
        IrLine {
            id: String::from("0"),
            flow_count: 0,
            sub_flow_count: 0,
            fallback_text: String::default(),
            text: String::default(),
            attributes: String::default(),
            block_type: String::default(),
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
        block_type: impl Into<String>,
    ) -> Self {
        IrLine {
            id: id.into(),
            flow_count,
            sub_flow_count,
            fallback_text: fallback_text.into(),
            text: text.into(),
            attributes: attributes.into(),
            block_type: block_type.into(),
        }
    }
}

#[derive(Debug)]
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
    fn parse_for_ir(
        content: &[&str],
        cursor_pos: &CursorPos,
    ) -> Result<(IrBlock, CursorPos), UmSyntaxError>;

    fn generate_ir_lines(&self) -> Vec<IrLine>;
}

pub trait WriteToIr {}
