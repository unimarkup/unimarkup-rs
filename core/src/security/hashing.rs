//! Provides hashing functionality

use std::{fs, path::Path};

use logid::{log, logging::event_entry::AddonKind};
use sha3::{Digest, Sha3_256};

use super::log_id::HashingError;

/// Calculates the sha3-256 hash of a given file
pub fn get_filehash(file: &Path) -> Result<Vec<u8>, HashingError> {
    let mut hasher = Sha3_256::new();
    let source = fs::read_to_string(file).map_err(|err| {
        log!(
            HashingError::FailedReadingFile,
            &format!("Could not read file: '{:?}'", file),
            add: AddonKind::Info(format!("Cause: {}", err))
        );
        HashingError::FailedReadingFile
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
