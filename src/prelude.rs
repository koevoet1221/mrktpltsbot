pub use log::{debug, error, info, warn};
use std::error::Error;

pub type Result<T = (), E = Box<dyn Error>> = std::result::Result<T, E>;
