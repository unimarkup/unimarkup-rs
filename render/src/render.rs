//! Contains the [`Render`] trait definition.

use crate::html::citeproc::CiteprocWrapper;
use logid::log;
use serde_json::Value;
use std::path::PathBuf;
use unimarkup_commons::config::Config;
use unimarkup_commons::{
    config::icu_locid::{locale, Locale},
    lexer::span::Span,
};
use unimarkup_inline::element::{
    base::{EscapedNewline, EscapedPlain, EscapedWhitespace, Newline, Plain},
    formatting::{
        Bold, Highlight, Italic, Math, Overline, Quote, Strikethrough, Subscript, Superscript,
        Underline, Verbatim,
    },
    textbox::{citation::Citation, hyperlink::Hyperlink, TextBox},
    Inline,
};
use unimarkup_parser::{
    document::Document,
    elements::{
        atomic::{Heading, Paragraph},
        blocks::Block,
        enclosed,
        indents::{BulletList, BulletListEntry},
    },
};

use crate::log_id::{GeneralWarning, RenderError};

pub struct Context<'a> {
    doc: &'a Document,
    rendered_citations: Vec<String>,
    pub footnotes: String,
    pub bibliography: String,
}

impl<'a> Context<'a> {
    /// Returns the locale for the natural language that is the main language for this rendering.
    pub fn get_lang(&self) -> Locale {
        self.doc
            .config
            .preamble
            .i18n
            .lang
            .clone()
            .unwrap_or(locale!("en"))
    }

    pub fn rendered_citation(&self, index: usize) -> Option<&String> {
        self.rendered_citations.get(index)
    }

    fn new(doc: &'a Document) -> Self {
        let rendered_citations: Vec<String>;
        let footnotes: String;
        let bibliography: String;

        match CiteprocWrapper::new() {
            Ok(mut citeproc) => {
                let for_pagedjs = false;
                let style = doc
                    .config
                    .preamble
                    .cite
                    .style
                    .clone()
                    .unwrap_or(PathBuf::from(String::from("ieee")));
                let doc_locale = doc
                    .config
                    .preamble
                    .i18n
                    .lang
                    .clone()
                    .unwrap_or(locale!("en-US"));
                let citation_locales = doc.config.preamble.cite.citation_locales.clone();
                let citation_ids = doc
                    .citations
                    .clone()
                    .into_iter()
                    .map(|c| match serde_json::to_value::<Vec<String>>(c) {
                        Ok(citation_ids) => citation_ids,
                        Err(e) => {
                            log!(
                                GeneralWarning::JSONSerialization,
                                format!("JSON serialization failed with error: '{:?}'", e)
                            );
                            serde_json::to_value::<Vec<String>>(vec![]).unwrap()
                        }
                    })
                    .collect::<Value>();
                rendered_citations = match citeproc.get_citation_strings(
                    &doc.config.preamble.cite.references,
                    doc_locale,
                    citation_locales,
                    style,
                    &[citation_ids],
                    for_pagedjs,
                ) {
                    Ok(rendered_citations) => rendered_citations,
                    Err(e) => {
                        log!(e);
                        vec![
                            "########### CITATION ERROR ###########".to_string();
                            doc.citations.len()
                        ]
                    }
                };
                footnotes = match citeproc.get_footnotes() {
                    Ok(footnotes) => footnotes,
                    Err(e) => {
                        log!(e);
                        "########### CITATION ERROR ###########".to_string()
                    }
                };
                bibliography = match citeproc.get_bibliography() {
                    Ok(bibliography) => bibliography,
                    Err(e) => {
                        log!(e);
                        "########### CITATION ERROR ###########".to_string()
                    }
                };
            }
            Err(e) => {
                log!(e);
                rendered_citations =
                    vec!["########### CITATION ERROR ###########".to_string(); doc.citations.len()];
                footnotes = "########### CITATION ERROR ###########".to_string();
                bibliography = "########### CITATION ERROR ###########".to_string();
            }
        }

        Context {
            doc,
            rendered_citations,
            footnotes,
            bibliography,
        }
    }

    pub fn get_config(&self) -> &Config {
        &self.doc.config
    }
}

pub fn render<T: OutputFormat>(
    doc: &Document,
    mut renderer: impl Renderer<T>,
) -> Result<T, RenderError> {
    let context = Context::new(doc);
    let mut t = T::new(&context);

    t.append(renderer.render_blocks(&doc.blocks, &context)?)?;

    t.append(renderer.render_footnotes(&context)?)?;
    t.append(renderer.render_bibliography(&context)?)?;

    Ok(t)
}

pub trait OutputFormat: Default {
    fn new(context: &Context) -> Self;

    fn append(&mut self, other: Self) -> Result<(), RenderError>;
}

