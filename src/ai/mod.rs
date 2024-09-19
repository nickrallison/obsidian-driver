//! # obsidian-driver::ai
//!
//! This module contains the AI API for the Obsidian Driver. The AI API provides a high-level interface to the AI models.
//!
//! @public api
//!
//! @public prompt
//!
//! @public generate_file
//!
//! @public generate_file_and_title
//!
//! @public merge_files

// std imports
use std::path::PathBuf;

// third-party imports
use futures::future;

// first-party imports
use crate::file::mdfile::MDFile;
use crate::prelude::*;

// module imports
use api::AIDriver;
use prompt::{Context, Prompt};

// submodules
pub mod api;
pub mod prompt;


/// Generate a file from a prompt and context
///
/// This function takes a prompt and a context and generates a file from the prompt. The prompt is substituted with the context and then passed to the AI model to generate the file. The file is then converted to a `crate::file::File` and returned.
///
/// # Arguments
/// @param driver: &AIDriver - The AI driver to use for generating the file
/// @param prompt: Prompt - The prompt to generate the file from
/// @param context: Context - The context to substitute into the prompt
/// @param title: String - The title of the file
/// @param output_folder: PathBuf - The output folder to save the file in
/// @returns Result<crate::file::File> - The generated file
///
/// # Example
///
/// ```
/// use std::path::PathBuf;
///
/// use obsidian_driver::ai::generate_file;
/// use obsidian_driver::ai::api::AIDriver;
/// use obsidian_driver::ai::prompt::{Prompt, Context};
///
/// async fn generate_file_example() {
/// 	let openai_config_path = PathBuf::from(".openai_config.json");
/// 	let driver = AIDriver::new_openai_from_config_path(openai_config_path).await.unwrap();
/// 	let prompt: Prompt = Prompt::new("You are a helpful assistant", "Summarize the following text in your own words:\n\n$text$", 512);
/// 	let mut context = Context::default();
/// 	context.insert("text", "This is a test text, it could be anything, even the entire works of Shakespeare");
/// 	let prompt: Prompt = prompt.substitute(&context).unwrap();
///
/// 	let file = generate_file(&driver, prompt, context, "test.md".to_string(), PathBuf::from("output")).await.unwrap();
/// }
/// ```
/// @public
pub async fn generate_file(driver: &AIDriver, prompt: Prompt, context: Context, title: String, output_folder: PathBuf) -> Result<crate::file::File> {
    let prompt: Prompt = prompt.substitute(&context)?;
    let file: String = driver.chat_smart(prompt).await?;
    let mdfile: MDFile = MDFile::from_string(file);
    let path = output_folder.join(title);
    let file = crate::file::File::from_mdfile(path, mdfile);
    Ok(file)

}

/// Generate a file and title from a file prompt and title prompt
///
/// This function takes a file prompt and a title prompt and generates a file and title from the prompts. The prompts are substituted with the context and then passed to the AI model to generate the file and title. The file is then converted to a `crate::file::File` and returned.
///
/// # Arguments
/// @param driver: &AIDriver - The AI driver to use for generating the file
/// @param file_prompt: Prompt - The prompt to generate the file from
/// @param title_prompt: Prompt - The prompt to generate the title from
/// @param context: Context - The context to substitute into the prompts
/// @param output_folder: PathBuf - The output folder to save the file in
/// @returns Result<crate::file::File> - The generated file
///
/// # Example
/// ```
/// use std::path::PathBuf;
///
/// use obsidian_driver::ai::generate_file_and_title;
/// use obsidian_driver::ai::api::AIDriver;
/// use obsidian_driver::ai::prompt::{Prompt, Context};
///
/// async fn generate_file_and_title_example() {
/// 	let openai_config_path = PathBuf::from(".openai_config.json");
/// 	let driver = AIDriver::new_openai_from_config_path(openai_config_path).await.unwrap();
/// 	let file_prompt: Prompt = Prompt::new("You are a helpful assistant", "Summarize the following text in your own words:\n\n$text$", 512);
/// 	let title_prompt: Prompt = Prompt::new("You are a helpful assistant", "Provide a title for the summary", 64);
/// 	let mut context = Context::default();
/// 	context.insert("text", "This is a test text, it could be anything, even the entire works of Shakespeare");
///
/// 	let file = generate_file_and_title(&driver, file_prompt, title_prompt, context, PathBuf::from("output")).await.unwrap();
/// }
/// ```
/// @public
pub async fn generate_file_and_title(driver: &AIDriver, file_prompt: Prompt, title_prompt: Prompt, context: Context, output_folder: PathBuf) -> Result<crate::file::File> {
    let file_prompt: Prompt = file_prompt.substitute(&context)?;
    let title_prompt: Prompt = title_prompt.substitute(&context)?;

    let (file, title) = future::join(driver.chat_smart(file_prompt), driver.chat_smart(title_prompt)).await;
    let title = title?;
    let file = file?;

    let mdfile: MDFile = MDFile::from_string(file);
    let path = output_folder.join(title);
    let file = crate::file::File::from_mdfile(path, mdfile);
    Ok(file)
}

