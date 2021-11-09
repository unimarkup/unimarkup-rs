use crate::middleend::ir::{write_ir_lines, WriteToIr};
use crate::middleend::ir_content::ContentIrLine;
use crate::middleend::ir_macros::MacroIrLine;
use crate::middleend::ir_metadata::MetadataIrLine;
use crate::middleend::ir_resources::ResourceIrLine;
use crate::middleend::ir_variables::VariableIrLine;
use crate::middleend::middleend_error::UmMiddleendError;
use rusqlite::Transaction;

#[derive(Debug)]
pub struct IrBlock {
    content_lines: Vec<ContentIrLine>,
    variable_lines: Vec<VariableIrLine>,
    macro_lines: Vec<MacroIrLine>,
    metadata_lines: Vec<MetadataIrLine>,
    resource_lines: Vec<ResourceIrLine>,
}

impl IrBlock {
    pub fn new() -> Self {
        IrBlock {
            // Vec::new() does not allocate on heap.
            // First allocation occurs when first element is pushed into the Vec
            content_lines: Vec::new(),
            variable_lines: Vec::new(),
            macro_lines: Vec::new(),
            metadata_lines: Vec::new(),
            resource_lines: Vec::new(),
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

    pub fn push_metadata_line(&mut self, line: MetadataIrLine) {
        self.metadata_lines.push(line);
    }

    pub fn append_metadata_lines(&mut self, lines: &mut Vec<MetadataIrLine>) {
        self.metadata_lines.append(lines);
    }

    pub fn get_metadata_lines(&self) -> &Vec<MetadataIrLine> {
        &self.metadata_lines
    }

    pub fn push_resource_line(&mut self, line: ResourceIrLine) {
        self.resource_lines.push(line);
    }

    pub fn append_resource_lines(&mut self, lines: &mut Vec<ResourceIrLine>) {
        self.resource_lines.append(lines);
    }

    pub fn get_resource_lines(&self) -> &Vec<ResourceIrLine> {
        &self.resource_lines
    }
}

impl Default for IrBlock {
    fn default() -> Self {
        Self::new()
    }
}

impl WriteToIr for IrBlock {
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmMiddleendError> {
        write_ir_lines(self.get_content_lines(), ir_transaction)?;
        write_ir_lines(self.get_macro_lines(), ir_transaction)?;
        write_ir_lines(self.get_variable_lines(), ir_transaction)?;
        write_ir_lines(self.get_metadata_lines(), ir_transaction)?;
        write_ir_lines(self.get_resource_lines(), ir_transaction)?;

        Ok(())
    }
}
