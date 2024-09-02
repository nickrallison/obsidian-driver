use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum File {
	MDFile(MDFile),
	Other(OtherFile)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct MDFile {
	contents: String,
	path: PathBuf,
	embedding: Option<Vec<f32>>,
	last_modified: u64
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct OtherFile {

}

