use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::ai_api::prompt::Prompt;
use reqwest::Client;

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct OpenAIDriver {
	config: OpenAIConfig,
	client: Client,
}

impl OpenAIDriver {
	pub fn new(config: OpenAIConfig) -> OpenAIDriver {
		config.validate();
		OpenAIDriver {
			config,
			client: Client::new(),
		}
	}

	pub async fn get_embedding(&self, text: &str) -> Result<String> {
		let request_body = serde_json::json!({
            "input": text,
            "model": &self.config.embedding_model,
        });

		let response = self.client.post(&self.config.embedding_url)
			.header("Content-Type", "application/json")
			.header("Authorization", format!("Bearer {}", self.config.api_key))
			.json(&request_body)
			.send()
			.await?;

		let response_text = response.text().await?;
		Ok(response_text)
	}
	pub async fn chat_smart(&self, prompt: Prompt) -> Result<String> {
		let tokens = prompt.max_characters / self.config.characters_per_token;
		if tokens > self.config.smart_model_max_tokens {
			return Err(Error::PromptExceedsModelTokenLimit(prompt));
		}
		let request_body = serde_json::json!({
			"model": &self.config.smart_text_model,
			"messages": vec![
				ChatMessage {
					role: "system".to_string(),
					content: prompt.system_prompt,
				},
				ChatMessage {
					role: "user".to_string(),
					content: prompt.user_prompt,
				},
			],
			"max_tokens": tokens,
		});

		let response = self.client.post(&self.config.chat_url)
			.header("Content-Type", "application/json")
			.header("Authorization", format!("Bearer {}", self.config.api_key))
			.json(&request_body)
			.send()
			.await?;

		let response_text = response.text().await?;
		Ok(response_text)
	}

	pub async fn chat_cheap(&self, prompt: Prompt) -> Result<String> {
		let tokens = prompt.max_characters / self.config.characters_per_token;
		if tokens > self.config.cheap_model_max_tokens {
			return Err(Error::PromptExceedsModelTokenLimit(prompt));
		}
		let request_body = serde_json::json!({
			"model": &self.config.cheap_text_model,
			"messages": vec![
				ChatMessage {
					role: "system".to_string(),
					content: prompt.system_prompt,
				},
				ChatMessage {
					role: "user".to_string(),
					content: prompt.user_prompt,
				},
			],
			"max_tokens": tokens,
		});

		let response = self.client.post(&self.config.chat_url)
			.header("Content-Type", "application/json")
			.header("Authorization", format!("Bearer {}", self.config.api_key))
			.json(&request_body)
			.send()
			.await?;

		let response_text = response.text().await?;
		Ok(response_text)
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenAIConfig {
	// Validation
	pub validation_url: String,

	// Models
	pub embedding_model: String,
	pub smart_text_model: String,
	pub cheap_text_model: String,

	pub smart_model_max_tokens: u32,
	pub cheap_model_max_tokens: u32,

	// Urls
	pub embedding_url: String,
	pub chat_url: String,

	// API key
	pub api_key: String,

	// Other
	pub characters_per_token: u32,

}

impl OpenAIConfig {
	// Example curl command to list models, used as a validation step
	// curl https://api.openai.com/v1/models \
	//   -H "Authorization: Bearer $OPENAI_API_KEY" \
	pub async fn validate(&self) -> Result<()> {
		let validator = OpenAIValidator::new(self.clone());
		validator.validate().await
	}

	pub(crate) fn from_file(config_path: PathBuf) -> OpenAIConfig {
		let config_file = std::fs::File::open(config_path).unwrap();
		let config: OpenAIConfig = serde_json::from_reader(config_file).unwrap();
		config
	}
}

pub struct OpenAIValidator {
	config: OpenAIConfig,
	client: Client,
}

impl OpenAIValidator {
	pub fn new(config: OpenAIConfig) -> OpenAIValidator {
		OpenAIValidator {
			config,
			client: Client::new(),
		}
	}

	pub async fn validate(&self) -> Result<()> {
		let response = self.client.get(&self.config.validation_url)
			.header("Authorization", format!("Bearer {}", self.config.api_key))
			.send()
			.await?;

		let response_text = response.text().await?;
		panic!("{}", response_text);
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ChatMessage {
	role: String,
	content: String,
}

