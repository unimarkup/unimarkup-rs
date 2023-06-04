//! Contains the [`Render`] trait definition.

use crate::html::Html;

/// Abstract type for Unimarkup elements that implement the [`Render`] trait
pub type RenderBlock = Box<dyn Render>;

/// Trait to provide render implementation for supported output formats
pub trait Render {
    /// Generates the HTML representation of an Unimarkup element.
    ///
    /// Returns a [`RenderErr`] if it's not possible to generate valid HTML, i.e. if
    /// provided id contains characters not allowed in HTML id attribute.
    fn render_html(&self) -> Result<Html, RenderErr>;
}

#[derive(Debug, Clone)]
pub struct RenderErr {
    cause: logid::evident::event::intermediary::FinalizedEvent<logid::log_id::LogId>,
}

impl std::error::Error for RenderErr {}

impl std::fmt::Display for RenderErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "eventId='{}', entryId='{}'",
            self.cause.event_id, self.cause.entry_id
        )
    }
}
