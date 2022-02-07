//! Entry module for unimarkup-rs.

use crate::backend;
use crate::backend::RenderBlock;
use crate::config::Config;
use crate::config::OutputFormat;
use crate::error::UmError;
use crate::frontend;
use crate::middleend;

/// Struct representing a Unimarkup document that can be rendered to supported output formats.
pub struct UnimarkupDocument {
    pub(crate) elements: Vec<RenderBlock>,
    pub(crate) config: Config,
}

impl UnimarkupDocument {
    /// Returns the HTML representation of this Unimarkup document
    pub fn html(&self) -> Html {
        Html {
            elements: &self.elements,
            _metadata: String::default(),
        }
    }

    /// Returns the configured output formats for this Unimarkup document
    pub fn output_formats(&self) -> Option<&Vec<OutputFormat>> {
        self.config.out_formats.as_ref()
    }
}

/// HTML representation of the Unimarkup document
pub struct Html<'a> {
    elements: &'a Vec<RenderBlock>,
    _metadata: String,
}

impl Html<'_> {
    /// Returns the HTML head content of the Unimarkup document
    pub fn head(&self) -> String {
        // metadata -> html
        String::default()
    }

    /// Returns the HTML body content of the Unimarkup document
    pub fn body(&self) -> String {
        let mut output = String::default();

        for block in self.elements {
            output.push_str(&block.render_html().unwrap());
        }

        output
    }
}

/// Compiles Unimarkup content and returns a [`UnimarkupDocument`] representing the given content.
///
/// # Arguments
///
/// * `um_content` - String containing Unimarkup elements.
/// * `config` - Unimarkup configuration constructed from command-line arguments passed to unimarkup-rs.
///
/// # Errors
///
/// Returns a [`UmError`], if error occurs during compilation.
pub fn compile(um_content: &str, mut config: Config) -> Result<UnimarkupDocument, UmError> {
    let mut connection = middleend::setup_ir_connection()?;
    middleend::setup_ir(&connection)?;

    frontend::run(um_content, &mut connection, &mut config)?;
    backend::run(&mut connection, config)
}
