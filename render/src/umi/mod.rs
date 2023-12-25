use std::collections::HashMap;
use std::path::PathBuf;

use crate::render::OutputFormat;

use crate::log_id::ParserError;
use spreadsheet_ods::{
    read_ods_buf, write_ods_buf_uncompressed, Sheet, Value, ValueType, WorkBook,
};
use unimarkup_commons::config::output::Output;
use unimarkup_commons::config::{Config, MergingConfig};
use unimarkup_commons::lexer::{
    position::Position,
    symbol::SymbolKind,
    token::{iterator::TokenIterator, lex_str, TokenKind},
};
use unimarkup_inline::{
    element::Inline,
    parser::{parse_inlines, InlineContext},
};
use unimarkup_parser::{
    document::Document,
    elements::{
        atomic::{Heading, Paragraph},
        blocks::Block,
        enclosed::VerbatimBlock,
        indents::{BulletList, BulletListEntry},
    },
};

pub mod render;

fn unpack_content_safe(value: Value) -> String {
    if value.value_type() == ValueType::Text {
        value.as_str_or("").into()
    } else {
        value.as_cow_str_or("").into()
    }
}

#[derive(Debug, Default, Clone)]
pub struct UmiRow {
    position: u8,
    id: String,
    kind: String,
    properties: String,
    depth: u8,
    content: String,
    attributes: String,
}

