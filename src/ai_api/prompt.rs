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
}