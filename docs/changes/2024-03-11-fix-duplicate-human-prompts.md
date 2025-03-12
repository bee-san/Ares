# Fix Duplicate Human Verification Prompts

## Issue
When running ciphey in top_results mode with parallel A* search, users would sometimes see duplicate human verification prompts for the same plaintext. This occurred because:

1. The parallel A* search could discover the same solution path multiple times
2. Each discovery would trigger Athena's checker
3. The human checker would prompt for verification each time, even for identical results

Example of duplicated prompts:
```
🕵️ I think the plaintext is Words.
Possible plaintext: 'hello this text...' (y/N):
...
🕵️ I think the plaintext is Words.
Possible plaintext: 'hello this text...' (y/N):
```

## Root Cause Analysis
The issue stemmed from multiple factors:
1. Parallel processing in A* search allowing multiple threads to find the same solution
2. Top_results mode continuing the search after finding a valid result
3. No deduplication of human verification prompts
4. State being maintained separately in each Athena checker instance

## Solution
Added prompt deduplication to the human checker using a thread-safe cache:

```rust
use dashmap::DashSet;
use std::sync::OnceLock;

static SEEN_PROMPTS: OnceLock<DashSet<String>> = OnceLock::new();

fn get_seen_prompts() -> &'static DashSet<String> {
    SEEN_PROMPTS.get_or_init(|| DashSet::new())
}
```

The human checker now checks if it has already prompted for a given plaintext:
```rust
let prompt_key = format!("{}{}", input.description, input.text);
if !get_seen_prompts().insert(prompt_key) {
    println!("DEBUG: Skipping duplicate human verification prompt");
    return true;  // Return true to allow the search to continue
}
```

Benefits of this approach:
1. Thread-safe using DashSet
2. Minimal code changes required
3. Maintains existing functionality while eliminating duplicates
4. Works regardless of which code path triggered the verification

## Alternative Approaches Considered
1. Result deduplication in A* search - Too late, prompts already shown
2. Modifying Athena checker - More complex, required state management
3. Disabling parallel processing - Would impact performance
4. Disabling top_results mode - Would limit functionality

The chosen solution provides the best balance of:
- Minimal code changes
- No performance impact
- Preserved functionality
- Clean user experience