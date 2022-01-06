use crate::{
    config::{Config, OutputFormat},
    um_error::UmError,
};
use log::info;
use rusqlite::Connection;

mod backend_error;
mod loader;
mod renderer;
pub(crate) mod inline_formatting;

pub use backend_error::BackendError;
pub use loader::ParseFromIr;
pub use renderer::*;

type RenderBlock = Box<dyn Render>;

pub fn run(connection: &mut Connection, config: &Config) -> Result<(), UmError> {
    let blocks: Vec<RenderBlock> = loader::get_blocks_from_ir(connection)?;

    let out_path = {
        if let Some(ref out_file) = config.out_file {
            out_file.clone()
        } else {
            let mut in_file = config.um_file.clone();
            in_file.set_extension("");

            in_file
        }
    };

    if let Some(ref output_formats) = config.out_formats {
        if output_formats.contains(&OutputFormat::Html) {
            let html = renderer::render_html(&blocks)?;

            let mut out_path_html = out_path;
            out_path_html.set_extension("html");

            let out_path = out_path_html.to_str().expect("Validation done in config");

            info!("Writing to {}", out_path);

            std::fs::write(&out_path_html, &html).map_err(|err| {
                BackendError::new(format!(
                    "Could not write to file '{}'.\nReason: {}",
                    out_path, err
                ))
            })?;
        }
    }

    Ok(())
}
