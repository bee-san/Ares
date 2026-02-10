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
        if let AppState::Results(ref mut rs) = self.state {
            let path_len = rs.result.path.len();
            if path_len > 0 {
                rs.selected_step = (rs.selected_step + 1) % path_len;
                rs.scroll_offset = 0;
                // Load AI explanation from cache for the new step
                rs.ai_explanation = rs.ai_explanation_cache.get(&rs.selected_step).cloned();
                rs.ai_loading = false;
                // Reset branch selection when changing steps
                rs.highlighted_branch = None;
                rs.branch_scroll_offset = 0;
                // Load branches for the new step
                if let Some(cid) = rs.cache_id {
                    rs.current_branches =
                        get_branches_for_step(cid, rs.selected_step).unwrap_or_default();
                } else {
                    rs.current_branches = Vec::new();
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
        if let AppState::Results(ref mut rs) = self.state {
            let path_len = rs.result.path.len();
            if path_len > 0 {
                rs.selected_step = if rs.selected_step == 0 {
                    path_len - 1
                } else {
                    rs.selected_step - 1
                };
                rs.scroll_offset = 0;
                // Load AI explanation from cache for the new step
                rs.ai_explanation = rs.ai_explanation_cache.get(&rs.selected_step).cloned();
                rs.ai_loading = false;
                // Reset branch selection when changing steps
                rs.highlighted_branch = None;
                rs.branch_scroll_offset = 0;
                // Load branches for the new step
                if let Some(cid) = rs.cache_id {
                    rs.current_branches =
                        get_branches_for_step(cid, rs.selected_step).unwrap_or_default();
                } else {
                    rs.current_branches = Vec::new();
                }
            }
        }
    }

    /// Navigates to the first step in the path.
    pub fn first_step(&mut self) {
        if let AppState::Results(ref mut rs) = self.state {
            rs.selected_step = 0;
            rs.scroll_offset = 0;
            rs.ai_explanation = rs.ai_explanation_cache.get(&0).cloned();
            rs.ai_loading = false;
            rs.highlighted_branch = None;
            rs.branch_scroll_offset = 0;
            if let Some(cid) = rs.cache_id {
                rs.current_branches =
                    get_branches_for_step(cid, rs.selected_step).unwrap_or_default();
            } else {
                rs.current_branches = Vec::new();
            }
        }
    }

    /// Navigates to the last step in the path.
    pub fn last_step(&mut self) {
        if let AppState::Results(ref mut rs) = self.state {
            let path_len = rs.result.path.len();
            if path_len > 0 {
                rs.selected_step = path_len - 1;
                rs.scroll_offset = 0;
                rs.ai_explanation = rs.ai_explanation_cache.get(&rs.selected_step).cloned();
                rs.ai_loading = false;
                rs.highlighted_branch = None;
                rs.branch_scroll_offset = 0;
                if let Some(cid) = rs.cache_id {
                    rs.current_branches =
                        get_branches_for_step(cid, rs.selected_step).unwrap_or_default();
                } else {
                    rs.current_branches = Vec::new();
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
        if let AppState::Results(ref rs) = self.state {
            rs.result.path.get(rs.selected_step)
        } else {
            None
        }
    }

    // ============================================================================
    // Branch Navigation Methods
    // ============================================================================

    /// Moves to the next branch in the branch list (j/Down).
    pub fn next_branch(&mut self) {
        if let AppState::Results(ref mut rs) = self.state {
            if rs.current_branches.is_empty() {
                return;
            }

            match rs.highlighted_branch {
                Some(ref mut idx) => {
                    if *idx < rs.current_branches.len() - 1 {
                        *idx += 1;
                    }
                }
                None => {
                    rs.highlighted_branch = Some(0);
                }
            }

            // Auto-scroll to keep highlighted branch visible
            let visible = rs.level_visible_rows;
            if let Some(idx) = rs.highlighted_branch {
                if idx >= rs.branch_scroll_offset + visible {
                    rs.branch_scroll_offset = idx.saturating_sub(visible - 1);
                }
            }
        }
    }

    /// Moves to the previous branch in the branch list (k/Up).
    pub fn prev_branch(&mut self) {
        if let AppState::Results(ref mut rs) = self.state {
            if rs.current_branches.is_empty() {
                return;
            }

            match rs.highlighted_branch {
                Some(ref mut idx) => {
                    if *idx > 0 {
                        *idx -= 1;
                        if *idx < rs.branch_scroll_offset {
                            rs.branch_scroll_offset = *idx;
                        }
                    }
                }
                None => {
                    // Start from the last branch if none selected
                    rs.highlighted_branch = Some(rs.current_branches.len().saturating_sub(1));
                }
            }
        }
    }

    /// Loads branches for the currently selected step.
    pub fn load_branches_for_step(&mut self) {
        if let AppState::Results(ref mut rs) = self.state {
            if let Some(cid) = rs.cache_id {
                rs.current_branches =
                    get_branches_for_step(cid, rs.selected_step).unwrap_or_default();
            } else {
                rs.current_branches = Vec::new();
            }
            rs.highlighted_branch = None;
            rs.branch_scroll_offset = 0;
        }
    }

    /// Gets the currently highlighted branch, if any.
    pub fn get_highlighted_branch(&self) -> Option<&BranchSummary> {
        if let AppState::Results(ref rs) = self.state {
            rs.highlighted_branch
                .and_then(|idx| rs.current_branches.get(idx))
        } else {
            None
        }
    }

    /// Checks if there are branches available at the current step.
    pub fn has_branches(&self) -> bool {
        if let AppState::Results(ref rs) = self.state {
            !rs.current_branches.is_empty()
        } else {
            false
        }
    }

    /// Gets the branch context for creating a new branch at the current step.
    ///
    /// Returns None if not in Results state or the selected step has no output.
    pub fn get_branch_context(&self) -> Option<BranchContext> {
        if let AppState::Results(ref rs) = self.state {
            // Get the text to decode (output from the selected step)
            let step = rs.result.path.get(rs.selected_step)?;
            let text_to_decode = step.unencrypted_text.as_ref()?.first()?.clone();

            // Build prefix path up to and including the selected step
            let prefix_path: Vec<CrackResult> = rs.result.path[..=rs.selected_step].to_vec();

            Some(BranchContext {
                text_to_decode,
                prefix_path,
                parent_cache_id: rs.cache_id,
                branch_step: rs.selected_step,
            })
        } else {
            None
        }
    }
}
