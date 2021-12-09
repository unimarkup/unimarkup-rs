use crate::{
    config::{Config, OutputFormat},
    um_error::UmError,
};
use rusqlite::Connection;

mod backend_error;
mod loader;
mod renderer;

pub use backend_error::BackendError;
pub use loader::ParseFromIr;
pub use renderer::Render;

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
            let html = render_html(&blocks)?;

            let mut out_path_html = out_path;
            out_path_html.set_extension("html");

            std::fs::write(&out_path_html, &html).map_err(|err| {
                BackendError::new(format!(
                    "Could not write to file '{}'.\nReason: {}",
                    out_path_html
                        .to_str()
                        .expect("Output path is valid UTF-8 String"),
                    err
                ))
            })?;
        }
    }

    Ok(())
}

fn render_html(blocks: &[RenderBlock]) -> Result<String, UmError> {
    let mut html = String::default();

    for block in blocks {
        html.push_str(&block.render_html()?);
    }

    Ok(html)
}
