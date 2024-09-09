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

    let (file, title) = future::join(driver.chat_smart(file_prompt), driver.chat_smart(title_prompt)).await;
    let title = title?;
    let file = file?;

    let mdfile: MDFile = MDFile::from_string(file);
    let path = output_folder.join(title);
    let file = crate::file::File::from_mdfile(path, mdfile);
    Ok(file)
}

const DEFAULT_SYSTEM_MERGE_PROMPT: &str = r#"
You are a student attending lectures
	I have the following rules:
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
	 - I must avoid formatting with inline code blocks: `[code]`, and prefer to use multiline code blocks"#;

const DEFAULT_USER_MERGE_PROMPT: &str = r#"
**User**
Below is a transcript of a lecture. Take detailed digital Notes on the topic. Use longer sections if you have the material to make it longer, but each subsection should be at least paragraph, Aim for 150 words per subsection, that is an appropriate size for a paragraph.

Be as detailed as possible. If an interesting word is used in the transcript, make a note of it, if a conflict or event happens, mention who was involved, what was the outcome...

If it is applicable, try to show examples (with code blocks, latex blocks). "An example provided in the lecture demonstrates a basic declarative macro `sayhello`, which expands into a print statement when compiled" is not preferred when you could use a code block of an example. Don't Talk about examples, show them, or come up with your own if you have to.

Finally you must include a 'take-aways' section at the end of the note. Include the most relevant items for exam review in a list format. Don't be afraid to make it long, longer and mentioning every take away is better than shorter and missing something.

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

**Note Template**
# [title]

notes subheadings, ...

**Transcript**
[current-file-contents]

**PS**
Since this is transcript, please ignore any details that may be missed due to this format (ignore mentions of images, or gestures and such). Also ignore non lecture material like advertisements, or personal sponsorships
Also do not ever directly use any non-ASCII characters in these notes.
"#;

const DEFAULT_SYSTEM_MERGE_TITLE_PROMPT: &str = r#"
**User**
Below is a transcript of a lecture. Provide a short title for a note on this topic. Shorted is better, aim for 4 or fewer words, 8 or more is too many. Avoid words that add nothing. "Rust Lifetimes" > "Understanding Rust Lifetimes" > "How Rust Lifetimes Work"

That comes with the caveat that you should not lose important detail to shorten the prompt. "All Quiet on The Western Front Summary" > "WW1 Summary"

This will be a filename, so only use characters which can make a valid filename (Spaces are fine, but no slashes, or anything that would actually make it invalid…)

**Transcript**
[current-file-contents]

**Response Format**
(Short Title)
"#;


pub async fn merge_files(files: Vec<crate::file::File>, output_folder: PathBuf) -> Result<crate::file::File> {
    todo!()
}

