//! # obsidian-driver::ai::api
//!
//! This module contains the interface for interacting with external AI models through their APIs. The AI models are used to generate content for the Obsidian Driver.
//!
//! @public openai
//!
//! @public AIDriver
//!
//! @public AIDriver::new_openai
//!
//! @public AIDriver::new_openai_from_config_path
//!
//! @public AIDriver::new_openai_no_validation
//!
//! @public AIDriver::new_openai_from_config_path_no_validation
//!
//! @public AIDriver::chat_smart
//!
//! @public AIDriver::chat_cheap
//!
//! @public AIDriver::get_embedding

// std imports
use std::path::PathBuf;

// third-party imports
use openai::{OpenAIConfig, OpenAIDriver};

// first-party imports
use crate::prelude::*;

// mod imports
pub mod openai;

/// The AI Driver enum.
///
/// This enum provides a high-level interface to the AI models.
///
/// # Examples
/// ```
/// use obsidian_driver::ai::api::{AIDriver, openai::OpenAIConfig};
/// use std::path::PathBuf;
///
/// async fn new_openai_example() {
/// 	let openai_config_path = PathBuf::from(".openai_config.json");
/// 	let openai_config = OpenAIConfig::from_file(openai_config_path).unwrap();
/// 	let driver = AIDriver::new_openai(openai_config).await;
/// }
/// ```
///
/// ```
/// use obsidian_driver::ai::api::{AIDriver, openai::OpenAIConfig};
/// use std::path::PathBuf;
/// 
///async fn new_openai_from_config_path_example() {
/// 	let openai_config_path = PathBuf::from(".openai_config.json");
/// 	let driver = AIDriver::new_openai_from_config_path(openai_config_path).await;
/// }
/// ```
/// @public
#[derive(Clone, Debug)]
pub enum AIDriver {
    OpenAI(OpenAIDriver),
}

impl AIDriver {
	/// This function creates a new OpenAI AIDriver from an OpenAIConfig.
	///
	/// # Arguments
	/// @param `config`: `OpenAIConfig` - The configuration for the OpenAI API.
	/// @returns `Result<AIDriver>` - The new AIDriver.
	///
	/// @public
	///
	/// # Examples
	/// ```
	/// use obsidian_driver::ai::api::{AIDriver, openai::OpenAIConfig};
	/// use std::path::PathBuf;
	///
	/// async fn new_openai_example() {
	/// 	let openai_config_path = PathBuf::from(".openai_config.json");
	/// 	let openai_config = OpenAIConfig::from_file(openai_config_path).unwrap();
	/// 	let driver = AIDriver::new_openai(openai_config).await;
	/// }
	/// ```
	/// @public
    pub async fn new_openai(config: OpenAIConfig) -> Result<AIDriver> {
        Ok(AIDriver::OpenAI(OpenAIDriver::new(config).await?))
    }

	/// This function creates a new OpenAI AIDriver from a config file.
	///
	/// # Arguments
	/// @param `config_path`: `PathBuf` - The path to the OpenAIConfig file.
	/// @returns `Result<AIDriver>` - The new AIDriver.
	///
	/// # Examples
	/// ```
	/// use obsidian_driver::ai::api::{AIDriver, openai::OpenAIConfig};
	/// use std::path::PathBuf;
	///
	/// async fn new_openai_from_config_path_example() {
	/// 	let openai_config_path = PathBuf::from(".openai_config.json");
	/// 	let driver = AIDriver::new_openai_from_config_path(openai_config_path).await;
	/// }
	/// ```
	/// @public
    pub async fn new_openai_from_config_path(config_path: PathBuf) -> Result<AIDriver> {
        let config = OpenAIConfig::from_file(config_path)?;
        Ok(AIDriver::OpenAI(OpenAIDriver::new(config).await?))
    }

	/// This function creates a new OpenAI AIDriver from an OpenAIConfig without validation.
	///
	/// # Arguments
	/// @param `config`: `OpenAIConfig` - The configuration for the OpenAI API.
	/// @returns `AIDriver` - The new AIDriver.
	///
	/// # Examples
	/// ```should_panic
	/// use obsidian_driver::ai::api::{AIDriver, openai::OpenAIConfig};
	/// use std::path::PathBuf;
	///
	/// let openai_config_path = PathBuf::from(".openai_config.json");
	/// let openai_config = OpenAIConfig::from_file(openai_config_path).unwrap();
	/// let driver = AIDriver::new_openai_no_validation(openai_config);
	/// ```
	/// @public
	pub fn new_openai_no_validation(config: OpenAIConfig) -> AIDriver {
		AIDriver::OpenAI(OpenAIDriver::new_no_validate(config))
	}

