use std::{collections::VecDeque, str::FromStr};

use logid::{
    capturing::{LogIdTracing, MappedLogId},
    log_id::LogId,
};
use rusqlite::Connection;

use crate::{
    elements::{
        types, types::ElementType, HeadingBlock, ParagraphBlock, UnimarkupBlock, UnimarkupBlocks,
        VerbatimBlock,
    },
    log_id::CORE_LOG_ID_MAP,
    middleend::{self, ContentIrLine},
};

use super::log_id::LoaderErrLogId;

/// Trait that must be implemented for a [`ElementType`] to be stored in IR
pub trait ParseFromIr {
    /// Parses a Unimarkup Block Element from Intermediate Representation (SQL Database)
    ///
    /// # Arguments
    ///
    /// * `content_lines` - reference to a slice containing all [`ContentIrLine`] lines
    /// * `line_index` - index of the [`ContentIrLine`] which is currently read
    ///
    /// Returns the Unimarkup block element on success.
    fn parse_from_ir(content_lines: &mut VecDeque<ContentIrLine>) -> Result<Self, MappedLogId>
    where
        Self: Sized;
}

/// Parses [`ContentIrLine`]s and returns all Unimarkup Block Elements that where stored in the IR.
/// The actual blocks are stored in `Vec` as trait objects of trait [`Render`](unimarkup_render::render) since different [`ElementType`]s
/// are stored in the IR.
///
/// # Arguments
///
/// * `connection` - [`rusqlite::Connection`] used for interaction with IR
pub fn get_blocks_from_ir(connection: &mut Connection) -> Result<UnimarkupBlocks, MappedLogId> {
    let mut blocks: UnimarkupBlocks = vec![];
    let mut content_lines: VecDeque<ContentIrLine> =
        middleend::get_content_lines(connection)?.into();

    while let Some(line) = content_lines.get(0) {
        let um_type = parse_um_type(&line.um_type)?;

        let block: Box<dyn UnimarkupBlock> = match um_type {
            // UnimarkupType::List => todo!(),
            ElementType::Heading => Box::new(HeadingBlock::parse_from_ir(&mut content_lines)?),
            ElementType::Paragraph => Box::new(ParagraphBlock::parse_from_ir(&mut content_lines)?),
            ElementType::VerbatimBlock => {
                Box::new(VerbatimBlock::parse_from_ir(&mut content_lines)?)
            }
            _ => {
                // unsupported types in middleend
                // TODO: log

                let _ = content_lines.pop_front();
                Box::new(ParagraphBlock::default())
            }
        };

        blocks.push(block);
    }

    Ok(blocks)
}

/// Returns the corresponding [`ElementType`] from a given String
///
/// # Accepted formats
///
/// This function accepts all formats produced by the Unimarkup parser:
///
/// - `"paragraph"`
/// - `"paragraph-start"`
/// - `"heading-level-1"`
/// - `"heading-level-1-start"` etc.
pub fn parse_um_type(type_as_str: &str) -> Result<ElementType, MappedLogId> {
    let type_string = type_as_str
        .split(types::ELEMENT_TYPE_DELIMITER)
        .map(|part| if part != "start" { part } else { "" })
        .enumerate()
        .fold(String::new(), |mut acc, (i, new)| {
            if !new.is_empty() {
                if i > 0 {
                    acc.push(types::ELEMENT_TYPE_DELIMITER);
                }

                acc.push_str(new);
            }

            acc
        });

    let level_delim = format!("{}level", types::ELEMENT_TYPE_DELIMITER);

    let type_string = if type_string.contains(&level_delim) {
        if let Some(val) = type_string.split(&level_delim).next() {
            val.into()
        } else {
            return Err(
                (LoaderErrLogId::InvalidElementType as LogId).set_event_with(
                    &CORE_LOG_ID_MAP,
                    &format!("Invalid type string provided: '{}'", type_string),
                    file!(),
                    line!(),
                ),
            );
        }
    } else {
        type_string
    };

    ElementType::from_str(&type_string).map_err(|err| {
        (LoaderErrLogId::InvalidElementType as LogId)
            .set_event_with(
                &CORE_LOG_ID_MAP,
                &format!("Failed to resolve Unimarkup type '{}'.", &type_string),
                file!(),
                line!(),
            )
            .add_cause(&format!("{}", err))
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

        assert!(um_type == ElementType::Paragraph);

        // heading test
        let um_type = super::parse_um_type("heading-level-1").unwrap();

        assert!(um_type == ElementType::Heading);
    }
}
