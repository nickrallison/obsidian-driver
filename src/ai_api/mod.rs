use serde::{Deserialize, Serialize};
use openai::OpenAIDriver;
use crate::config::openai::OpenAIConfig;

mod openai;
pub(crate) mod prompt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

pub enum AIDriver {
	OpenAI(OpenAIDriver),
}

impl AIDriver {

	fn new_openai(config: OpenAIConfig) -> AIDriver {
		AIDriver::OpenAI(OpenAIDriver::new(config))
	}
}
