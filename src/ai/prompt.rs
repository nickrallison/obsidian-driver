use std::collections::HashMap;
use std::fmt::Display;
use serde::{Serialize, Deserialize};

use crate::prelude::*;


// Any substrings surrounded by $ will be replaced with the value of the key in the context when prompted
// or return an error if the key is not found
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
	pub system_prompt: String,
	pub user_prompt: String,
	pub max_characters: u32,
}

impl Prompt {
	pub fn new(system_prompt: &str, user_prompt: &str, max_characters: u32) -> Prompt {
		Prompt {
			system_prompt: system_prompt.to_string(),
			user_prompt: user_prompt.to_string(),
			max_characters,
		}
	}
	pub fn substitute(&self, context: &Context) -> Result<Self> {
		let mut system_prompt = self.system_prompt.clone();
		let mut user_prompt = self.user_prompt.clone();
		let mut matches: Vec<String> = Vec::new();
		let pattern = regex::Regex::new(r"\$\w+\$").unwrap();
		for cap in pattern.captures_iter(&system_prompt) {
			matches.push(cap[0].to_string());
		}
		for cap in pattern.captures_iter(&user_prompt) {
			matches.push(cap[0].to_string());
		}
		for m in matches {
			let key = m.trim_matches('$');
			match context.get(key) {
				Some(value) => {
					system_prompt = system_prompt.replace(&m, value);
					user_prompt = user_prompt.replace(&m, value);
				},
				None => return Err(Error::InvalidContextKey(format!("Key not found in context: {}", key)))
			}
		}
		Ok(Prompt {
			system_prompt,
			user_prompt,
			max_characters: self.max_characters,
		})
	}
}

pub struct Context {
	context: HashMap<String, String>,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
	pub fn new() -> Context {
		Context {
			context: HashMap::new(),
		}
	}
	pub fn insert(&mut self, key: &str, value: &str) {
		self.context.insert(key.to_string(), value.to_string());
	}
	pub fn get(&self, key: &str) -> Option<&String> {
		self.context.get(key)
	}
}

impl Display for Prompt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "#### System Prompt ####\n{}\n\n#### User Prompt####\n{}\n\n#### Max Characters ####\n{}", self.system_prompt, self.user_prompt, self.max_characters)
	}
}

#[cfg(test)]
mod prompt_tests {
	use super::*;

	#[test]
	fn test_prompt_new() {
		let prompt = Prompt::new("You are a helpful assistant", "This is a sample prompt", 100);
		assert_eq!(prompt.system_prompt, "You are a helpful assistant");
		assert_eq!(prompt.user_prompt, "This is a sample prompt");
		assert_eq!(prompt.max_characters, 100);
	}

	#[test]
	fn test_prompt_substitube_valid() {
		let prompt = Prompt::new("You are a helpful assistant named $name$", "This is a sample prompt", 100);
		let mut context = Context::new();
		context.insert("name", "Bob");
		let expected = Prompt::new("You are a helpful assistant named Bob", "This is a sample prompt", 100);
		let actual = prompt.substitute(&context).unwrap();
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_prompt_substitube_invalid() {
		let prompt = Prompt::new("You are a helpful assistant named $name$", "This is a sample prompt", 100);
		let context = Context::new();
		let actual = prompt.substitute(&context);
		assert!(actual.is_err());
	}
}