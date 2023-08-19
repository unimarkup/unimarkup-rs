use clap::Parser;
use logid::{
    event_handler::builder::LogEventHandlerBuilder,
    log,
    log_id::LogLevel,
    logging::{
        event_entry::AddonKind,
        filter::{AddonFilter, FilterConfigBuilder},
    },
};
use unimarkup_commons::config::{Config, ConfigFns};

use crate::log_id::{GeneralError, GeneralInfo};

mod compiler;
mod log_id;

fn main() {
    let _ = logid::logging::filter::set_filter(
        FilterConfigBuilder::new(LogLevel::Info)
            .allowed_addons(AddonFilter::Infos)
            .build(),
    );

    let _handler = LogEventHandlerBuilder::new()
        .to_stderr()
        .all_log_events()
        .build();

    'outer: {
        match Config::try_parse() {
            Ok(cfg) => {
                if let Err(err) = cfg.validate() {
                    logid::log!(
                        GeneralError::Compile,
                        format!("Configuration is invalid: {err}")
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
}
