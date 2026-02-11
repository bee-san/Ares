# Plan: A* Search Algorithm Rewrite with Decoder-Specific Nodes

## Overview

This document outlines the steps to rewrite the A* search algorithm in `src/searchers/astar.rs` for ciphey. The goal is to improve the algorithm's efficiency by making nodes more specific about which decoder to try next:

1. Update the `AStarNode` struct to include the next decoder to use
2. Modify the A* search algorithm to use this field when determining which decoder to apply next
3. Update the node creation process to set the `next_decoder` field appropriately
4. Ensure the heuristic function works with the new node structure
5. Modify the `Crack` trait to add a `get_popularity()` method

## 1. Update Node Structure (`src/searchers/astar.rs`)

### Task: Modify the `AStarNode` struct to include the next decoder to use

```rust
struct AStarNode {
    /// Current state containing the decoded text and path of decoders used
    state: DecoderResult,

    /// Cost so far (g) - represents the depth in the search tree
    /// This increases by 1 for each decoder applied
    cost: u32,

    /// Heuristic value (h) - estimated cost to reach the goal
    heuristic: f32,

    /// Total cost (f = g + h) used for prioritization in the queue
    /// Nodes with lower total_cost are explored first
    total_cost: f32,
    
    /// The next decoder to try when this node is expanded
    /// If None, all decoders should be considered (only for the initial node)
    next_decoder: Option<Box<dyn Crack + Sync>>,
}
```

**Details:**
- Add the `next_decoder` field to the `AStarNode` struct
- This field is an `Option<Box<dyn Crack + Sync>>` that holds the specific decoder to apply when this node is expanded
- If it's `None`, it means we should consider all available decoders for the next step (this is only for the initial state)
- Add a `use crate::decoders::interface::Crack;` import to the top of the file if not already present

## 2. Modify the Crack Trait (`src/decoders/interface.rs`)

### Task: Add a `get_popularity()` method to the `Crack` trait

```rust
pub trait Crack {
    /// This function generates a new crack trait
    fn new() -> Self
    where
        Self: Sized;
    /// Crack is the function that actually does the decoding
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult;
    /// Get all tags for the current decoder
    fn get_tags(&self) -> &Vec<&str>;
    /// Get the name of the current decoder
    fn get_name(&self) -> &str;
    /// Get the popularity of the decoder (a value between 0.0 and 1.0)
    fn get_popularity(&self) -> f32;
}
```

**Details:**
- Add a new method `get_popularity()` to the `Crack` trait
- This method should return a float value between 0.0 and 1.0, representing the popularity of the decoder
- Implement this method for all decoder types, returning the value from their `popularity` field
- For example:

```rust
impl Crack for Decoder<Base64Decoder> {
    // Existing implementations...
    
    fn get_popularity(&self) -> f32 {
        self.popularity
    }
}
```

## 3. Modify A* Search Algorithm (`src/searchers/astar.rs`)

### Task: Update the main A* loop to use the next_decoder field

