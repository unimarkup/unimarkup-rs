use clap::Parser;
use logid::{event_handler::LogEventHandlerBuilder, log, logging::event_entry::AddonKind};
use unimarkup_commons::config::{Config, ConfigFns};

use crate::log_id::{GeneralError, GeneralInfo};

mod compiler;
mod log_id;

fn main() {
    let log_handler = LogEventHandlerBuilder::new()
        .write_to_console()
        .all_log_events()
        .build();

    'outer: {
        match Config::try_parse() {
            Ok(cfg) => {
                if let Err(err) = cfg.validate() {
                    logid::log!(
                        GeneralError::Compile,
                        &format!("Configuration is invalid: {err}")
                    );
                    break 'outer;
                }

                match compiler::compile(cfg) {
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
    }

    log_handler.shutdown();
}
