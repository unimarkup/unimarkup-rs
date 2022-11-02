//! Provides hashing functionality

use std::{fs, path::Path};

use logid::{
    capturing::{LogIdTracing, MappedLogId},
    log_id::LogId,
};
use sha3::{Digest, Sha3_256};

use crate::log_id::CORE_LOG_ID_MAP;

use super::log_id::HashingErrLogId;

/// Calculates the sha3-256 hash of a given file
pub fn get_filehash(file: &Path) -> Result<Vec<u8>, MappedLogId> {
    let mut hasher = Sha3_256::new();
    let source = fs::read_to_string(file).map_err(|err| {
        (HashingErrLogId::FailedReadingFile as LogId)
            .set_event_with(
                &CORE_LOG_ID_MAP,
                &format!("Could not read file: '{:?}'", file),
                file!(),
                line!(),
            )
            .add_info(&format!("Cause: {}", err))
    })?;

    hasher.update(source);

    let hash = hasher.finalize();
    Ok(hash.to_vec())
}

/// Calculates the sha3-256 hash of the given content
pub fn get_contenthash(content: &str) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(content);

    let hash = hasher.finalize();
    hash.to_vec()
}
