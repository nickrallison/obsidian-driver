use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

use mdfile::MDFile;

mod mdfile;

pub struct File {
	path: PathBuf,
	last_modified: Option<u128>,

	contents: FileContents,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FileContents {
	MDFile(MDFile),
// 	Other(OtherFile)
}

impl File {
	fn new_raw(path: PathBuf, ext: &str, contents: String, last_modified: Option<u128>) -> Result<Self> {
		match ext {
			"md" => {
				let mdfile_contents = MDFile::new(contents);
				return Ok(Self {
					path,
					contents: FileContents::MDFile(mdfile_contents),
					last_modified: last_modified
				})
			},
			_ => Err(Error::Generic(f!("Unsupported extension found for file: {}", path.display())).into())
		}
	}
	pub fn read(path: PathBuf) -> Result<Self> {
		let ext = path.extension().ok_or(Error::Generic(f!("No extension found for file: {}", path.display())))?.to_str().ok_or(Error::Generic(f!("Invalid extension for file: {}", path.display())))?;
		let last_modified = std::fs::metadata(&path)?.modified()?.elapsed()?.as_millis();
		match ext {
			"md" => {
				let contents = std::fs::read_to_string(&path)?;
				let mdfile_contents = MDFile::new(contents);
				return Ok(File {
					path,
					contents: FileContents::MDFile(mdfile_contents),
					last_modified: Some(last_modified)
				})
			},
			_ => Err(Error::Generic(f!("Unsupported extension found for file: {}", path.display())).into())
		}
	}

	pub fn save(self) -> Result<Self> {
		let mut owned = self;
		let file_saved: u128 = std::fs::metadata(&owned.path)?.modified()?.elapsed()?.as_millis();
		let prev_last_modified: u128 = owned.last_modified.unwrap_or(0);
		if file_saved == prev_last_modified {
			return Ok(owned);
		}
		match owned.contents {
			FileContents::MDFile(f) => {
				let contents = std::fs::read_to_string(&owned.path)?;
				let f = f.update(contents)?;
				let f = f.save(&owned.path)?;
				owned.contents = FileContents::MDFile(f);
				owned.last_modified = Some(std::fs::metadata(&owned.path)?.modified()?.elapsed()?.as_millis());
				return Ok(owned);
			}
		}
	}

	pub fn update(self) -> Result<Self> {
		let file_updated: u128 = std::fs::metadata(&self.path)?.modified()?.elapsed()?.as_millis();
		if self.last_modified.is_some() {
			if file_updated == self.last_modified.expect("last_modified is None") {
				return Ok(self);
			}
		}
		let contents = std::fs::read_to_string(&self.path)?;
		match self.contents {
			FileContents::MDFile(f) => {
				let f = f.update(contents)?;
				return Ok(File {
					path: self.path,
					contents: FileContents::MDFile(f),
					last_modified: Some(file_updated)
				})
			}
		}


	}
}

#[cfg(test)]
mod file_tests {
	use reqwest::header::LAST_MODIFIED;
use serde_json::error;

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
			contents: FileContents::MDFile(MDFile::new_raw(contents, None))
		};
	}
}