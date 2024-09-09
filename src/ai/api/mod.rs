use openai::{OpenAIConfig, OpenAIDriver};
use std::path::PathBuf;

pub mod openai;

use crate::prelude::*;

#[derive(Clone, Debug)]
pub enum AIDriver {
    OpenAI(OpenAIDriver),
}

impl AIDriver {
    pub async fn new_openai(config: OpenAIConfig) -> Result<AIDriver> {
        Ok(AIDriver::OpenAI(OpenAIDriver::new(config).await?))
    }
    pub async fn new_openai_from_config_path(config_path: PathBuf) -> Result<AIDriver> {
        let config = OpenAIConfig::from_file(config_path)?;
        Ok(AIDriver::OpenAI(OpenAIDriver::new(config).await?))
    }

	pub fn new_openai_no_validation(config: OpenAIConfig) -> AIDriver {
		AIDriver::OpenAI(OpenAIDriver::new_no_validate(config))
	}
	pub fn new_openai_from_config_path_no_validation(config_path: PathBuf) -> Result<AIDriver> {
		let config = OpenAIConfig::from_file(config_path)?;
		Ok(AIDriver::OpenAI(OpenAIDriver::new_no_validate(config)))
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
    pub async fn get_embedding(&self, text: &str) -> Result<Vec<f64>> {
        match self {
            AIDriver::OpenAI(driver) => driver.get_embedding(text).await,
        }
    }
}
