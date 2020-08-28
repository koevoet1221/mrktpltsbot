use crate::prelude::*;
use log::LevelFilter;
use simplelog::{ConfigBuilder, TermLogger, TerminalMode};

/// Initialize logging.
pub fn init() -> Result {
    let mut config_builder = ConfigBuilder::new();
    config_builder
        .set_thread_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
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

pub fn log_result<T>(result: Result<T>) {
    if let Err(error) = result {
        async_std::task::spawn(async move {
            let uuid = capture_anyhow(&error);
            error!("{}, Sentry ID: {}", error, uuid);
        });
    }
}
