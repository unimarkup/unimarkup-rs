//! Log-id handling functionality for unimarkup-rs

use logid::{
    id_map::LogIdMap,
};
use once_cell::sync::Lazy;

/// Map to store [`LogId`]s set in the [`core`] crate.
pub(crate) static CORE_LOG_ID_MAP: Lazy<LogIdMap> = Lazy::new(LogIdMap::new);

/// Log-id sub group for the core crate
pub enum LogIdMainGrp {
    /// General log-id sub group
    General = 0,
    /// Frontend log-id sub group
    Frontend = 1,
    /// Middleend log-id sub group
    Middleend = 2,
    /// Backend log-id sub group
    Backend = 3,
    /// Element log-id sub group
    Element = 4,
    /// Security log-id sub group
    Security = 6,
}