```rust
pub fn astar(input: String, result_sender: Sender<Option<DecoderResult>>, stop: Arc<AtomicBool>) {
    // Calculate heuristic before moving input
    let initial_heuristic = generate_heuristic(&input, &[], &None);

    let initial = DecoderResult {
        text: vec![input],
        path: vec![],
    };

    // Create initial node with no next_decoder (start with any decoder)
    let initial_node = AStarNode {
        state: initial,
        cost: 0,
        heuristic: initial_heuristic,
        total_cost: 0.0,
        next_decoder: None,
    };

    // Set to track visited states to prevent cycles
    let mut seen_strings = HashSet::new();
    let mut seen_count = 0;

    // Priority queue for open set
    let mut open_set = BinaryHeap::new();

    // Add initial node to open set
    open_set.push(initial_node);

    let mut curr_depth: u32 = 1;

    let mut prune_threshold = INITIAL_PRUNE_THRESHOLD;

    // Main A* loop
    while !open_set.is_empty() && !stop.load(std::sync::atomic::Ordering::Relaxed) {
        trace!(
            "Current depth is {:?}, open set size: {}",
            curr_depth,
            open_set.len()
        );

        // Get the node with the lowest f value (total cost)
        let current_node = open_set.pop().unwrap();

        trace!(
            "Processing node with cost {}, heuristic {}, total cost {}",
            current_node.cost,
            current_node.heuristic,
            current_node.total_cost
        );

        // Check stop signal again before processing node
        if stop.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }

        // If there is a next_decoder, use it. Otherwise, get all decoders.
        let decoders = match &current_node.next_decoder {
            Some(decoder) => {
                // We used the decoder, so update its stats
                helper_functions::update_decoder_stats(decoder.get_name(), true);
                // Create a new Decoders struct with just this decoder
                Decoders { components: vec![decoder.clone()] }
            }
            None => {
                // For the initial node or if no specific decoder is set,
                // get all available decoders
                get_all_decoders()
            }
        };

        let athena_checker = Checker::<Athena>::new();
        let checker = CheckerTypes::CheckAthena(athena_checker);
        let decoder_results = decoders.run(&current_node.state.text[0], checker);

        match decoder_results {
            MyResults::Break(res) => {
                // Handle successful decoding
                // ... (existing code for handling successful decoding)
            }
            MyResults::Continue(results_vec) => {
                // Process results and add to open set with heuristic prioritization
                trace!(
                    "Processing {} results from decoders",
                    results_vec.len()
                );

                for mut r in results_vec {
                    let mut decoders_used = current_node.state.path.clone();
                    let mut text = r.unencrypted_text.take().unwrap_or_default();

                    // Filter out strings that can't be decoded or have been seen before
                    text.retain(|s| {
                        if check_if_string_cant_be_decoded(s) {
                            // Add stats update for failed decoding
                            update_decoder_stats(r.decoder, false);
                            return false;
                        }

                        if seen_strings.insert(s.clone()) {
                            seen_count += 1;

                            // Prune the HashSet if it gets too large
                            if seen_count > prune_threshold {
                                // ... (existing pruning code)
                            }

                            true
                        } else {
                            false
                        }
                    });

                    if text.is_empty() {
                        // Add stats update for failed decoding (no valid outputs)
                        update_decoder_stats(r.decoder, false);
                        continue;
                    }

                    decoders_used.push(r.clone());

                    // Create new nodes for each available decoder
                    // This is where we create nodes with specific next_decoder values
                    let all_available_decoders = get_all_decoders();
                    
                    for next_decoder in all_available_decoders.components {
                        // Create new node with updated cost, heuristic, and next_decoder
                        let cost = current_node.cost + 1;
                        let heuristic = generate_heuristic(&text[0], &decoders_used, &Some(next_decoder.clone()));
                        let total_cost = cost as f32 + heuristic;

                        let new_node = AStarNode {
                            state: DecoderResult {
                                text: text.clone(),
                                path: decoders_used.clone(),
                            },
                            cost,
                            heuristic,
                            total_cost,
                            next_decoder: Some(next_decoder),
                        };

                        // Add to open set
                        open_set.push(new_node);
                    }

                    // Update decoder stats - mark as successful since it produced valid output
                    update_decoder_stats(r.decoder, true);
                }
            }
        }

        curr_depth += 1;
    }

    // Check if we were stopped or if we genuinely couldn't find a solution
    if stop.load(std::sync::atomic::Ordering::Relaxed) {
        trace!("A* search stopped by external signal");
    } else {
        trace!("A* search completed without finding a solution");
        result_sender.try_send(None).ok();
    }
}
```

**Details:**
- Update the initial node creation to set `next_decoder: None`
- Modify the main loop to use the `next_decoder` field to determine which decoder to apply
- When creating new nodes, set the `next_decoder` field to a specific decoder
- Create a new node for each available decoder, each with a different `next_decoder` value
- Use the `generate_heuristic` function with the appropriate `next_decoder` parameter

## 4. Update Node Creation Process

### Task: Ensure the node creation process sets the next_decoder field appropriately

The key change is in how we create new nodes after a successful decoding. Instead of creating a single node and trying all decoders when expanding it, we now create multiple nodes, each with a specific decoder to try next.

```rust
// Create new nodes for each available decoder
let all_available_decoders = get_all_decoders();

for next_decoder in all_available_decoders.components {
    // Create new node with updated cost, heuristic, and next_decoder
    let cost = current_node.cost + 1;
    let heuristic = generate_heuristic(&text[0], &decoders_used, &Some(next_decoder.clone()));
    let total_cost = cost as f32 + heuristic;

    let new_node = AStarNode {
        state: DecoderResult {
            text: text.clone(),
            path: decoders_used.clone(),
        },
        cost,
        heuristic,
        total_cost,
        next_decoder: Some(next_decoder),
    };

    // Add to open set
    open_set.push(new_node);
}
```

**Details:**
- Get all available decoders using `get_all_decoders()`
- For each decoder, create a new node with that decoder as the `next_decoder`
- Calculate the heuristic using the `generate_heuristic` function with the appropriate `next_decoder` parameter
- Add each new node to the open set

## 5. Update Heuristic Function

### Task: Update the heuristic function to use the decoder's popularity

```rust
pub fn generate_heuristic(_text: &str, path: &[CrackResult], next_decoder: &Option<Box<dyn Crack + Sync>>) -> f32 {
    let mut base_score = 0.0;

    // 1. Popularity component - directly use (1.0 - popularity)
    if let Some(decoder) = next_decoder {
        // Use the decoder's popularity via the get_popularity method
        base_score += (1.0 - decoder.get_popularity());
    } else {
        // If next decoder is None, add a moderate penalty
        base_score += 0.5;
    }

    // 2. Depth penalty - exponential growth but not too aggressive
    base_score += (0.05 * path.len() as f32).powi(2);

    // 3. Penalty for uncommon pairings
    if path.len() > 1 {
        if let Some(previous_decoder) = path.last() {
            if let Some(next_decoder) = next_decoder {
                if !is_common_sequence(previous_decoder.decoder, next_decoder.get_name()) {
                    base_score += 0.25;
                }
            }
        }
    }
    
    base_score
}
```

