//! Traits and helper functions for logging functionality.

use env_logger::fmt::Color;
use log::LevelFilter;
use std::io::Write;

/// Used to convert log [`Level`](log::Level) into [`Color`](Color)
pub trait IntoColor {
    /// Consumes self and returns a [`Color`]
    fn into_color(self) -> Color;
}

impl IntoColor for log::Level {
    fn into_color(self) -> Color {
        match self {
            log::Level::Error => Color::Red,
            log::Level::Warn => Color::Yellow,
            log::Level::Info => Color::Green,
            log::Level::Debug => Color::Blue,
            log::Level::Trace => Color::Magenta,
        }
    }
}

/// Initializes [`env_logger`](env_logger) used for logging functionality of
/// [`unimarkup-rs`]
pub fn init_logger() {
    env_logger::builder()
        .format_level(true)
        .filter_level(LevelFilter::Debug)
        .format_timestamp(None)
        .format_target(false)
        .format_module_path(false)
        .format_suffix("")
        .format(|buf, record| {
            let mut style = buf.style();
            style.set_color(record.level().into_color()).set_bold(true);
            writeln!(buf, "{:<5}: {}", style.value(record.level()), record.args())
        })
        .init();
}
