use unimarkup_inline::types::*;

use crate::render::{Context, Renderer, OutputFormat};

use super::{Umi, UmiRow};

#[derive(Debug, Default)]
pub struct UmiRenderer{
    pub pos: u8,
    pub depth: u8,
}

impl UmiRenderer{
    fn step_in(&mut self){
        self.depth += 1;
    }

    fn step_out(&mut self){
        self.depth -= 1;
    }

    fn proceed(&mut self){
        self.pos += 1;
    }
}

impl Renderer<Umi> for UmiRenderer{
    fn render_paragraph(
        &mut self,
        paragraph: &unimarkup_parser::elements::atomic::Paragraph,
        context: &Context,
    ) -> Result<Umi, crate::log_id::RenderError> {
        let content = self.render_inlines(&paragraph.content, context)?;

        let paragraph = UmiRow::new(
            self.pos, 
            "".to_owned(), 
            "paragraph".to_owned(), 
            0, 
            self.depth, 
            content.elements[0].content.to_owned(), 
            paragraph.attributes.to_owned().unwrap_or("".to_owned()),
        );

        self.proceed();
        Ok(Umi::with(vec!(paragraph), context.get_lang().to_string()))
    }

    fn render_verbatim_block(
        &mut self,
        verbatim: &unimarkup_parser::elements::enclosed::Verbatim,
        context: &Context,
    ) -> Result<Umi, crate::log_id::RenderError> {


        Ok(Umi::new(context))
    }

    fn render_heading(
        &mut self, 
        heading: &unimarkup_parser::elements::atomic::Heading, 
        context: &Context
    ) -> Result<Umi, crate::log_id::RenderError> {
        Ok(Umi::new(context))
    }

    fn render_bullet_list(
            &mut self,
            bullet_list: &unimarkup_parser::elements::indents::BulletList,
            context: &Context,
        ) -> Result<Umi, crate::log_id::RenderError> {
        Ok(Umi::new(context))
    }

    fn render_bullet_list_entry(
            &mut self,
            bullet_list_entry: &unimarkup_parser::elements::indents::BulletListEntry,
            context: &Context,
        ) -> Result<Umi, crate::log_id::RenderError> {
        Ok(Umi::new(context))
    }

    fn render_inlines(
        &mut self, 
        inlines: &[unimarkup_inline::Inline], 
        context: &Context
    ) -> Result<Umi, crate::log_id::RenderError> {
        // TODO refine inline appending
        let mut res = "".to_owned();
        for inline in inlines{
            res += &mut inline.as_string();   
        }

        Ok(Umi::with(vec!(UmiRow::new(
            self.pos, 
            "".to_owned(), 
            "inline".to_owned(), 
            0, 
            self.depth, 
            res, 
            "".to_owned()
        )), context.get_lang().to_string()))
    }
}