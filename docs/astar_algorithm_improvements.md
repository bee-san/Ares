# A* Search Algorithm Improvements

This document outlines the improvements made to the A* search algorithm used in ciphey for finding the correct sequence of decoders to decode encrypted or encoded text, as well as potential future enhancements.

## Implemented Improvements

### 1. Decoder-Specific Nodes

We've implemented decoder-specific nodes by adding a `next_decoder_name` field to the `AStarNode` struct:

```rust
struct AStarNode {
    // Existing fields
    state: DecoderResult,
    cost: u32,
    heuristic: f32,
    total_cost: f32,
    
    // New field
    next_decoder_name: Option<String>,
}
```

The A* search algorithm now creates nodes with a specific decoder to try next. This makes the search more focused and efficient by avoiding the need to try all decoders for each state. Instead, the algorithm creates multiple nodes for each successful decoding, each with a different decoder to try next.

When creating new nodes, we set the `next_decoder_name` field to the name of the decoder that produced the result:

```rust
let new_node = AStarNode {
    state: DecoderResult {
        text,
        path: decoders_used,
    },
    cost,
    heuristic,
    total_cost,
    next_decoder_name: Some(r.decoder.to_string()),
};
```

When expanding a node, we check if it has a specific decoder name, and if so, we filter the available decoders to only include that one:

```rust
if let Some(decoder_name) = &current_node.next_decoder_name {
    // If we have a specific decoder name, filter all decoders to only include that one
    trace!("Using specific decoder: {}", decoder_name);
    let mut all_decoders = get_all_decoders();
    all_decoders.components.retain(|d| d.get_name() == decoder_name);
    
    // Update stats for the decoder
    if !all_decoders.components.is_empty() {
        update_decoder_stats(decoder_name, true);
    }
    decoders = all_decoders;
} else {
    decoders = get_decoder_tagged_decoders(&current_node.state);
}
```

This approach avoids the need to clone trait objects, which would be required if we stored the decoder itself in the node.

### 2. Simplified Heuristic Function

We've simplified the heuristic function to have three main components:

1. **Popularity Component**: Uses a fixed value of 0.5 due to trait limitations (ideally would use `1.0 - decoder.popularity`)
2. **Depth Penalty**: Adds an exponential penalty of `(0.05 * path.len() as f32).powi(2)` to discourage very deep paths
3. **Uncommon Sequence Penalty**: Adds a fixed penalty of 0.25 for uncommon decoder sequences

This simplified heuristic is more intuitive and easier to understand, while still providing good guidance for the search.

## Future Improvements

While the current implementation is a significant improvement over the original, there are still several enhancements that could make the algorithm even more efficient:

### 1. ✅ Expose Decoder Popularity in the Trait (Implemented)

### 2. ✅ Adaptive Depth Penalty (Implemented)

### 3. ✅ Incorporate String Quality (Implemented)

### 4. Learning-Based Sequence Penalties

Instead of a fixed penalty for uncommon sequences, we could learn from successful decodings:

```rust
// Store successful decoder sequences in a global cache
static SUCCESSFUL_SEQUENCES: Lazy<Mutex<HashMap<(String, String), usize>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));

// In the heuristic function
if path.len() > 1 {
    if let Some(previous_decoder) = path.last() {
        if let Some(next_decoder) = next_decoder {
            let key = (previous_decoder.decoder.to_string(), next_decoder.get_name().to_string());
            let success_count = SUCCESSFUL_SEQUENCES.lock().unwrap().get(&key).copied().unwrap_or(0);
            
            if success_count > 0 {
                // Reward sequences that have been successful before
                base_score -= 0.1 * (success_count as f32).min(5.0);
            } else if !is_common_sequence(previous_decoder.decoder, next_decoder.get_name()) {
                base_score += 0.25;
            }
        }
    }
}
```

### 5. Beam Search Variant

We could implement a beam search variant that only keeps the k most promising nodes at each level:

```rust
// In the A* search function
// After adding all new nodes to the open set
if open_set.len() > MAX_BEAM_WIDTH {
    // Keep only the MAX_BEAM_WIDTH most promising nodes
    open_set = open_set.into_sorted_vec().into_iter().take(MAX_BEAM_WIDTH).collect();
}
```

### 6. Caching Mechanism

Implement a caching mechanism to avoid recomputing heuristics for the same state:

```rust
// Global cache for heuristic values
static HEURISTIC_CACHE: Lazy<Mutex<HashMap<String, f32>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));

// In the heuristic function
let cache_key = format!("{}:{}", text, path.len());
if let Some(cached_value) = HEURISTIC_CACHE.lock().unwrap().get(&cache_key) {
    return *cached_value;
}

// Compute the heuristic
// ...

// Cache the result
HEURISTIC_CACHE.lock().unwrap().insert(cache_key, base_score);
```

### 7. Parallel Node Expansion

Modify the algorithm to expand multiple promising nodes in parallel:

```rust
// Instead of processing one node at a time
let current_nodes: Vec<_> = open_set.iter().take(PARALLEL_EXPANSION_COUNT).collect();
let results: Vec<_> = current_nodes.par_iter().map(|node| {
    // Process node
    // ...
}).collect();
```

## Impact Analysis and Implementation Priority

These improvements would make the A* search algorithm more efficient and effective at finding the correct decoding path:

1. **✅ Exposing Decoder Popularity**: High impact, as it allows the algorithm to properly prioritize more popular decoders.
2. **✅ Adaptive Depth Penalty**: Medium impact, as it helps prevent the algorithm from going too deep in unproductive paths.
3. **✅ String Quality Component**: High impact, as it helps the algorithm focus on paths that lead to higher quality strings.
4. **Learning-Based Sequence Penalties**: Medium impact, as it would help the algorithm learn from past successes.
5. **Beam Search Variant**: Medium impact, as it would help focus computational resources on the most promising paths.
6. **Caching Mechanism**: Medium impact, as it would help avoid redundant computations.
7. **Parallel Node Expansion**: High impact, as it would allow the algorithm to explore multiple paths simultaneously.

Based on the impact analysis, the recommended implementation order is:

1. ✅ Expose Decoder Popularity in the Trait (Implemented)
2. ✅ Adaptive Depth Penalty (Implemented)
3. ✅ Incorporate String Quality (Implemented)
4. Adaptive Depth Penalty
5. Learning-Based Sequence Penalties
6. Caching Mechanism
7. Beam Search Variant

## Conclusion

The A* search algorithm has been significantly improved with decoder-specific nodes, a simplified heuristic function, the use of decoder popularity in the heuristic, an adaptive depth penalty, and a string quality component. These changes make the search more focused and efficient, allowing it to find the correct sequence of decoders more quickly.

However, there is still room for further improvement. By implementing the enhancements described above, we can make the algorithm even more efficient and effective at finding the correct decoding path.

For a more detailed description of the decoder-specific nodes implementation, see `docs/astar_decoder_specific_nodes.md`.