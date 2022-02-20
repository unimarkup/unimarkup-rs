use std::path::Path;

use clap::StructOpt;
use unimarkup_core::{config::Config, elements::types::UnimarkupFile, middleend::ContentIrLine};

pub fn get_config(path: &str) -> Config {
    Config::parse_from(vec!["unimarkup", "--output-formats=html", path])
}

pub fn get_file_content(path: &Path) -> String {
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
    assert_eq!(
        line.id, heading_ir_line.id,
        "id of heading does not match expected result"
    );
    assert_eq!(
        line.line_nr, heading_ir_line.line_nr,
        "line_number of heading does not match expected result"
    );
    assert_eq!(
        line.um_type, heading_ir_line.um_type,
        "um_type of heading does not match expected result",
    );
    assert_eq!(
        line.fallback_text, heading_ir_line.fallback_text,
        "fallback text of heading does not match expected result"
    );
    assert_eq!(
        line.attributes, heading_ir_line.attributes,
        "attributes of heading does not match expected result"
    );
    assert_eq!(
        line.fallback_attributes, heading_ir_line.fallback_attributes,
        "fallback_attributes of heading does not match expected result"
    );
    ir_lines_text_eq(line.text, heading_ir_line.text);
}

fn ir_lines_text_eq(line_text: String, heading_ir_line_text: String) {
    let mut ir_lines = heading_ir_line_text.lines();

    for line in line_text.lines() {
        assert_eq!(
            line,
            ir_lines.next().expect("predefined text"),
            "Text of heading does not match expected result"
        );
    }
}
