use crate::prelude::*;

pub trait ResultExtensions<T> {
    fn log_result(self);
}

impl<T> ResultExtensions<T> for Result<T> {
    fn log_result(self) {
        log_result(self);
    }
}