/// The [`Renderer`] trait allows to create custom output formats for a Unimarkup [`unimarkup_parser::document::Document`].
pub trait Renderer<T: OutputFormat> {
    // Note: Default implementation with `Err(RenderError::Unimplemented)` prevents breaking changes when adding new functions to this trait.

    //--------------------------------- BLOCKS ---------------------------------

    /// Render a Unimarkup [`Paragraph`] to the output format `T`.
    fn render_paragraph(
        &mut self,
        _paragraph: &Paragraph,
        _context: &Context,
    ) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render a Unimarkup [`Heading`] to the output format `T`.
    fn render_heading(&mut self, _heading: &Heading, _context: &Context) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render a Unimarkup [`Verbatim` block](enclosed::Verbatim) to the output format `T`.
    fn render_verbatim_block(
        &mut self,
        _verbatim: &enclosed::VerbatimBlock,
        _context: &Context,
    ) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render a Unimarkup [`BulletList`] to the output format `T`.
    fn render_bullet_list(
        &mut self,
        _bullet_list: &BulletList,
        _context: &Context,
    ) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render a Unimarkup [`BulletListEntry`] to the output format `T`.
    fn render_bullet_list_entry(
        &mut self,
        _bullet_list_entry: &BulletListEntry,
        _context: &Context,
    ) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    fn render_blankline(
        &mut self,
        _blankline: &Span,
        _context: &Context,
    ) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    //--------------------------------- INLINES ---------------------------------

    /// Render a [`TextBox`] to the output format `T`.
    fn render_textbox(&mut self, _textbox: &TextBox, _context: &Context) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render a [`Hyperlink`] to the output format `T`.
    fn render_hyperlink(
        &mut self,
        _hyperlink: &Hyperlink,
        _context: &Context,
    ) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    fn render_citation(
        &mut self,
        _citation: &Citation,
        _context: &Context,
    ) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    fn render_bibliography(&mut self, _context: &Context) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    fn render_footnotes(&mut self, _context: &Context) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render a [`Bold` formatting](unimarkup_inline::inlines::Inline) to the output format `T`.
    fn render_bold(&mut self, _bold: &Bold, _context: &Context) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render a [`Italic` formatting](unimarkup_inline::inlines::Inline) to the output format `T`.
    fn render_italic(&mut self, _italic: &Italic, _context: &Context) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render a [`Underline` formatting](unimarkup_inline::inlines::Inline) to the output format `T`.
    fn render_underline(
        &mut self,
        _underline: &Underline,
        _context: &Context,
    ) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render a [`Subscript` formatting](unimarkup_inline::inlines::Inline) to the output format `T`.
    fn render_subscript(
        &mut self,
        _subscript: &Subscript,
        _context: &Context,
    ) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render a [`Superscript` formatting](unimarkup_inline::inlines::Inline) to the output format `T`.
    fn render_superscript(
        &mut self,
        _superscript: &Superscript,
        _context: &Context,
    ) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render a [`Overline` formatting](unimarkup_inline::inlines::Inline) to the output format `T`.
    fn render_overline(
        &mut self,
        _overline: &Overline,
        _context: &Context,
    ) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render a [`Strikethrough` formatting](unimarkup_inline::inlines::Inline) to the output format `T`.
    fn render_strikethrough(
        &mut self,
        _strikethrough: &Strikethrough,
        _context: &Context,
    ) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render a [`Highlight` formatting](unimarkup_inline::inlines::Inline) to the output format `T`.
    fn render_highlight(
        &mut self,
        _highlight: &Highlight,
        _context: &Context,
    ) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render a [`Quote` formatting](unimarkup_inline::inlines::Inline) to the output format `T`.
    fn render_quote(&mut self, _quote: &Quote, _context: &Context) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render a [`Verbatim` formatting](unimarkup_inline::inlines::Inline) to the output format `T`.
    fn render_inline_verbatim(
        &mut self,
        _verbatim: &Verbatim,
        _context: &Context,
    ) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render a [`Math`] to the output format `T`.
    fn render_inline_math(&mut self, _math: &Math, _context: &Context) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render [`Plain` content](unimarkup_inline::inlines::Inline) to the output format `T`.
    fn render_plain(&mut self, _plain: &Plain, _context: &Context) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render [`Newline` content](unimarkup_inline::inlines::Inline) to the output format `T`.
    fn render_newline(&mut self, _newline: &Newline, _context: &Context) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render [`EscapedNewline` content](unimarkup_inline::inlines::Inline) to the output format `T`.
    fn render_escaped_newline(
        &mut self,
        _escaped_newline: &EscapedNewline,
        _context: &Context,
    ) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render implicit [`Newline` content](unimarkup_inline::inlines::Inline) to the output format `T`.
    fn render_implicit_newline(
        &mut self,
        _implicit_newline: &Newline,
        _context: &Context,
    ) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render [`EscapedWhitespace` content](unimarkup_inline::inlines::Inline) to the output format `T`.
    fn render_escaped_whitespace(
        &mut self,
        _escaped_whitespace: &EscapedWhitespace,
        _context: &Context,
    ) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    /// Render [`EscapedPlain` content](unimarkup_inline::inlines::Inline) to the output format `T`.
    fn render_escaped_plain(
        &mut self,
        _escaped_plain: &EscapedPlain,
        _context: &Context,
    ) -> Result<T, RenderError> {
        Err(RenderError::Unimplemented)
    }

