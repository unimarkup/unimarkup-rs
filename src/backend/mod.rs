use crate::um_error::UmError;
use rusqlite::Connection;

mod backend_error;
mod loader;
mod renderer;

pub use backend_error::BackErr;
pub use loader::ParseFromIr;
pub use renderer::Render;

type RenderBlock = Box<dyn Render>;

pub fn run(
    /* here comes a config as parameter */ connection: &mut Connection,
) -> Result<(), UmError> {
    let blocks: Vec<RenderBlock> = loader::get_blocks_from_ir(connection)?;

    let _html = render_html(&blocks)?;

    Ok(())
}

fn render_html(blocks: &[RenderBlock]) -> Result<String, UmError> {
    let mut html = String::default();

    for block in blocks {
        html.push_str(&block.render_html()?);
    }

    Ok(html)
}
