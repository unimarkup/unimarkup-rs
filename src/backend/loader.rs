use std::str::FromStr;

use rusqlite::Connection;

use crate::{
    backend::{BackErr, Render},
    middleend::{prepare_content_rows, ContentIrLine, RetrieveFromIr},
    um_elements::{
        heading_block::{HeadingBlock, HeadingLevel},
        types::UnimarkupType,
    },
    um_error::UmError,
};

use super::RenderBlock;

pub trait ParseFromIr {
    fn parse_from_ir(content_lines: &[ContentIrLine]) -> Self;
}

pub fn get_blocks_from_ir(connection: &mut Connection) -> Result<Vec<RenderBlock>, UmError> {
    let mut blocks: Vec<Box<dyn Render>> = vec![];
    let content_lines = get_content_lines(connection)?;

    for line in content_lines {
        if line.um_type.contains("start") {
            // is start of a block. parse the block
            let um_type = parse_um_type(&line.um_type)?;

            let block = match um_type {
                UnimarkupType::Heading => HeadingBlock {
                    id: "first-heading".into(),
                    level: HeadingLevel::Level1,
                    content: "This is a heading".into(),
                    attributes: String::default(),
                },
                UnimarkupType::Paragraph => todo!(),
                UnimarkupType::List => todo!(),
                UnimarkupType::Verbatim => todo!(),
            };

            blocks.push(Box::new(block));
        } else {
            return Err(BackErr::new(format!(
                "expected content ir line with a start of a block. \nInstead got a: {}",
                line.um_type
            ))
            .into());
        }
    }

    Ok(blocks)
}

fn get_content_lines(connection: &mut Connection) -> Result<Vec<ContentIrLine>, UmError> {
    let mut rows_statement = prepare_content_rows(connection, true)
        .map_err(|err| BackErr::new(format!("Failed to prepare rows. \nReason: {}", err)))?;

    let mut rows = rows_statement
        .query([])
        .map_err(|err| BackErr::new(format!("Failed to query rows in backend. \nReason{}", err)))?;

    let mut lines: Vec<ContentIrLine> = Vec::new();

    while let Ok(Some(row)) = rows.next() {
        let content_ir = ContentIrLine::from_ir(row).map_err(|err| {
            BackErr::new(format!(
                "Failed to fetch content ir lines. \nReason: {}",
                err
            ))
        })?;
        lines.push(content_ir);
    }

    Ok(lines)
}

/// # Parses the [UnimarkupType] from String
/// Takes ownership of type as string, since the content ir line will be consumed in order
/// to return corresponding [UnimarkupType]
///
/// ## Accepted formats
/// This function accepts all formats produced by the unimarkup parser:
/// - `"paragraph"`
/// - `"paragraph_start"`
/// - `"heading_level_1"`
/// - `"heading_level_1_start"` etc.
pub fn parse_um_type(type_as_str: &str) -> Result<UnimarkupType, UmError> {
    let type_string = type_as_str
        .split('_')
        .map(|part| if part != "start" { part } else { "" })
        .enumerate()
        .fold(String::new(), |mut acc, (i, new)| {
            if !new.is_empty() {
                if i > 0 {
                    acc.push('_');
                }

                acc.push_str(new);
            }

            acc
        });

    UnimarkupType::from_str(&type_string).map_err(|err| {
        BackErr::new(format!(
            "Failed to resolve unimarkup type. \nMore info: {}",
            err
        ))
        .into()
    })
}

#[cfg(test)]
mod loader_tests {
    use super::*;
    use crate::um_error::UmError;

    #[test]
    fn parse_type() -> Result<(), UmError> {
        // paragraph test
        let um_type = super::parse_um_type("paragraph_start")?;

        assert!(um_type == UnimarkupType::Paragraph);

        Ok(())
    }
}
