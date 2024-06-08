use thiserror::Error;

pub type BattlenetClientResult<T> = Result<T, BattlenetClientError>;

#[derive(Error, Debug)]
pub enum BattlenetClientError {
    /// Represents any reqwest that has failed, propagating the error context.
    #[error("{0}")]
    ClientRequestFailed(#[from] reqwest::Error),

    /// Represents any serde_json error that has failed, propagating the error context.
    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),

    /// Client token is not set (or something weird happened)
    #[error("client token is not available")]
    ClientTokenNotAvailable,

    /// Error accessing client token mutex.
    #[error("error accessing client token mutex: {0}")]
    ClientTokenMutex(String),

    /// An error, but we don't know what it is (should only be used during dev)
    #[error("unknown error")]
    Unknown,
}
