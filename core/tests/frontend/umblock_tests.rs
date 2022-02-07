use std::path::PathBuf;

use clap::StructOpt;
use unimarkup_core::{config::Config, elements::types::UnimarkupFile, middleend::ContentIrLine};

pub fn get_config(path: &str) -> Config {
    Config::parse_from(vec!["unimarkup", "--output-formats=html", path])
}

pub fn get_file_content(path: &PathBuf) -> String {
    std::fs::read_to_string(path).unwrap()
}

/// [`loop_through_ir_lines`] is a public function to loop through the generated ir_lines and assert them with the expected output
pub fn loop_through_ir_lines(um_file: &UnimarkupFile, mut blocks_vector: Vec<ContentIrLine>) {
    for block in &um_file.blocks {
        for ir_line in block.as_ir_lines() {
            ir_lines_assert_eq(
                ir_line,
                blocks_vector
                    .pop()
                    .expect("UnimarkupBlock from Vector that matches output"),
            );
        }
    }
}

fn ir_lines_assert_eq(line: ContentIrLine, heading_ir_line: ContentIrLine) {
    assert_eq!(line.id, heading_ir_line.id);
    assert_eq!(line.line_nr, heading_ir_line.line_nr);
    assert_eq!(line.um_type, heading_ir_line.um_type);
    assert_eq!(line.fallback_text, heading_ir_line.fallback_text);
    assert_eq!(line.attributes, heading_ir_line.attributes);
    assert_eq!(
        line.fallback_attributes,
        heading_ir_line.fallback_attributes
    );
    ir_lines_text_eq(line.text, heading_ir_line.text);
}

fn ir_lines_text_eq(line_text: String, heading_ir_line_text: String) {
    let mut ir_lines = heading_ir_line_text.lines();

    for line in line_text.lines() {
        assert_eq!(line, ir_lines.next().expect("predefined text"));
    }
}
