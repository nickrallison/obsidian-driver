//! Main Crate Error

use crate::ai_api::prompt::Prompt;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Generic Error:\n{0}")]
	Generic(String),

	// Specific Errors for application
	// #[error("Invalid Config File:\n{0}")]
	// InvalidConfigFile(String),

	#[error("Prompt Exceeds Model Token Limit:\n{0}")]
	PromptExceedsModelTokenLimit(Prompt),

	// Transparent Errors
	#[error(transparent)]
	IO(#[from] std::io::Error),

	#[error(transparent)]
	Reqwest(#[from] reqwest::Error),

}