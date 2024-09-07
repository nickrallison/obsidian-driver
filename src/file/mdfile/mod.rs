use serde::{Deserialize, Serialize};
use regex::Regex;

// use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MDFile {
	yaml: Option<serde_yaml::Value>,
	body: String,
	embedding: Option<Vec<f32>>,
}

impl MDFile {
	// Constructor
	fn new_raw(yaml: Option<serde_yaml::Value>, body: String) -> Self {
		Self {
			yaml,
			body,
			embedding: None,
		}
	}

	// Setter / Getter Methods
	pub fn set_yaml(&mut self, yaml: serde_yaml::Value) {
		if self.yaml.as_ref() != Some(&yaml) {
			self.embedding = None;
		}
		self.yaml = Some(yaml);
	}
	pub fn add_yaml_key(&mut self, key: String, value: serde_yaml::Value) {
		self.embedding = None;
		if let Some(yaml) = &mut self.yaml {
			if let serde_yaml::Value::Mapping(mapping) = yaml {
				mapping.insert(serde_yaml::Value::String(key), value);
			}
		} else {
			let mut new_yaml: serde_yaml::Mapping = serde_yaml::Mapping::new();
			new_yaml.insert(serde_yaml::Value::String(key), value);
			self.yaml = Some(serde_yaml::Value::Mapping(new_yaml));
		}
	}
	pub fn get_yaml(&self) -> Option<&serde_yaml::Value> {
		self.yaml.as_ref()
	}
	pub fn get_yaml_key(&self, key: &str) -> Option<&serde_yaml::Value> {
		if let Some(yaml) = &self.yaml {
			if let serde_yaml::Value::Mapping(mapping) = yaml {
				mapping.get(&serde_yaml::Value::String(key.to_string()))
			} else {
				None
			}
		} else {
			None
		}
	}

	pub fn set_body(&mut self, body: String) {
		self.embedding = None;
		self.body = body;
	}
	pub fn get_body(&self) -> &String {
		&self.body
	}

	// Serialization Methods
	pub fn from_string(contents: String) -> Self {
		let yaml_pattern = Regex::new(r"^(\-\-\-\n(?P<yaml>[\s\S]*?)\n?\-\-\-\n?)?(?P<body>[\s\S]*)").unwrap();
		let captures = yaml_pattern.captures(&contents).unwrap();
		let yaml = captures.name("yaml").map(|m| serde_yaml::from_str(m.as_str()).unwrap());
		let body = captures.name("body").unwrap().as_str().to_string();
		Self::new_raw(yaml, body)
	}
	pub fn to_string(&self) -> String {
		if let Some(yaml) = &self.yaml {
			let yaml_str = serde_yaml::to_string(yaml).unwrap();
			format!("---\n{}---\n{}", yaml_str, self.body)
		} else {
			self.body.clone()
		}
	}

	// Embedding Methods
	pub fn create_embedding(&self) -> Vec<f32> {
		todo!()
	}
	pub fn update_embedding(&mut self) {
		todo!()
	}
	pub fn get_embedding(&self) -> Vec<f32> {
		todo!()
	}
}

#[cfg(test)]
mod mdfile_tests {

	use super::*;

	// Setter / Getter Tests
	#[test]
	fn test_get_set_yaml() {
		let mut mdfile = MDFile::new_raw(None, "# Test\n\nThis is a test file.".to_string());
		let mut yaml = serde_yaml::Value::Mapping(serde_yaml::Mapping::new());
		yaml.as_mapping_mut().unwrap().insert(serde_yaml::Value::String("key".to_string()), serde_yaml::Value::String("value".to_string()));
		mdfile.set_yaml(yaml.clone());

		let expected = Some(&yaml);
		let actual = mdfile.get_yaml();
		assert_eq!(actual, expected);
	}
	#[test]
	fn test_add_yaml_key() {
		let mut mdfile = MDFile::new_raw(None, "# Test\n\nThis is a test file.".to_string());
		mdfile.add_yaml_key("key".to_string(), serde_yaml::Value::String("value".to_string()));

		let binding = serde_yaml::Value::String("value".to_string());
  		let expected = Some(&binding);
		let actual = mdfile.get_yaml_key("key");
		assert_eq!(actual, expected);
	}
	#[test]
	fn test_get_yaml_key() {
		let mut mdfile = MDFile::new_raw(None, "# Test\n\nThis is a test file.".to_string());
		let mut yaml = serde_yaml::Value::Mapping(serde_yaml::Mapping::new());
		yaml.as_mapping_mut().unwrap().insert(serde_yaml::Value::String("key".to_string()), serde_yaml::Value::String("value".to_string()));
		mdfile.set_yaml(yaml);

		let binding = serde_yaml::Value::String("value".to_string());
  		let expected = Some(&binding);
		let actual = mdfile.get_yaml_key("key");
		assert_eq!(actual, expected);
	}
	#[test]
	fn test_get_set_body() {
		let mut mdfile = MDFile::new_raw(None, "# Test\n\nThis is a test file.".to_string());
		mdfile.set_body("# New Test\n\nThis is a new test file.".to_string());

		let expected = "# New Test\n\nThis is a new test file.".to_string();
		let actual = mdfile.get_body();
		assert_eq!(actual, &expected);
	}


	// Serialization Tests
	#[test]
	fn test_from_string_with_yaml() {
		let contents = "---\nkey: value\n---\n# Test\n\nThis is a test file.".to_string();

		let actual = MDFile::from_string(contents);
		let mut yaml_expected: serde_yaml::Mapping = serde_yaml::Mapping::new();
		yaml_expected.insert(serde_yaml::Value::String("key".to_string()), serde_yaml::Value::String("value".to_string()));
		let expected = MDFile {
			yaml: Some(serde_yaml::Value::Mapping(yaml_expected)),
			body: "# Test\n\nThis is a test file.".to_string(),
			embedding: None,
		};
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_to_string_with_yaml() {
		let mut yaml_expected: serde_yaml::Mapping = serde_yaml::Mapping::new();
		yaml_expected.insert(serde_yaml::Value::String("key".to_string()), serde_yaml::Value::String("value".to_string()));
		let mdfile = MDFile {
			yaml: Some(serde_yaml::Value::Mapping(yaml_expected)),
			body: "# Test\n\nThis is a test file.".to_string(),
			embedding: None,
		};
		let actual = mdfile.to_string();
		let expected = r#"---
key: value
---
# Test

This is a test file."#.to_string();
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_from_string_without_yaml() {
		let contents = "# Test\n\nThis is a test file.".to_string();

		let actual = MDFile::from_string(contents);
		let expected = MDFile {
			yaml: None,
			body: "# Test\n\nThis is a test file.".to_string(),
			embedding: None,
		};
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_to_string_without_yaml() {
		let mdfile = MDFile {
			yaml: None,
			body: "# Test\n\nThis is a test file.".to_string(),
			embedding: None,
		};
		let actual = mdfile.to_string();
		let expected = r#"# Test

This is a test file."#.to_string();
		assert_eq!(actual, expected);
	}


}
