use crate::render::OutputFormat;

pub mod render;

#[derive(Debug, Default)]
pub struct UmiRow {
    position: u8,
    id: String,
    kind: String, // Maybe change to Enum
    symbol_count: u8,
    depth: u8,
    content: String,
    attributes: String,
}

impl UmiRow {
    fn new(
        position: u8,
        id: String,
        kind: String,
        symbol_count: u8,
        depth: u8,
        content: String,
        attributes: String,
    ) -> Self {
        UmiRow {
            position: position,
            id: id,
            kind: kind,
            symbol_count: symbol_count,
            depth: depth,
            content: content,
            attributes: attributes,
        }
    }
}

#[derive(Debug, Default)]
pub struct Umi {
    pub elements: Vec<UmiRow>,
    pub lang: String,
}

impl Umi {
    fn with(elements: Vec<UmiRow>, lang: String) -> Self {
        Umi {
            elements: elements,
            lang: lang,
        }
    }
    /// Merge two
    fn merge(&mut self, other_umi: &mut Umi) {
        assert!(self.lang == other_umi.lang);
        self.elements.append(&mut other_umi.elements);
    }
}

impl OutputFormat for Umi {
    fn new(context: &crate::render::Context) -> Self {
        Umi {
            elements: Vec::new(),
            lang: context.get_lang().to_string(),
        }
    }

    fn append(&mut self, mut other: Self) -> Result<(), crate::log_id::RenderError> {
        // Append two Umi Elements
        assert!(self.lang == other.lang);
        self.elements.append(&mut other.elements);
        Ok(())
    }
}
