use serde::{Deserialize, Serialize};
use regex::Regex;

// use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct MDFile {
    yaml: Option<serde_yaml::Value>,
	body: String,
	embedding: Option<Vec<f32>>,
}

impl MDFile {
	pub(crate) fn from_string(contents: String) -> Self {
        let yaml_pattern = Regex::new(r"^(\-\-\-\n(?P<yaml>[\s\S]*?)\n?\-\-\-\n?)?(?P<body>[\s\S]*)").unwrap();
        let captures = yaml_pattern.captures(&contents).unwrap();
        let yaml = captures.name("yaml").map(|m| serde_yaml::from_str(m.as_str()).unwrap());
        let body = captures.name("body").unwrap().as_str().to_string();

		Self {
			body,
            yaml,
			embedding: None,
		}
	}

    pub(crate) fn to_string(&self) -> String {
        if let Some(yaml) = &self.yaml {
            let yaml_str = serde_yaml::to_string(yaml).unwrap();
            format!("---\n{}---\n{}", yaml_str, self.body)
        } else {
            self.body.clone()
        }
    }
    pub(crate) fn create_embedding(&self) -> Vec<f32> {
        todo!()
    }
    pub(crate) fn update_embedding(&mut self) {
        todo!()
    }
    pub(crate) fn get_embedding(&self) -> Vec<f32> {
        todo!()
    }
}

#[cfg(test)]
mod mdfile_tests {
    
        use super::*;
    
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
