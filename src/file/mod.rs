
mod vault;
mod mdfile;

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::prelude::*;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
	path: PathBuf,
	last_modified: Option<u128>,

	contents: FileContents,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FileContents {
	MDFile(mdfile::MDFile),
// 	Other(OtherFile)
}

impl File {
	fn new_raw(path: PathBuf, ext: &str, contents: String, last_modified: Option<u128>) -> Result<Self> {
		match ext {
			"md" => {
				let mdfile_contents = mdfile::MDFile::from_string(contents);
				Ok(Self {
					path,
					contents: FileContents::MDFile(mdfile_contents),
					last_modified
				})
			},
			_ => Err(Error::Generic(f!("Unsupported extension found for file: {}", path.display())))
		}
	}
	pub fn read(path: PathBuf) -> Result<Self> {
		let path_clone = path.clone();
		let ext = path_clone.extension().ok_or(Error::Generic(f!("No extension found for file: {}", path.display())))?.to_str().ok_or(Error::Generic(f!("Invalid extension for file: {}", path.display())))?;
		let last_modified = std::fs::metadata(&path)?.modified()?.elapsed()?.as_millis();
		let contents = std::fs::read_to_string(&path)?;
		Self::new_raw(path, ext, contents, Some(last_modified))
	}
}

#[cfg(test)]
mod file_tests {

	use super::*;

	#[test]
	fn test_read() {
		let path = PathBuf::from("test.md");
		let last_modified: Option<u128> = None;
		let contents: String = "# Test\n\nThis is a test file.".to_string();
		let ext: &str = "md";
		let actual = File::new_raw(path, ext, contents.clone(), last_modified).unwrap();
		let expected = File {
			path: PathBuf::from("test.md"),
			last_modified: None,
			contents: FileContents::MDFile(mdfile::MDFile::from_string(contents))
		};
		assert_eq!(actual, expected);
	}
}