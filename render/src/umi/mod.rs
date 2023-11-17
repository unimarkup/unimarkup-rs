use crate::render::OutputFormat;

use spreadsheet_ods::{WorkBook, Sheet, write_ods_buf_uncompressed};

pub mod render;

#[derive(Debug, Default, Clone)]
pub struct UmiRow {
    position: u8,
    id: String,
    kind: String,
    depth: u8,
    content: String,
    attributes: String,
}

impl UmiRow {
    fn new(
        position: u8,
        id: String,
        kind: String,
        depth: u8,
        content: String,
        attributes: String,
    ) -> Self {
        UmiRow {
            position,
            id,
            kind,
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
    pub ods: Vec<u8>,
}

impl Umi {
    fn with_um(elements: Vec<UmiRow>, lang: String) -> Self {
        Umi { 
            elements, 
            lang, 
            ods: vec![] }
    }

    // fn with_wb() -> Self{}
    // Merge two Umi Structs
    fn merge(&mut self, other_umi: &mut Umi) {
        self.elements.append(&mut other_umi.elements);
    }

    pub fn create_workbook(&mut self) -> &mut Self {
        let mut wb = WorkBook::new_empty();

        let mut sheet = Sheet::new("umi");
        // Set the Header Row, could be left out if it is not meant for viewing
        sheet.set_value(0, 0, "position");
        sheet.set_value(0, 1, "id");
        sheet.set_value(0, 2, "kind");
        sheet.set_value(0, 3, "depth");
        sheet.set_value(0, 4, "content-en-US");
        sheet.set_value(0, 5, "attributes-en-US");

        for element in &self.elements{
            let row = element.position + 1;
            sheet.set_value(row.into(), 0, element.position);
            sheet.set_value(row.into(), 1, element.id.clone());
            sheet.set_value(row.into(), 2, element.kind.clone());
            sheet.set_value(row.into(), 3, element.depth);
            sheet.set_value(row.into(), 4, element.content.clone());
            sheet.set_value(row.into(), 5, element.attributes.clone());
        }

        wb.push_sheet(sheet);

        let res = write_ods_buf_uncompressed(&mut wb, vec![]);

        self.ods = res.unwrap_or_default();

        self
    }

    // fn create_um() {}
}

impl std::fmt::Display for Umi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe{
        write!(
            f,
            "{}",
            String::from_utf8_unchecked(self.ods.clone())
        )
    }
    }
}

impl OutputFormat for Umi {
    fn new(context: &crate::render::Context) -> Self {
        Umi {
            elements: Vec::new(),
            lang: context.get_lang().to_string(),
            ods: vec![]
        }
    }

    fn append(&mut self, mut other: Self) -> Result<(), crate::log_id::RenderError> {
        // Append two Umi Elements
        self.elements.append(&mut other.elements);
        Ok(())
    }
}