    //----------------------------- GENERIC ELEMENTS -----------------------------

    /// Render Unimarkup [`Block`s](Block) to the output format `T`.
    fn render_blocks(&mut self, blocks: &[Block], context: &Context) -> Result<T, RenderError> {
        let mut t = T::default();

        for block in blocks {
            let rendered_block = match self.render_block(block, context) {
                Err(err) if err == RenderError::Unimplemented => {
                    logid::log!(
                        err,
                        format!(
                            "Rendering of block '{}' is not implemented",
                            block.variant_str()
                        ),
                    );
                    continue;
                }
                res => res,
            }?;

            t.append(rendered_block)?;
        }

        Ok(t)
    }

    /// Render a Unimarkup [`Block`] to the output format `T`.
    fn render_block(&mut self, block: &Block, context: &Context) -> Result<T, RenderError> {
        match block {
            Block::Heading(heading) => self.render_heading(heading, context),
            Block::Paragraph(paragraph) => self.render_paragraph(paragraph, context),
            Block::VerbatimBlock(verbatim) => self.render_verbatim_block(verbatim, context),
            Block::BulletList(bullet_list) => self.render_bullet_list(bullet_list, context),
            Block::Blankline(blankline) => self.render_blankline(blankline, context),
            Block::BulletListEntry(_) => {
                debug_assert!(
                    false,
                    "Bullet list entries are rendered directly insie a bullet list."
                );
                Err(RenderError::Unimplemented)
            }
        }
    }

    /// Render Unimarkup [`Inline`s](Inline) to the output format `T`.
    fn render_inlines(&mut self, inlines: &[Inline], context: &Context) -> Result<T, RenderError> {
        let mut t = T::default();

        for inline in inlines {
            let render_res = match self.render_inline(inline, context) {
                Err(err) if err == RenderError::Unimplemented => {
                    logid::log!(
                        err,
                        format!(
                            "Rendering of inline '{}' is not implemented",
                            inline.variant_str()
                        ),
                    );
                    continue;
                }
                res => res,
            }?;

            t.append(render_res)?;
        }

        Ok(t)
    }

    /// Render a Unimarkup [`Inline`] to the output format `T`.
    fn render_inline(&mut self, inline: &Inline, context: &Context) -> Result<T, RenderError> {
        match inline {
            Inline::Bold(bold) => self.render_bold(bold, context),
            Inline::Italic(italic) => self.render_italic(italic, context),
            Inline::Underline(underline) => self.render_underline(underline, context),
            Inline::Subscript(subscript) => self.render_subscript(subscript, context),
            Inline::Superscript(superscript) => self.render_superscript(superscript, context),
            Inline::Overline(overline) => self.render_overline(overline, context),
            Inline::Strikethrough(strikethrough) => {
                self.render_strikethrough(strikethrough, context)
            }
            Inline::Highlight(highlight) => self.render_highlight(highlight, context),
            Inline::Quote(quote) => self.render_quote(quote, context),
            Inline::Verbatim(verbatim) => self.render_inline_verbatim(verbatim, context),
            Inline::Plain(plain) => self.render_plain(plain, context),
            Inline::Newline(newline) => self.render_newline(newline, context),
            Inline::EscapedNewline(escaped_newline) => {
                self.render_escaped_newline(escaped_newline, context)
            }
            Inline::EscapedWhitespace(escaped_whitespace) => {
                self.render_escaped_whitespace(escaped_whitespace, context)
            }
            Inline::EscapedPlain(escaped_plain) => {
                self.render_escaped_plain(escaped_plain, context)
            }
            Inline::ImplicitNewline(implicit_newline) => {
                self.render_implicit_newline(implicit_newline, context)
            }
            Inline::Math(math) => self.render_inline_math(math, context),
            Inline::TextBox(textbox) => self.render_textbox(textbox, context),
            Inline::Hyperlink(hyperlink) => self.render_hyperlink(hyperlink, context),
            Inline::Citation(citation) => self.render_citation(citation, context),

            Inline::NamedSubstitution(_) => todo!(),
            Inline::ImplicitSubstitution(_) => todo!(),
            Inline::DirectUri(_) => todo!(),
        }
    }

    fn render_nested_inline(
        &mut self,
        nested: &[Inline],
        context: &Context,
    ) -> Result<T, RenderError> {
        let mut t = T::default();

        for inline in nested.iter() {
            t.append(self.render_inline(inline, context)?)?;
        }

        Ok(t)
    }
}
