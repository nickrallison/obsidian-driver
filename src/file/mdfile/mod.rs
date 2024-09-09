//! obsidian-driver::file::mdfile
//!
//! The `mdfile` module provides the `MDFile` struct, which represents a markdown file with optional YAML front matter.
//!
//! @public MDFile
//!
//! @public MDFile::new
//!
//! @public MDFile::set_yaml
//!
//! @public MDFile::add_yaml_key
//!
//! @public MDFile::get_yaml
//!
//! @public MDFile::get_yaml_key
//!
//! @public MDFile::set_body
//!
//! @public MDFile::get_body
//!
//! @public MDFile::set_path
//!
//! @public MDFile::get_path
//!
//! @public MDFile::from_string
//!
//! @public MDFile::to_string
//!
//! @public MDFile::update_embedding
//!
//! @public MDFile::get_embedding

// std imports
use std::path::{Path, PathBuf};

// third-party imports
use regex::Regex;
use serde::{Deserialize, Serialize};

// first-party imports
use crate::prelude::*;

/// The `MDFile` struct represents a markdown file with optional YAML front matter.
///
/// # Example
/// ```
/// use std::path::PathBuf;
///
/// use obsidian_driver::file::mdfile::MDFile;
///
/// let file_string = "---\nkey: value\n---\n# Test\n\nThis is a test file.".to_string();
/// let file = MDFile::from_string(file_string);
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MDFile {
    yaml: Option<serde_yaml::Value>,
    body: String,
    embedding: Option<Vec<f64>>,
    // path: Option<PathBuf>
}

impl MDFile {


    /// Creates a new `MDFile` struct with the given YAML front matter, body, and path.
	///
	/// # Arguments
	/// @param yaml: Option<serde_yaml::Value> - The YAML front matter of the markdown file.
	/// @param body: String - The body of the markdown file.
	/// @param path: Option<PathBuf> - The path of the markdown file.
	///
	/// # Example
	/// ```
	/// use std::path::PathBuf;
	///
	/// use obsidian_driver::file::mdfile::MDFile;
	///
	/// let file = MDFile::new(None, "# Test\n\nThis is a test file.".to_string());
	/// ```
	///
	/// ```
	/// use std::path::PathBuf;
	///
	/// use obsidian_driver::file::mdfile::MDFile;
	///
	/// let mut yaml: Option<serde_yaml::Value> = Some(serde_yaml::Value::Mapping(serde_yaml::Mapping::new()));
	/// yaml.as_mut().unwrap().as_mapping_mut().unwrap().insert("key".into(), "value".into());
	///
	/// let file = MDFile::new(yaml, "# Test\n\nThis is a test file.".to_string());
	/// ```
    pub fn new(yaml: Option<serde_yaml::Value>, body: String) -> Self {
        Self {
            yaml,
            body,
            embedding: None
        }
    }

    /// Sets the YAML front matter of the markdown file.
	///
	/// # Arguments
	/// @param yaml: serde_yaml::Value - The YAML front matter of the markdown file.
	///
	/// # Example
	/// ```
	/// use obsidian_driver::file::mdfile::MDFile;
	///
	/// let mut actual = MDFile::new(None, "# Test\n\nThis is a test file.".to_string());
	/// let yaml = serde_yaml::from_str("key: value").unwrap();
	/// actual.set_yaml(yaml);
	///
	/// let expected = MDFile::new(Some(serde_yaml::from_str("key: value").unwrap()), "# Test\n\nThis is a test file.".to_string());
	/// assert_eq!(actual, expected);
	/// ```
    pub fn set_yaml(&mut self, yaml: serde_yaml::Value) {
        if self.yaml.as_ref() != Some(&yaml) {
            self.embedding = None;
        }
        self.yaml = Some(yaml);
    }

	/// Adds a key-value pair to the YAML front matter of the markdown file.
	///
	/// # Arguments
	/// @param key: String - The key of the key-value pair.
	/// @param value: serde_yaml::Value - The value of the key-value pair.
	///
	/// # Example
	/// ```
	/// use obsidian_driver::file::mdfile::MDFile;
	///
	/// let mut actual = MDFile::new(None, "# Test\n\nThis is a test file.".to_string());
	/// actual.add_yaml_key("key".to_string(), serde_yaml::Value::String("value".to_string()));
	///
	/// let expected = MDFile::new(Some(serde_yaml::from_str("key: value").unwrap()), "# Test\n\nThis is a test file.".to_string());
	/// assert_eq!(actual, expected);
	/// ```
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

