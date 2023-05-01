use std::path::Path;

use unimarkup_commons::config::{Config, OutputFormat};
// use unimarkup_core::{config::Config, document::Document, elements::Blocks};

pub fn get_config(path: &str) -> Config {
    let mut cfg = Config::default();

    cfg.preamble.output.formats.insert(OutputFormat::Html);
    cfg.input = path.into();

    cfg
}

pub fn get_file_content(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap()
}

#[macro_export]
macro_rules! assert_blocks_match {
    ($document:ident, $blocks:expr) => {
        let parsed_blocks = $document.blocks;

        for (parsed, expected) in parsed_blocks.into_iter().zip($blocks.into_iter()) {
            assert_eq!(parsed, expected);
        }
    };
}
