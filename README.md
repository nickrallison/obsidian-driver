# Obsidian Driver

This is a rust driver for Obsidian. It is meant to provide reusable interfaces for common functionality like:
1. Reading and Writing files / directories
2. Calls to the OpenAI API
   1. Creating New Files
   2. Modifying the current File
   3. Appending to the current File
3. Using Embeddings

Planned optimizations include:
1. File Caching on load

Some more advanced functionality is also planned but not implemented as of now like:
1. Finding Similar Notes with embeddings
2. Merging Similar Notes found via embeddings
3. Prompt structs for OpenAI API customization
4. Transcribing different information formats like the following into markdown:
   1. Files
      1. Text Files (duh)
      2. PDFs
      3. MP3s / Other Audio
      4. MKVs / Other Video
   2. Weblinks
      1. Youtube
      2. Other Text Links