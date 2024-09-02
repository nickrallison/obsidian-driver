use serde::{Deserialize, Serialize};
use crate::config::openai::OpenAIConfig;

const EMBEDDING_MODEL: &str = "text-embedding-3-small";
const SMART_TEXT_MODEL: &str = "gpt-4o";
const CHEAP_TEXT_MODEL: &str = "gpt-4o-mini";
const EMBEDDING_URL: &str = "https://api.openai.com/v1/embeddings";
const CHAT_URL: &str = "https://api.openai.com/v1/chat/completions";


// Example curl command to get embeddings
// curl https://api.openai.com/v1/embeddings \
//   -H "Content-Type: application/json" \
//   -H "Authorization: Bearer $OPENAI_API_KEY" \
//   -d '{
//     "input": "Your text string goes here",
//     "model": "text-embedding-3-small"
//   }'

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenAIDriver {
	config: OpenAIConfig,
}

impl OpenAIDriver {
	pub fn new(config: OpenAIConfig) -> OpenAIDriver {
		config.validate();
		OpenAIDriver { config }
	}
}

pub fn get_embedding(text: &str) -> String {
	// let api
	todo!("Implement get_embedding");
}