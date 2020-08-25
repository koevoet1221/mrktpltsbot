use crate::prelude::*;
use log::LevelFilter;
use simplelog::{ConfigBuilder, TermLogger, TerminalMode};

pub fn init() -> Result {
    let mut config_builder = ConfigBuilder::new();
    config_builder
        .set_thread_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Error)
        .set_location_level(LevelFilter::Debug)
        .set_time_format_str("%m-%d %T%.3f")
        .set_time_to_local(true);
    TermLogger::init(
        LevelFilter::Info,
        config_builder.build(),
        TerminalMode::Stderr,
    )?;
    Ok(())
}
