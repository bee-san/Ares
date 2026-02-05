# Decoder Tree Branching Implementation Plan

## Overview

This document outlines the implementation of decoder tree branching with physical branch navigation in the TUI. Users can explore alternative decoding paths directly from the path viewer using keyboard navigation.

### Key Principles

1. **Branches are visible** - When a node has branches, they appear in a scrollable list below the path
2. **Physical navigation** - Up/Down arrows navigate branches, Left/Right navigate the path
3. **Confirm to switch** - Highlight a branch with arrows, press Enter to switch to it
4. **Compact breadcrumbs** - Deep branch paths shown as `Main › B2 › B1` notation
5. **Persistence** - Branches stored in database via foreign keys to cache table

---

## UI Layout

### Results Screen (Redesigned)

The Results screen removes the Input/Output side panels and makes the Path panel full-width:

```
+---------------------------- Ciphey -----------------------------+
|                                                                  |
|  +--- Path (Main) ---------------------------------------------+ |
|  |                                                              | |
|  |  [Input] --> [Base64] --> [Caesar] --> [Plaintext]          | |
|  |                  ^                                           | |
|  |              SELECTED                                        | |
|  |               [^2 v1]  <- branch indicator                   | |
|  |                                                              | |
|  |  --- Branches from [Base64] (3 total) ---------------------- | |
|  |                                                              | |
|  |  > [Caesar] --> "hello world" checkmark                      | |
|  |    [ROT13] --> [Hex] --> "48454c..." (2 sub)                | |
|  |    [Reverse] --> "dlrow olleh" checkmark                     | |
|  |                                                              | |
|  +--------------------------------------------------------------+ |
|                                                                  |
| ---------------------------------------------------------------- |
|                                                                  |
|  +--- Step Details --------------------------------------------+ |
|  |  Decoder: Base64                                            | |
|  |  Key: N/A                                                   | |
|  |  Input: "SGVsbG8gV29ybGQ=" (16 chars, 16 bytes, ...)        | |
|  |  Output: "Hello World" (11 chars, 11 bytes, ...)            | |
|  +-------------------------------------------------------------+ |
|                                                                  |
|  [h/l] Path  [j/k] Branches  [Enter] Switch  [/] Add  [?] Help  |
+------------------------------------------------------------------+
```

### When Viewing a Branch

Breadcrumb shows compact path notation:

```
+--- Path (Main > B2 > B1) --------------------------------------+
|                                                                 |
|  [Input] --> [ROT13] --> [Hex] --> [Plaintext]                 |
|                                                                 |
|  [Backspace] Return to parent                                   |
+-----------------------------------------------------------------+
```

### When No Branches Exist

The branch list section simply doesn't appear - only the path viewer is shown:

```
+--- Path (Main) ------------------------------------------------+
|                                                                 |
|  [Input] --> [Base64] --> [Caesar] --> [Plaintext]             |
|                              ^                                  |
|                          SELECTED                               |
|                                                                 |
+-----------------------------------------------------------------+
```

---

## Navigation Model

### Vim-Style Keybindings

| Key | Action |
|-----|--------|
| `h` / `Left` | Previous step in path |
| `l` / `Right` | Next step in path |
| `j` / `Down` | Next branch in list (when branches exist) |
| `k` / `Up` | Previous branch in list (when branches exist) |
| `gg` | Go to first step in path |
| `G` | Go to last step in path |
| `Enter` | Switch to highlighted branch / Open branch prompt if no branches |
| `Backspace` | Return to parent path (when viewing branch) |
| `/` | Open decoder search modal |
| `y` / `c` | Copy selected step's output to clipboard |
| `q` / `Esc` | Quit / Back |
| `?` | Toggle help overlay |
| `Ctrl+S` | Open settings |

### Navigation Flow