const MERGE_SYSTEM_PROMPT: &str = r#"```system
You are a organized student making lecture notes.
	I have the following rules:
	 - Tabs must be used for indentation.
	 - Text must formatted in markdown.
	 - Math must be formatted in LaTeX. ($$ …$$ for multiline, $ … $ for inline)
		 - Prefer to use LaTeX for special characters over raw utf8. THIS IS IMPORTANT, MAKE SURE TO USE $ SIGNS AROUND LATEX
		 - Wrong Example -> A substring \( α \) of a string \( \omega \) is a sequence of symbols that appears consecutively within \( \omega \)
		 - Correct Example -> A substring $\mu$ of a string $\omega$ is a sequence of symbols that appears consecutively within $\omega$
	 - Algorithm Pseudocode must be formatted in the following manor:
	```pseudo
		\\begin{algorithm}
			\\caption{A-Star Search Algorithm}
			\\begin{algorithmic}
				\\Procedure{AStar}{$Graph, start, goal$}
					\\State …
					…
				\\EndProcedure
			\\end{algorithmic}
		\\end{algorithm}
	```
	 - I must avoid formatting with inline code blocks: `code block here`, and prefer to use multiline code blocks
```"#;
const MERGE_USER_PROMPT: &str = r#"**User**
Below are multiple notes from similar topics. Merge these together preserving as much information as you can. Your goal is to keep a logical ordering within each of the sections and between each of the sections.

Do not lose any information from any of the child notes, if they can be merged together them merge them. If not, then keep the information and order them logically within the resulting note.

If a Takeaways section is present in any of the notes, make sure to include it in the final note, and do remove any lines from it. Moreover, if there are multiple Takeaways sections, merge them together. If any notes don't have a Takeaways section, then add what you think are the takeaways from that note into the final takeaways section. When compared, the final note should include each and every takeaway from the child notes, none can be missing.

The takeaways should be formatted like this:
## Takeaways
- **Alphabet**: Finite non-empty set of symbols (e.g., $( \Sigma )$, $( \Gamma ))$.
- **String**: A finite sequence of elements from an alphabet; length denoted by $( |\omega| )$.
- **Empty String**: Denoted by $( \lambda )$, signifies a string of length zero.
- **Concatenation**: Operation of combining two strings $( \mu · ν )$ resulting in a new string.
- **Substring**: Sequential part of a string; exists in the order without rearrangement.
- **Prefix and Suffix**: Parts of a string from the start (prefix) or the end (suffix) respectively.
- **Language**: A subset of $( \Sigma^* )$, representing a set of strings that encode decision problems.
- **Decision Problems**: Problems requiring a binary answer, which can be represented using strings over an alphabet.
- **Algorithm Exploration**: Exploration of algorithms that decide language membership and analyze language properties.

**Notes**

[notes]

**PS**
Since this is transcript, please ignore any details that may be missed due to this format (ignore mentions of images, or gestures and such). Also ignore non lecture material like advertisements, or personal sponsorships
Also do not ever directly use any non-ASCII characters in these notes.
"#;

pub async fn merge_files(driver: AIDriver, files: Vec<&crate::file::File>) -> crate::file::mdfile::MDFile {
    let mut prompt = Prompt::new(MERGE_SYSTEM_PROMPT, MERGE_USER_PROMPT, None);
	let mut context = Context::default();
	let mut notes = String::new();
	for (index, file) in files.into_iter().enumerate() {
		let mdfile = file.get_mdfile();
		if (mdfile.is_none()) {
			continue;
		}
		let mdfile = mdfile.unwrap();
		context.insert(&format!("**note {}**", index), &mdfile.to_string());
		notes.push_str(&mdfile.to_string());
		notes.push_str("\n\n");
	}
	context.insert("notes", &notes);
	prompt = prompt.substitute(&context).unwrap();
	let file = driver.chat_smart(prompt).await.unwrap();
	MDFile::from_string(file)
}