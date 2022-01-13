use unimarkup_rs::{middleend::ContentIrLine, um_elements::types::UnimarkupFile};

/// [`loop_through_ir_lines`] is a public function to loop through the generated ir_lines and assert them with the expected output
pub fn loop_through_ir_lines(um_blocks: &UnimarkupFile, mut blocks_vector: Vec<ContentIrLine>) {
    for block in &um_blocks.blocks {
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
