//! Traits and helper functions for logging functionality.

use tracing::metadata::LevelFilter;

/// Initializes a [`tracing_subscriber`] used for logging functionality of
/// [`unimarkup-rs`]
pub fn init_logger() {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .without_time()
        .with_target(false)
        .with_max_level(LevelFilter::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
