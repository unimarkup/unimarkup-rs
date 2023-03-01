//! Contains the Unimarkup Document structure used to store all information of a Unimarkup document in one structure.

use unimarkup_render::{html::Html, render::Render};

use crate::{
    config::{Config, OutputFormat},
    elements::Blocks,
    metadata::Metadata,
};

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
            let try_render = block.render_html();

            match try_render {
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
    pub fn output_formats(&self) -> Option<&Vec<OutputFormat>> {
        self.config.out_formats.as_ref()
    }
}
