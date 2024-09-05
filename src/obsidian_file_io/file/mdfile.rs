use std::path::{PathBuf, Path};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct MDFile {
	contents: String,
	embedding: Option<Vec<f32>>,
}

impl MDFile {
	pub(crate) fn new(contents: String) -> Self {
		Self {
			contents,
			embedding: None,
		}
	}
	pub(crate) fn new_raw(contents: String, embedding: Option<Vec<f32>>) -> Self {
		Self {
			contents,
			embedding,
		}
	}

	// pub(crate) fn new_from_path(path: Path) -> Result<Self> {
	// 	let contents = std::fs::read_to_string(&path)?;
	// 	Ok(Self {
	// 		contents,
	// 		embedding: None,
	// 	})
	// }

	pub(crate) fn save(self, path: &Path) -> Result<Self> {
		let mut owned = self;
		owned.save_mut(path)?;
		Ok(owned)
	}

	pub(crate) fn save_mut(&mut self, path: &Path) -> Result<()> {
		std::fs::write(path, &self.contents)?;
		Ok(())
	}


	pub(crate) fn update(self, contents: String) -> Result<Self> {
		let mut owned = self;
		owned.update_mut(contents)?;
		Ok(owned)
	}

	pub(crate) fn update_mut(&mut self, contents: String) -> Result<()> {


		let contents_updated: bool = contents != self.contents;
		self.contents = contents;
		if contents_updated {
			self.embedding = None;
		}
		Ok(())
	}


}

