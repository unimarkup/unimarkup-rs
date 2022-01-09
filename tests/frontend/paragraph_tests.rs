use unimarkup_rs::{middleend::ContentIrLine, um_elements::types};

pub fn paragraph1_expected_result() -> Vec<ContentIrLine> {
    let mut blocks_vector: Vec<ContentIrLine> = Vec::new();
    blocks_vector.push(ContentIrLine::new(
        "head1",
        1,
        format!("heading{delim}level{delim}1", delim = types::DELIMITER),
        "head1",
        "",
        "{}",
        "",
    ));

    blocks_vector.push(ContentIrLine::new(
        "paragraph-3",
        3,
        "paragraph",
        "paragraph 1",
        "",
        "{}",
        "",
    ));

    blocks_vector.reverse();
    blocks_vector
}

pub fn paragraph2_expected_result() -> Vec<ContentIrLine> {
    let mut blocks_vector: Vec<ContentIrLine> = Vec::new();
    blocks_vector.push(ContentIrLine::new(
        "multi-line-header",
        1,
        format!("heading{delim}level{delim}1", delim = types::DELIMITER),
        "multi\nline header",
        "",
        "{}",
        "",
    ));

    blocks_vector.push(ContentIrLine::new(
        "paragraph-4",
        4,
        "paragraph",
        "paragraph",
        "",
        "{}",
        "",
    ));

    blocks_vector.reverse();
    blocks_vector
}

pub fn paragraph3_expected_result() -> Vec<ContentIrLine> {
    let mut blocks_vector: Vec<ContentIrLine> = Vec::new();
    blocks_vector.push(ContentIrLine::new(
        "head2",
        1,
        format!("heading{delim}level{delim}1", delim = types::DELIMITER),
        "head2",
        "",
        "{}",
        "",
    ));

    blocks_vector.push(ContentIrLine::new(
        "paragraph-3",
        3,
        "paragraph",
        "paragraph1",
        "",
        "{}",
        "",
    ));

    blocks_vector.push(ContentIrLine::new(
        "subhead2",
        5,
        format!("heading{delim}level{delim}2", delim = types::DELIMITER),
        "subhead2",
        "",
        "{}",
        "",
    ));

    blocks_vector.push(ContentIrLine::new(
        "paragraph-7",
        7,
        "paragraph",
        "paragraph2",
        "",
        "{}",
        "",
    ));

    blocks_vector.reverse();
    blocks_vector
}