```
Results Screen Navigation:

1. PATH NAVIGATION (horizontal)
   h/Left  <----  [Step 1] -- [Step 2] -- [Step 3]  ----> l/Right
                                 |
                             SELECTED
                                 |
2. BRANCH NAVIGATION (vertical, only when branches exist)
                                 |
                     k/Up   [Branch A]  <-- highlighted
                                 |
                            [Branch B]
                                 |
                     j/Down [Branch C]

3. SWITCH TO BRANCH
   Press Enter on highlighted branch -> View that branch's path
   
4. RETURN TO PARENT
   Press Backspace -> Go back up one level in branch hierarchy
```

### Selection Styling

Only one visual style needed for highlighted branch:
- **Highlighted**: Reversed background + bold text
- Non-highlighted branches: Normal text

The "current" branch (the one you're viewing) is implicit - it's the path shown above.

---

## Database Schema Changes

### Modify Existing `cache` Table

```sql
ALTER TABLE cache ADD COLUMN parent_cache_id INTEGER REFERENCES cache(rowid);
ALTER TABLE cache ADD COLUMN branch_step INTEGER;
ALTER TABLE cache ADD COLUMN branch_type TEXT;
```

| Column | Type | Description |
|--------|------|-------------|
| `parent_cache_id` | INTEGER | Foreign key to parent cache entry (NULL for root entries) |
| `branch_step` | INTEGER | Index into parent's path where branch occurred |
| `branch_type` | TEXT | One of: `'auto'`, `'single_layer'`, `'manual'` |

### Storage Rules

| Action | Store Result? |
|--------|---------------|
| Full A* search (rerun Ciphey) | Only if successful (finds plaintext) |
| Single layer (run all decoders once) | Store all that decode successfully |
| Manual decoder via `/` search | Always store (user explicitly requested) |

### Query Functions

```rust
/// Get all branches from a cache entry (all steps)
fn get_branches_for_cache(cache_id: i64) -> Vec<BranchInfo>;

/// Get branches from a specific step of a cache entry
fn get_branches_for_step(cache_id: i64, step: usize) -> Vec<BranchInfo>;

/// Insert a cache entry as a branch of another
fn insert_branch(
    entry: &CacheEntry,
    parent_cache_id: i64,
    branch_step: usize,
    branch_type: &str,
) -> Result<i64, Error>;

/// Get parent info for a branch
fn get_parent_info(cache_id: i64) -> Option<ParentBranchInfo>;
```

---

## Data Structures

### New Structs

```rust
/// Context for creating a branch from a node
#[derive(Debug, Clone)]
pub struct BranchContext {
    /// Text to decode (output from the branch point)
    pub text_to_decode: String,
    /// Path prefix up to (and including) the branch point
    pub prefix_path: Vec<CrackResult>,
    /// Parent cache ID for database linking
    pub parent_cache_id: Option<i64>,
    /// Step index in parent's path where we're branching
    pub branch_step: usize,
}

/// Information about a branch's parent
#[derive(Debug, Clone)]
pub struct ParentBranchInfo {
    pub parent_cache_id: i64,
    pub branch_step: usize,
}

/// Summary of a branch for display in UI
#[derive(Debug, Clone)]
pub struct BranchSummary {
    pub cache_id: i64,
    pub branch_type: String,
    pub first_decoder: String,
    pub final_text_preview: String,
    pub successful: bool,
    pub path_length: usize,
    pub sub_branch_count: usize,
}

/// Tracks current position in branch hierarchy
#[derive(Debug, Clone, Default)]
pub struct BranchPath {
    /// Stack of (cache_id, branch_step) representing path from root
    pub stack: Vec<(i64, usize)>,
}

impl BranchPath {
    /// Get compact display string: "Main > B2 > B1"
    pub fn display(&self) -> String {
        if self.stack.is_empty() {
            "Main".to_string()
        } else {
            let mut parts = vec!["Main".to_string()];
            for (i, _) in self.stack.iter().enumerate() {
                parts.push(format!("B{}", i + 1));
            }
            parts.join(" > ")
        }
    }
    
    /// Check if currently viewing a branch (not main)
    pub fn is_branch(&self) -> bool {
        !self.stack.is_empty()
    }
}

/// Branch mode options (for creating new branches)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BranchMode {
    /// Run full A* search to find plaintext
    FullSearch,
    /// Run all decoders once and show results  
    SingleLayer,
}
```

---

## TUI State Changes

### Extended Results State

```rust
Results {
    /// The successful decoding result
    result: DecoderResult,
    /// Currently selected step in the path
    selected_step: usize,
    /// Cache ID for this result (for branch linking)
    cache_id: Option<i64>,
    
    // Branch navigation
    /// Current position in branch hierarchy
    branch_path: BranchPath,
    /// Branches for the currently selected step (loaded on demand)
    current_branches: Vec<BranchSummary>,
    /// Index of highlighted branch in list (None = no branch highlighted)
    highlighted_branch: Option<usize>,
    /// Scroll offset for branch list
    branch_scroll_offset: usize,
}
```

### New AppState Variants

```rust
/// Modal for selecting branch mode (when creating new branch)
BranchModePrompt {
    selected_mode: BranchMode,
    branch_context: BranchContext,
}

/// Vim-style decoder search modal
DecoderSearch {
    text_input: TextInput,
    all_decoders: Vec<&'static str>,
    filtered_decoders: Vec<&'static str>,
    selected_index: usize,
    branch_context: BranchContext,
}
```

---

## UI Components

### 1. Path Panel with Branch List

The PathViewer widget is redesigned to include:

**Section A: Horizontal Path**
- Decoder boxes with arrows between them
- Selected box has double border + reversed text
- Branch indicator `[^N vM]` below selected node (if branches exist)

**Section B: Branch List (conditional)**
- Only appears when `current_branches` is non-empty
- Shows scrollable list of sibling branches
- Highlighted branch has reversed background

```rust
// PathViewer render signature
pub fn render(
    &self,
    area: Rect,
    buf: &mut Buffer,
    path: &[CrackResult],
    selected_step: usize,
    branches: &[BranchSummary],
    highlighted_branch: Option<usize>,
    branch_scroll_offset: usize,
    branch_path: &BranchPath,
    colors: &TuiColors,
);
```

### 2. Branch Indicator

Shows count of branches above/below current view:

```
[^2 v1]  = 2 branches scrolled above, 1 below visible area
[^0 v3]  = at top, 3 more below
```

### 3. Breadcrumb Header

Compact notation when viewing a branch:

```
Path (Main > B2 > B1)
```

### 4. BranchModePrompt Modal

Centered modal for choosing how to create a new branch:

```
+======================================+
|      How do you want to branch?      |
|                                      |
|   > Full A* Search                   |
|     Run complete search to find      |
|     plaintext automatically          |
|                                      |
|     Single Layer                     |
|     Run all decoders once and        |
|     show successful results          |
|                                      |
|   [Enter] Select  [Esc] Cancel       |
+======================================+
```

### 5. DecoderSearch Modal

Vim-style search in bottom-left corner:

```
+-- /decoder ----------------+
| > caesar                   |
|   base64                   |
|   hexadecimal              |
|   ...                      |
|                            |
| [Enter] Run  [Esc] Cancel  |
+----------------------------+
```

---

## Core Functions

### New Functions in `src/lib.rs`

```rust
/// Run all decoders once on the input text, returning successful decodes.
/// Does not recurse - only runs one layer of decoding.
pub fn run_single_layer(
    text: &str,
    checker: &CheckerTypes,
) -> Vec<CrackResult>;

/// Run a specific decoder on the input text.
/// Returns the CrackResult regardless of whether a checker confirmed it.
pub fn run_specific_decoder(
    text: &str,
    decoder_name: &str,
    checker: &CheckerTypes,
) -> Option<CrackResult>;
```

---

## Implementation Tasks

### Phase 1: Database (Priority: High)

| Task | File | Description |
|------|------|-------------|
| 1.1 | `src/storage/database.rs` | Add migration for new columns |
| 1.2 | `src/storage/database.rs` | Add `get_branches_for_step()` function |
| 1.3 | `src/storage/database.rs` | Add `insert_branch()` function |
| 1.4 | `src/storage/database.rs` | Add `get_parent_info()` function |
| 1.5 | `src/storage/database.rs` | Add `BranchInfo` struct |

### Phase 2: Core Logic (Priority: High)

| Task | File | Description |
|------|------|-------------|
| 2.1 | `src/lib.rs` | Implement `run_single_layer()` |
| 2.2 | `src/lib.rs` | Implement `run_specific_decoder()` |
| 2.3 | `src/decoders/mod.rs` | Add `get_decoder_by_name()` helper |

### Phase 3: Data Structures (Priority: High)

| Task | File | Description |
|------|------|-------------|
| 3.1 | `src/tui/app/state.rs` | Add `BranchContext` struct |
| 3.2 | `src/tui/app/state.rs` | Add `BranchPath` struct with `display()` |
| 3.3 | `src/tui/app/state.rs` | Add `BranchSummary` struct |
| 3.4 | `src/tui/app/state.rs` | Add `ParentBranchInfo` struct |
| 3.5 | `src/tui/app/state.rs` | Add `BranchMode` enum |
| 3.6 | `src/tui/app/state.rs` | Extend `Results` state with branch fields |
| 3.7 | `src/tui/app/state.rs` | Add `BranchModePrompt` state |
| 3.8 | `src/tui/app/state.rs` | Add `DecoderSearch` state |

### Phase 4: State Transitions (Priority: High)

| Task | File | Description |
|------|------|-------------|
| 4.1 | `src/tui/app/mod.rs` | Add `load_branches_for_step()` method |
| 4.2 | `src/tui/app/mod.rs` | Add `switch_to_branch()` method |
| 4.3 | `src/tui/app/mod.rs` | Add `return_to_parent()` method |
| 4.4 | `src/tui/app/mod.rs` | Add `open_branch_prompt()` method |
| 4.5 | `src/tui/app/mod.rs` | Add `open_decoder_search()` method |

### Phase 5: Input Handling (Priority: High)

| Task | File | Description |
|------|------|-------------|
| 5.1 | `src/tui/input.rs` | Add vim keybindings (`gg`, `G`, `h`, `j`, `k`, `l`) |
| 5.2 | `src/tui/input.rs` | Handle `j`/`k` for branch list navigation |
| 5.3 | `src/tui/input.rs` | Handle `Enter` for branch switching |
| 5.4 | `src/tui/input.rs` | Handle `Backspace` for return to parent |
| 5.5 | `src/tui/input.rs` | Handle `/` for decoder search |
| 5.6 | `src/tui/input.rs` | Create `handle_branch_mode_prompt_keys()` |
| 5.7 | `src/tui/input.rs` | Create `handle_decoder_search_keys()` |

### Phase 6: UI Layout Changes (Priority: High)

| Task | File | Description |
|------|------|-------------|
| 6.1 | `src/tui/ui.rs` | Remove 3-column split in `draw_results_screen()` |
| 6.2 | `src/tui/ui.rs` | Make Path panel full-width |
| 6.3 | `src/tui/ui.rs` | Remove Input/Output `render_text_panel()` calls |
| 6.4 | `src/tui/ui.rs` | Add `draw_branch_mode_prompt()` |
| 6.5 | `src/tui/ui.rs` | Add `draw_decoder_search()` |
| 6.6 | `src/tui/ui.rs` | Update status bar keybinding hints |

### Phase 7: PathViewer Redesign (Priority: High)

| Task | File | Description |
|------|------|-------------|
| 7.1 | `src/tui/widgets/path_viewer.rs` | Update `render()` signature for branches |
| 7.2 | `src/tui/widgets/path_viewer.rs` | Add breadcrumb header rendering |
| 7.3 | `src/tui/widgets/path_viewer.rs` | Add branch indicator `[^N vM]` |
| 7.4 | `src/tui/widgets/path_viewer.rs` | Add branch list section rendering |
| 7.5 | `src/tui/widgets/path_viewer.rs` | Add highlight styling for selected branch |

### Phase 8: New Widgets (Priority: Medium)

| Task | File | Description |
|------|------|-------------|
| 8.1 | `src/tui/widgets/branch_mode_prompt.rs` | Create centered modal widget |
| 8.2 | `src/tui/widgets/decoder_search.rs` | Create bottom-left search widget |
| 8.3 | `src/tui/widgets/mod.rs` | Export new widgets |

### Phase 9: Event Loop (Priority: Medium)

| Task | File | Description |
|------|------|-------------|
| 9.1 | `src/tui/run.rs` | Handle branch switching actions |
| 9.2 | `src/tui/run.rs` | Handle `RunBranchFullSearch` action |
| 9.3 | `src/tui/run.rs` | Handle `RunBranchSingleLayer` action |
| 9.4 | `src/tui/run.rs` | Handle `RunBranchDecoder` action |
| 9.5 | `src/tui/run.rs` | Store branch results to database |

### Phase 10: Testing (Priority: Medium)

| Task | File | Description |
|------|------|-------------|
| 10.1 | `src/storage/database.rs` | Test branch CRUD operations |
| 10.2 | `src/lib.rs` | Test `run_single_layer()` |
| 10.3 | `src/lib.rs` | Test `run_specific_decoder()` |
| 10.4 | `src/tui/widgets/path_viewer.rs` | Test branch list rendering |

### Phase 11: Documentation (Priority: Low)

| Task | File | Description |
|------|------|-------------|
| 11.1 | `AGENTS.md` | Document new TUI states |
| 11.2 | `AGENTS.md` | Document database schema changes |
| 11.3 | `AGENTS.md` | Document new keybindings |

---

## File Changes Summary

| File | Type | Description |
|------|------|-------------|
| `src/storage/database.rs` | Modify | Branch columns, query functions |
| `src/lib.rs` | Modify | `run_single_layer()`, `run_specific_decoder()` |
| `src/decoders/mod.rs` | Modify | `get_decoder_by_name()` helper |
| `src/tui/app/state.rs` | Modify | New structs, extended Results state |
| `src/tui/app/mod.rs` | Modify | Branch navigation methods |
| `src/tui/input.rs` | Modify | Vim keybindings, branch navigation |
| `src/tui/run.rs` | Modify | Action handlers, branch storage |
| `src/tui/ui.rs` | Modify | Remove 3-column, full-width path |
| `src/tui/widgets/path_viewer.rs` | Modify | Branch list, breadcrumb, indicator |
| `src/tui/widgets/branch_mode_prompt.rs` | **New** | Branch mode modal |
| `src/tui/widgets/decoder_search.rs` | **New** | Decoder search modal |
| `src/tui/widgets/mod.rs` | Modify | Export new widgets |
| `AGENTS.md` | Modify | Documentation |

---

## Migration Notes

Users upgrading from previous versions:

1. Database schema has new columns (auto-migrated via `ALTER TABLE`)
2. Existing cache entries have `NULL` for branch columns (they are root entries)
3. Results screen layout has changed (no more Input/Output side panels)
4. New keybindings: `j`/`k` for branch navigation, `gg`/`G` for jump to start/end
5. `Enter` behavior changed: switches branch if one highlighted, opens prompt if none

---

## Design Decisions Log

| Decision | Rationale |
|----------|-----------|
| Remove Input/Output panels | More space for branch list; Step Details still shows I/O |
| Single highlight style | Simpler UX; "current" branch is implicit (it's the displayed path) |
| No empty state message | Cleaner UI; users learn branches via discovery |
| Compact breadcrumb `Main > B2` | Prevents breadcrumb overflow with deep branches |
| `gg`/`G` vim bindings | Familiar to vim users, efficient path navigation |
| Branch list in Path panel | Physical navigation feels more direct than modal |
