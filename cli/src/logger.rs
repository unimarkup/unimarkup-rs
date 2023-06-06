//! Traits and helper functions for logging functionality.

use std::thread::JoinHandle;

use logid::logging::LOGGER;

use crate::log_id::GeneralInfo;

/// Creates a thread to capture logs set during compilation, and prints them to stdout.
///
/// Set a log with log-id `GeneralInfo::StopLogging` to stop the thread.
pub fn init_log_thread() -> JoinHandle<()> {
    std::thread::spawn(|| {
        if let Ok(recv) = LOGGER.subscribe_to_all_events() {
            while let Ok(log_event) = recv.get_receiver().recv() {
                if log_event.get_id() == &GeneralInfo::StopLogging.into() {
                    break;
                }

                println!(
                    "{}: {}",
                    log_event.get_id().get_log_level(),
                    log_event.get_msg()
                );
            }
        }
        LOGGER.shutdown();
    })
}
