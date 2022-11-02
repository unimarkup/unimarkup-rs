use std::path::PathBuf;

use logid::capturing::MappedLogId;

use crate::middleend::{AsIrLines, MetadataIrLine, WriteToIr};

/// Represents a Unimarkup metadata
#[derive(Debug, Default, Clone)]
pub struct Metadata {
    /// Unimarkup file this metadata is from
    pub file: PathBuf,
    /// The sha256 hash of the content this metadata points to
    pub contenthash: Vec<u8>,
    /// Preamble of the Unimarkup file
    pub preamble: String,
    /// Kind of the Unimarkup file
    pub kind: MetadataKind,
    /// Namespace of the Unimarkup file
    pub namespace: String,
}

/// The kind of a Unimarkup file
#[derive(Debug, Clone, Copy)]
pub enum MetadataKind {
    /// Identifies the Unimarkup file as the root of this document
    ///
    /// **Note:** Only one metadata entry of an IR may be `Root`
    Root,
    /// The Unimarkup file must be considered as a theme file
    Theme,
    /// The Unimarkup file is inserted inside an element of the root file
    Insert,
}

impl Default for MetadataKind {
    fn default() -> Self {
        Self::Insert
    }
}

impl AsIrLines<MetadataIrLine> for Metadata {
    fn as_ir_lines(&self) -> Vec<MetadataIrLine> {
        let filepath = self.file.to_string_lossy().into_owned();
        let err_filename_conversion =
            format!("Given file `{}` is not a valid metadata file!", &filepath);

        let metadata = MetadataIrLine {
            filehash: self.contenthash.clone(),
            filename: self
                .file
                .file_name()
                .expect(&err_filename_conversion)
                .to_string_lossy()
                .into_owned(),
            path: self.file.to_string_lossy().into_owned(),
            preamble: self.preamble.clone(),
            fallback_preamble: String::new(),
            root: true,
        };

        vec![metadata]
    }
}

impl From<Metadata> for MetadataIrLine {
    fn from(metadata: Metadata) -> Self {
        metadata.as_ir_lines().pop().unwrap()
    }
}

impl WriteToIr for Metadata {
    fn write_to_ir(&self, ir_transaction: &rusqlite::Transaction) -> Result<(), MappedLogId> {
        let ir_metadata: MetadataIrLine = self.as_ir_lines().pop().unwrap();
        ir_metadata.write_to_ir(ir_transaction)
    }
}
