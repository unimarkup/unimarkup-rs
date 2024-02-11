use unimarkup_inline::element::InlineElement;
use unimarkup_parser::elements::blocks::Block;

use crate::log_id::RenderError;
use crate::render::{Context, OutputFormat, Renderer};
use std::collections::HashMap;

use super::{Umi, UmiRow};

#[derive(Debug, Default)]
pub struct UmiRenderer {
    pub umi: Umi,
    pub pos: u8,
    pub depth: u8,
}

impl UmiRenderer {
    // Step into a nested element (Should be called by parent)
    fn step_in(&mut self) {
        self.depth += 1;
    }

    // Step out of a nested element (Should be called by parent)
    fn step_out(&mut self) {
        self.depth -= 1;
    }

    // Should always be called after element has been fully rendered to Umi
    fn proceed(&mut self, new_umi: Umi) -> Result<Umi, crate::log_id::RenderError> {
        if self.depth != 0 {
            Ok(new_umi.clone())
        } else {
            let _ = self.umi.append(new_umi);
            Ok(self.umi.clone())
        }
    }
}

impl Renderer<Umi> for UmiRenderer {
    fn render_paragraph(
        &mut self,
        paragraph: &unimarkup_parser::elements::atomic::Paragraph,
        context: &Context,
    ) -> Result<Umi, crate::log_id::RenderError> {
        let content = self.render_inlines(&paragraph.content, context)?;

        let hashmap: HashMap<String, String> = HashMap::new();
        let properties = serde_json::to_string(&hashmap).unwrap_or(String::from("{}"));

        let paragraph = UmiRow::new(
            self.pos,
            String::new(),
            String::from(Block::Paragraph(paragraph.to_owned()).variant_str()),
            properties,
            self.depth,
            content.elements[0].content.clone(),
            String::new(),
        );
        self.pos += 1;

        self.proceed(Umi::with_um(
            vec![paragraph],
            context.get_config().clone(),
            context.get_lang().to_string(),
        ))
    }

    fn render_verbatim_block(
        &mut self,
        verbatim: &unimarkup_parser::elements::enclosed::VerbatimBlock,
        context: &Context,
    ) -> Result<Umi, crate::log_id::RenderError> {
        let mut hashmap: HashMap<String, String> = HashMap::new();
        hashmap.insert(
            String::from("data_lang"),
            verbatim.data_lang.clone().unwrap_or_default(),
        );
        hashmap.insert(String::from("tick_len"), verbatim.tick_len.to_string());
        hashmap.insert(
            String::from("implicit_closed"),
            verbatim.implicit_closed.to_string(),
        );
        let properties = serde_json::to_string(&hashmap).unwrap_or(String::from("{}"));

        let verbatim = UmiRow::new(
            self.pos,
            String::new(),
            String::from("VerbatimBlock"),
            properties,
            self.depth,
            verbatim.content.clone(),
            verbatim.attributes.clone().unwrap_or_default(),
        );
        self.pos += 1;

        self.proceed(Umi::with_um(
            vec![verbatim],
            context.get_config().clone(),
            context.get_lang().to_string(),
        ))
    }

    fn render_heading(
        &mut self,
        heading: &unimarkup_parser::elements::atomic::Heading,
        context: &Context,
    ) -> Result<Umi, crate::log_id::RenderError> {
        let mut hashmap: HashMap<String, String> = HashMap::new();
        hashmap.insert(String::from("level"), heading.level.as_str().to_string());
        let properties = serde_json::to_string(&hashmap).unwrap_or(String::from("{}"));

        let content = self.render_inlines(&heading.content, context)?;

        let heading = UmiRow::new(
            self.pos,
            heading.id.clone(),
            String::from("Heading"),
            properties,
            self.depth,
            content.elements[0].content.clone(),
            heading.attributes.clone().unwrap_or_default(),
        );
        self.pos += 1;

        self.proceed(Umi::with_um(
            vec![heading],
            context.get_config().clone(),
            context.get_lang().to_string(),
        ))
    }

