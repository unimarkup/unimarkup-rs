#![deny(missing_docs)]
use std::str::FromStr;

use rusqlite::Connection;

use crate::{
    backend::{BackErr, Render},
    middleend::{prepare_content_rows, ContentIrLine, RetrieveFromIr},
    um_elements::{heading_block::HeadingBlock, types::UnimarkupType},
    um_error::UmError,
};

use super::RenderBlock;

/// Trait which should be implemented by any [`UnimarkupType`] which can be stored in IR
pub trait ParseFromIr {
    /// Parses a Unimarkup Block Element from Intermediate Representation (SQL Database)
    ///
    /// # Arguments
    /// * `content_lines` - reference to a slice containing all [`ContentIrLine`] lines
    /// * `line_index` - index of the [`ContentIrLine`] which is currently read
    ///
    /// As part of the return value is `usize`, which represents
    /// the index of the next Content Line which should be read.
    fn parse_from_ir(
        content_lines: &mut [ContentIrLine],
        line_index: usize,
    ) -> Result<(Self, usize), UmError>
    where
        Self: Sized;
}

/// Parses the `[ContentIrLine]s`, creates Unimarkup Block Elements and gives them back.
/// The actual blocks are stored in `Vec` as trait objects of trait [`Render`] since different types
/// are needed.
///
/// # Arguments
/// * `connection` - [`rusqlite::Connection`] used for interaction with IR
pub fn get_blocks_from_ir(connection: &mut Connection) -> Result<Vec<RenderBlock>, UmError> {
    let mut blocks: Vec<Box<dyn Render>> = vec![];
    let mut line_index = 0;
    let mut content_lines = get_content_lines(connection)?;

    while let Some(line) = content_lines.get(line_index) {
        let um_type = parse_um_type(&line.um_type)?;

        let (block, new_line_index) = match um_type {
            // UnimarkupType::List => todo!(),
            // UnimarkupType::Verbatim => todo!(),
            _ => HeadingBlock::parse_from_ir(&mut content_lines, line_index)?,
        };

        if new_line_index == line_index {
            line_index += 1;
        } else {
            line_index = new_line_index;
        }

        blocks.push(Box::new(block));
        line_index += 1;
    }

    Ok(blocks)
}

/// Loads the [`ContentIrLine`]s from IR and gives them contained in a vector
///
/// # Arguments
/// * `connection` - [`rusqlite::Connection`] for interacting with IR
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

    let type_string = if type_string.contains("_level") {
        if let Some(val) = type_string.split("_level").next() {
            val.into()
        } else {
            return Err(
                BackErr::new(format!("Invalid type string provided: {}", type_string)).into(),
            );
        }
    } else {
        type_string
    };

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

        // heading test
        let um_type = super::parse_um_type("heading_level_1")?;

        assert!(um_type == UnimarkupType::Heading);

        Ok(())
    }
}
