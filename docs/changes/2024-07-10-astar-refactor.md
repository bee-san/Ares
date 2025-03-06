# Change: AStar Refactoring and String Quality Enhancement

## Purpose
Refactor the AStar search implementation to improve code organization and enhance string quality assessment by filtering out strings with high percentages of invisible characters.

## Trade-offs
### Advantages
- Improved code organization with helper functions in a separate module
- Better memory efficiency by quickly rejecting strings with >50% invisible characters
- Enhanced maintainability through clearer separation of concerns
- Easier testing of individual helper functions

### Disadvantages
- Slight increase in module complexity with an additional file
- Potential for minor performance overhead from cross-module function calls

## Technical Implementation
- Split AStar implementation into two files:
  - `astar.rs`: Core A* search algorithm implementation
  - `helper_functions.rs`: Supporting functions for heuristics, quality assessment, and statistics
- Enhanced `calculate_string_quality` function to immediately reject strings with >50% invisible characters
- Added a new test case to verify the invisible character filtering functionality
- Updated module imports and exports in `mod.rs`

## Future Improvements
- Persist decoder success statistics to disk for learning across sessions
- Further optimize string quality assessment with more sophisticated language detection
- Consider moving more common utility functions to the helper module for reuse by other search algorithms 