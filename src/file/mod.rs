
pub mod mdfile;

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
pub(crate) enum FileContents {
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
	fn read_file(path: PathBuf) -> Result<Self> {
		let path_clone = path.clone();
		let ext = path_clone.extension().ok_or(Error::Generic(f!("No extension found for file: {}", path.display())))?.to_str().ok_or(Error::Generic(f!("Invalid extension for file: {}", path.display())))?;
		let last_modified = std::fs::metadata(&path)?.modified()?.elapsed()?.as_millis();
		let contents = std::fs::read_to_string(&path)?;
		Self::new_raw(path, ext, contents, Some(last_modified))
	}
	fn read_cached(path: PathBuf) -> Result<Self> {
		// Err(Error::Generic("Not implemented".to_string()))
		todo!()
	}
	pub fn read(path: PathBuf) -> Result<Self> {
		let cached = Self::read_cached(path.clone());
		let file_last_modified = std::fs::metadata(&path)?.modified()?.elapsed()?.as_millis();
		match cached {
			Ok(file) => {
				if file.last_modified.unwrap() < file_last_modified {
					Self::read_file(path)
				} else {
					Ok(file)
				}
			},
			Err(_) => Self::read_file(path)
		}
	}

	pub fn write(&self) -> Result<()> {
		match &self.contents {
			FileContents::MDFile(mdfile) => {
				let contents = mdfile.to_string();
				std::fs::write(&self.path, contents)?;
				Ok(())
			},
		}
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