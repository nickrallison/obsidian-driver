use crate::ai::api::AIDriver;
use crate::ai::prompt::Prompt;
use crate::file::mdfile::MDFile;

pub use crate::prelude::*;

pub mod api;
pub mod prompt;

pub fn generate_file(driver: &AIDriver, prompt: Prompt, context: String, title: String) -> crate::file::File {
    todo!()
}

pub fn generate_file_and_title(driver: &AIDriver, file_prompt: Prompt, title_prompt: Prompt) -> crate::file::File {
    todo!()
}

pub fn modify_file(driver: &AIDriver, file: &mut MDFile, prompt: Prompt) -> Result<()> {
    todo!()
}

pub fn append_file(driver: &AIDriver, file: &mut MDFile, prompt: Prompt) -> Result<()> {
    todo!()
}