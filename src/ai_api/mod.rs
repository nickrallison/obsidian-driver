use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use openai::{OpenAIDriver, OpenAIConfig};

pub mod openai;
pub(crate) mod prompt;

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
}
