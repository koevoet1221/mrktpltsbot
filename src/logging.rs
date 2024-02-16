use log::LevelFilter;
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

use crate::prelude::*;

/// Initialize logging.
pub fn init() -> Result {
    let config = ConfigBuilder::new()
        .set_thread_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Error)
        .set_location_level(LevelFilter::Debug)
        .set_time_level(LevelFilter::Off)
        .build();
    TermLogger::init(
        LevelFilter::Info,
        config,
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;
    Ok(())
}

pub fn log_result<T>(result: Result<T>) {
    if let Err(error) = result {
        log_error(error);
    }
}

pub fn log_error<E: Into<anyhow::Error> + Send + 'static>(error: E) {
    async_std::task::spawn(async move {
        let error = error.into();
        error!("{}, Sentry ID: {}", error, capture_anyhow(&error));
    });
}
