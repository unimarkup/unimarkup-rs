use clap::Parser;
use logid::{log, logging::event_entry::AddonKind};
use unimarkup_commons::config::Config;

use crate::log_id::{GeneralError, GeneralInfo};

mod log_id;
mod logger;
mod unimarkup;

fn main() {
    let log_thread = logger::init_log_thread();

    match Config::try_parse() {
        Ok(cfg) => {
            match unimarkup::compile(cfg) {
                Ok(_) => {
                    log!(GeneralInfo::FinishedCompiling);
                }
                Err(error) => {
                    log!(
                        GeneralError::Compile,
                        add: AddonKind::Info(format!("Cause: {:?}", error))
                    );
                }
            };
        }
        Err(error) => {
            log!(
                GeneralError::ArgParse,
                add: AddonKind::Info(
                    format!("Cause:\n\n{}", error.to_string().replace("error: ", "")),
                )
            );
        }
    }

    // Notify log-thread to stop waiting for logs
    log!(GeneralInfo::StopLogging);

    let _ = log_thread.join();
}
