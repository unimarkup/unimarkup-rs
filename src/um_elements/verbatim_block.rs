use pest::iterators::Pairs;

use crate::frontend::parser::{Rule, UmParse};
use crate::frontend::UnimarkupBlocks;
use crate::middleend::{AsIrLines, ContentIrLine};
use crate::um_elements::types::UnimarkupType;
use crate::um_error::UmError;

/// Structure of a Unimarkup verbatim block element.
#[derive(Debug, PartialEq, Eq)]
pub struct VerbatimBlock {
    /// Unique identifier for a verbatim block.
    pub id: String,

    /// The content of the verbatim block.
    pub content: String,

    /// Attributes of the verbatim block.
    pub attributes: String,

    /// Line number, where the verbatim block occurs in
    /// the Unimarkup document.
    pub line_nr: usize,
}

impl UmParse for VerbatimBlock {
    fn parse(pairs: &mut Pairs<Rule>, span: pest::Span) -> Result<UnimarkupBlocks, UmError>
    where
        Self: Sized,
    {
        let verbatim = pairs
            .next()
            .expect("Tried to parse invalid verbatim block.");

        let (line_nr, _column_nr) = span.start_pos().line_col();

        let mut block = VerbatimBlock {
            id: String::new(),
            content: String::new(),
            attributes: String::new(),
            line_nr,
        };

        for rule in verbatim.into_inner() {
            match rule.as_rule() {
                Rule::verbatim_lang => {
                    let attr = format!("{{ \"language\": \"{}\" }}", rule.as_str().trim());

                    block.attributes = attr;
                }
                Rule::verbatim_content => {
                    block.content = String::from(rule.as_str().trim());
                }
                _ => continue,
            }
        }

        log::debug!("Parsed verbatim block: \n{:#?}", block);

        Ok(vec![])
    }
}

impl AsIrLines for VerbatimBlock {
    fn as_ir_lines(&self) -> Vec<ContentIrLine> {
        let line = ContentIrLine::new(
            &self.id,
            self.line_nr,
            UnimarkupType::VerbatimBlock.to_string(),
            &self.content,
            "",
            &self.attributes,
            "",
        );

        vec![line]
    }
}
