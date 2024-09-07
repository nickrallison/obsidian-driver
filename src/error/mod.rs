//! Main Crate Error

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic Error:\n{0}")]
    Generic(String),

    #[error("Invalid Context Key:\n{0}")]
    InvalidContextKey(String),

    #[error("Invalid Embedding Response:\n{0}")]
    InvalidEmbeddingResponse(String),

    // Specific Errors for application
    // #[error("Invalid Config File:\n{0}")]
    // InvalidConfigFile(String),
    #[error("Prompt Exceeds Model Token Limit:\n{0}")]
    PromptExceedsModelTokenLimit(crate::ai::prompt::Prompt),

    // Transparent Errors
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    SysTime(#[from] std::time::SystemTimeError),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}
