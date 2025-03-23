# Plan: A* Search Algorithm Rewrite for ciphey (Simplified Heuristic - Junior Engineer Edition)

## Overview

This document outlines the steps to rewrite the A* search algorithm in `src/searchers/astar.rs` for ciphey. The goal is to improve the algorithm's efficiency and focus on more promising decoding paths by:

1.  Using a new node structure that explicitly defines the next decoder to use.
2.  Removing the initial "do all decoders first" phase.
3.  Implementing a simplified heuristic function.
4. Adding a check before node creation to prevent wasted calculations on unlikely solutions.

## 1. Update Node Structure (`src/searchers/astar.rs`)

-   **Task:** Modify the `AStarNode` struct to include the next decoder to use.

```rust
struct AStarNode {
    state: DecoderResult,
    cost: u32,
    heuristic: f32,
    total_cost: f32,
    next_decoder: Option<Box<dyn Crack + Sync>>, // **New Field!** The next decoder to try.  Can be None for the initial node.
}
```

**Details:**

*   `next_decoder`: This field is an `Option<Box<dyn Crack + Sync>>`. It holds the *specific* decoder that should be applied when this node is expanded. If it's `None`, it means we should consider *all* available decoders for the next step (this is only for the initial state). The type `Box<dyn Crack + Sync>` is a trait object, allowing us to store any type that implements the `Crack` trait.  The `Sync` bound is needed for multithreading.
*   You'll need to add a `use crate::decoders::interface::Crack;` import to the top of the file if you don't have it already.
*  Test your changes after this.  Create a dummy `AStarNode` and ensure it compiles.

-   **Task:** Update the `DecoderResult` struct in `src/lib.rs` to only store *one* decoded text (a `String`) and the path taken.

```rust
/// DecoderResult is the result of decoders
#[derive(Debug, Clone)]
pub struct DecoderResult {
    /// The text we have from the decoder
    pub text: String,
    /// The list of decoders we have so far
    /// The CrackResult contains more than just each decoder, such as the keys used
    /// or the checkers used.
    pub path: Vec<CrackResult>,
}
```

**Details:**

*   Change `text: Vec<String>` to `text: String`.  This simplifies the code and is important for this project.
*   You'll have to update *every* part of the code that touches this struct to account for the change! Search for all usages of `DecoderResult` and fix the errors.
*  Test your changes after this.  Go through the project and fix all the compilation errors that appear as a result of these changes.

## 2. Remove the "Do All Decoders First" Phase (`src/searchers/astar.rs`)

-   **Task:** Remove the initial code block that runs all decoders on the input text *before* starting the A* search.

**Details:**

*   This section of code is at the beginning of the `astar` function. It will look something like:
  ```rust
    // First, execute all "decoder"-tagged decoders immediately
    // To ensure all simple solutions get caught
    // This is to prevent more complex decoders from returning
    // https://github.com/bee-san/ciphey/issues/79
    let mut decoder_tagged_decoders = get_decoder_tagged_decoders(&initial);

    if !decoder_tagged_decoders.components.is_empty() { ... }
  ```
*   Delete this entire block of code. It's no longer needed.
*  Test your changes after this.  Run the existing tests to make sure you didn't accidentally break anything.

## 3. Implement Simplified Heuristic Function (`src/searchers/helper_functions.rs`)

-   **Task:** Modify the `generate_heuristic` function in `src/searchers/helper_functions.rs` to follow these rules (remove the hardcoded 0.3 and 0.6):

