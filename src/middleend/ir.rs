use crate::frontend::{parser::CursorPos, syntax_error::UmSyntaxError};

#[allow(dead_code)]
#[derive(Debug)]
pub struct IrLine {
    pub id: String,
    pub um_type: String,
    pub line_nr: usize,
    pub text: String,
    pub fallback_text: String,
    pub attributes: String,
    pub fallback_attributes: String,
}

impl Default for IrLine {
    fn default() -> Self {
        IrLine {
            id: String::from("0"),
            um_type: String::default(),
            line_nr: 0,
            text: String::default(),
            fallback_text: String::default(),
            attributes: String::default(),
            fallback_attributes: String::default(),
        }
    }
}

impl IrLine {
    pub fn new(
        id: impl Into<String>,
        um_type: impl Into<String>,
        line_nr: usize,
        text: impl Into<String>,
        fallback_text: impl Into<String>,
        attributes: impl Into<String>,
        fallback_attributes: impl Into<String>,
    ) -> Self {
        IrLine {
            id: id.into(),
            um_type: um_type.into(),
            line_nr,
            text: text.into(),
            fallback_text: fallback_text.into(),
            attributes: attributes.into(),
            fallback_attributes: fallback_attributes.into(),
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

    fn generate_ir_lines(&self, line_nr: usize) -> Vec<IrLine>;
}

pub trait WriteToIr {}
