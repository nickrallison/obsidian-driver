use std::path::PathBuf;
use futures::future;

use crate::ai::api::AIDriver;
use crate::ai::prompt::{Context, Prompt};
use crate::file::mdfile::MDFile;

pub use crate::prelude::*;

pub mod api;
pub mod prompt;

pub async fn generate_file(driver: &AIDriver, prompt: Prompt, context: Context, title: String, output_folder: PathBuf) -> Result<crate::file::File> {
    let prompt: Prompt = prompt.substitute(&context)?;
    let file: String = driver.chat_smart(prompt).await?;
    let mdfile: MDFile = MDFile::from_string(file);
    let path = output_folder.join(title);
    let file = crate::file::File::from_mdfile(path, mdfile);
    Ok(file)

}

pub async fn generate_file_and_title(driver: &AIDriver, file_prompt: Prompt, title_prompt: Prompt, context: Context, output_folder: PathBuf) -> Result<crate::file::File> {
    let file_prompt: Prompt = file_prompt.substitute(&context)?;
    let title_prompt: Prompt = title_prompt.substitute(&context)?;

    // generate title and file content concurrently
    // let file: String = driver.chat_smart(file_prompt).await?;
    let (file, title) = future::join(driver.chat_smart(file_prompt), driver.chat_smart(title_prompt)).await;
    let title = title?;
    let file = file?;

    let mdfile: MDFile = MDFile::from_string(file);
    let path = output_folder.join(title);
    let file = crate::file::File::from_mdfile(path, mdfile);
    Ok(file)
}

pub async fn merge_files(files: Vec<crate::file::File>, output_folder: PathBuf) -> Result<crate::file::File> {
    todo!()
}