```rust
fn generate_heuristic(text: &str, path: &[CrackResult], next_decoder: &Option<Box<dyn Crack + Sync>>) -> f32 {
    let mut base_score = 0.0;

    //1. If the decoder it wants to use is tagged "cipher" we use CipherIdentifier
    if let Some(decoder) = next_decoder {
        if decoder.get_tags().contains(&"cipher") {
            let (cipher, score) = get_cipher_identifier_score(text);
        base_score += (1.0 - (score / 100.0) as f32);
        } else {
        base_score += (1.0 - decoder.popularity);
        }
    } else {
// If next decoder is None, this is not ideal and needs to be punished.
    base_score += 0.7;
    }

//2. We do an exponential punishment based on the depth of the search tree.
//So we could do something like (0.1 * DEPTH)^2 added to the score,
//this means the deeper we are in the search tree the higher the punishment.
//Nodes really deep in the search tree tend to be crap
base_score += (0.1 * path.len() as f32).powi(2);

//3. We penalise uncommon pairings with say a 0.25 penalty.
//When people use encryption schemes they tend to do stuff like (base64 ->  base32 -> base58).
//We want to prioritise these common pairings
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

*   Remove the multiplication by `0.6` and `0.3`. This allows the base score to scale linearly with those inputs. This will mean Cipher identifier has a greater importance.
*  Test your changes after this.  Run the existing tests to make sure you didn't accidentally break anything.

## 4. Code to prevent mostly invisible/short strings from being pushed onto the stack (`src/searchers/helper_functions.rs`)

-   **Task:** Add a call to `check_if_string_cant_be_decoded` into `astar.rs`.

```rust
                MyResults::Continue(results_vec) => {
                    // Process results and add to open set with heuristic prioritization
                    trace!(
                        "Processing {} results from decoders",
                        results_vec.len()
                    );

                    for r in results_vec {
                        let mut decoders_used = current_node.state.path.clone();
                        let text = r.unencrypted_text.clone();

                        // Make sure this string passes the checks
if check_if_string_cant_be_decoded(&text) {
// Add stats update for failed decoding
helper_functions::update_decoder_stats(r.decoder, false);
continue;
}

                        // Remove decoded texts from crack result
                        // Because all it holds is unencrypted text
                        let mut r = r;
                        r.unencrypted_text = None;

decoders_used.push(r);
        }
}
```

**Details:**

*   We need to call this function *before* the node is added to the queue.

## 5. A* Core Logic and Node Expansion (`src/searchers/astar.rs`)

-   **Task:** Modify the main A* loop to use the new node structure.

```rust
pub fn astar(input: String, result_sender: Sender<Option<DecoderResult>>, stop: Arc<AtomicBool>) {
    let initial = DecoderResult {
        text: input,
        path: vec![],
    };

    // Create initial node with no next_decoder (start with any decoder)
    let initial_node = AStarNode {
        state: initial,
        cost: 0,
        heuristic: 0.0,
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
        let decoders = match current_node.next_decoder {
            Some(decoder) => {
// We used the decoder, so update it's stats
helper_functions::update_decoder_stats(decoder.get_name(), true);
Decoders { components: vec![decoder] }
}
            None => get_all_decoders(),
        };

        let athena_checker = Checker::<Athena>::new();
        let checker = CheckerTypes::CheckAthena(athena_checker);
        let decoder_results = decoders.run(&current_node.state.text, checker);

        match decoder_results {
            MyResults::Break(res) => {
                // Handle successful decoding
                trace!("Found successful decoding with specific decoder");
                cli_pretty_printing::success(&format!(
                    "DEBUG: astar.rs - Decoder {} succeeded, short-circuiting",
                    res.decoder
                ));

                // Only exit if the result is truly successful (not rejected by human checker)
                if res.success {
                    let mut decoders_used = current_node.state.path.clone();
                    let text = res.unencrypted_text.clone();
                    decoders_used.push(res.clone());
                    let result_text = DecoderResult {
                        text,
                        path: decoders_used,
                    };

                    decoded_how_many_times(curr_depth);
                    cli_pretty_printing::success(&format!("DEBUG: astar.rs -  Sending successful result with {} decoders", result_text.path.len()));

                    // If in top_results mode, store the result in the WaitAthena storage
                    if get_config().top_results {
                        // Get the last decoder used
                        let decoder_name =
                            if let Some(last_decoder) = result_text.path.last() {
                                last_decoder.decoder.to_string()
                            } else {
                                "Unknown".to_string()
                            };

                        // Get the checker name from the last decoder
                        let checker_name =
                            if let Some(last_decoder) = result_text.path.last() {
                                last_decoder.checker_name.to_string()
                            } else {
                                "Unknown".to_string()
                            };

                        // Only store results that have a valid checker name
                        if !checker_name.is_empty() && checker_name != "Unknown" {
                            trace!(
                                "Storing plaintext in WaitAthena storage: {} (decoder: {}, checker: {})",
                                result_text.text,
                                decoder_name,
                                checker_name
                            );
                            wait_athena_storage::add_plaintext_result(
                                result_text.text.clone(),
                                format!("Decoded successfully at depth {}", curr_depth),
                                checker_name,
                                decoder_name,
                            );
                        }

                    }

                    result_sender
                        .send(Some(result_text))
                        .expect("Should successfully send the result");

                    // Only stop if not in top_results mode
                    if !get_config().top_results {
                        // Stop further iterations
                        stop.store(true, std::sync::atomic::Ordering::Relaxed);
                        return;
                    }
                    // In top_results mode, continue searching
                } else {
                    // If human checker rejected, continue the search
                    trace!("Human checker rejected the result, continuing search");
                }
            }
            MyResults::Continue(results_vec) => {
                // Process results and add to open set with heuristic prioritization
                trace!(
                    "Processing {} results from decoders",
                    results_vec.len()
                );

                for r in results_vec {
let decoders_used = current_node.state.path.clone();
                    let text = r.unencrypted_text.clone();

// Make sure this string passes the checks
if check_if_string_cant_be_decoded(&text) {
// Add stats update for failed decoding
helper_functions::update_decoder_stats(r.decoder, false);
continue;
}

                    if seen_strings.insert(text.clone()) {
                        seen_count += 1;

                        // Prune the HashSet if it gets too large
                        if seen_count > prune_threshold {
                            debug!(
                                "Pruning seen_strings HashSet (size: {})",
                                seen_strings.len()
                            );
                            seen_strings.retain(|s| s.len() < 100);
                            debug!("Pruned to {} entries", seen_strings.len());
                            prune_threshold = INITIAL_PRUNE_THRESHOLD
                                - (curr_depth * 1000) as usize; // Decrease threshold by 1000 per depth
                            debug!("Setting new prune threshold to {}", prune_threshold);
                        }
                    } else {
                        // This string has been seen before, don't process it
                        continue;
                    }

                    // Remove decoded texts from crack result
                    // Because all it holds is unencrypted text
                    let mut r = r;
                    r.unencrypted_text = None;

decoders_used.push(r);

                    let mut new_node = AStarNode {
                        state: DecoderResult {
                            text,
                            path: decoders_used,
                        },
                        cost: current_node.cost + 1,
                        heuristic: generate_heuristic(&current_node.state.text, &decoders_used, &non_decoder_decoders[0]),
                        total_cost: 0.0, // TODO calculate
next_decoder: None
                    };
if num % 100 == 0 {
// test printout
}

                    open_set.push(new_node);

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

*   The `next_decoder` field in `AStarNode` is now used to determine which decoder to apply next.

## 6. Testing (`src/searchers/astar.rs`)

-   **Task:** Update existing tests to account for the new logic.
-   **Task:** Add new tests that specifically target the A* code path with the new logic.

**Details:**

*   Ensure that all tests pass after the changes. If tests fail, you will need to debug the code and fix the errors.
*   Write comprehensive tests to cover all possible scenarios.
