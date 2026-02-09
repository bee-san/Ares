//! Navigation methods for stepping through decoder results.

use crate::decoders::crack_results::CrackResult;
use crate::storage::database::{get_branches_for_step, BranchSummary};

use super::state::{AppState, BranchContext};
use super::App;

impl App {
    /// Navigates to the next step in the decoding path.
    ///
    /// Only has an effect when in the Results state. Wraps around to the
    /// beginning when reaching the end of the path. Also reloads branches
    /// for the new step.
    pub fn next_step(&mut self) {
        if let AppState::Results {
            result,
            selected_step,
            scroll_offset,
            cache_id,
            highlighted_branch,
            branch_scroll_offset,
            current_branches,
            ai_explanation,
            ai_loading,
            ..
        } = &mut self.state
        {
            let path_len = result.path.len();
            if path_len > 0 {
                *selected_step = (*selected_step + 1) % path_len;
                *scroll_offset = 0;
                // Clear AI explanation when changing steps
                *ai_explanation = None;
                *ai_loading = false;
                // Reset branch selection when changing steps
                *highlighted_branch = None;
                *branch_scroll_offset = 0;
                // Load branches for the new step
                if let Some(cid) = cache_id {
                    *current_branches =
                        get_branches_for_step(*cid, *selected_step).unwrap_or_default();
                } else {
                    *current_branches = Vec::new();
                }
            }
        }
    }

    /// Navigates to the previous step in the decoding path.
    ///
    /// Only has an effect when in the Results state. Wraps around to the
    /// end when at the beginning of the path. Also reloads branches
    /// for the new step.
    pub fn prev_step(&mut self) {
        if let AppState::Results {
            result,
            selected_step,
            scroll_offset,
            cache_id,
            highlighted_branch,
            branch_scroll_offset,
            current_branches,
            ai_explanation,
            ai_loading,
            ..
        } = &mut self.state
        {
            let path_len = result.path.len();
            if path_len > 0 {
                *selected_step = if *selected_step == 0 {
                    path_len - 1
                } else {
                    *selected_step - 1
                };
                *scroll_offset = 0;
                // Clear AI explanation when changing steps
                *ai_explanation = None;
                *ai_loading = false;
                // Reset branch selection when changing steps
                *highlighted_branch = None;
                *branch_scroll_offset = 0;
                // Load branches for the new step
                if let Some(cid) = cache_id {
                    *current_branches =
                        get_branches_for_step(*cid, *selected_step).unwrap_or_default();
                } else {
                    *current_branches = Vec::new();
                }
            }
        }
    }

    /// Navigates to the first step in the path.
    pub fn first_step(&mut self) {
        if let AppState::Results {
            selected_step,
            scroll_offset,
            cache_id,
            highlighted_branch,
            branch_scroll_offset,
            current_branches,
            ai_explanation,
            ai_loading,
            ..
        } = &mut self.state
        {
            *selected_step = 0;
            *scroll_offset = 0;
            *ai_explanation = None;
            *ai_loading = false;
            *highlighted_branch = None;
            *branch_scroll_offset = 0;
            if let Some(cid) = cache_id {
                *current_branches = get_branches_for_step(*cid, *selected_step).unwrap_or_default();
            } else {
                *current_branches = Vec::new();
            }
        }
    }

    /// Navigates to the last step in the path.
    pub fn last_step(&mut self) {
        if let AppState::Results {
            result,
            selected_step,
            scroll_offset,
            cache_id,
            highlighted_branch,
            branch_scroll_offset,
            current_branches,
            ai_explanation,
            ai_loading,
            ..
        } = &mut self.state
        {
            let path_len = result.path.len();
            if path_len > 0 {
                *selected_step = path_len - 1;
                *scroll_offset = 0;
                *ai_explanation = None;
                *ai_loading = false;
                *highlighted_branch = None;
                *branch_scroll_offset = 0;
                if let Some(cid) = cache_id {
                    *current_branches =
                        get_branches_for_step(*cid, *selected_step).unwrap_or_default();
                } else {
                    *current_branches = Vec::new();
                }
            }
        }
    }

