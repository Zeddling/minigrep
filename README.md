# Minigrep
A simple search tool for querying words in a text file.

### Requirements
1.  Download <a href="Zeddling/Califorinia_Housing_Predictions">Rust</a>

### Run
1. Ensure tests are successful. Move to minigrep subdirectory and run.<br><code>cargo test</code>
2. Run <code>cargo run **query** **path-to-file** > output.txt</code> in the main directory. Where:
I)  query -> word/phrase to be querried
II) path-to-file -> path to the file in question

##### ENVIRONMENT VARIABLES
**CASE_INSENSITIVE**: By default the such is case sensitive. Setting this argument to any number renders the search case insensitive.