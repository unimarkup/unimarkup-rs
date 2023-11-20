//! Defines the Metadata structure of a Unimarkup document.

use std::path::PathBuf;

use unimarkup_commons::config::preamble::Preamble;

/// Represents a Unimarkup metadata
#[derive(Debug, Default, Clone)]
pub struct Metadata {
    /// Unimarkup file this metadata is from
    pub file: PathBuf,
    /// The sha256 hash of the content this metadata points to
    pub contenthash: Vec<u8>,
    /// Preamble of the Unimarkup file
    pub preamble: Option<Preamble>,
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
