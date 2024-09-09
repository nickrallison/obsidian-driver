# Obsidian Driver

**This is a rust driver for Obsidian. It is meant to provide reusable interfaces for common functionality like:**

- [ ] Reading and Writing files / directories
	- [x] Writing Files
	- [x] Reading Files
	- [ ] Making Directories
	- [ ] Reading Directories
- [ ] Calls to the OpenAI API
	- [ ] Creating New Files
	- [ ] Modifying the current File
	- [ ] Appending to the current File
- [x] Using Embeddings
	- [x] Nearest N Files
	- [x] All Files closer than threshold

**Planned optimizations include:**

- [x] File Caching on load
	- [x] Using Cached Files to not repeat embeddings

**Some more advanced functionality is also planned:**

- [x] Finding Similar Notes with embeddings
- [ ] Merging Similar Notes found via embeddings
- [ ] Prompt structs for OpenAI API customization
	- [ ] Either do action
	- [ ] Or print the simplified prompt
- [ ] Transcribing different information formats like the following into markdown:
	- [ ] Files
		- [ ] Text Files (duh)
		- [ ] PDFs
		- [ ] MP3s / Other Audio
		- [ ] MKVs / Other Video
	- [ ] Weblinks
		- [ ] Youtube
		- [ ] Other Text Links
- [ ] Mapping existing links between files
	- [ ] Finding links with embeddings that don't match to show possible bad links
- [ ] Recommend Tags Based on existing tags / need new tag
	- [ ] Tags recommend based on file embeddings
	- [ ] Tags recommend based on tag embeddings

**Documentation Progress**

- [ ] obsidian-driver
	- [ ] ai
		- [x] api
			- [x] openai
		- [x] prompt
	- [ ] file
		- [ ] mdfile
			- [ ] yaml
		- [ ] vault
	- [ ] error

