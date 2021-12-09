use clap::StructOpt;
use unimarkup_rs::{
    backend::{self, BackendError, Render},
    config::Config,
    middleend::{self, ParseForIr},
    um_elements::heading_block::{HeadingBlock, HeadingLevel},
    um_error::UmError,
};

use super::super::middleend::ir_test_setup;

#[test]
fn run() -> Result<(), UmError> {
    let mut connection = ir_test_setup::setup_test_ir();

    let block = HeadingBlock {
        id: "some-id".into(),
        level: HeadingLevel::Level1,
        content: "This is a heading".into(),
        attributes: "{}".into(),
    };

    let lines = block.generate_ir_lines(0);

    {
        let transaction = ir_test_setup::get_test_transaction(&mut connection);
        middleend::write_ir_lines(&lines, &transaction)?;

        transaction.commit().unwrap();
    }

    let cfg: Config = Config::parse_from(vec!["unimarkup", "--output-formats=html", "in_file.um"]);

    #[allow(clippy::redundant_clone)]
    let mut out_path = cfg.um_file.clone();
    out_path.set_extension("html");

    backend::run(&mut connection, &cfg)?;

    let output = std::fs::read_to_string(&out_path);

    match output {
        Ok(content) => {
            assert_eq!(block.render_html().expect("Block is checked"), content);
        }
        _ => {
            return Err(BackendError::new(format!(
                "Could not write file to {}",
                out_path.to_str().unwrap()
            ))
            .into())
        }
    }

    if out_path.exists() {
        let _ = std::fs::remove_file(out_path);
    }

    Ok(())
}
