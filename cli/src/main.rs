use clap::Parser;
use logid::{event_handler::LogEventHandlerBuilder, log, logging::event_entry::AddonKind};
use unimarkup_commons::config::{Config, ConfigFns};

use crate::log_id::{GeneralError, GeneralInfo};

mod compile;
mod log_id;

fn main() {
    let log_handler = LogEventHandlerBuilder::new()
        .write_to_console()
        .all_log_events()
        .build();

    match Config::try_parse() {
        Ok(cfg) => {
            cfg.validate().unwrap();

            match compile::compile(cfg) {
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

    log_handler.shutdown();
}
