# Change: Remove get_decoder_popularity Function

## Purpose
Remove the redundant `get_decoder_popularity` function from `helper_functions.rs` since decoders already have a `popularity` attribute in their implementation. This eliminates duplication and ensures that popularity values are maintained in a single location.

## Trade-offs
### Advantages
- Eliminates redundant code that duplicated popularity values
- Simplifies maintenance by having popularity values defined only in the decoder implementations
- Reduces the risk of inconsistencies between the function and the actual decoder attributes

### Disadvantages
- The `generate_heuristic` function no longer has direct access to the popularity values
- Using success rate as a proxy for popularity may not perfectly match the original behavior

## Technical Implementation
- Removed the `get_decoder_popularity` function from `helper_functions.rs`
- Modified the `generate_heuristic` function to use the decoder's success rate as a proxy for popularity
- Updated tests to verify that success rate affects the heuristic calculation
- Removed the now-obsolete `test_popularity_affects_heuristic` test

## Future Improvements
- Consider modifying the `CrackResult` struct to include the decoder's popularity attribute
- Explore ways to directly access the decoder's popularity attribute in the `generate_heuristic` function
- Evaluate whether success rate is an appropriate proxy for popularity or if another approach would be better 