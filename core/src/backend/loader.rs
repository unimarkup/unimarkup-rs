use std::{collections::VecDeque, str::FromStr};

use rusqlite::Connection;

use crate::{
    backend::BackendError,
    elements::{types, types::UnimarkupType, HeadingBlock, ParagraphBlock, VerbatimBlock},
    log_id::{LogId, SetLog},
    middleend::{self, ContentIrLine},
    unimarkup_block::UnimarkupBlockKind,
};

use super::log_id::LoaderErrLogId;

/// Trait that must be implemented for a [`UnimarkupType`] to be stored in IR
pub trait ParseFromIr {
    /// Parses a Unimarkup Block Element from Intermediate Representation (SQL Database)
    ///
    /// # Arguments
    ///
    /// * `content_lines` - reference to a slice containing all [`ContentIrLine`] lines
    /// * `line_index` - index of the [`ContentIrLine`] which is currently read
    ///
    /// Returns the Unimarkup block element on success.
    fn parse_from_ir(content_lines: &mut VecDeque<ContentIrLine>) -> Result<Self, BackendError>
    where
        Self: Sized;
}

/// Parses `[ContentIrLine]s` and returns all Unimarkup Block Elements that where stored in the IR.
/// The actual blocks are stored in `Vec` as trait objects of trait [`Render`] since different [`UnimarkupType`]s
/// are stored in the IR.
///
/// # Arguments
///
/// * `connection` - [`rusqlite::Connection`] used for interaction with IR
pub fn get_blocks_from_ir(
    connection: &mut Connection,
) -> Result<Vec<UnimarkupBlockKind>, BackendError> {
    let mut blocks: Vec<UnimarkupBlockKind> = vec![];
    let mut content_lines: VecDeque<ContentIrLine> =
        middleend::get_content_lines(connection)?.into();

    while let Some(line) = content_lines.get(0) {
        let um_type = parse_um_type(&line.um_type)?;

        let block = match um_type {
            // UnimarkupType::List => todo!(),
            UnimarkupType::Heading => {
                UnimarkupBlockKind::Heading(HeadingBlock::parse_from_ir(&mut content_lines)?)
            }
            UnimarkupType::Paragraph => {
                UnimarkupBlockKind::Paragraph(ParagraphBlock::parse_from_ir(&mut content_lines)?)
            }
            UnimarkupType::VerbatimBlock => {
                UnimarkupBlockKind::Verbatim(VerbatimBlock::parse_from_ir(&mut content_lines)?)
            }
            _ => {
                let _ = content_lines.pop_front();

                UnimarkupBlockKind::Paragraph(ParagraphBlock::default())
            }
        };

        blocks.push(block);
    }

    Ok(blocks)
}

/// Returns the corresponding [`UnimarkupType`] from a given String
///
/// # Accepted formats
///
/// This function accepts all formats produced by the Unimarkup parser:
///
/// - `"paragraph"`
/// - `"paragraph-start"`
/// - `"heading-level-1"`
/// - `"heading-level-1-start"` etc.
pub fn parse_um_type(type_as_str: &str) -> Result<UnimarkupType, BackendError> {
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
            return Err(BackendError::Loader(
                (LoaderErrLogId::InvalidElementType as LogId).set_log(
                    &format!("Invalid type string provided: '{}'", type_string),
                    file!(),
                    line!(),
                ),
            ));
        }
    } else {
        type_string
    };

    UnimarkupType::from_str(&type_string).map_err(|err| {
        BackendError::Loader(
            (LoaderErrLogId::InvalidElementType as LogId)
                .set_log(
                    &format!("Failed to resolve Unimarkup type '{}'.", &type_string),
                    file!(),
                    line!(),
                )
                .add_info(&format!("Cause: {}", err)),
        )
    })
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test__parse__type_paragraph_header() {
        // paragraph test
        let um_type = super::parse_um_type("paragraph-start").unwrap();

        assert!(um_type == UnimarkupType::Paragraph);

        // heading test
        let um_type = super::parse_um_type("heading-level-1").unwrap();

        assert!(um_type == UnimarkupType::Heading);
    }
}
