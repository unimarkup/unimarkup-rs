//! Provides hashing functionality

use std::{fs, path::Path};

use sha3::{Digest, Sha3_256};

use crate::log_id::{LogId, SetLog};

use super::{error::SecurityError, log_id::HashingErrLogId};

/// Calculates the sha3-256 hash of a given file
pub fn get_filehash(file: &Path) -> Result<Vec<u8>, SecurityError> {
    let mut hasher = Sha3_256::new();
    let source = fs::read_to_string(file).map_err(|err| {
        SecurityError::Hashing(
            (HashingErrLogId::FailedReadingFile as LogId)
                .set_log(
                    &format!("Could not read file: '{:?}'", file),
                    file!(),
                    line!(),
                )
                .add_info(&format!("Cause: {}", err)),
        )
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
