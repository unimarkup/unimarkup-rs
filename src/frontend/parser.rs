use super::blocks::heading_block::{HeadingBlock, HeadingLevel};

pub fn test_function() {
    println!("Basic setup");
}

pub fn parse_heading(input: &[&str], index: &mut usize) -> HeadingBlock {
    let mut content: String = "".into();

    let mut level = HeadingLevel::Invalid;
    let mut level_counter = 0;
    let mut level_set = false;

    while let Some(word) = input.get(*index) {
        if word.matches('#').count() > 0 && !level_set {
            level_counter += 1;
        } else {
            level_set = true;
            level = HeadingLevel::from(level_counter);
        }

        if level_set {
            // read content
            content.push_str(word);
        }

        *index += 1;
    }

    HeadingBlock {
        level,
        content: content.trim().to_string(),
    }
}
