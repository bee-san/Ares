# Parallel Node Expansion Implementation for A* Search

This document provides a step-by-step guide to implement parallel node expansion in the A* search algorithm in the Ares project.

## 1. Dependencies

First, add the necessary dependencies to `Cargo.toml`:

```toml
[dependencies]
# Existing dependencies...
rayon = "1.7.0"           # For parallel processing
crossbeam = "0.8.2"       # For concurrent data structures
dashmap = "5.4.0"         # For concurrent hash maps/sets
```

## 2. Imports

Add the following imports to the top of `src/searchers/astar.rs`:

```rust
use rayon::prelude::*;
use crossbeam::queue::SegQueue;
use dashmap::DashSet;
use std::sync::Mutex;
use std::cmp::Reverse;
```

## 3. Constants

Add a new constant to control the batch size for parallel processing:

```rust
/// Number of nodes to process in parallel
const PARALLEL_BATCH_SIZE: usize = 10;
```

## 4. Data Structure Modifications

Replace the existing `HashSet` with a thread-safe `DashSet`:

```rust
// Replace this:
let mut seen_strings = HashSet::new();

// With this:
let seen_strings = DashSet::new();
```

Create a thread-safe wrapper for the priority queue:

```rust
// Define a thread-safe priority queue wrapper
struct ThreadSafePriorityQueue {
    queue: Mutex<BinaryHeap<AStarNode>>,
}

impl ThreadSafePriorityQueue {
    fn new() -> Self {
        ThreadSafePriorityQueue {
            queue: Mutex::new(BinaryHeap::new()),
        }
    }

    fn push(&self, node: AStarNode) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(node);
    }

    fn pop(&self) -> Option<AStarNode> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop()
    }

    fn is_empty(&self) -> bool {
        let queue = self.queue.lock().unwrap();
        queue.is_empty()
    }

    fn len(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        queue.len()
    }

    // Extract a batch of nodes with highest priority
    fn extract_batch(&self, batch_size: usize) -> Vec<AStarNode> {
        let mut queue = self.queue.lock().unwrap();
        let mut batch = Vec::with_capacity(batch_size);
        
        for _ in 0..batch_size {
            if let Some(node) = queue.pop() {
                batch.push(node);
            } else {
                break;
            }
        }
        
        batch
    }
}
```

## 5. Node Expansion Function

Create a separate function for node expansion:

```rust
/// Expands a single node and returns a vector of new nodes
fn expand_node(
    current_node: &AStarNode,
    seen_strings: &DashSet<String>,
    stop: &Arc<AtomicBool>,
    prune_threshold: usize,
) -> Vec<AStarNode> {
    let mut new_nodes = Vec::new();
    
    // Check stop signal
    if stop.load(std::sync::atomic::Ordering::Relaxed) {
        return new_nodes;
    }
    
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

    // Prevent reciprocal decoders from being applied consecutively
    if let Some(last_decoder) = current_node.state.path.last() {
        if last_decoder.checker_description.contains("reciprocal") {
            let excluded_name = last_decoder.decoder;
            decoders
                .components
                .retain(|d| d.get_name() != excluded_name);
        }
    }

    if !decoders.components.is_empty() {
        trace!(
            "Found {} decoder-tagged decoders to execute immediately",
            decoders.components.len()
        );

        // Check stop signal before processing decoders
        if stop.load(std::sync::atomic::Ordering::Relaxed) {
            return new_nodes;
        }

        let athena_checker = Checker::<Athena>::new();
        let checker = CheckerTypes::CheckAthena(athena_checker);
        let decoder_results = decoders.run(&current_node.state.text[0], checker);

        // Process decoder results
        match decoder_results {
            MyResults::Break(res) => {
                // Handle successful decoding
                // This part remains mostly unchanged, but instead of sending results directly,
                // we'll return a special marker node that indicates a successful result
                if res.success {
                    let mut decoders_used = current_node.state.path.clone();
                    let text = res.unencrypted_text.clone().unwrap_or_default();
                    decoders_used.push(res.clone());
                    
                    // Create a special "result" node with a very low total_cost to ensure it's processed first
                    let result_node = AStarNode {
                        state: DecoderResult {
                            text: text.clone(),
                            path: decoders_used,
                        },
                        cost: current_node.cost + 1,
                        heuristic: -1000.0, // Very negative to ensure highest priority
                        total_cost: -1000.0, // Very negative to ensure highest priority
                        next_decoder_name: Some("__RESULT__".to_string()), // Special marker
                    };
                    
                    new_nodes.push(result_node);
                }
            }
            MyResults::Continue(results) => {
                // Process each result
                for r in results {
                    // Skip if stop signal is set
                    if stop.load(std::sync::atomic::Ordering::Relaxed) {
                        break;
                    }

                    // Clone path to avoid modifying the original
                    let mut decoders_used = current_node.state.path.clone();
                    
                    // Get decoded text
                    let text = r.unencrypted_text.clone().unwrap_or_default();
                    
                    // Skip if text is empty or already seen
                    if text.is_empty() {
                        update_decoder_stats(r.decoder, false);
                        continue;
                    }
                    
                    // Check if we've seen this string before to prevent cycles
                    let text_hash = calculate_hash(&text[0]);
                    if !seen_strings.insert(text_hash) {
                        update_decoder_stats(r.decoder, false);
                        continue;
                    }
                    
                    decoders_used.push(r.clone());
                    
                    // Create new node with updated cost and heuristic
                    let cost = current_node.cost + 1;
                    let heuristic = generate_heuristic(&text[0], &decoders_used, &None);
                    let total_cost = cost as f32 + heuristic;
                    
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
                    
                    // Add to new nodes
                    new_nodes.push(new_node);
                    
                    // Update decoder stats - mark as successful since it produced valid output
                    update_decoder_stats(r.decoder, true);
                }
            }
            MyResults::None => {
                // No results, continue with other decoders
            }
        }
    }
    
    // If no decoder-tagged decoders or they didn't produce results,
    // try all available decoders
    if new_nodes.is_empty() {
        // This part remains similar to the original implementation
        // but adapted to return nodes instead of adding them to open_set
        
        // Get all decoders
        let all_decoders = get_all_decoders();
        
        // Process each decoder
        for decoder in all_decoders.components {
            // Skip if stop signal is set
            if stop.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
            
            // Skip decoders that were already tried
            if let Some(last_decoder) = current_node.state.path.last() {
                if last_decoder.decoder == decoder.get_name() {
                    continue;
                }
                
                // Skip reciprocal decoders if the last one was reciprocal
                if last_decoder.checker_description.contains("reciprocal") 
                   && last_decoder.decoder == decoder.get_name() {
                    continue;
                }
            }
            
            // Run the decoder
            let result = decoder.decode(&current_node.state.text[0]);
            
            // Process the result
            if let Some(decoded_text) = result {
                // Skip if text is empty
                if decoded_text.is_empty() {
                    update_decoder_stats(decoder.get_name(), false);
                    continue;
                }
                
                // Check if we've seen this string before
                let text_hash = calculate_hash(&decoded_text);
                if !seen_strings.insert(text_hash) {
                    update_decoder_stats(decoder.get_name(), false);
                    continue;
                }
                
                // Create decoder result
                let mut decoders_used = current_node.state.path.clone();
                let decoder_result = DecoderResult {
                    text: vec![decoded_text.clone()],
                    decoder: decoder.get_name().to_string(),
                    checker_name: "".to_string(),
                    checker_description: "".to_string(),
                    success: false,
                    unencrypted_text: Some(vec![decoded_text]),
                };
                
                decoders_used.push(decoder_result.clone());
                
                // Create new node
                let cost = current_node.cost + 1;
                let heuristic = generate_heuristic(&decoded_text, &decoders_used, &None);
                let total_cost = cost as f32 + heuristic;
                
                let new_node = AStarNode {
                    state: DecoderResult {
                        text: vec![decoded_text],
                        path: decoders_used,
                    },
                    cost,
                    heuristic,
                    total_cost,
                    next_decoder_name: Some(decoder.get_name().to_string()),
                };
                
                // Add to new nodes
                new_nodes.push(new_node);
                
                // Update decoder stats
                update_decoder_stats(decoder.get_name(), true);
            } else {
                // Update decoder stats for failed decoding
                update_decoder_stats(decoder.get_name(), false);
            }
        }
    }
    
    new_nodes
}
```

## 6. Modified A* Search Function

Replace the main A* search function with this parallel version:

