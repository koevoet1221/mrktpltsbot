use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("re-authenticate")]
    Reauthenticate,

    #[error("request error: {0:#}")]
    #[expect(clippy::enum_variant_names)]
    RequestError(#[from] reqwest::Error),

    #[error("request error: {0:#}")]
    #[expect(clippy::enum_variant_names)]
    RequestMiddlewareError(#[from] reqwest_middleware::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}
