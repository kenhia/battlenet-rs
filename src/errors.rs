use thiserror::Error;

pub type BattlenetClientResult<T> = Result<T, BattleNetClientError>;

#[derive(Error, Debug)]
pub enum BattleNetClientError {
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

    /// User token key not found in Redis (expired or never set).
    #[error("user token is not available")]
    UserTokenNotAvailable,

    /// Redis connection or command failure.
    #[cfg(feature = "redis")]
    #[error("{0}")]
    RedisError(#[from] redis::RedisError),

    /// An error, but we don't know what it is (should only be used during dev)
    #[error("unknown error")]
    Unknown,
}
