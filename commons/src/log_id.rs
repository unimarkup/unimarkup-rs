use logid::id_map::LogIdMap;
use once_cell::sync::Lazy;

/// Map to store [`LogId`]s set in the [`core`] crate.
pub(crate) static COMMONS_LOG_ID_MAP: Lazy<LogIdMap> = Lazy::new(LogIdMap::new);

logid::setup_logid_map!(&COMMONS_LOG_ID_MAP);

/// Log-id sub group for the core crate
pub enum LogIdMainGrp {
    /// General log-id sub group
    General = 0,
    /// Config log-id sub group
    Config = 1,
}
