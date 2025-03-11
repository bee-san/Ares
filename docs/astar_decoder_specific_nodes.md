# A* Search Algorithm with Decoder-Specific Nodes

## Overview

This document describes the implementation of the A* search algorithm with decoder-specific nodes in ciphey. This enhancement makes the search more efficient by making nodes more specific about which decoder to try next.

## Background

The A* search algorithm is a best-first search algorithm that uses a heuristic function to prioritize which paths to explore. In the context of ciphey, the A* algorithm is used to find the correct sequence of decoders to decode encrypted or encoded text.

In the original implementation, each node in the search tree represented a state with a decoded text and a path of decoders used to reach that state. When expanding a node, the algorithm would try all available decoders on the decoded text, which could be inefficient.

## Decoder-Specific Nodes

The enhanced implementation adds a `next_decoder_name` field to the `AStarNode` struct, which specifies which decoder to try when expanding the node. This makes the search more focused and efficient by avoiding the need to try all decoders for each state.

### Node Structure

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
    
    /// The name of the next decoder to try when this node is expanded
    next_decoder_name: Option<String>,
}
```

### Node Creation

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

This creates a node that specifies which decoder to try next. When this node is expanded, the algorithm will only try the specified decoder, rather than all available decoders.

### Node Expansion

When expanding a node, we check if it has a specific decoder name, and if so, we filter the available decoders to only include that one:

```rust
// Determine which decoders to use based on next_decoder_name
let mut decoders;
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

## Benefits

The decoder-specific nodes approach has several benefits:

1. **More Efficient Search**: By specifying which decoder to try next, the algorithm can focus on the most promising paths and avoid wasting time on unpromising ones.

2. **Better Heuristic Guidance**: The heuristic function can now guide not just which node to expand next, but also which decoder to try from that node.

3. **Reduced Computational Overhead**: The algorithm no longer needs to try all decoders for each state, which can significantly reduce the computational overhead.

4. **Improved Scalability**: As the number of decoders increases, the efficiency gains from decoder-specific nodes become more significant.

## Limitations and Future Work

While the decoder-specific nodes approach improves the efficiency of the A* search algorithm, there are still some limitations and areas for future work:

1. **Trait Limitations**: The current implementation uses the decoder's name to identify it, rather than storing the decoder itself, due to limitations with cloning trait objects. A more elegant solution would be to modify the `Crack` trait to support cloning.

2. **Heuristic Function**: The heuristic function has been improved to use the decoder's popularity and an adaptive depth penalty. It could be further enhanced to incorporate the quality of the decoded text or learn from past successes.

3. **Parallel Expansion**: The algorithm could be modified to expand multiple nodes in parallel, which would take advantage of multi-core processors.

4. **Memory Optimization**: As the search progresses, the number of nodes in the open set can grow significantly. Techniques like beam search or pruning could be used to limit the memory usage.

## Conclusion

The A* search algorithm with decoder-specific nodes is a significant improvement over the original implementation. By making nodes more specific about which decoder to try next, the algorithm can more efficiently find the correct sequence of decoders to decode encrypted or encoded text.