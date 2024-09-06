//! Main Crate Error

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Generic Error:\n{0}")]
	Generic(String),

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

}