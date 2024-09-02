use std::fmt::Display;
use serde::{Serialize, Deserialize};
use serde_json;

// pub trait Prompt {
// 	fn get_system_prompt(&self) -> String;
// 	fn get_user_prompt(&self) -> String;
// }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
	pub system_prompt: String,
	pub user_prompt: String,
	pub max_characters: u32,
}

impl Display for Prompt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "#### System Prompt ####\n{}\n\n#### User Prompt####\n{}\n\n#### Max Characters ####\n{}", self.system_prompt, self.user_prompt, self.max_characters)
	}
}