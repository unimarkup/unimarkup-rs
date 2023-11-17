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
    fn step_in(&mut self) {
        self.depth += 1;
    }

    fn step_out(&mut self) {
        self.depth -= 1;
    }

    fn proceed(&mut self, new_umi: &mut Umi) -> Result<Umi, crate::log_id::RenderError> {
        self.pos += 1;

        if self.depth != 0 {
            Ok(new_umi.clone())
        } else {
            self.umi.merge(new_umi);
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

        let paragraph = UmiRow::new(
            self.pos,
            paragraph.id.clone(),
            String::from("paragraph"),
            self.depth,
            content.elements[0].content.clone(),
            paragraph.attributes.clone().unwrap_or_default(),
        );

        self.proceed(&mut Umi::with_um(
            vec![paragraph],
            context.get_lang().to_string(),
        ))
    }

    fn render_verbatim_block(
        &mut self,
        verbatim: &unimarkup_parser::elements::enclosed::Verbatim,
        context: &Context,
    ) -> Result<Umi, crate::log_id::RenderError> {
        let mut hashmap: HashMap<String, String> = HashMap::new();
        hashmap.insert(
            String::from("attributes"),
            verbatim.attributes.clone().unwrap_or_default(),
        );
        hashmap.insert(String::from("elem_count"), String::from("3"));

        let attributes = format!("{:?}", hashmap);

        let verbatim = UmiRow::new(
            self.pos,
            verbatim.id.clone(),
            String::from("verbatim"),
            self.depth,
            verbatim.content.clone(),
            attributes,
        );

        self.proceed(&mut Umi::with_um(
            vec![verbatim],
            context.get_lang().to_string(),
        ))
    }

    fn render_heading(
        &mut self,
        heading: &unimarkup_parser::elements::atomic::Heading,
        context: &Context,
    ) -> Result<Umi, crate::log_id::RenderError> {
        let content = self.render_inlines(&heading.content, context)?;

        let heading = UmiRow::new(
            self.pos,
            heading.id.clone(),
            String::from("heading"),
            self.depth,
            content.elements[0].content.clone(),
            heading.attributes.clone().unwrap_or_default(),
        );

        self.proceed(&mut Umi::with_um(
            vec![heading],
            context.get_lang().to_string(),
        ))
    }

    fn render_bullet_list(
        &mut self,
        bullet_list: &unimarkup_parser::elements::indents::BulletList,
        context: &Context,
    ) -> Result<Umi, crate::log_id::RenderError> {
        let bullet_list_heading = UmiRow::new(
            self.pos,
            bullet_list.id.clone(),
            String::from("bullet-list"),
            self.depth,
            String::new(),
            String::new(),
        );

        let mut bullet_list_content =
            Umi::with_um(vec![bullet_list_heading], context.get_lang().to_string());

        self.step_in();
        for entry in &bullet_list.entries {
            bullet_list_content.append(self.render_bullet_list_entry(entry, context)?)?;
        }
        self.step_out();

        self.proceed(&mut bullet_list_content)
    }

    fn render_bullet_list_entry(
        &mut self,
        bullet_list_entry: &unimarkup_parser::elements::indents::BulletListEntry,
        context: &Context,
    ) -> Result<Umi, crate::log_id::RenderError> {
        let mut entry = Umi::with_um(vec![], context.get_lang().to_string());

        if !bullet_list_entry.body.is_empty() {
            entry.merge(&mut self.render_blocks(&bullet_list_entry.body, context)?);
        }

        self.proceed(&mut entry)
    }

    fn render_inlines(
        &mut self,
        inlines: &[unimarkup_inline::Inline],
        context: &Context,
    ) -> Result<Umi, crate::log_id::RenderError> {
        let mut res = String::from("");
        for inline in inlines {
            res += &mut inline.as_string();
        }

        Ok(Umi::with_um(
            vec![UmiRow::new(
                self.pos,
                String::new(),
                String::from("inline"),
                self.depth,
                res,
                String::new(),
            )],
            context.get_lang().to_string(),
        ))
    }
}
