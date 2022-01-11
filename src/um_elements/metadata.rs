use std::fs;
use std::path::{Path, PathBuf};

use sha3::{Digest, Sha3_256};

use crate::middleend::{MetadataIrLine, WriteToIr};
use crate::um_error::UmError;

/// Represents a Unimarkup metadata
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

impl From<&Metadata> for MetadataIrLine {
    fn from(metadata: &Metadata) -> Self {
        let filepath = metadata.file.to_string_lossy().into_owned();
        let err_filehash_calc = format!("Could not calculate hash for file `{}`!", &filepath);
        let err_filename_conversion =
            format!("Given file `{}` is not a valid metadata file!", &filepath);

        MetadataIrLine {
            filehash: get_filehash(&metadata.file).expect(&err_filehash_calc),
            filename: metadata
                .file
                .file_name()
                .expect(&err_filename_conversion)
                .to_string_lossy()
                .into_owned(),
            path: metadata.file.to_string_lossy().into_owned(),
            preamble: metadata.preamble.clone(),
            fallback_preamble: String::new(),
            root: true,
        }
    }
}

impl From<Metadata> for MetadataIrLine {
    fn from(metadata: Metadata) -> Self {
        MetadataIrLine::from(&metadata)
    }
}

impl WriteToIr for Metadata {
    fn write_to_ir(&self, ir_transaction: &rusqlite::Transaction) -> Result<(), UmError> {
        let ir_metadata: MetadataIrLine = self.into();
        ir_metadata.write_to_ir(ir_transaction)
    }
}

/// Calculates the sha3-256 hash of a given file
fn get_filehash(file: &Path) -> Result<Vec<u8>, UmError> {
    let mut hasher = Sha3_256::new();
    let source = fs::read_to_string(file).map_err(|err| UmError::General {
        msg: String::from("Could not read file."),
        error: Box::new(err),
    })?;

    hasher.update(source);

    let hash = hasher.finalize();
    Ok(hash.to_vec())
}
