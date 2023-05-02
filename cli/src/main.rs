use std::collections::HashSet;

use clap::Parser;
use log_id::CLI_LOG_ID_MAP;
use logid::{capturing::LogIdTracing, log_id::LogId};
use unimarkup_commons::config::OutputFormat;
use unimarkup_core::config::{Config, OutputFormat as CoreOutputFormat};

use crate::log_id::{GeneralErrLogId, GeneralInfLogId};

mod log_id;
mod logger;
mod unimarkup;

fn main() {
    logger::init_logger();

    match Config::try_parse() {
        Ok(config) => {
            let mut cfg: unimarkup_commons::config::Config = unimarkup_commons::config::Config {
                input: config.um_file,
                ..Default::default()
            };

            let out_formats: Vec<_> = config
                .out_formats
                .map(|formats| {
                    formats
                        .iter()
                        .filter_map(|format| {
                            if matches!(format, CoreOutputFormat::Html) {
                                Some(OutputFormat::Html)
                            } else {
                                None
                            }
                        })
                        .collect()
                })
                .unwrap_or_default();

            cfg.preamble.output.formats = HashSet::from_iter(out_formats.into_iter());
            cfg.preamble.output.file = config.out_file;

            let input = cfg.input.clone();

            match unimarkup::compile(cfg) {
                Ok(_) => {
                    (GeneralInfLogId::FinishedCompiling as LogId).set_event_with(
                        &CLI_LOG_ID_MAP,
                        &format!("Finished compiling: {:?}", input),
                        file!(),
                        line!(),
                    );
                }
                Err(err) => {
                    (GeneralErrLogId::FailedCompiling as LogId)
                        .set_event_with(
                            &CLI_LOG_ID_MAP,
                            &format!("Failed compiling: {:?}", input),
                            file!(),
                            line!(),
                        )
                        .add_info(&format!("Cause: {:?}", err));
                }
            };
        }
        Err(error) => {
            (GeneralErrLogId::FailedParsingArgs as LogId)
                .set_event_with(
                    &CLI_LOG_ID_MAP,
                    "Failed parsing comandline arguments!",
                    file!(),
                    line!(),
                )
                .add_info(&format!(
                    "Cause: {}",
                    error.to_string().replace("error: ", "")
                ));
        }
    }
}