**Details:**
- Update the heuristic function to use the `get_popularity()` method on the decoder
- This allows us to directly use the decoder's popularity in the heuristic calculation
- The rest of the heuristic calculation remains the same

## 6. Memory Optimization Strategies

To mitigate the potential increase in memory usage from creating multiple nodes for each successful decoding, consider implementing these strategies:

### 1. Early Pruning of Low-Quality Nodes

Before creating nodes for all available decoders, filter out decoders that are unlikely to be useful:

```rust
// Filter out decoders that are unlikely to be useful
let filtered_decoders = all_available_decoders.components.into_iter()
    .filter(|decoder| {
        // Filter based on decoder properties
        // For example, only keep decoders with popularity above a threshold
        decoder.get_popularity() > 0.2
    })
    .collect::<Vec<_>>();

// Create nodes only for the filtered decoders
for next_decoder in filtered_decoders {
    // Create new node...
}
```

### 2. Beam Search Approach

Limit the number of nodes in the open set to prevent excessive memory usage:

```rust
// After adding all new nodes to the open set
if open_set.len() > MAX_BEAM_WIDTH {
    // Keep only the MAX_BEAM_WIDTH most promising nodes
    open_set = open_set.into_sorted_vec().into_iter().take(MAX_BEAM_WIDTH).collect();
}
```

### 3. Dynamic Node Creation Based on Text Quality

Create more nodes for high-quality text and fewer nodes for low-quality text:

```rust
// Calculate text quality
let quality = calculate_string_quality(&text[0]);

// Determine how many decoders to consider based on quality
let decoder_limit = if quality > 0.8 {
    // High-quality text - consider all decoders
    all_available_decoders.components.len()
} else if quality > 0.5 {
    // Medium-quality text - consider top 50% of decoders by popularity
    all_available_decoders.components.len() / 2
} else {
    // Low-quality text - consider only top 25% of decoders by popularity
    all_available_decoders.components.len() / 4
};

// Sort decoders by popularity (highest first)
let mut sorted_decoders = all_available_decoders.components;
sorted_decoders.sort_by(|a, b| b.get_popularity().partial_cmp(&a.get_popularity()).unwrap_or(Ordering::Equal));

// Take only the top N decoders
let limited_decoders = sorted_decoders.into_iter().take(decoder_limit).collect::<Vec<_>>();

// Create nodes only for the limited decoders
for next_decoder in limited_decoders {
    // Create new node...
}
```

## 7. Testing

### Task: Update existing tests to account for the new node structure

```rust
#[test]
fn astar_handles_empty_input() {
    // Test that A* handles empty input gracefully
    let (tx, rx) = bounded::<Option<DecoderResult>>(1);
    let stopper = Arc::new(AtomicBool::new(false));
    astar("".into(), tx, stopper);
    let result = rx.recv().unwrap();
    assert!(result.is_none());
}

#[test]
fn astar_prevents_cycles() {
    // Test that the algorithm doesn't revisit states
    // We'll use a string that could potentially cause cycles
    let (tx, rx) = bounded::<Option<DecoderResult>>(1);
    let stopper = Arc::new(AtomicBool::new(false));

    // This is a base64 encoding of "hello" that when decoded and re-encoded
    // could potentially cause cycles if not handled properly
    astar("aGVsbG8=".into(), tx, stopper);

    // The algorithm should complete without hanging
    let result = rx.recv().unwrap();
    assert!(result.is_some());
}

#[test]
fn astar_uses_next_decoder() {
    // Test that the algorithm uses the next_decoder field
    // Create a mock decoder that we can track
    // ...
}
```

**Details:**
- Update existing tests to account for the new node structure
- Add a new test to verify that the algorithm uses the `next_decoder` field correctly
- Ensure all tests pass after the changes

## Conclusion

This plan outlines the steps to rewrite the A* search algorithm to use decoder-specific nodes. By making nodes more specific about which decoder to try next, we can make the search more efficient and focused.

The key changes are:
1. Adding a `next_decoder` field to the `AStarNode` struct
2. Modifying the `Crack` trait to add a `get_popularity()` method
3. Modifying the A* search algorithm to use the `next_decoder` field
4. Updating the node creation process to create multiple nodes, each with a specific decoder to try next
5. Updating the heuristic function to use the decoder's popularity
6. Implementing memory optimization strategies to mitigate the increased memory usage

These changes should make the A* search algorithm more efficient and effective at finding the correct decoding path.