use thiserror::Error;

/// Custom errors for the SDK.
#[derive(Error, Debug)]
pub enum SdkError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error), // Wraps reqwest errors.

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error), // Wraps serde errors.
}