impl UmiRow {
    fn new(
        position: u8,
        id: String,
        kind: String,
        properties: String,
        depth: u8,
        content: String,
        attributes: String,
    ) -> Self {
        UmiRow {
            position,
            id,
            kind,
            properties,
            depth,
            content,
            attributes,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Umi {
    pub elements: Vec<UmiRow>,
    pub lang: String,
    pub config: Config,
    pub ods: Vec<u8>,
}

impl Umi {
    fn with_um(elements: Vec<UmiRow>, config: Config, lang: String) -> Self {
        Umi {
            elements,
            lang,
            config,
            ods: vec![],
        }
    }

    pub fn create_workbook(&mut self) -> &mut Self {
        let mut wb = WorkBook::new_empty();

        let mut sheet = Sheet::new("umi");
        // Set the Header Row
        sheet.set_value(0, 0, "position");
        sheet.set_value(0, 1, "id");
        sheet.set_value(0, 2, "kind");
        sheet.set_value(0, 3, "properties");
        sheet.set_value(0, 4, "depth");
        sheet.set_value(
            0,
            5,
            String::from("content-") + self.lang.to_string().as_str(),
        );
        sheet.set_value(
            0,
            6,
            String::from("attributes-") + self.lang.to_string().as_str(),
        );

        // Row 1: Config
        sheet.set_value(1, 0, 0);
        sheet.set_value(1, 2, "Preamble");
        sheet.set_value(
            1,
            5,
            serde_yaml::to_string(&self.config.preamble).unwrap_or_default(),
        );

        for element in &self.elements {
            let row = element.position + 2;
            sheet.set_value(row.into(), 0, element.position + 1);
            sheet.set_value(row.into(), 1, element.id.clone());
            sheet.set_value(row.into(), 2, element.kind.clone());
            sheet.set_value(row.into(), 3, element.properties.clone());
            sheet.set_value(row.into(), 4, element.depth);
            sheet.set_value(row.into(), 5, element.content.clone());
            sheet.set_value(row.into(), 6, element.attributes.clone());
        }

        wb.push_sheet(sheet);

        let res = write_ods_buf_uncompressed(&mut wb, vec![]);

        self.ods = res.unwrap_or_default();

        self
    }

    fn read_inlines(&mut self, content: String) -> Vec<Inline> {
        let token_vec = lex_str(&content);
        let iterator = TokenIterator::from(token_vec.as_slice());
        let inlines = parse_inlines(iterator, InlineContext::default(), None, None);
        inlines.2.to_inlines()
    }

    fn fetch_next_line(&mut self, new_line_index: usize) -> Option<UmiRow> {
        if new_line_index < self.elements.len() {
            Some(self.elements[new_line_index].clone())
        } else {
            None
        }
    }

    fn read_row(&mut self, line: usize) -> Result<Block, ParserError> {
        let mut current_line = self.elements[line].clone();
        let properties: HashMap<String, String> =
            serde_json::from_str(&current_line.properties).unwrap_or_default();
        match current_line.kind.as_str() {
            "Heading" => {
                let heading = Heading {
                    id: current_line.id.clone(),
                    level: unimarkup_parser::elements::atomic::HeadingLevel::try_from(
                        properties
                            .get("level")
                            .ok_or(ParserError::MissingProperty((
                                "level".into(),
                                current_line.position,
                            )))?
                            .as_str(),
                    )
                    .ok()
                    .ok_or(ParserError::InvalidPropertyValue((
                        "level".into(),
                        current_line.position,
                    )))?,
                    content: self.read_inlines(current_line.content.clone()),
                    attributes: Some(current_line.attributes).filter(|s| !s.is_empty()),
                    start: Position::new(1, 1),
                    end: Position::new(1, 1),
                };
                Ok(Block::Heading(heading))
            }
            "Paragraph" => {
                let paragraph = Paragraph {
                    content: self.read_inlines(current_line.content.clone()),
                };
                Ok(Block::Paragraph(paragraph))
            }
            "VerbatimBlock" => {
                let verbatim = VerbatimBlock {
                    content: current_line.content.clone(),
                    data_lang: properties.get("data_lang").cloned(),
                    attributes: Some(current_line.attributes).filter(|s| !s.is_empty()),
                    implicit_closed: properties
                        .get("implicit_closed")
                        .ok_or(ParserError::MissingProperty((
                            "implicit_closed".into(),
                            current_line.position,
                        )))?
                        .parse()
                        .unwrap_or_default(),
                    tick_len: properties
                        .get("tick_len")
                        .ok_or(ParserError::MissingProperty((
                            "tick_len".into(),
                            current_line.position,
                        )))?
                        .parse()
                        .unwrap_or_default(),
                    start: Position::new(1, 1),
                    end: Position::new(1, 1),
                };
                Ok(Block::VerbatimBlock(verbatim))
            }
            "BulletList" => {
                let mut bullet_list = BulletList {
                    entries: vec![],
                    start: Position::new(1, 1),
                    end: Position::new(1, 1),
                };

                let bullet_list_depth = current_line.depth;
                let mut current_line_index = line + 1;
                current_line = self.fetch_next_line(current_line_index).unwrap_or_default();

                while current_line.depth > bullet_list_depth {
                    if current_line.depth == bullet_list_depth + 1 {
                        // Append Element to Bullet List
                        let block = self.read_row(current_line_index);
                        let bullet_list_entry = match block {
                            Ok(Block::BulletListEntry(block)) => block,
                            _ => break,
                        };
                        bullet_list.entries.append(&mut vec![bullet_list_entry]);
                    }

                    current_line_index += 1;
                    let fetched = self.fetch_next_line(current_line_index);
                    if fetched.is_none() {
                        break;
                    }
                    current_line = fetched.unwrap_or_default();
                }

                Ok(Block::BulletList(bullet_list))
            }
            "BulletListEntry" => {
                let mut bullet_list_entry = BulletListEntry {
                    keyword: TokenKind::from(SymbolKind::from(
                        properties
                            .get("keyword")
                            .ok_or(ParserError::MissingProperty((
                                "keyword".into(),
                                current_line.position,
                            )))?
                            .as_str(),
                    ))
                    .try_into()
                    .ok()
                    .ok_or(ParserError::InvalidPropertyValue((
                        "keyword".into(),
                        current_line.position,
                    )))?,
                    heading: self.read_inlines(
                        properties
                            .get("heading")
                            .ok_or(ParserError::MissingProperty((
                                "heading".into(),
                                current_line.position,
                            )))?
                            .to_string(),
                    ),
                    body: vec![],
                    start: Position::new(1, 1),
                    end: Position::new(1, 1),
                };

                let bullet_list_entry_depth = current_line.depth;
                let mut current_line_index = line + 1;
                current_line = self.fetch_next_line(current_line_index).unwrap_or_default();

                while current_line.depth > bullet_list_entry_depth {
                    if current_line.depth == bullet_list_entry_depth + 1 {
                        // Append Element to Bullet List Entry Body
                        let block = self.read_row(current_line_index)?;
                        bullet_list_entry.body.append(&mut vec![block]);
                    }

                    current_line_index += 1;

                    let fetched = self.fetch_next_line(current_line_index);
                    if fetched.is_none() {
                        break;
                    }
                    current_line = fetched.unwrap_or_default();
                }

                Ok(Block::BulletListEntry(bullet_list_entry))
            }
            &_ => Err(ParserError::UnknownKind(current_line.position)),
        }
    }

    pub fn create_um(&mut self) -> Result<Document, ParserError> {
        self.elements.clear();
        debug_assert!(!self.ods.is_empty());

        let wb: WorkBook = read_ods_buf(&self.ods).unwrap_or_default();
        let sheet = wb.sheet(0);
        let rows = sheet.used_grid_size().0;

        for row_index in 2..rows {
            self.elements.push(UmiRow::new(
                sheet
                    .cell(row_index, 0)
                    .unwrap_or_default()
                    .value
                    .as_u8_opt()
                    .unwrap_or(0),
                sheet
                    .cell(row_index, 1)
                    .unwrap_or_default()
                    .value
                    .as_str_opt()
                    .unwrap_or_default()
                    .to_string(),
                sheet
                    .cell(row_index, 2)
                    .unwrap_or_default()
                    .value
                    .as_str_opt()
                    .unwrap_or_default()
                    .to_string(),
                sheet
                    .cell(row_index, 3)
                    .unwrap_or_default()
                    .value
                    .as_str_opt()
                    .unwrap_or_default()
                    .to_string(),
                sheet
                    .cell(row_index, 4)
                    .unwrap_or_default()
                    .value
                    .as_u8_opt()
                    .unwrap_or(0),
                unpack_content_safe(sheet.cell(row_index, 5).unwrap_or_default().value),
                sheet
                    .cell(row_index, 6)
                    .unwrap_or_default()
                    .value
                    .as_str_opt()
                    .unwrap_or_default()
                    .to_string(),
            ))
        }

        let mut um: Vec<Block> = vec![];

        let mut index = 0;
        while index < self.elements.len() {
            if self.elements[index].depth == 0 {
                um.push(self.read_row(index)?);
            }
            index += 1;
        }
        let config = Config {
            preamble: serde_yaml::from_str(
                sheet
                    .cell(1, 4)
                    .unwrap_or_default()
                    .value
                    .as_cow_str_or("")
                    .to_string()
                    .as_str(),
            )
            .unwrap_or_default(),
            output: Output::default(),
            input: PathBuf::default(),
            merging: MergingConfig::default(),
        };

        Ok(Document {
            blocks: um,
            config,
            macros: vec![],
            variables: vec![],
            metadata: vec![],
            resources: vec![],
        })
    }
}

impl std::fmt::Display for Umi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { write!(f, "{}", String::from_utf8_unchecked(self.ods.clone())) }
    }
}

impl OutputFormat for Umi {
    fn new(context: &crate::render::Context) -> Self {
        Umi {
            elements: Vec::new(),
            lang: context.get_lang().to_string(),
            config: context.get_config().clone(),
            ods: vec![],
        }
    }

    // Merge two umi elements
    fn append(&mut self, mut other: Self) -> Result<(), crate::log_id::RenderError> {
        self.elements.append(&mut other.elements);
        Ok(())
    }
}
