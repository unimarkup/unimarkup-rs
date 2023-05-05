//! Contains the Unimarkup Document structure used to store all information of a Unimarkup document in one structure.

use unimarkup_render::{html::Html, render::Render};

use crate::{elements::Blocks, metadata::Metadata};
use unimarkup_commons::config::{output::OutputFormat, Config};

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
    /// Returns the HTML representation of this Unimarkup document
    pub fn html(&self) -> Html {
        let mut output = Html::default();

        for block in &self.blocks {
            match block.render_html() {
                Ok(html) => {
                    output.body.push_str(&html.body);
                    output.head.push_str(&html.head);
                }
                Err(id) => {
                    id.add_info("This error caused HTML rendering to fail!");
                }
            }
        }

        output
    }

    /// Returns the configured output formats for this Unimarkup document
    pub fn output_formats(&self) -> Vec<&OutputFormat> {
        self.config.preamble.output.formats.iter().collect()
    }
}
