//! # obsidian-driver::ai::prompt
//!
//! This module contains the Prompt struct and the Context struct.
//!
//! @public Prompt
//!
//! @public Prompt::new
//!
//! @public Prompt::substitute
//!
//! @public Context
//!
//! @public Context::default
//!
//! @public Context::from<HashMap<String, String>>
//!
//! @public Context::insert
//!
//! @public Context::get

// std imports
use std::collections::HashMap;
use std::fmt::Display;

// third-party imports
use serde::{Serialize, Deserialize};

// first-party imports
use crate::prelude::*;


// Any substrings surrounded by $ will be replaced with the value of the key in the context when prompted
// or return an error if the key is not found

/// The Prompt struct.
///
/// This struct contains the system prompt, user prompt, and the maximum number of characters allowed in the response.
///
/// # Examples
/// ```
/// use obsidian_driver::ai::prompt::Prompt;
///
/// let prompt = Prompt::new("You are a helpful assistant", "This is a sample prompt", 100);
/// ```
///
/// ```
/// use obsidian_driver::ai::prompt::{Prompt, Context};
///
/// let prompt = Prompt::new("You are a helpful assistant named $name$", "This is a sample prompt", 100);
/// let mut context = Context::default();
/// context.insert("name", "Bob");
///
/// let actual = prompt.substitute(&context).unwrap();
/// let expected = Prompt::new("You are a helpful assistant named Bob", "This is a sample prompt", 100);
/// assert_eq!(actual, expected);
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
	pub system_prompt: String,
	pub user_prompt: String,
	pub max_characters: Option<u32>,
}

impl Prompt {
	/// Create a new Prompt.
	///
	/// # Arguments
	/// @param system_prompt: &str - The system prompt.
	/// @param user_prompt: &str - The user prompt.
	/// @param max_characters: u32 - The maximum number of characters allowed in the response.
	/// @returns Prompt - The new Prompt.
	///
	/// # Examples
	/// ```
	/// use obsidian_driver::ai::prompt::Prompt;
	///
	/// let prompt = Prompt::new("You are a helpful assistant", "This is a sample prompt", 100);
	/// ```
	pub fn new(system_prompt: &str, user_prompt: &str, max_characters: Option<u32>) -> Prompt {
		Prompt {
			system_prompt: system_prompt.to_string(),
			user_prompt: user_prompt.to_string(),
			max_characters,
		}
	}

	/// Substitute the keys in the prompt with the values in the context.
	///
	/// # Arguments
	/// @param context: &Context - The context to substitute the keys with.
	/// @returns Result<Prompt> - The new Prompt with the keys substituted.
	///
	/// # Examples
	/// ```
	/// use obsidian_driver::ai::prompt::{Prompt, Context};
	///
	/// let prompt = Prompt::new("You are a helpful assistant named $name$", "This is a sample prompt", 100);
	/// let mut context = Context::default();
	/// context.insert("name", "Bob");
	///
	/// let actual = prompt.substitute(&context).unwrap();
	/// let expected = Prompt::new("You are a helpful assistant named Bob", "This is a sample prompt", 100);
	/// assert_eq!(actual, expected);
	/// ```
	///
	/// ```
	/// use obsidian_driver::ai::prompt::{Prompt, Context};
	///
	/// let prompt = Prompt::new("You are a helpful $profession$ named $name$", "This is a sample prompt for someone who's job is a(n) $profession$", 100);
	/// let mut context = Context::default();
	/// context.insert("name", "Bob");
	/// context.insert("profession", "assistant");
	///
	/// let actual = prompt.substitute(&context).unwrap();
	/// let expected = Prompt::new("You are a helpful assistant named Bob", "This is a sample prompt for someone who's job is a(n) assistant", 100);
	/// assert_eq!(actual, expected);
	/// ```
	pub fn substitute(&self, context: &Context) -> Result<Self> {
		let mut system_prompt = self.system_prompt.clone();
		let mut user_prompt = self.user_prompt.clone();
		let mut matches: Vec<String> = Vec::new();
		let pattern = regex::Regex::new(r"\[\w+\]").unwrap();
		for cap in pattern.captures_iter(&system_prompt) {
			matches.push(cap[0].to_string());
		}
		for cap in pattern.captures_iter(&user_prompt) {
			matches.push(cap[0].to_string());
		}
		for m in matches {
			let key = m.trim_start_matches('[').trim_end_matches(']');
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

/// The Context struct.
///
/// This struct contains a hashmap of keys and values.
///
/// # Examples
/// ```
/// use obsidian_driver::ai::prompt::Context;
///
/// let context = Context::default();
/// ```
///
/// ```
/// use obsidian_driver::ai::prompt::Context;
///
/// let mut context = Context::default();
/// context.insert("name", "Bob");
/// let name = context.get("name").unwrap();
/// ```
///
/// ```
/// use obsidian_driver::ai::prompt::Context;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("name".to_string(), "Bob".to_string());
/// let context = Context::from(map);
/// ```
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Context {
	context: HashMap<String, String>,
}


impl Context {

	/// Insert a key-value pair into the context.
	///
	/// # Arguments
	/// @param key: &str - The key to insert.
	/// @param value: &str - The value to insert.
	///
	/// # Examples
	/// ```
	/// use obsidian_driver::ai::prompt::Context;
	///
	/// let mut context = Context::default();
	/// context.insert("name", "Bob");
	/// ```
	pub fn insert(&mut self, key: &str, value: &str) {
		self.context.insert(key.to_string(), value.to_string());
	}

	/// Get the value of a key in the context.
	///
	/// # Arguments
	/// @param key: &str - The key to get the value of.
	/// @returns Option<&String> - The value of the key.
	///
	/// # Examples
	///
	/// ```
	/// use obsidian_driver::ai::prompt::Context;
	///
	/// let mut context = Context::default();
	/// context.insert("name", "Bob");
	///
	/// let name = context.get("name").unwrap();
	/// ```
	pub fn get(&self, key: &str) -> Option<&String> {
		self.context.get(key)
	}
}

impl From<HashMap<String, String>> for Context {
	fn from(context: HashMap<String, String>) -> Self {
		Context {
			context
		}
	}
}

impl Display for Prompt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "#### System Prompt ####\n{}\n\n#### User Prompt####\n{}\n\n#### Max Characters ####\n{:?}", self.system_prompt, self.user_prompt, self.max_characters)
	}
}

#[cfg(test)]
mod prompt_tests {
	use super::*;

	#[test]
	fn test_prompt_new() {
		let prompt = Prompt::new("You are a helpful assistant", "This is a sample prompt", Some(100));
		assert_eq!(prompt.system_prompt, "You are a helpful assistant");
		assert_eq!(prompt.user_prompt, "This is a sample prompt");
		assert_eq!(prompt.max_characters, Some(100));
	}

	#[test]
	fn test_prompt_substitube_valid() {
		let prompt = Prompt::new("You are a helpful assistant named [name]", "This is a sample prompt", Some(100));
		let mut context = Context::default();
		context.insert("name", "Bob");
		let expected = Prompt::new("You are a helpful assistant named Bob", "This is a sample prompt", Some(100));
		let actual = prompt.substitute(&context).unwrap();
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_prompt_substitube_invalid() {
		let prompt = Prompt::new("You are a helpful assistant named [name]", "This is a sample prompt", Some(100));
		let context = Context::default();
		let actual = prompt.substitute(&context);
		assert!(actual.is_err());
	}
}