use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum File {
	MDFile(MDFile),
// 	Other(OtherFile)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct MDFile {
	contents: String,
	path: PathBuf,
	embedding: Option<Vec<f32>>,
	last_modified: u128
}

impl MDFile {
	fn new(contents: String, path: PathBuf, last_modified: u128) -> Self {
		Self {
			contents,
			path,
			embedding: None,
			last_modified
		}
	}

	fn new_from_path(path: PathBuf) -> Result<Self> {
		let contents = std::fs::read_to_string(&path)?;
		let last_modified = std::fs::metadata(&path)?.modified()?.elapsed()?.as_millis();
		Ok(Self {
			contents,
			path,
			embedding: None,
			last_modified
		})
	}

	fn save(&self) -> Result<()> {
		std::fs::write(&self.path, &self.contents)?;
		Ok(())
	}

	fn update(&mut self) -> Result<()> {
		let file_updated: u128 = std::fs::metadata(&self.path)?.modified()?.elapsed()?.as_millis();
		if file_updated == self.last_modified {
			return Ok(());
		}

		let contents_new = std::fs::read_to_string(&self.path)?;
		let contents_updated: bool = contents_new != self.contents;
		self.contents = contents_new;
		self.last_modified = file_updated;
		if contents_updated {
			self.embedding = None;
		}
		Ok(())
	}


}

