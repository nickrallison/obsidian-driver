mod ai_api;
mod obsidian_file_io;
mod prelude;
mod error;

use ai_api::openai::OpenAIConfig;


const OPENAI_CONFIG_FILE: &str = "config/openai_config.json";

const SYSTEM_PROMPT: &str = r#"You are a student attending lectures
	I have the following rules:
	 - Text must formatted in markdown.
	 - Math must be formatted in LaTeX. ($$ … $$ for multiline, $ … $ for inline)
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

const USER_PROMPT: &str = r#"**User**
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
Also do not ever directly use any non-ASCII characters in these notes."#;

#[tokio::main]
async fn main() {

	let openai_config = OpenAIConfig {
		// Validation
		validation_url: "https://api.openai.com/v1/models".to_string(),

		// Models
		embedding_model: "https://api.openai.com/v1/embeddings".to_string(),
		smart_text_model: "gpt-4o".to_string(),
		cheap_text_model: "gpt-4o-mini".to_string(),
		smart_model_max_tokens: 128000,
		cheap_model_max_tokens: 128000,

		// URLs
		embedding_url: "https://api.openai.com/v1/embeddings".to_string(),
		chat_url: "https://api.openai.com/v1/chat/completions".to_string(),
		api_key: std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"),

		// Other
		characters_per_token: 4,
	};

	// serialize config to "config/openai_config.json"
	let serialized_config = serde_json::to_string_pretty(&openai_config).unwrap();
	std::fs::write(OPENAI_CONFIG_FILE, serialized_config).expect("Unable to write file");

	// let ai_driver = AIDriver::new_openai(openai_config);



}