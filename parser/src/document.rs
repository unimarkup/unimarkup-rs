//! Contains the Unimarkup Document structure used to store all information of a Unimarkup document in one structure.

// use unimarkup_render::{html::Html, render::Render};

use crate::{elements::Blocks, metadata::Metadata};
use unimarkup_commons::config::{output::OutputFormatKind, Config};

/// Struct representing a Unimarkup document
#[derive(Default, Debug)]
pub struct Document {
    /// Blocks of this Unimarkup document
    pub blocks: Blocks,
    /// Configuration used to create this Unimarkup document
    pub config: Config,

    // Below fields not yet used!
    /// Field containing all macros defined in this Unimarkup document
    pub macros: Vec<String>,
    /// Field containing all variables defined in this Unimarkup document
    pub variables: Vec<String>,
    /// Field containing metadata for this Unimarkup document
    pub metadata: Vec<Metadata>,
    /// Field containing all external resources used in this Unimarkup document
    pub resources: Vec<String>,
}

impl Document {
    /// Returns the configured output formats for this Unimarkup document
    pub fn output_formats(&self) -> impl Iterator<Item = &OutputFormatKind> {
        self.config.preamble.output.formats.iter()
    }
}
