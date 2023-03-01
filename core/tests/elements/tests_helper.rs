use std::path::Path;

use clap::StructOpt;
use unimarkup_core::{config::Config, document::Document, elements::Blocks};

pub fn get_config(path: &str) -> Config {
    Config::parse_from(vec!["unimarkup", "--output-formats=html", path])
}

pub fn get_file_content(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap()
}

pub fn assert_blocks_match(document: Document, blocks: Blocks) {
    let parsed_blocks = document.blocks;

    for (parsed, expected) in parsed_blocks.into_iter().zip(blocks.into_iter()) {
        assert_eq!(parsed, expected);
    }
}