```rust
pub fn astar(input: String, result_sender: Sender<Option<DecoderResult>>, stop: Arc<AtomicBool>) {
    // Calculate heuristic before moving input
    let initial_heuristic = generate_heuristic(&input, &[], &None);

    let initial = DecoderResult {
        text: vec![input],
        path: vec![],
    };

    // Thread-safe set to track visited states to prevent cycles
    let seen_strings = DashSet::new();
    let seen_count = Arc::new(AtomicUsize::new(0));

    // Thread-safe priority queue for open set
    let open_set = ThreadSafePriorityQueue::new();

    // Add initial node to open set
    open_set.push(AStarNode {
        state: initial,
        cost: 0,
        heuristic: initial_heuristic,
        total_cost: 0.0,
        next_decoder_name: None,
    });

    let curr_depth = Arc::new(AtomicU32::new(1));
    let prune_threshold = Arc::new(AtomicUsize::new(INITIAL_PRUNE_THRESHOLD));

    // Main A* loop
    while !open_set.is_empty() && !stop.load(std::sync::atomic::Ordering::Relaxed) {
        trace!(
            "Current depth is {:?}, open set size: {}",
            curr_depth.load(std::sync::atomic::Ordering::Relaxed),
            open_set.len()
        );

        // Extract a batch of nodes to process in parallel
        let batch_size = std::cmp::min(PARALLEL_BATCH_SIZE, open_set.len());
        let batch = open_set.extract_batch(batch_size);
        
        trace!("Processing batch of {} nodes in parallel", batch.len());
        
        // Process nodes in parallel
        let new_nodes: Vec<AStarNode> = batch.par_iter()
            .flat_map(|node| {
                expand_node(
                    node, 
                    &seen_strings, 
                    &stop, 
                    prune_threshold.load(std::sync::atomic::Ordering::Relaxed)
                )
            })
            .collect();
        
        // Check for result nodes
        for node in &new_nodes {
            if let Some(decoder_name) = &node.next_decoder_name {
                if decoder_name == "__RESULT__" {
                    // Found a result node
                    decoded_how_many_times(curr_depth.load(std::sync::atomic::Ordering::Relaxed));
                    
                    cli_pretty_printing::success(&format!(
                        "DEBUG: astar.rs - Sending successful result with {} decoders", 
                        node.state.path.len()
                    ));
                    
                    // If in top_results mode, store the result in the WaitAthena storage
                    if get_config().top_results {
                        // Store the first text in the vector (there should only be one)
                        if let Some(plaintext) = node.state.text.first() {
                            // Get the last decoder used
                            let decoder_name =
                                if let Some(last_decoder) = node.state.path.last() {
                                    last_decoder.decoder.to_string()
                                } else {
                                    "Unknown".to_string()
                                };

                            // Get the checker name from the last decoder
                            let checker_name =
                                if let Some(last_decoder) = node.state.path.last() {
                                    last_decoder.checker_name.to_string()
                                } else {
                                    "Unknown".to_string()
                                };

                            // Only store results that have a valid checker name
                            if !checker_name.is_empty() && checker_name != "Unknown" {
                                log::trace!(
                                    "Storing plaintext in WaitAthena storage: {} (decoder: {}, checker: {})",
                                    plaintext,
                                    decoder_name,
                                    checker_name
                                );
                                wait_athena_storage::add_plaintext_result(
                                    plaintext.clone(),
                                    format!("Decoded successfully at depth {}", 
                                        curr_depth.load(std::sync::atomic::Ordering::Relaxed)),
                                    checker_name,
                                    decoder_name,
                                );
                            }
                        }
                    }
                    
                    // Send the result
                    result_sender
                        .send(Some(node.state.clone()))
                        .expect("Should successfully send the result");
                    
                    // Only stop if not in top_results mode
                    if !get_config().top_results {
                        // Stop further iterations
                        stop.store(true, std::sync::atomic::Ordering::Relaxed);
                        return;
                    }
                    // In top_results mode, continue searching
                }
            }
        }
        
        // Filter out result nodes and add remaining nodes to open set
        for node in new_nodes {
            if let Some(decoder_name) = &node.next_decoder_name {
                if decoder_name != "__RESULT__" {
                    open_set.push(node);
                }
            } else {
                open_set.push(node);
            }
        }
        
        // Update current depth based on the nodes in the open set
        if let Some(top_node) = open_set.pop() {
            let new_depth = top_node.cost;
            curr_depth.store(new_depth, std::sync::atomic::Ordering::Relaxed);
            
            // Put the node back
            open_set.push(top_node);
            
            // Prune seen strings if we've accumulated too many
            let current_seen_count = seen_strings.len();
            if current_seen_count > prune_threshold.load(std::sync::atomic::Ordering::Relaxed) {
                // Prune seen strings (implementation depends on how you want to handle this)
                // This is a simplified version - you might want a more sophisticated approach
                seen_strings.clear();
                
                // Adjust threshold based on search progress
                let progress_factor = new_depth as f32 / MAX_DEPTH as f32;
                let new_threshold = INITIAL_PRUNE_THRESHOLD - (progress_factor * 5000.0) as usize;
                prune_threshold.store(new_threshold, std::sync::atomic::Ordering::Relaxed);
                
                debug!(
                    "Pruned seen strings (new threshold: {})",
                    new_threshold
                );
            }
        }
    }
    
    // If we get here, we've exhausted all possibilities without finding a solution
    if !stop.load(std::sync::atomic::Ordering::Relaxed) {
        result_sender
            .send(None)
            .expect("Should successfully send the result");
    }
}
```

