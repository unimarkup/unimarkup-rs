use std::io::Write;

use clap::Parser;
use log::LevelFilter;
use unimarkup_rs::{config::Config, unimarkup};

fn main() {
    env_logger::builder()
        .format_level(true)
        .filter_level(LevelFilter::Info)
        .format_timestamp(None)
        .format_target(false)
        .format_module_path(false)
        .format_suffix("")
        // .format(|buf, record| {
        //     // buf.write_fmt(format_args!("{}: {}\n", record.level(), record.args()))
        //     writeln!(buf, "{}", record.level().)
        // })
        .init();

    let config = Config::parse();

    match unimarkup::compile(config) {
        Ok(_) => println!("Done"),
        Err(err) => println!("Error: {}", err),
    }
}
