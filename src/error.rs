use thiserror::Error;

#[derive(Error, Debug)]
pub enum PigeonError {
    #[cfg(feature = "binrw")]
    #[error("Binary parse error: {0}")]
    BinError(#[from] binrw::Error),
    #[cfg(feature = "json")]
    #[error("Serde JSON parse error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Tokio error: {0}")]
    TokioError(#[from] tokio::io::Error),
}