# Change: Add WaitAthena Checker for Collecting Multiple Plaintexts

## Purpose
Implement a variant of the Athena checker that collects all potential plaintexts found during the search instead of exiting immediately when the first plaintext is found. This allows users to see all possible interpretations of their ciphertext, which is particularly useful for ambiguous encodings or when multiple valid plaintexts might exist.

## Trade-offs
### Advantages
- Provides users with multiple potential plaintexts instead of just the first one found
- Allows for more comprehensive analysis of ambiguous ciphertexts
- Maintains compatibility with all existing decoders and checkers
- Simple to use via a single command-line flag (`--top-results`)
- Automatically disables the human checker to avoid interrupting the search process
- Continues searching until the timer expires, maximizing the number of potential plaintexts found

### Disadvantages
- May take longer to complete as it continues searching even after finding valid plaintexts
- Could potentially return false positives along with true plaintexts
- Increases memory usage as all results must be stored until the timer expires

## Technical Implementation
- Created a new `WaitAthena` checker that is a variant of `Athena` but stores results instead of returning immediately
- Implemented a thread-safe storage mechanism using `Mutex` and `lazy_static` to store plaintext results
- Modified the timer module to display all collected plaintext results when the timer expires
- Added a new configuration option (`top_results`) to enable WaitAthena mode
- Added a new command-line flag (`--top-results`) to enable WaitAthena mode
- Updated the library interface to use WaitAthena when the `top_results` option is enabled
- Automatically disabled the human checker when `--top-results` is specified to avoid interrupting the search process
- Modified the search algorithm to continue searching until the timer expires when in top_results mode

## Future Improvements
- Add filtering options for WaitAthena results to reduce false positives
- Implement sorting of results by confidence level or other metrics
- Add an option to save results to a file for later analysis
- Implement deduplication logic if duplicate plaintexts become an issue in practice 