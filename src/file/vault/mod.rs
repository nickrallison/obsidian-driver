//! obsidian-driver::file::vault
//!
//! This module contains the Vault struct and its implementations. This struct is used to provide the main public interface for the library.

// std imports
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// third-party imports
use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use serde::{Deserialize, Serialize};

// first-party imports
use crate::file::mdfile::MDFile;
use crate::prelude::*;

/// Vault struct
///
/// This struct represents the vault.
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
    /// Create a new Vault from a given path.
    ///
    /// # Arguments
    /// @param vault_root: PathBuf
    /// @return Result<Self>
    ///
    /// # Example
    /// ```should_panic
    /// use std::path::PathBuf;
    ///
    /// use obsidian_driver::file::vault::Vault;
    ///
    /// let vault_root = PathBuf::from("vault");
    /// let vault = Vault::from_path(vault_root).unwrap();
    /// ```
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
            aidriver,
        })
    }

    /// Create a new Vault from a given path.
    ///
    /// # Arguments
    /// @param vault_root: PathBuf
    /// @return Result<Self>
    ///
    /// # Example
    /// ```should_panic
    /// use std::path::PathBuf;
    ///
    /// use obsidian_driver::file::vault::Vault;
    ///
    /// let vault_root = PathBuf::from("vault");
    /// let cache_path = PathBuf::from("vault_cache.json");
    /// let vault = Vault::from_cache(vault_root, &cache_path).unwrap();
    /// ```
    pub fn from_cache(vault_root: PathBuf, cache_path: &PathBuf) -> Result<Self> {
        // if cache_path does not exist, make vault from scratch

        if !cache_path.exists() {
            return Self::from_path(vault_root);
        }
        let cache_str: String = std::fs::read_to_string(cache_path)?;
        let mut vault: Self = serde_json::from_str(&cache_str)?;

        let vault_root = vault_root.canonicalize()?;
        vault.vault_root.clone_from(&vault_root);

        // for all files in vault root, insert / update them if they are not in the cache / not up to date
        for entry in std::fs::read_dir(&vault_root)? {
            let entry = entry?;
            let path = entry.path();
            let local_path = path.canonicalize()?;
            let local_path = local_path.strip_prefix(&vault_root)?.to_path_buf();
            if let std::collections::hash_map::Entry::Vacant(e) =
                vault.files.entry(local_path.clone())
            {
                let file = crate::file::File::read_file(path.clone())?;
                e.insert(file);
            } else {
                let file = vault
                    .files
                    .get_mut(&local_path)
                    .expect("File not found in vault");
                let last_modified = std::fs::metadata(&path)?
                    .modified()?
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)?
                    .as_millis();
                if file.last_modified < Some(last_modified) {
                    let contents = std::fs::read_to_string(&path)?;
                    *file = crate::file::File::new_raw(
                        path.clone(),
                        path.extension()
                            .expect("No extension found")
                            .to_str()
                            .expect("Invalid extension"),
                        contents,
                        Some(last_modified),
                    )?;
                }
            }
        }
        Ok(vault)
    }

    /// Write the Vault to a cache file.
    ///
    /// # Arguments
    /// @param cache_path: &PathBuf
    /// @return Result<()>
    ///
    pub fn to_cache(&self, cache_path: &PathBuf) -> Result<()> {
        let cache_str = serde_json::to_string(self)?;
        std::fs::write(cache_path, cache_str)?;
        Ok(())
    }

    /// Add a file to the Vault.
    ///
    /// # Arguments
    /// @param path: PathBuf
    /// @param file: crate::file::File
    /// @return Result<()>
    ///
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

	/// Add a file to the Vault from a path.
	/// 
	/// # Arguments
	/// @param path: PathBuf
	/// @return Result<()>
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

	/// Get a file from the Vault. Uses the path relative to the vault root.
	/// 
	/// # Arguments
	/// @param path: &PathBuf
	/// @return Option<&crate::file::File>
    pub fn get_file(&self, path: &PathBuf) -> Option<&crate::file::File> {
        self.files.get(path)
    }
    
	/// Get a mutable file from the Vault. Uses the path relative to the vault root.
	/// 
	/// # Arguments
	/// @param path: &PathBuf
	/// @return Option<&mut crate::file::File>
	pub fn get_file_mut(&mut self, path: &PathBuf) -> Option<&mut crate::file::File> {
        self.files.get_mut(path)
    }

	/// Get all files in the Vault.
	/// 
	/// # Arguments
	/// @return &HashMap<PathBuf, crate::file::File>
    pub fn get_files(&self) -> &HashMap<PathBuf, crate::file::File> {
        &self.files
    }

	/// Adds an AIDriver to the Vault.
	/// 
	/// # Arguments
	/// @param aidriver: crate::ai::api::AIDriver
    pub fn add_ai_driver(&mut self, aidriver: crate::ai::api::AIDriver) {
        self.aidriver = Some(aidriver);
    }
    
	/// Updates the embeddings of all files in the Vault.
	/// 
	/// # Arguments
	/// @return Result<()>
	pub async fn update_embeddings(&mut self) -> Result<()> {
        if self.aidriver.is_none() {
            return Err(Error::NoAIDriver);
        }

        let mut mdfiles: Vec<(&mut MDFile, &Path)> = Vec::new();

        for (path, file) in self.files.iter_mut() {
            let last_modified = std::fs::metadata(&file.path)?
                .modified()?
                .elapsed()?
                .as_millis();
            if file.get_mdfile().is_none() {
                continue;
            }
            if file.last_modified <= Some(last_modified)
                && file
                    .get_mdfile()
                    .as_ref()
                    .unwrap()
                    .get_embedding()
                    .is_some()
            {
                continue;
            }
            if let Some(mdfile) = file.get_mdfile_mut() {
                mdfiles.push((mdfile, path));
            }
        }

        let mut futures = Vec::new();
        for (mdfile, path) in mdfiles {
            futures.push(
                mdfile.update_embedding(self.aidriver.as_ref().expect("AIDriver not found"), path),
            );
        }

        let results = futures::future::join_all(futures).await;
        for result in results {
            if let Err(e) = result {
                eprintln!("{}", e);
                // Optionally retry or handle the error
            }
        }

        Ok(())
    }

    /// Get the closest files to a given file by embedding distance.
	/// 
	/// # Arguments
	/// @param path: &PathBuf
	/// @param n: usize
	/// @return Result<Vec<(PathBuf, f64)>>
    pub fn get_closest_files(&self, path: &PathBuf, n: usize) -> Result<Vec<(PathBuf, f64)>> {
        let file = self
            .files
            .get(path)
            .ok_or(Error::Generic(f!("Path Not Found: {}", path.display())))?;
        let mdfile = file
            .get_mdfile()
            .ok_or(Error::Generic(f!("Not MDFile: {}", path.display())))?;
        let embedding = mdfile.get_embedding().ok_or(Error::Generic(f!(
            "Noo embedding for file: {}",
            path.display()
        )))?;

        let mut embeddings = Vec::new();

        for (other_path, other_file) in self.files.iter() {
            // if other_path == path {
            // 	continue;
            // }
            let other_mdfile = other_file.get_mdfile();
            if other_mdfile.is_none() {
                continue;
            }
            let other_mdfile = other_mdfile.unwrap();
            let other_embedding = other_mdfile.get_embedding();
            if other_embedding.is_none() {
                continue;
            }
            let other_embedding = other_embedding.unwrap();

            embeddings.push(other_embedding.clone());
        }

        let dimensions = embedding.len();
        let mut kdtree = KdTree::new(dimensions);

        for (i, embedding) in embeddings.iter().enumerate() {
            kdtree.add(embedding, i).unwrap();
        }

        let nearest: Vec<(f64, &usize)> = kdtree.nearest(embedding, n, &squared_euclidean).unwrap();
        let mut distances = Vec::new();
        for (distance, &index) in nearest {
            let other_path = self.files.keys().nth(index).unwrap();
            distances.push((other_path.clone(), distance.sqrt()));
        }
        Ok(distances)
    }

	/// Get the closest files to a given file by embedding distance with a threshold.
	/// 
	/// # Arguments
	/// @param path: &PathBuf
	/// @param threshold: f64
	/// @return Result<Vec<(PathBuf, f64)>>
    pub fn get_closest_files_by_threshold(
        &self,
        path: &PathBuf,
        threshold: f64,
    ) -> Result<Vec<(PathBuf, f64)>> {
        let file = self
            .files
            .get(path)
            .ok_or(Error::Generic(f!("Path Not Found: {}", path.display())))?;
        let mdfile = file
            .get_mdfile()
            .ok_or(Error::Generic(f!("Not MDFile: {}", path.display())))?;
        let embedding = mdfile.get_embedding().ok_or(Error::Generic(f!(
            "Noo embedding for file: {}",
            path.display()
        )))?;

        let mut embeddings = Vec::new();

        for (other_path, other_file) in self.files.iter() {
            let other_mdfile = other_file.get_mdfile();
            if other_mdfile.is_none() {
                continue;
            }
            let other_mdfile = other_mdfile.unwrap();
            let other_embedding = other_mdfile.get_embedding();
            if other_embedding.is_none() {
                continue;
            }
            let other_embedding = other_embedding.unwrap();

            embeddings.push(other_embedding.clone());
        }

        let dimensions = embedding.len();
        let mut kdtree = KdTree::new(dimensions);

        for (i, embedding) in embeddings.iter().enumerate() {
            kdtree.add(embedding, i).unwrap();
        }
        let threshold = threshold.powi(2);
        let nearest: Vec<(f64, &usize)> = kdtree
            .iter_nearest(embedding, &squared_euclidean)
            .unwrap()
            .filter(|(distance, _)| *distance <= threshold)
            .collect();
        let mut distances = Vec::new();
        for (distance, &index) in nearest {
            let other_path = self.files.keys().nth(index).unwrap();
            distances.push((other_path.clone(), distance.sqrt()));
        }
        Ok(distances)
    }
}
