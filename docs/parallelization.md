# Parallelization in ciphey

This document describes how parallelization is implemented in the ciphey project, with a focus on the decoder execution system and its relationship to search algorithms.

## Overview

ciphey uses the [Rayon](https://github.com/rayon-rs/rayon) library to implement data parallelism for computationally intensive operations. Rayon provides a simple API for converting sequential iterators into parallel ones, making it straightforward to parallelize operations across multiple CPU cores.

## Decoder Parallelization

### Implementation

The primary parallelization in ciphey occurs in the filtration system, specifically in the `run` method of the `Decoders` struct in `src/filtration_system/mod.rs`:

```rust
pub fn run(&self, text: &str, checker: CheckerTypes) -> MyResults {
    trace!("Running .crack() on all decoders");
    let (sender, receiver) = channel();
    self.components
        .into_par_iter()
        .try_for_each_with(sender, |s, i| {
            let results = i.crack(text, &checker);
            if results.success {
                s.send(results).expect("expected no send error!");
                // returning None short-circuits the iterator
                // we don't process any further as we got success
                return None;
            }
            s.send(results).expect("expected no send error!");
            // return Some(()) to indicate that continue processing
            Some(())
        });

    let mut all_results: Vec<CrackResult> = Vec::new();

    while let Ok(result) = receiver.recv() {
        // if we recv success, break.
        if result.success {
            return MyResults::Break(result);
        }
        all_results.push(result)
    }

    MyResults::Continue(all_results)
}
```

### Key Components

1. **Parallel Iterator**: The `into_par_iter()` method converts the sequential iterator over decoders into a parallel one, allowing multiple decoders to be executed concurrently.

2. **Channel-based Communication**: A channel (`sender`, `receiver`) is used to collect results from parallel decoder executions.

3. **Early Termination**: The `try_for_each_with` method allows for early termination of the parallel iteration when a successful decoding is found, using the `None` return value to short-circuit the iterator.

4. **Result Collection**: Results are collected from the channel and either returned immediately (on success) or aggregated into a vector for further processing.

## CPU Utilization and Performance Considerations

### Saturation Point

The decoder execution is typically the most computationally intensive part of the ciphey workflow. By parallelizing this operation, ciphey can effectively utilize multiple CPU cores to speed up the decoding process. However, there is a saturation point beyond which adding more parallelism may not improve performance:

1. **CPU Core Utilization**: If all available CPU cores are already fully utilized by the parallel decoder execution, adding additional layers of parallelism (such as processing multiple nodes in parallel in the search algorithm) may not provide significant performance benefits.

2. **Overhead**: Each layer of parallelism introduces some overhead for thread management, synchronization, and context switching. If this overhead exceeds the benefits of parallelization, performance may actually degrade.

3. **Memory Bandwidth**: In some cases, the limiting factor may be memory bandwidth rather than CPU processing power. Multiple threads competing for memory access can lead to contention and reduced performance.

### Amdahl's Law

Amdahl's Law provides a theoretical limit to the speedup that can be achieved through parallelization:

```
Speedup = 1 / ((1 - P) + P/N)
```

Where:
- P is the proportion of the program that can be parallelized
- N is the number of processors

This means that even if we parallelize the decoder execution perfectly, the overall speedup is limited by the sequential portions of the algorithm.

## Relationship to Search Algorithms

### A* Search Algorithm

The A* search algorithm in ciphey (`src/searchers/astar.rs`) uses the parallelized decoder execution system but maintains a sequential approach to node processing:

1. **Sequential Node Processing**: Nodes are processed one at a time from the priority queue, in order of their f-score (f = g + h, where g is the cost so far and h is the heuristic value).

2. **Parallel Decoder Execution**: For each node, the decoder execution is parallelized as described above.

3. **Priority Queue Bottleneck**: The priority queue introduces a sequential bottleneck, as nodes must be processed in order of their f-score to maintain the optimality of the A* algorithm.

### Optimization Opportunities

While the core node processing in A* is inherently sequential, there are still opportunities for optimization:

1. **Pruning Operations**: The quality scoring and sorting during pruning operations could be parallelized, as these operations are independent of the decoder execution.

2. **Heuristic Calculations**: Heuristic calculations for multiple nodes could potentially be parallelized.

3. **Memory Usage Patterns**: Improving memory usage patterns for better cache utilization could provide performance benefits without adding additional parallelism.

4. **Load Balancing**: Ensuring that the workload is evenly distributed across threads can improve overall performance.

## Conclusion

The parallelization of decoder execution in ciphey provides significant performance benefits by utilizing multiple CPU cores. However, there are limits to the benefits of parallelization, and adding additional layers of parallelism may not always improve performance.

When optimizing the performance of ciphey, it's important to consider the entire system and identify the true bottlenecks. In some cases, optimizing memory usage, improving algorithms, or reducing overhead may provide better performance improvements than adding more parallelism.

## References

- [Rayon Documentation](https://docs.rs/rayon/latest/rayon/)
- [Amdahl's Law](https://en.wikipedia.org/wiki/Amdahl%27s_law)
- [A* Search Algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm)