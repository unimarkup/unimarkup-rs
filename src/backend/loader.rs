#![deny(missing_docs)]
use std::{collections::VecDeque, str::FromStr};

use rusqlite::Connection;

use crate::{
    backend::{BackendError, Render},
    middleend::{self, ContentIrLine},
    um_elements::{
        heading_block::HeadingBlock, paragraph_block::ParagraphBlock, types, types::UnimarkupType,
    },
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
    fn parse_from_ir(content_lines: &mut VecDeque<ContentIrLine>) -> Result<Self, UmError>
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
    let mut content_lines: VecDeque<ContentIrLine> =
        middleend::get_content_lines(connection)?.into();

    while let Some(line) = content_lines.get(0) {
        let um_type = parse_um_type(&line.um_type)?;

        let block: Box<dyn Render> = match um_type {
            // UnimarkupType::List => todo!(),
            // UnimarkupType::Verbatim => todo!(),
            UnimarkupType::Heading => Box::new(HeadingBlock::parse_from_ir(&mut content_lines)?),
            UnimarkupType::Paragraph => {
                Box::new(ParagraphBlock::parse_from_ir(&mut content_lines)?)
            }
            _ => {
                let _ = content_lines.pop_front();

                Box::new(HeadingBlock::default())
            }
        };

        blocks.push(block);
    }

    Ok(blocks)
}

/// # Parses the [UnimarkupType] from String
/// Takes ownership of type as string, since the content ir line will be consumed in order
/// to return corresponding [UnimarkupType]
///
/// ## Accepted formats
/// This function accepts all formats produced by the unimarkup parser:
/// - `"paragraph"`
/// - `"paragraph-start"`
/// - `"heading-level-1"`
/// - `"heading-level-1-start"` etc.
pub fn parse_um_type(type_as_str: &str) -> Result<UnimarkupType, UmError> {
    let type_string = type_as_str
        .split(types::DELIMITER)
        .map(|part| if part != "start" { part } else { "" })
        .enumerate()
        .fold(String::new(), |mut acc, (i, new)| {
            if !new.is_empty() {
                if i > 0 {
                    acc.push(types::DELIMITER);
                }

                acc.push_str(new);
            }

            acc
        });

    let level_delim = format!("{}level", types::DELIMITER);

    let type_string = if type_string.contains(&level_delim) {
        if let Some(val) = type_string.split(&level_delim).next() {
            val.into()
        } else {
            return Err(BackendError::new(format!(
                "Invalid type string provided: {}",
                type_string
            ))
            .into());
        }
    } else {
        type_string
    };

    UnimarkupType::from_str(&type_string).map_err(|err| {
        BackendError::new(format!(
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
        let um_type = super::parse_um_type("paragraph-start")?;

        assert!(um_type == UnimarkupType::Paragraph);

        // heading test
        let um_type = super::parse_um_type("heading-level-1")?;

        assert!(um_type == UnimarkupType::Heading);

        Ok(())
    }
}
