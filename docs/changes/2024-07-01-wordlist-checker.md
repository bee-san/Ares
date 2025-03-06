# Change: Add Wordlist Checker

## Purpose
Implement a wordlist checker that performs exact matching against a user-provided list of words. This allows users to check if the input text exactly matches any word in their custom wordlist, which is useful for targeted decoding tasks where the expected output is known to be within a specific set of words.

## Trade-offs

### Advantages
- Provides exact matching against custom wordlists
- Efficient O(1) lookups using HashSet
- Memory-mapped file handling for large wordlists (>10MB)
- Takes precedence over other checkers when specified, allowing for targeted checking
- Supports both CLI argument and config file specification

### Disadvantages
- Requires additional memory to store the wordlist
- Only performs exact matching (no partial or fuzzy matching)
- Case-sensitive matching only
- No support for multiple wordlists

## Technical Implementation
- Added `wordlist_path` and `wordlist` fields to the `Config` struct
- Implemented `load_wordlist` function using memory mapping for large files
- Created a new `WordlistChecker` that performs exact matching against the wordlist
- Updated Athena checker to prioritize wordlist checking when a wordlist is provided
- Added `--wordlist` CLI argument that takes precedence over config file
- Updated library API to accept pre-loaded wordlists

## Future Improvements
- Add support for case-insensitive matching
- Implement partial matching options
- Support multiple wordlist files
- Add progress indicator for loading large wordlists
- Implement wordlist caching
- Add support for alternative wordlist formats (CSV, JSON, etc.) 