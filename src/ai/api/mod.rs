use std::path::PathBuf;
use openai::{OpenAIDriver, OpenAIConfig};

mod openai;

use crate::prelude::*;

#[derive(Clone, Debug)]
pub enum AIDriver {
	OpenAI(OpenAIDriver),
}

impl AIDriver {
	pub fn new_openai(config: OpenAIConfig) -> AIDriver {
		AIDriver::OpenAI(OpenAIDriver::new(config))
	}
	pub fn new_openai_from_config_path(config_path: PathBuf) -> AIDriver {
		let config = OpenAIConfig::from_file(config_path);
		AIDriver::OpenAI(OpenAIDriver::new(config))
	}

	pub async fn chat_smart(&self, prompt: super::prompt::Prompt) -> Result<String> {
		match self {
			AIDriver::OpenAI(driver) => driver.chat_smart(prompt).await,
		}
	}

	pub async fn chat_cheap(&self, prompt: super::prompt::Prompt) -> Result<String> {
		match self {
			AIDriver::OpenAI(driver) => driver.chat_cheap(prompt).await,
		}
	}
}
