use std::fs;
use std::path::{Path, PathBuf};

use sha3::{Digest, Sha3_256};

use crate::log_id::{SetLog, LogId};
use crate::middleend::{AsIrLines, MetadataIrLine, WriteToIr, error::MiddleendError};

use super::error::MetaDataError;
use super::log_id::MetaDataErrLogId;

/// Represents a Unimarkup metadata
#[derive(Debug, Default, Clone)]
pub struct Metadata {
    /// Unimarkup file this metadata is from
    pub file: PathBuf,
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
        let err_filehash_calc = format!("Could not calculate hash for file `{}`!", &filepath);
        let err_filename_conversion =
            format!("Given file `{}` is not a valid metadata file!", &filepath);

        let metadata = MetadataIrLine {
            filehash: get_filehash(&self.file).expect(&err_filehash_calc),
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
    fn write_to_ir(&self, ir_transaction: &rusqlite::Transaction) -> Result<(), MiddleendError> {
        let ir_metadata: MetadataIrLine = self.as_ir_lines().pop().unwrap();
        ir_metadata.write_to_ir(ir_transaction)
    }
}

/// Calculates the sha3-256 hash of a given file
fn get_filehash(file: &Path) -> Result<Vec<u8>, MetaDataError> {
    let mut hasher = Sha3_256::new();
    let source = fs::read_to_string(file).map_err(|err| 
        MetaDataError::General(
            (MetaDataErrLogId::FailedReadingFile as LogId).set_log(&format!("Could not read file: '{:?}'", file), 
            file!(), line!()).add_info(&format!("Cause: {}", err))
        ))?;

    hasher.update(source);

    let hash = hasher.finalize();
    Ok(hash.to_vec())
}