    fn render_bullet_list(
        &mut self,
        bullet_list: &unimarkup_parser::elements::indents::BulletList,
        context: &Context,
    ) -> Result<Umi, crate::log_id::RenderError> {
        let hashmap: HashMap<String, String> = HashMap::new();
        let properties = serde_json::to_string(&hashmap).unwrap_or(String::from("{}"));

        let bullet_list_heading = UmiRow::new(
            self.pos,
            String::new(),
            String::from("BulletList"),
            properties,
            self.depth,
            String::new(),
            String::new(),
        );
        self.pos += 1;

        let mut bullet_list_content = Umi::with_um(
            vec![bullet_list_heading],
            context.get_config().clone(),
            context.get_lang().to_string(),
        );

        self.step_in();
        for entry in &bullet_list.entries {
            bullet_list_content.append(self.render_bullet_list_entry(entry, context)?)?;
        }
        self.step_out();

        self.proceed(bullet_list_content)
    }

    fn render_bullet_list_entry(
        &mut self,
        bullet_list_entry: &unimarkup_parser::elements::indents::BulletListEntry,
        context: &Context,
    ) -> Result<Umi, crate::log_id::RenderError> {
        let mut hashmap: HashMap<String, String> = HashMap::new();
        hashmap.insert(
            String::from("keyword"),
            bullet_list_entry.keyword.as_str().to_string(),
        );
        let properties = serde_json::to_string(&hashmap).unwrap_or(String::from("{}"));
        let mut entry = Umi::with_um(
            vec![UmiRow::new(
                self.pos,
                String::new(),
                Block::BulletListEntry(bullet_list_entry.to_owned())
                    .variant_str()
                    .to_string(),
                properties,
                self.depth,
                self.render_inlines(&bullet_list_entry.heading, context)?
                    .elements[0]
                    .content
                    .clone(),
                String::new(),
            )],
            context.get_config().clone(),
            context.get_lang().to_string(),
        );
        self.pos += 1;

        // Render All Bullet List Body Elements
        self.step_in();
        if !bullet_list_entry.body.is_empty() {
            let next_entry = self.render_blocks(&bullet_list_entry.body, context)?;
            let _ = entry.append(next_entry);
        }
        self.step_out();

        self.proceed(entry)
    }

    fn render_inlines(
        &mut self,
        inlines: &[unimarkup_inline::element::Inline],
        context: &Context,
    ) -> Result<Umi, crate::log_id::RenderError> {
        let mut res = String::new();
        for inline in inlines {
            res += &mut inline.as_unimarkup();
        }

        Ok(Umi::with_um(
            vec![UmiRow::new(
                self.pos,
                String::new(),
                String::from("Inline"),
                String::new(),
                self.depth,
                res,
                String::new(),
            )],
            context.get_config().clone(),
            context.get_lang().to_string(),
        ))
    }

    fn get_target(&mut self) -> Result<Umi, RenderError> {
        Ok(Umi::default())
    }

    fn render_bibliography(
        &mut self,
        context: &Context,
    ) -> Result<Umi, crate::log_id::RenderError> {
        if context.bibliography.is_some() {
            let entry = UmiRow::new(
                self.pos,
                String::new(),
                "Bibliography".to_string(),
                String::new(),
                self.depth,
                "{$um.bibliography}".to_string(),
                String::new(),
            );

            self.pos += 1;

            self.proceed(Umi::with_um(
                vec![entry],
                context.get_config().clone(),
                context.get_lang().to_string(),
            ))
        } else {
            Ok(Umi::default())
        }
    }

    fn render_footnotes(&mut self, context: &Context) -> Result<Umi, crate::log_id::RenderError> {
        if context.footnotes.is_some() {
            let entry = UmiRow::new(
                self.pos,
                String::new(),
                "Footnotes".to_string(),
                String::new(),
                self.depth,
                "{$um.footnotes}".to_string(),
                String::new(),
            );

            self.pos += 1;

            self.proceed(Umi::with_um(
                vec![entry],
                context.get_config().clone(),
                context.get_lang().to_string(),
            ))
        } else {
            Ok(Umi::default())
        }
    }
}