	/// This function creates a new OpenAI AIDriver from a config file without validation.
	///
	/// # Arguments
	/// @param `config_path`: `PathBuf` - The path to the OpenAIConfig file.
	/// @returns `AIDriver` - The new AIDriver.
	///
	/// # Examples
	/// ```
	/// use obsidian_driver::ai::api::{AIDriver, openai::OpenAIConfig};
	/// use std::path::PathBuf;
	///
	/// let openai_config_path = PathBuf::from(".openai_config.json");
	/// let driver = AIDriver::new_openai_from_config_path_no_validation(openai_config_path);
	/// ```
	/// @public
	pub fn new_openai_from_config_path_no_validation(config_path: PathBuf) -> Result<AIDriver> {
		let config = OpenAIConfig::from_file(config_path)?;
		Ok(AIDriver::OpenAI(OpenAIDriver::new_no_validate(config)))
	}

	/// This function sends a prompt to the smart AI model and returns the response.
	///
	/// # Arguments
	/// @param `prompt`: `Prompt` - The prompt to send to the AI model.
	/// @returns `Result<String>` - The response from the AI model.
	///
	/// # Examples
	/// ```
	/// use obsidian_driver::ai::api::{AIDriver, openai::OpenAIConfig};
	/// use obsidian_driver::ai::prompt::Prompt;
	/// use std::path::PathBuf;
	///
	/// async fn chat_smart_example() {
	/// 	let openai_config_path = PathBuf::from(".openai_config.json");
	/// 	let driver = AIDriver::new_openai_from_config_path(openai_config_path).await.unwrap();
	///
	/// 	let prompt = Prompt::new("You are a helpful assistant", "Provide a good morning message", 128);
	/// 	let response = driver.chat_smart(prompt).await.unwrap();
	/// }
	/// ```
	/// @public
    pub async fn chat_smart(&self, prompt: super::prompt::Prompt) -> Result<String> {
        match self {
            AIDriver::OpenAI(driver) => driver.chat_smart(prompt).await,
        }
    }

	/// This function sends a prompt to the cheap AI model and returns the response.
	///
	/// # Arguments
	/// @param `prompt`: `Prompt` - The prompt to send to the AI model.
	/// @returns `Result<String>` - The response from the AI model.
	///
	/// # Examples
	/// ```
	/// use obsidian_driver::ai::api::{AIDriver, openai::OpenAIConfig};
	/// use obsidian_driver::ai::prompt::Prompt;
	/// use std::path::PathBuf;
	///
	/// async fn chat_cheap_example() {
	/// 	let openai_config_path = PathBuf::from(".openai_config.json");
	/// 	let driver = AIDriver::new_openai_from_config_path(openai_config_path).await.unwrap();
	///
	/// 	let prompt = Prompt::new("You are a helpful assistant", "Provide a good morning message", 128);
	/// 	let response = driver.chat_cheap(prompt).await.unwrap();
	/// }
	/// ```
	/// @public
    pub async fn chat_cheap(&self, prompt: super::prompt::Prompt) -> Result<String> {
        match self {
            AIDriver::OpenAI(driver) => driver.chat_cheap(prompt).await,
        }
    }
	
	/// This function gets the embedding for a given text.
	/// 
	/// # Arguments
	/// @param `text`: `&str` - The text to get the embedding for.
	/// @returns `Result<Vec<f64>>` - The embedding for the text.
	/// 
	/// # Examples
	/// ```
	/// use obsidian_driver::ai::api::{AIDriver, openai::OpenAIConfig};
	/// use std::path::PathBuf;
	/// 
	/// async fn get_embedding_example() {
	/// 	let openai_config_path = PathBuf::from(".openai_config.json");
	/// 	let driver = AIDriver::new_openai_from_config_path(openai_config_path).await.unwrap();
	/// 
	/// 	let text = "Hello, World!";
	/// 
	/// 	let embedding = driver.get_embedding(text).await.unwrap();
	/// }
	/// ```
	/// @public
    pub async fn get_embedding(&self, text: &str) -> Result<Vec<f64>> {
        match self {
            AIDriver::OpenAI(driver) => driver.get_embedding(text).await,
        }
    }
}
