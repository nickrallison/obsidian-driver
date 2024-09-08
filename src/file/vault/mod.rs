use std::collections::HashMap;

use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::file::mdfile::MDFile;

use crate::prelude::*;


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Vault {
	// the key is the local path to the file from the vault root
	files: HashMap<PathBuf, crate::file::File>,
	// ignore the vault root for now
	#[serde(skip)]
	vault_root: PathBuf,

	#[serde(skip)]
	aidriver: Option<crate::ai::api::AIDriver>,

}

impl Vault {
	#[allow(dead_code)]
	pub fn from_path(vault_root: PathBuf) -> Result<Self> {
		let mut files = HashMap::new();
		let vault_root = vault_root.canonicalize()?;
		for entry in std::fs::read_dir(&vault_root)? {
			let entry = entry?;
			let path = entry.path();
			let file = crate::file::File::read_file(path.clone())?;

			let path = path.canonicalize()?;
			let path = path.strip_prefix(&vault_root)?.to_path_buf();

			files.insert(path.clone(), file);
		}


		let aidriver = None;

		Ok(Self {
			files,
			vault_root,
			aidriver
		})
	}

	#[allow(dead_code)]
	pub fn from_cache(vault_root: PathBuf, cache_path: &PathBuf) -> Result<Self> {
		// if cache_path does not exist, make vault from scratch

		if !cache_path.exists() {
			return Self::from_path(vault_root);
		}
		// println!("Cache path: {:?}", cache_path);
		let cache_str: String = std::fs::read_to_string(cache_path)?;
		let mut vault: Self = serde_json::from_str(&cache_str)?;


		let vault_root = vault_root.canonicalize()?;
		vault.vault_root = vault_root.clone();

		// for all files in vault root, insert / update them if they are not in the cache / not up to date
		for entry in std::fs::read_dir(&vault_root)? {
			let entry = entry?;
			let path = entry.path();
			let local_path = path.canonicalize()?;
			let local_path = local_path.strip_prefix(&vault_root)?.to_path_buf();
			if let std::collections::hash_map::Entry::Vacant(e) = vault.files.entry(local_path.clone()) {
				println!("File not found in vault: {:?}", local_path.display());
				let file = crate::file::File::read_file(path.clone())?;
				e.insert(file);
			} else {
				
				let file = vault.files.get_mut(&local_path).expect("File not found in vault");
				let last_modified = std::fs::metadata(&path)?.modified()?.duration_since(std::time::SystemTime::UNIX_EPOCH)?.as_millis();
				if file.last_modified < Some(last_modified) {
					println!("File found in vault but not up to date: {:?}, stored last modified: {:?}, file last modified: {}", local_path.display(), file.last_modified, last_modified);
					let contents = std::fs::read_to_string(&path)?;
					*file = crate::file::File::new_raw(path.clone(), &path.extension().expect("No extension found").to_str().expect("Invalid extension"), contents, Some(last_modified))?;
				}
				else {
					println!("File found in vault and up to date: {:?}", local_path.display());
				}
			}
		}
		// panic!();
		// println!("Vault: {:?}", vault);
		Ok(vault)
	}

	#[allow(dead_code)]
	pub fn to_cache(&self, cache_path: &PathBuf) -> Result<()> {
		let cache_str = serde_json::to_string(self)?;
		std::fs::write(cache_path, cache_str)?;
		Ok(())
	}

	#[allow(dead_code)]
	pub fn add_file(&mut self, path: PathBuf, file: crate::file::File) -> Result<()> {
		// if path is not a child of the vault root, return an error
		if !path.starts_with(&self.vault_root) {
			return Err(Error::PathNotInVaultRoot(path, self.vault_root.clone()));
		}

		if self.files.contains_key(&path) {
			return Err(Error::VaultAlreadyContainsPath(path));
		}

		let path = path.canonicalize()?;
		let path = path.strip_prefix(&self.vault_root)?.to_path_buf();

		self.files.insert(path, file);
		Ok(())
	}
	#[allow(dead_code)]
	pub fn add_path(&mut self, path: PathBuf) -> Result<()> {
		if !path.starts_with(&self.vault_root) {
			return Err(Error::PathNotInVaultRoot(path, self.vault_root.clone()));
		}

		let file = crate::file::File::read_file(path.clone())?;

		let path = path.canonicalize()?;
		let path = path.strip_prefix(&self.vault_root)?.to_path_buf();

		self.add_file(path, file)?;
		Ok(())
	}
	#[allow(dead_code)]
	pub fn get_file(&self, path: &PathBuf) -> Option<&crate::file::File> {
		self.files.get(path)
	}
	#[allow(dead_code)]
	pub fn get_file_mut(&mut self, path: &PathBuf) -> Option<&mut crate::file::File> {
		self.files.get_mut(path)
	}

	#[allow(dead_code)]
	pub fn get_files(&self) -> &HashMap<PathBuf, crate::file::File> {
		&self.files
	}

	pub fn add_ai_driver(&mut self, aidriver: crate::ai::api::AIDriver) {
		self.aidriver = Some(aidriver);
	}
	pub async fn get_embeddings(&mut self) -> Result<()> {
		if self.aidriver.is_none() {
			return Err(Error::NoAIDriver);
		}

		let mut mdfiles: Vec<&mut MDFile> = Vec::new();

		for (path, file) in self.files.iter_mut() {
			let last_modified = std::fs::metadata(&file.path)?.modified()?.elapsed()?.as_millis();
			println!("Path: {:?}", path.display());
			if file.get_mdfile().is_none() {
				println!("No mdfile found for file: {:?}", path);
				continue;
			}
			println!("file.get_mdfile().is_none(): {}", file.get_mdfile().is_none());
			if file.last_modified <= Some(last_modified) && file.get_mdfile().as_ref().unwrap().get_embedding().is_some() {
				println!("File: {:?} is up to date", path);
				continue;
			}
			println!("file.last_modified <= Some(last_modified): {}", file.last_modified <= Some(last_modified));
			println!("file.get_mdfile().as_ref().unwrap().get_embedding().is_some(): {}", file.get_mdfile().as_ref().unwrap().get_embedding().is_some());
			if let Some(mdfile) = file.get_mdfile_mut() {
				mdfiles.push(mdfile);
			}
		}

		let mut futures = Vec::new();
		for mdfile in mdfiles {
			futures.push(mdfile.update_embedding(self.aidriver.as_ref().expect("AIDriver not found")));
		}

		let results = futures::future::join_all(futures).await;
		for (path, result) in results {
			if let Err(e) = result {
				eprintln!("Error updating embedding: {:?} for file {}", e, path.display());
				// Optionally retry or handle the error
			}
		}

		Ok(())
	}
}