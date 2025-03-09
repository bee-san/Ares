# Change: Make Sensitivity an Optional Trait

## Purpose
Implement an optional `SensitivityAware` trait for checkers that use sensitivity for gibberish detection. This separates the sensitivity functionality from the core `Check` trait, allowing checkers like the WordlistChecker to avoid implementing sensitivity-related methods that they don't actually use.

## Trade-offs

### Advantages
- Cleaner separation of concerns between core checking functionality and sensitivity handling
- Checkers that don't use sensitivity don't need to implement unused methods
- More accurate representation of which checkers actually use sensitivity
- Reduces code duplication and improves maintainability
- Makes it clearer to developers which checkers support sensitivity adjustment

### Disadvantages
- Requires changes to existing code that assumes all checkers implement sensitivity methods
- Slightly more complex trait hierarchy
- Requires careful handling in composite checkers like Athena

## Technical Implementation
- Created a new `SensitivityAware` trait in `checker_type.rs` with the sensitivity-related methods
- Removed sensitivity methods from the core `Check` trait
- Updated the WordlistChecker to not implement the `SensitivityAware` trait
- Updated the Athena checker to handle both sensitivity-aware and non-sensitivity-aware checkers
- Kept the sensitivity field in the `Checker` struct for backward compatibility
- Added documentation to clarify which checkers use sensitivity

## Future Improvements
- Implement the `SensitivityAware` trait for all checkers that actually use sensitivity
- Add runtime detection of whether a checker implements `SensitivityAware`
- Consider making the sensitivity field optional in the `Checker` struct
- Add helper methods to safely apply sensitivity only to checkers that support it
- Update documentation to clearly indicate which checkers support sensitivity adjustment 