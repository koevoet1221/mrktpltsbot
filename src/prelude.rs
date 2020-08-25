use std::error::Error;
pub use std::time::Duration;

pub use log::{debug, error, info, warn};
pub use serde::{Deserialize, Serialize};
pub use structopt::clap::crate_version;

pub use crate::client::CLIENT;

pub type Result<T = (), E = Box<dyn Error>> = std::result::Result<T, E>;
