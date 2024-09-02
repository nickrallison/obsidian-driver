mod ai_api;
mod obsidian_file_io;
mod prelude;
mod error;

use std::path::PathBuf;
use crate::ai_api::AIDriver;


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

# HarrisAndHarris_DigitalLogicGlitches

2.9.2 Glitches

So far we have discussed the case where a single input transition causes a single output transition. However, it is possible that a single input transition can cause multiple output transitions. These are called glitches or hazards. Although glitches usually don’t cause problems, it is important to realize that they exist and recognize them when looking at timing diagrams. Figure 2.75 shows a circuit with a glitch and the Karnaugh map of the circuit.

The Boolean equation is correctly minimized, but let’s look at what happens when A = 0, C = 1, and B transitions from 1 to 0. Figure 2.76 (see page 94) illustrates this scenario. The short path (shown in gray) goes through two gates, the AND and OR gates. The critical path (shown in blue) goes through an inverter and two gates, the AND and OR gates.

Hazards have another meaning related to microarchitecture
in Chapter 7, so we will stick with the term glitches for multiple output transitions to avoid confusion.

S0 D0

D1

D2

S1
Y

A B

C

Y

S1 S0

2.9 Timing

93

S1 S0

D0 D1 D2 D3

D0

D1

D2

Out

Figure 2.73 4:1 multiplexer propagation delays:
(a) two-level logic,
(b) tristate

Out
tpd_sy = tpd_INV + tpd_AND3 + tpd_OR4

=30 ps+80 ps+90 ps (a) = 200 ps

tpd_dy = tpd_AND3 + tpd_OR4 = 170 ps

(b)

D3
tpd_sy = tpd_INV + tpd_AND2 + tpd_TRI_sy

=30 ps+60 ps+35 ps

= 125 ps tpd_dy = tpd_TRI_ay

= 50 ps

2:1 mux

Y AB C

|   |   |   |   |
|---|---|---|---|
|1|0|0|0|
|1|1|1|0|

D3
tpd_s0y = tpd_TRLSY + tpd_TRI_AY = 85 ns

tpd_dy = 2 tpd_TRI_AY = 100 ns
Figure 2.74 4:1 multiplexer propagation

delays: hierarchical using 2:1 multiplexers

00 01 11 10 0

1

Y=AB+BC
Figure 2.75 Circuit with a glitch

2:1 mux

2:1 mux

As B transitions from 1 to 0, n2 (on the short path) falls before n1 (on the critical path) can rise. Until n1 rises, the two inputs to the OR gate are 0, and the output Y drops to 0. When n1 eventually rises, Y returns to 1. As shown in the timing diagram of Figure 2.76, Y starts at 1 and ends at 1 but momentarily glitches to 0.

94 CHAPTER TWO

Combinational Logic Design

A=0 01 B=1 0 n1

n2 C=1 10

Short Path

B

n2

n1

Y

Time

Y=101

Critical Path

Figure 2.76 Timing of a glitch

glitch

As long as we wait for the propagation delay to elapse before we depend on the output, glitches are not a problem, because the output eventually settles to the right answer.

If we choose to, we can avoid this glitch by adding another gate to the implementation. This is easiest to understand in terms of the K-map. Figure 2.77 shows how an input transition on B from ABC=001 to ABC = 011 moves from one prime implicant circle to another. The transition across the boundary of two prime implicants in the K-map indicates a possible glitch.

As we saw from the timing diagram in Figure 2.76, if the circuitry implementing one of the prime implicants turns off before the circuitry of the other prime implicant can turn on, there is a glitch. To fix this, we add another circle that covers that prime implicant boundary, as shown in Figure 2.78. You might recognize this as the consensus theorem, where the added term, AC, is the consensus or redundant term.

1

|   |   |   |   |
|---|---|---|---|
|1|0|0|0|
|1|1|1|0|

Figure 2.77 Input change crosses implicant boundary

00 01 11 10 0

Y AB C

Y=AB+BC

Y AB C

00 01 11 10 0

1

AC

A=0 B=1 0

C=1

Figure 2.78 K-map without glitch

2.10 Summary 95

|   |   |   |   |
|---|---|---|---|
|1|0|0|0|
|1|1|1|0|

Y=AB+BC+AC

Y=1

Figure 2.79 Circuit without glitch

Figure 2.79 shows the glitch-proof circuit. The added AND gate is highlighted in blue. Now a transition on B when A = 0 and C = 1 does not cause a glitch on the output, because the blue AND gate outputs 1 throughout the transition.

In general, a glitch can occur when a change in a single variable crosses the boundary between two prime implicants in a K-map. We can eliminate the glitch by adding redundant implicants to the K-map to cover these boundaries. This of course comes at the cost of extra hardware.

However, simultaneous transitions on multiple inputs can also cause glitches. These glitches cannot be fixed by adding hardware. Because the vast majority of interesting systems have simultaneous (or nearsimultaneous) transitions on multiple inputs, glitches are a fact of life in most circuits. Although we have shown how to eliminate one kind of glitch, the point of discussing glitches is not to eliminate them but to be aware that they exist. This is especially important when looking at timing diagrams on a simulator or oscilloscope.


**PS**
Since this is transcript, please ignore any details that may be missed due to this format (ignore mentions of images, or gestures and such). Also ignore non lecture material like advertisements, or personal sponsorships
Also do not ever directly use any non-ASCII characters in these notes."#;

#[tokio::main]
async fn main() {

	let ai_driver = AIDriver::new_openai_from_config_path(PathBuf::from(OPENAI_CONFIG_FILE));
	let prompt = ai_api::prompt::Prompt::new(SYSTEM_PROMPT, USER_PROMPT, 1000);
	let response = ai_driver.chat_smart(prompt.clone()).await.unwrap();
	println!("{}", prompt);

	println!("#### Response ####\n{}", response);

}