use crate::middleend::ir::{self, WriteToIr};
use crate::middleend::ContentIrLine;
use crate::middleend::MacroIrLine;
use crate::middleend::MetadataIrLine;
use crate::middleend::ResourceIrLine;
use crate::middleend::VariableIrLine;
use crate::um_error::UmError;
use rusqlite::Transaction;

/// IR compatible representation of various Unimarkup Elements such as Blocks, Variables, Macros etc.
#[derive(Debug)]
pub struct IrBlock {
    /// IR compatible representation of UnimarkupBlock (i.e. HeadingBlock)
    content_lines: Vec<ContentIrLine>,
    /// IR compatible representation of Unimarkup Variable
    variable_lines: Vec<VariableIrLine>,
    /// IR compatible representation of Unimarkup Macro definition
    macro_lines: Vec<MacroIrLine>,
    /// IR compatible representation of various metadata (i.e. about input file)
    metadata_lines: Vec<MetadataIrLine>,
    /// IR compatible representation of Unimarkup Resource Elements (i.e. image reference)
    resource_lines: Vec<ResourceIrLine>,
}

impl IrBlock {
    /// Constructs an empty IR Block
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

    /// Add single [`ContentIrLine`] into [`IrBlock`]
    pub fn push_content_line(&mut self, line: ContentIrLine) {
        self.content_lines.push(line);
    }

    /// Add a [`Vec`] of [`ContentIrLine`]s into [`IrBlock`]
    pub fn append_content_lines(&mut self, lines: &mut Vec<ContentIrLine>) {
        self.content_lines.append(lines);
    }

    /// Get immutable reference to [`ContentIrLine`]s [`Vec`]
    pub fn get_content_lines(&self) -> &Vec<ContentIrLine> {
        &self.content_lines
    }

    /// Add single [`VariableIrLine`] into [`IrBlock`]
    pub fn push_variable_line(&mut self, line: VariableIrLine) {
        self.variable_lines.push(line);
    }

    /// Add a [`Vec`] of [`VariableIrLine`]s into [`IrBlock`]
    pub fn append_variable_lines(&mut self, lines: &mut Vec<VariableIrLine>) {
        self.variable_lines.append(lines);
    }

    /// Get immutable reference to [`VariableIrLine`]s [`Vec`]
    pub fn get_variable_lines(&self) -> &Vec<VariableIrLine> {
        &self.variable_lines
    }

    /// Add single [`MacroIrLine`] into [`IrBlock`]
    pub fn push_macro_line(&mut self, line: MacroIrLine) {
        self.macro_lines.push(line);
    }

    /// Add a [`Vec`] of [`MacroIrLine`]s into [`IrBlock`]
    pub fn append_macro_lines(&mut self, lines: &mut Vec<MacroIrLine>) {
        self.macro_lines.append(lines);
    }

    /// Get immutable reference to [`MacroIrLine`]s [`Vec`]
    pub fn get_macro_lines(&self) -> &Vec<MacroIrLine> {
        &self.macro_lines
    }

    /// Add single [`MetadataIrLine`] into [`IrBlock`]
    pub fn push_metadata_line(&mut self, line: MetadataIrLine) {
        self.metadata_lines.push(line);
    }

    /// Add a [`Vec`] of [`MetadataIrLine`]s into [`IrBlock`]
    pub fn append_metadata_lines(&mut self, lines: &mut Vec<MetadataIrLine>) {
        self.metadata_lines.append(lines);
    }

    /// Get immutable reference to [`MetadataIrLine`]s [`Vec`]
    pub fn get_metadata_lines(&self) -> &Vec<MetadataIrLine> {
        &self.metadata_lines
    }

    /// Add single [`ResourceIrLine`] into [`IrBlock`]
    pub fn push_resource_line(&mut self, line: ResourceIrLine) {
        self.resource_lines.push(line);
    }

    /// Add a [`Vec`] of [`ResourceIrLine`]s into [`IrBlock`]
    pub fn append_resource_lines(&mut self, lines: &mut Vec<ResourceIrLine>) {
        self.resource_lines.append(lines);
    }

    /// Get immutable reference to [`ResourceIrLine`]s [`Vec`]
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
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmError> {
        ir::write_ir_lines(self.get_content_lines(), ir_transaction)?;
        ir::write_ir_lines(self.get_macro_lines(), ir_transaction)?;
        ir::write_ir_lines(self.get_variable_lines(), ir_transaction)?;
        ir::write_ir_lines(self.get_metadata_lines(), ir_transaction)?;
        ir::write_ir_lines(self.get_resource_lines(), ir_transaction)?;

        Ok(())
    }
}