	/// Gets the YAML front matter of the markdown file.
	///
	/// # Arguments
	/// @returns Option<&serde_yaml::Value> - The YAML front matter of the markdown file.
	///
	/// # Example
	/// ```
	/// use obsidian_driver::file::mdfile::MDFile;
	///
	/// let file = MDFile::new(None, "# Test\n\nThis is a test file.".to_string());
	/// let actual = file.get_yaml();
	/// let expected = None;
	///
	/// assert_eq!(actual, expected);
	/// ```
	///
	/// ```
	/// use obsidian_driver::file::mdfile::MDFile;
	///
	/// let mut yaml = serde_yaml::from_str("key: value").unwrap();
	/// let file = MDFile::new(Some(yaml), "# Test\n\nThis is a test file.".to_string());
	/// let actual: Option<serde_yaml::Value> = file.get_yaml().cloned();
	/// let expected: Option<serde_yaml::Value> = Some(serde_yaml::from_str("key: value").unwrap());
	/// assert_eq!(actual, expected);
	/// ```
    pub fn get_yaml(&self) -> Option<&serde_yaml::Value> {
        self.yaml.as_ref()
    }

	/// Gets the value of a key in the YAML front matter of the markdown file.
	///
	/// # Arguments
	/// @param key: &str - The key of the key-value pair.
	/// @returns Option<&serde_yaml::Value> - The value of the key-value pair.
	///
	/// # Example
	/// ```
	/// use obsidian_driver::file::mdfile::MDFile;
	///
	/// let file = MDFile::new(None, "# Test\n\nThis is a test file.".to_string());
	///
	/// let actual = file.get_yaml_key("key");
	/// let expected = None;
	///
	/// assert_eq!(actual, expected);
	/// ```
	///
	/// ```
	/// use obsidian_driver::file::mdfile::MDFile;
	///
	/// let mut yaml = serde_yaml::from_str("key: value").unwrap();
	/// let file = MDFile::new(Some(yaml), "# Test\n\nThis is a test file.".to_string());
	///
	/// let actual: Option<serde_yaml::Value>  = file.get_yaml_key("key").cloned();
	/// let expected: Option<serde_yaml::Value>  = Some(serde_yaml::from_str("value").unwrap());
	///
	/// assert_eq!(actual, expected);
	/// ```
	///
	/// ```
	/// use obsidian_driver::file::mdfile::MDFile;
	///
	/// let mut file = MDFile::new(None, "# Test\n\nThis is a test file.".to_string());
	///
	/// file.add_yaml_key("key".to_string(), serde_yaml::Value::String("value".to_string()));
	///
	/// let actual: Option<serde_yaml::Value>  = file.get_yaml_key("key").cloned();
	/// let expected : Option<serde_yaml::Value> = Some(serde_yaml::from_str("value").unwrap());
	///
	/// assert_eq!(actual, expected);
	/// ```
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

	/// Sets the body of the markdown file.
	///
	/// # Arguments
	/// @param body: String - The body of the markdown file.
	///
	/// # Example
	/// ```
	/// use obsidian_driver::file::mdfile::MDFile;
	///
	/// let mut actual = MDFile::new(None, "# Test\n\nThis is a test file.".to_string());
	/// actual.set_body("# New Test\n\nThis is a new test file.".to_string());
	/// let expected = MDFile::new(None, "# New Test\n\nThis is a new test file.".to_string());
	/// assert_eq!(actual, expected);
	/// ```
    pub fn set_body(&mut self, body: String) {
        self.embedding = None;
        self.body = body;
    }
	/// Gets the body of the markdown file.
	///
	/// # Arguments
	/// @returns &String - The body of the markdown file.
	///
	/// # Example
	/// ```
	/// use obsidian_driver::file::mdfile::MDFile;
	///
	/// let file = MDFile::new(None, "# Test\n\nThis is a test file.".to_string());
	/// let actual = file.get_body();
	/// let expected = "# Test\n\nThis is a test file.".to_string();
	/// assert_eq!(actual, &expected);
	/// ```
    pub fn get_body(&self) -> &String {
        &self.body
    }


    /// Creates a new `MDFile` struct with the given YAML front matter, body from a string.
	/// 
	/// # Arguments
	/// @param contents: String - The contents of the markdown file.
	/// @returns MDFile - The markdown file.
	/// 
	/// # Example
	/// ```
	/// use obsidian_driver::file::mdfile::MDFile;
	/// 
	/// let contents = "---\nkey: value\n---\n# Test\n\nThis is a test file.".to_string();
	/// let actual = MDFile::from_string(contents);
	/// let expected = MDFile::new(Some(serde_yaml::from_str("key: value").unwrap()), "# Test\n\nThis is a test file.".to_string());
	/// 
	/// assert_eq!(actual, expected);
	/// ```
    pub fn from_string(contents: String) -> Self {
        let yaml_pattern =
            Regex::new(r"^(\-\-\-\n(?P<yaml>[\s\S]*?)\n?\-\-\-\n?)?(?P<body>[\s\S]*)").unwrap();
        let captures = yaml_pattern.captures(&contents).unwrap();
        let yaml = captures
            .name("yaml")
            .map(|m| serde_yaml::from_str(m.as_str()).unwrap());
        let body = captures.name("body").unwrap().as_str().to_string();
        Self::new(yaml, body)
    }
	