## 7. Helper Functions

Add these helper functions if they don't already exist:

```rust
/// Calculate a hash for a string to use in the seen_strings set
fn calculate_hash(text: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish().to_string()
}

/// Atomic counter for tracking how many times we've decoded
fn decoded_how_many_times(depth: u32) {
    static DECODED_COUNT: AtomicUsize = AtomicUsize::new(0);
    let count = DECODED_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    
    debug!("Decoded {} times, at depth {}", count, depth);
}
```

## 8. Testing

Add a test for the parallel A* implementation:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parallel_astar() {
        // Create channels for result communication
        let (sender, receiver) = std::sync::mpsc::channel();
        
        // Create stop signal
        let stop = Arc::new(AtomicBool::new(false));
        
        // Run A* in a separate thread
        let input = "SGVsbG8gV29ybGQ=".to_string(); // Base64 for "Hello World"
        let stop_clone = stop.clone();
        
        std::thread::spawn(move || {
            astar(input, sender, stop_clone);
        });
        
        // Wait for result
        let result = receiver.recv().unwrap();
        
        // Verify result
        assert!(result.is_some());
        if let Some(decoder_result) = result {
            assert_eq!(decoder_result.text[0], "Hello World");
            assert!(!decoder_result.path.is_empty());
        }
    }
}
```

## 9. Performance Considerations

1. **Batch Size Tuning**:
   - Start with `PARALLEL_BATCH_SIZE = 10`
   - Adjust based on your system's core count and performance characteristics
   - For systems with many cores, try increasing to 16-32

2. **Memory Management**:
   - The parallel implementation may use more memory due to multiple nodes being processed simultaneously
   - Consider implementing more aggressive pruning if memory usage becomes an issue

3. **Load Balancing**:
   - Rayon handles load balancing automatically, but you may need to adjust batch sizes if some nodes take much longer to process than others

4. **Thread Pool Configuration**:
   - You can configure Rayon's thread pool if needed:
   ```rust
   // At the start of your program
   rayon::ThreadPoolBuilder::new()
       .num_threads(num_cpus::get())
       .build_global()
       .unwrap();
   ```

## 10. Implementation Notes

1. This implementation maintains the core A* algorithm while adding parallel node expansion.
2. The thread-safe priority queue ensures nodes are still processed in order of priority.
3. Special "result" nodes with very low total cost ensure results are processed immediately.
4. The parallel implementation should provide significant speedup on multi-core systems.
5. The code has been structured to minimize lock contention and maximize parallel processing.

## 11. Potential Issues and Solutions

1. **Issue**: Lock contention on the priority queue
   - **Solution**: Use a lock-free priority queue or implement work stealing

2. **Issue**: Excessive memory usage
   - **Solution**: Implement more aggressive pruning or limit the maximum open set size

3. **Issue**: Uneven workload distribution
   - **Solution**: Adjust batch sizes dynamically based on node processing times

4. **Issue**: Race conditions in result handling
   - **Solution**: Use atomic operations and proper synchronization for result sending 