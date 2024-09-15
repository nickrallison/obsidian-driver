//! Main Crate Error
//!
//! This module contains the Error type and its implementations.
//! 
//! @public Error

// std imports
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic Error:\n{0}")]
    Generic(String),

    #[error("Invalid Context Key:\n{0}")]
    InvalidContextKey(String),

    #[error("Invalid Embedding Response:\n{0}")]
    InvalidEmbeddingResponse(String),

    #[error("Prompt Exceeds Model Token Limit:\n{0}")]
    PromptExceedsModelTokenLimit(crate::ai::prompt::Prompt),

    #[error("Vault Already Contains Path:\n{0}")]
    VaultAlreadyContainsPath(PathBuf),

    #[error("Path: {0}\nNot In Vault Root:\n{0}")]
    PathNotInVaultRoot(PathBuf, PathBuf),

    #[error("No AI Driver Provided")]
    NoAIDriver,

    // Transparent Errors
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    SysTime(#[from] std::time::SystemTimeError),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    
    #[error(transparent)]
    StripPrefixError(#[from] std::path::StripPrefixError),
    
	#[error(transparent)]
	OpenAIValidationError(#[from] crate::ai::api::openai::OpenAIValidationError),

    #[error(transparent)]
    WalkDirError(#[from] walkdir::Error),
}
