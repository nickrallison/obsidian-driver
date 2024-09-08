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
- [ ] Using Embeddings

**Planned optimizations include:**
- [ ] File Caching on load

**Some more advanced functionality is also planned but not implemented as of now like:**
- [ ] Finding Similar Notes with embeddings
- [ ] Merging Similar Notes found via embeddings 
- [ ] Prompt structs for OpenAI API customization 
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
