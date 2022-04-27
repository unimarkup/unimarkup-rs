//! Entry module for unimarkup-rs.

use crate::backend;
use crate::backend::Render;
use crate::config::Config;
use crate::config::OutputFormat;
use crate::error::CoreError;
use crate::frontend;
use crate::log_id::LogId;
use crate::log_id::SetLog;
use crate::middleend;
use crate::unimarkup_block::UnimarkupBlockKind;

/// Struct representing a Unimarkup document that can be rendered to supported output formats.
#[derive(Debug, Clone)]
pub struct UnimarkupDocument {
    /// Elements of a Unimarkup document
    pub elements: Vec<UnimarkupBlockKind>,
    /// Configuration used to create this Unimarkup document
    pub config: Config,
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
    elements: &'a Vec<UnimarkupBlockKind>,
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
            let try_render = block.render_html();

            // FIX: This must change after we move inline formatting
            match try_render {
                Ok(html) => output.push_str(&html),
                Err(err) => {
                    let id: LogId = err.into();
                    id.add_info("Failed rendering HTML due to this error!");
                }
            }
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
/// Returns a [`CoreError`], if error occurs during compilation.
pub fn compile(um_content: &str, mut config: Config) -> Result<UnimarkupDocument, CoreError> {
    let mut connection = middleend::setup_ir_connection()?;
    middleend::setup_ir(&connection)?;

    frontend::run(um_content, &mut connection, &mut config)?;
    backend::run(&mut connection, config).map_err(|err| err.into())
}
