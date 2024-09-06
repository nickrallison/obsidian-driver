use std::path::{PathBuf, Path};
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
        // if yaml does capture, use the yaml capture group as the yaml value
        // otherwise, set yaml to None

        // if body does capture, use the body capture group as the body value
        // otherwise, set body to ""
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

	// pub(crate) fn save(self, path: &Path) -> Result<Self> {
	// 	let mut owned = self;
	// 	owned.save_mut(path)?;
	// 	Ok(owned)
	// }

	// pub(crate) fn save_mut(&mut self, path: &Path) -> Result<()> {
	// 	std::fs::write(path, &self.contents)?;
	// 	Ok(())
	// }


	// pub(crate) fn update(self, contents: String) -> Result<Self> {
	// 	let mut owned = self;
	// 	owned.update_mut(contents)?;
	// 	Ok(owned)
	// }

	// pub(crate) fn update_mut(&mut self, contents: String) -> Result<()> {


	// 	let contents_updated: bool = contents != self.contents;
	// 	self.contents = contents;
	// 	if contents_updated {
	// 		self.embedding = None;
	// 	}
	// 	Ok(())
	// }


}

#[cfg(test)]
mod mdfile_tests {
    
        use super::*;
    
        #[test]
        fn test_from_string() {
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
        fn test_to_string() {
            let mut yaml_expected: serde_yaml::Mapping = serde_yaml::Mapping::new();
            yaml_expected.insert(serde_yaml::Value::String("key".to_string()), serde_yaml::Value::String("value".to_string()));
            let mdfile = MDFile {
                yaml: Some(serde_yaml::Value::Mapping(yaml_expected)),
                body: "# Test\n\nThis is a test file.".to_string(),
                embedding: None,
            };
            let actual = mdfile.to_string();
            let expected = format!(r#"---
key: value
---
# Test

This is a test file."#);
            assert_eq!(actual, expected);
        }
    
        // #[test]
        // fn test_save() {
        // 	let path = PathBuf::from("test.md");
        // 	let last_modified: Option<u128> = None;
        // 	let contents: String = "# Test\n\nThis is a test file.".to_string();
        // 	let ext: &str = "md";
        // 	let file = File::new_raw(path, ext, contents.clone(), last_modified).unwrap();
        // 	let actual = file.save().unwrap();
        // 	let expected = File {
}