    /// Gets the currently selected step in the decoding path.
    ///
    /// # Returns
    ///
    /// `Some(&CrackResult)` if in Results state with a valid selection,
    /// `None` otherwise.
    pub fn get_current_step(&self) -> Option<&CrackResult> {
        if let AppState::Results {
            result,
            selected_step,
            ..
        } = &self.state
        {
            result.path.get(*selected_step)
        } else {
            None
        }
    }

    // ============================================================================
    // Branch Navigation Methods
    // ============================================================================

    /// Moves to the next branch in the branch list (j/Down).
    pub fn next_branch(&mut self) {
        if let AppState::Results {
            current_branches,
            highlighted_branch,
            branch_scroll_offset,
            level_visible_rows,
            ..
        } = &mut self.state
        {
            if current_branches.is_empty() {
                return;
            }

            match highlighted_branch {
                Some(idx) => {
                    if *idx < current_branches.len() - 1 {
                        *idx += 1;
                    }
                }
                None => {
                    *highlighted_branch = Some(0);
                }
            }

            // Auto-scroll to keep highlighted branch visible
            let visible = *level_visible_rows;
            if let Some(idx) = highlighted_branch {
                if *idx >= *branch_scroll_offset + visible {
                    *branch_scroll_offset = idx.saturating_sub(visible - 1);
                }
            }
        }
    }

    /// Moves to the previous branch in the branch list (k/Up).
    pub fn prev_branch(&mut self) {
        if let AppState::Results {
            current_branches,
            highlighted_branch,
            branch_scroll_offset,
            ..
        } = &mut self.state
        {
            if current_branches.is_empty() {
                return;
            }

            match highlighted_branch {
                Some(idx) => {
                    if *idx > 0 {
                        *idx -= 1;
                        if *idx < *branch_scroll_offset {
                            *branch_scroll_offset = *idx;
                        }
                    }
                }
                None => {
                    // Start from the last branch if none selected
                    *highlighted_branch = Some(current_branches.len().saturating_sub(1));
                }
            }
        }
    }

    /// Loads branches for the currently selected step.
    pub fn load_branches_for_step(&mut self) {
        if let AppState::Results {
            cache_id,
            selected_step,
            current_branches,
            highlighted_branch,
            branch_scroll_offset,
            ..
        } = &mut self.state
        {
            if let Some(cid) = cache_id {
                *current_branches = get_branches_for_step(*cid, *selected_step).unwrap_or_default();
            } else {
                *current_branches = Vec::new();
            }
            *highlighted_branch = None;
            *branch_scroll_offset = 0;
        }
    }

    /// Gets the currently highlighted branch, if any.
    pub fn get_highlighted_branch(&self) -> Option<&BranchSummary> {
        if let AppState::Results {
            current_branches,
            highlighted_branch,
            ..
        } = &self.state
        {
            highlighted_branch.and_then(|idx| current_branches.get(idx))
        } else {
            None
        }
    }

    /// Checks if there are branches available at the current step.
    pub fn has_branches(&self) -> bool {
        if let AppState::Results {
            current_branches, ..
        } = &self.state
        {
            !current_branches.is_empty()
        } else {
            false
        }
    }

    /// Gets the branch context for creating a new branch at the current step.
    ///
    /// Returns None if not in Results state or the selected step has no output.
    pub fn get_branch_context(&self) -> Option<BranchContext> {
        if let AppState::Results {
            result,
            selected_step,
            cache_id,
            ..
        } = &self.state
        {
            // Get the text to decode (output from the selected step)
            let step = result.path.get(*selected_step)?;
            let text_to_decode = step.unencrypted_text.as_ref()?.first()?.clone();

            // Build prefix path up to and including the selected step
            let prefix_path: Vec<CrackResult> = result.path[..=*selected_step].to_vec();

            Some(BranchContext {
                text_to_decode,
                prefix_path,
                parent_cache_id: *cache_id,
                branch_step: *selected_step,
            })
        } else {
            None
        }
    }
}