	/// Converts the markdown file to a string.
	/// 
	/// # Arguments
	/// @returns String - The markdown file as a string.
	/// 
	/// # Example
	/// ```
	/// use obsidian_driver::file::mdfile::MDFile;
	/// 
	/// let mut yaml = serde_yaml::from_str("key: value").unwrap();
	/// let file = MDFile::new(Some(yaml), "# Test\n\nThis is a test file.".to_string());
	/// 
	/// let actual = file.to_string();
	/// let expected = "---\nkey: value\n---\n# Test\n\nThis is a test file.".to_string();
	/// 
	/// assert_eq!(actual, expected);
	/// ```
    pub fn to_string(&self) -> String {
        if let Some(yaml) = &self.yaml {
            let yaml_str = serde_yaml::to_string(yaml).unwrap();
            format!("---\n{}---\n{}", yaml_str, self.body)
        } else {
            self.body.clone()
        }
    }

    /// Updates the embedding of the markdown file.
	/// 
	/// # Arguments
	/// @param driver: &crate::ai::api::AIDriver - The AI driver.
	/// @param path: &Path - The path of the markdown file.
	///
    pub async fn update_embedding(&mut self, driver: &crate::ai::api::AIDriver, path: &Path) -> Result<()> {
        if self.embedding.is_some() {
            return Ok(());
        }
        let embedding = driver.get_embedding(&self.to_string()).await;
		match &embedding {
			Err(e) => {
				match e {
					Error::InvalidEmbeddingResponse(string) => {
						let new_error = Error::InvalidEmbeddingResponse(format!("{} for file: {:?}", string, path.to_string_lossy()));
						return Err(new_error);
					}
					_ => {}
				}
			}
			_ => {}
		}

		self.embedding = Some(embedding.unwrap());

        println!("Updating embedding for file: {:?}", path.to_string_lossy());
        Ok(())
    }
    
	/// Gets the embedding of the markdown file.
	/// 
	/// # Arguments
	/// @returns Option<&Vec<f64> - The embedding of the markdown file.
	///
	pub fn get_embedding(&self) -> Option<&Vec<f64>> {
        self.embedding.as_ref()
    }
}

#[cfg(test)]
mod mdfile_tests {

    use super::*;

    // Setter / Getter Tests
    #[test]
    fn test_get_set_yaml() {
        let mut mdfile = MDFile::new(None, "# Test\n\nThis is a test file.".to_string());
        let mut yaml = serde_yaml::Value::Mapping(serde_yaml::Mapping::new());
        yaml.as_mapping_mut().unwrap().insert(
            serde_yaml::Value::String("key".to_string()),
            serde_yaml::Value::String("value".to_string()),
        );
        mdfile.set_yaml(yaml.clone());

        let expected = Some(&yaml);
        let actual = mdfile.get_yaml();
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_add_yaml_key() {
        let mut mdfile = MDFile::new(None, "# Test\n\nThis is a test file.".to_string());
        mdfile.add_yaml_key(
            "key".to_string(),
            serde_yaml::Value::String("value".to_string()),
        );

        let binding = serde_yaml::Value::String("value".to_string());
        let expected = Some(&binding);
        let actual = mdfile.get_yaml_key("key");
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_get_yaml_key() {
        let mut mdfile = MDFile::new(None, "# Test\n\nThis is a test file.".to_string());
        let mut yaml = serde_yaml::Value::Mapping(serde_yaml::Mapping::new());
        yaml.as_mapping_mut().unwrap().insert(
            serde_yaml::Value::String("key".to_string()),
            serde_yaml::Value::String("value".to_string()),
        );
        mdfile.set_yaml(yaml);

        let binding = serde_yaml::Value::String("value".to_string());
        let expected = Some(&binding);
        let actual = mdfile.get_yaml_key("key");
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_get_set_body() {
        let mut mdfile = MDFile::new(None, "# Test\n\nThis is a test file.".to_string());
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
        yaml_expected.insert(
            serde_yaml::Value::String("key".to_string()),
            serde_yaml::Value::String("value".to_string()),
        );
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
        yaml_expected.insert(
            serde_yaml::Value::String("key".to_string()),
            serde_yaml::Value::String("value".to_string()),
        );
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

This is a test file."#
            .to_string();
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

This is a test file."#
            .to_string();
        assert_eq!(actual, expected);
    }
}
