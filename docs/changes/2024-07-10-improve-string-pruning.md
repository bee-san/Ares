# Change: Improve String Pruning for Low-Quality Inputs

## Purpose
Enhance the pruning mechanism to skip decoding of low-quality strings, which improves efficiency by avoiding wasted computation on strings that are unlikely to produce meaningful results.

## Trade-offs
### Advantages
- Reduces computational resources spent on strings unlikely to yield useful results
- Speeds up the overall decoding process by focusing on higher-quality candidates
- Prevents the search algorithm from exploring unproductive paths
- Improves memory usage by pruning low-quality strings early

### Disadvantages
- May occasionally reject valid encodings that have unusual characteristics
- Requires careful tuning of thresholds to balance efficiency and thoroughness
- Adds additional computation for quality checks (though this is minimal compared to the savings)

## Technical Implementation
- Enhanced the `check_if_string_cant_be_decoded` function to consider multiple quality factors:
  - String length (rejects strings with 2 or fewer characters)
  - Non-printable character ratio (rejects strings with >30% non-printable characters)
  - Overall string quality (rejects strings with quality score <0.2)
- Added comprehensive tests to verify the pruning behavior
- Updated documentation to explain the rationale behind each pruning criterion

## Future Improvements
- Fine-tune the thresholds based on real-world usage data
- Consider adding more sophisticated quality metrics (e.g., entropy, character distribution)
- Implement adaptive thresholds that adjust based on the search context
- Add logging to track how many strings are being pruned and why 