use crate::frontend::{parser::CursorPos, syntax_error::UmSyntaxError};

#[derive(Debug)]
pub struct ContentIrLine {
    pub id: String,
    pub um_type: String,
    pub line_nr: usize,
    pub text: String,
    pub fallback_text: String,
    pub attributes: String,
    pub fallback_attributes: String,
}

impl Default for ContentIrLine {
    fn default() -> Self {
        ContentIrLine {
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

impl ContentIrLine {
    pub fn new(
        id: impl Into<String>,
        um_type: impl Into<String>,
        line_nr: usize,
        text: impl Into<String>,
        fallback_text: impl Into<String>,
        attributes: impl Into<String>,
        fallback_attributes: impl Into<String>,
    ) -> Self {
        ContentIrLine {
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
pub struct VariableIrLine {
    pub name: String,
    pub macro_type: String,
    pub value: String,
    pub fallback_value: String,
}

impl Default for VariableIrLine {
    fn default() -> Self {
        VariableIrLine {
            name: String::default(),
            macro_type: String::default(),
            value: String::default(),
            fallback_value: String::default(),
        }
    }
}

impl VariableIrLine {
    pub fn new(
        name: impl Into<String>,
        macro_type: impl Into<String>,
        value: impl Into<String>,
        fallback_value: impl Into<String>,
    ) -> Self {
        VariableIrLine {
            name: name.into(),
            macro_type: macro_type.into(),
            value: value.into(),
            fallback_value: fallback_value.into(),
        }
    }
}

#[derive(Debug)]
pub struct MacroIrLine {
    pub name: String,
    pub macro_type: String,
    pub parameters: String,
    pub value: String,
    pub fallback_value: String,
}

impl Default for MacroIrLine {
    fn default() -> Self {
        MacroIrLine {
            name: String::default(),
            macro_type: String::default(),
            parameters: String::default(),
            value: String::default(),
            fallback_value: String::default(),
        }
    }
}

impl MacroIrLine {
    pub fn new(
        name: impl Into<String>,
        macro_type: impl Into<String>,
        parameters: impl Into<String>,
        value: impl Into<String>,
        fallback_value: impl Into<String>,
    ) -> Self {
        MacroIrLine {
            name: name.into(),
            macro_type: macro_type.into(),
            parameters: parameters.into(),
            value: value.into(),
            fallback_value: fallback_value.into(),
        }
    }
}

#[derive(Debug)]
pub struct IrBlock {
    content_lines: Vec<ContentIrLine>,
    variable_lines: Vec<VariableIrLine>,
    macro_lines: Vec<MacroIrLine>,
}

impl IrBlock {
    pub fn new() -> Self {
        IrBlock {
            // Vec::new() does not allocate on heap.
            // First allocation occurs when first element is pushed into the Vec
            content_lines: Vec::new(),
            variable_lines: Vec::new(),
            macro_lines: Vec::new(),
        }
    }

    pub fn push_content_line(&mut self, line: ContentIrLine) {
        self.content_lines.push(line);
    }

    pub fn append_content_lines(&mut self, lines: &mut Vec<ContentIrLine>) {
        self.content_lines.append(lines);
    }

    pub fn get_content_lines(&self) -> &Vec<ContentIrLine> {
        &self.content_lines
    }

    pub fn push_variable_line(&mut self, line: VariableIrLine) {
        self.variable_lines.push(line);
    }

    pub fn append_variable_lines(&mut self, lines: &mut Vec<VariableIrLine>) {
        self.variable_lines.append(lines);
    }

    pub fn get_variable_lines(&self) -> &Vec<VariableIrLine> {
        &self.variable_lines
    }

    pub fn push_macro_line(&mut self, line: MacroIrLine) {
        self.macro_lines.push(line);
    }

    pub fn append_macro_lines(&mut self, lines: &mut Vec<MacroIrLine>) {
        self.macro_lines.append(lines);
    }

    pub fn get_macro_lines(&self) -> &Vec<MacroIrLine> {
        &self.macro_lines
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

    fn generate_ir_lines(&self, line_nr: usize) -> Vec<ContentIrLine>;
}

pub trait WriteToIr {}
