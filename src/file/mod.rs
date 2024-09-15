//! obsidian-driver::file
//!
//! This module contains the File struct and its implementations.
//!
//! @public File
//!
//! @public File::read
//!
//! @public File::write
//!
//! @public File::get_mdfile
//!
//! @public File::get_mdfile_mut
//!
//! @public File::from_mdfile

// std imports
use std::path::PathBuf;

// third-party imports
use serde::{Deserialize, Serialize};

// first-party imports
use crate::prelude::*;

// submodules
pub mod mdfile;
pub mod vault;

/// File struct
///
/// This struct represents a file in the vault.
///
/// # Example
///
/// ```should_panic
/// use std::path::PathBuf;
///
/// use obsidian_driver::file::File;
///
/// let path = PathBuf::from("test.md");
/// let file = File::read(path).unwrap();
/// ```
/// @public
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    path: PathBuf,
    last_modified: Option<u128>,

    contents: FileContents,
}

/// FileContents enum
///
/// This enum represents the contents of a file.
///
/// @private
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum FileContents {
    MDFile(mdfile::MDFile),
    // 	Other(OtherFile)
}

impl File {
    /// Create a new File struct from a raw file
    ///
    /// This function creates a new File struct from a raw file.
    ///
    /// # Arguments
    /// @param path: PathBuf
    /// @param ext: &str
    /// @param contents: String
    /// @param last_modified: Option<u128>
    /// @returns Result<Self>
    ///
    fn new_raw(
        path: PathBuf,
        ext: &str,
        contents: String,
        last_modified: Option<u128>,
    ) -> Result<Self> {
        match ext {
            "md" => {
                let mdfile_contents = mdfile::MDFile::from_string(contents);
                Ok(Self {
                    path,
                    contents: FileContents::MDFile(mdfile_contents),
                    last_modified,
                })
            }
            _ => Err(Error::Generic(f!(
                "Unsupported extension found for file: {}",
                path.display()
            ))),
        }
    }

    /// Read a file into a File struct
    ///
    /// # Arguments
    /// @param path: PathBuf
    /// @returns Result<Self>
    ///
    fn read_file(path: PathBuf) -> Result<Self> {
        let path_clone = path.clone();

        let ext = path_clone
            .extension()
            .ok_or(Error::Generic(f!(
                "No extension found for file: {}",
                path.display()
            )))?
            .to_str()
            .ok_or(Error::Generic(f!(
                "Invalid extension for file: {}",
                path.display()
            )))?;
        let last_modified = std::fs::metadata(&path)?
            .modified()?
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_millis();
        let contents = std::fs::read_to_string(&path)?;
        Self::new_raw(path, ext, contents, Some(last_modified))
    }

    /// Read a cached file
    ///
    /// todo: Implement this function
    fn read_cached(_path: PathBuf) -> Result<Self> {
        Err(Error::Generic("Not implemented".to_string()))
    }

    /// Read a file
    ///
    /// This function reads a file from the filesystem.
    ///
    /// # Arguments
    /// @param path: PathBuf
    /// @returns Result<Self>
    ///
    /// # Example
    /// ```should_panic
    ///  use std::path::PathBuf;
    ///
    /// use obsidian_driver::file::File;
    ///
    /// let path = PathBuf::from("test.md");
    /// let file = File::read(path).unwrap();
    /// ```
    pub fn read(path: PathBuf) -> Result<Self> {
        let cached = Self::read_cached(path.clone());
        let file_last_modified = std::fs::metadata(&path)?
            .modified()?
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_millis();
        match cached {
            Ok(file) => {
                if file.last_modified.unwrap() < file_last_modified {
                    Self::read_file(path)
                } else {
                    Ok(file)
                }
            }
            Err(_) => Self::read_file(path),
        }
    }

    /// Write a file
    ///
    /// This function writes a file to the filesystem.
    ///
    /// # Arguments
    /// @returns Result<()> - Ok if successful, Err otherwise
    pub fn write(&self) -> Result<()> {
        match &self.contents {
            FileContents::MDFile(mdfile) => {
                let contents = mdfile.to_string();
                std::fs::write(&self.path, contents)?;
                Ok(())
            }
        }
    }

    /// Get the MDFile struct if the file is a markdown file
    ///
    /// # Arguments
    /// @returns Option<&mdfile::MDFile> - Some if the file is a markdown file, None otherwise
    pub fn get_mdfile(&self) -> Option<&mdfile::MDFile> {
        match &self.contents {
            FileContents::MDFile(mdfile) => Some(mdfile),
        }
    }

    /// Get the mutable MDFile struct if the file is a markdown file
    ///
    /// # Arguments
    /// @returns Option<&mut mdfile::MDFile> - Some if the file is a markdown file, None otherwise
    pub fn get_mdfile_mut(&mut self) -> Option<&mut mdfile::MDFile> {
        match &mut self.contents {
            FileContents::MDFile(mdfile) => Some(mdfile),
        }
    }

    /// Create a File struct from an MDFile struct
    ///
    /// # Arguments
    /// @param path: PathBuf
    /// @param mdfile: mdfile::MDFile
    /// @returns Self
    ///
    /// # Example
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use obsidian_driver::file::File;
    /// use obsidian_driver::file::mdfile;
    ///
    /// let path = PathBuf::from("test.md");
    /// let mdfile = mdfile::MDFile::from_string("# Test\n\nThis is a test file.".to_string());
    ///
    /// let file = File::from_mdfile(path, mdfile);
    /// ```
    pub fn from_mdfile(path: PathBuf, mdfile: mdfile::MDFile) -> Self {
        Self {
            path,
            last_modified: None,
            contents: FileContents::MDFile(mdfile),
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
        let expected_mdfile = mdfile::MDFile::from_string(contents);
        let expected = File {
            path: PathBuf::from("test.md"),
            last_modified: None,
            contents: FileContents::MDFile(expected_mdfile),
        };
        assert_eq!(actual, expected);
    }
}
