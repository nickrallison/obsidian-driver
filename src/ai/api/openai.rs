//! # obsidian-driver::ai::api::openai
//!
//! This module provides a driver for the OpenAI API.
//!
//! @public OpenAIConfig
//! @public OpenAIConfig::from_file
//! @super OpenAIDriver
//! @super OpenAIDriver::new
//! @super OpenAIDriver::new_no_validate
//! @super OpenAIDriver::get_embedding
//! @super OpenAIDriver::chat_smart
//! @super OpenAIDriver::chat_cheap
//! @super OpenAIValidator
//! @super OpenAIValidator::new
//! @super OpenAIValidator::validate
//! @private ChatMessage

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::prelude::*;

/// Driver for the OpenAI API.
///
/// This struct provides a pub(super) internal wrapper for the openai API.
///
/// # Examples
///
/// ```
/// use obsidian_driver::ai::api::openai::{OpenAIConfig};
/// use obsidian_driver::ai::api::AIDriver;
/// use std::path::PathBuf;
///
/// let openai_config_path = PathBuf::from("openai_config.json");
/// let openai_config = OpenAIConfig::from_file(openai_config_path).unwrap();
/// let driver = AIDriver::new_openai(openai_config);
/// ```
/// @super
#[derive(Clone, Debug)]
pub(super) struct OpenAIDriver {
    config: OpenAIConfig,
    client: Client,
}

impl OpenAIDriver {

    /// Internal constructor to create a new OpenAIDriver instance.
    ///
    /// # Arguments
    /// @param config: OpenAIConfig - The configuration for the OpenAI API.
    /// @returns Result<OpenAIDriver> - The new OpenAIDriver instance.
    ///
    /// @super
    pub(super) async fn new(config: OpenAIConfig) -> Result<OpenAIDriver> {
        config.validate().await;
        Ok(OpenAIDriver {
            config,
            client: Client::new(),
        })
    }
    /// Internal constructor to create a new OpenAIDriver instance without validation.
    ///
    /// # Arguments
    /// @param config: OpenAIConfig - The configuration for the OpenAI API.
    /// @returns OpenAIDriver - The new OpenAIDriver instance.
    ///
    /// @super
    pub(super) fn new_no_validate(config: OpenAIConfig) -> OpenAIDriver {
        OpenAIDriver{
            config,
            client: Client::new(),
        }
    }

    /// Get the embedding for a given text.
    ///
    /// # Arguments
    /// @param text: &str - The text to get the embedding for.
    /// @returns Result<Vec<f64>> - The embedding for the text.
    ///
    /// @super
    pub(super) async fn get_embedding(&self, text: &str) -> Result<Vec<f64>> {
        let request_body = serde_json::json!({
            "input": text,
            "model": &self.config.embedding_model,
        });

        let response = self
            .client
            .post(&self.config.embedding_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&request_body)
            .send()
            .await?;

        let response_text = response.text().await?;
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;
        let vec = response_json["data"][0]["embedding"]
            .as_array()
            .ok_or(Error::InvalidEmbeddingResponse(response_text))?
            .iter()
            .map(|v| v.as_f64().unwrap())
            .collect();
        Ok(vec)
    }

    /// Chat with the smart model.
    ///
    /// # Arguments
    /// @param prompt: crate::ai::prompt::Prompt - The prompt to chat with.
    /// @returns Result<String> - The response from the chat.
    ///
    /// @super
    pub(super) async fn chat_smart(&self, prompt: crate::ai::prompt::Prompt) -> Result<String> {
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

        let response = self
            .client
            .post(&self.config.chat_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&request_body)
            .send()
            .await?;

        let response_text = response.text().await?;
        Ok(response_text)
    }

    /// Chat with the cheap model.
    ///
    /// # Arguments
    /// @param prompt: crate::ai::prompt::Prompt - The prompt to chat with.
    /// @returns Result<String> - The response from the chat.
    ///
    /// @super
    pub(super) async fn chat_cheap(&self, prompt: crate::ai::prompt::Prompt) -> Result<String> {
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

        let response = self
            .client
            .post(&self.config.chat_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&request_body)
            .send()
            .await?;

        let response_text = response.text().await?;
        Ok(response_text)
    }
}

/// Configuration for the OpenAI API.
///
/// This struct provides a configuration for the OpenAI API.
///
/// # Examples
/// ```
/// use obsidian_driver::ai::api::openai::OpenAIConfig;
/// use std::path::PathBuf;
///
/// let openai_config_path = PathBuf::from("openai_config.json");
/// let openai_config = OpenAIConfig::from_file(openai_config_path).unwrap();
/// ```
///
/// ```
/// use obsidian_driver::ai::api::openai::OpenAIConfig;
///
/// let openai_config = OpenAIConfig {
///     validation_url: "https://api.openai.com/v1/models".to_string(),
///     embedding_model: "text-embedding-3-small".to_string(),
///     smart_text_model: "gpt-4o".to_string(),
///     cheap_text_model: "gpt-4o-mini".to_string(),
///     smart_model_max_tokens: 128,
///     cheap_model_max_tokens: 128,
///     embedding_url: "https://api.openai.com/v1/embeddings".to_string(),
///     chat_url: "https://api.openai.com/v1/chat/completions".to_string(),
///     api_key: "sk-...".to_string(),
///     characters_per_token: 4,
/// };
/// ```
///
/// @public
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
    /// Validate the OpenAIConfig.
    ///
    /// # Arguments
    /// @returns Result<()> - The result of the validation.
    ///
    /// @private
    async fn validate(&self) -> Result<()> {
        let validator = OpenAIValidator::new(self.clone());
        validator.validate().await
    }

    /// Create an OpenAIConfig from a file.
    ///
    /// # Arguments
    /// @param config_path: PathBuf - The path to the configuration file.
    /// @returns Result<OpenAIConfig> - The OpenAIConfig from the file.
    ///
    /// @public
    pub fn from_file(config_path: PathBuf) -> Result<OpenAIConfig> {
        let config_file = std::fs::File::open(config_path)?;
        let config: OpenAIConfig = serde_json::from_reader(config_file)?;
        Ok(config)
    }
}

/// Validator for the OpenAI API.
///
/// This struct provides a validator for the OpenAI API.
///
/// @super
pub(super) struct OpenAIValidator {
    config: OpenAIConfig,
    client: Client,
}

impl OpenAIValidator {

    /// Internal constructor to create a new OpenAIValidator instance.
    ///
    /// # Arguments
    /// @param config: OpenAIConfig - The configuration for the OpenAI API.
    /// @returns OpenAIValidator - The new OpenAIValidator instance.
    ///
    /// @super
    pub(super) fn new(config: OpenAIConfig) -> OpenAIValidator {
        OpenAIValidator {
            config,
            client: Client::new(),
        }
    }

    /// Validate the OpenAI API.
    ///
    /// # Arguments
    /// @returns Result<()> - The result of the validation.
    ///
    /// @super
    pub(super) async fn validate(&self) -> Result<()> {
        let response = self
            .client
            .get(&self.config.validation_url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?;

        let response_text = response.text().await?;
        panic!("{}", response_text);
    }
}

/// Chat message for the OpenAI API.
///
/// This struct provides a chat message for the OpenAI API.
/// Used for serialization and deserialization.
///
/// @private
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}
