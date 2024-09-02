use serde::{Deserialize, Serialize};

use crate::ai_api::prompt::Prompt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenAIConfig {
	// Models
	pub embedding_model: String,
	pub smart_text_model: String,
	pub cheap_text_model: String,

	// Urls
	pub embedding_url: String,
	pub chat_url: String,

	// API key
	pub api_key: String,

	// Prompts
	pub system_prompt: Prompt,
	pub user_prompt: Prompt
}

impl OpenAIConfig {
	pub fn validate(&self) {
		todo!("Implement validate for OpenAIConfig")
	}
}