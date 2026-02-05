//! Navigation methods for stepping through decoder results.

use crate::decoders::crack_results::CrackResult;

use super::state::AppState;
use super::App;

impl App {
    /// Navigates to the next step in the decoding path.
    ///
    /// Only has an effect when in the Results state. Wraps around to the
    /// beginning when reaching the end of the path.
    pub fn next_step(&mut self) {
        if let AppState::Results {
            result,
            selected_step,
            scroll_offset,
        } = &mut self.state
        {
            let path_len = result.path.len();
            if path_len > 0 {
                *selected_step = (*selected_step + 1) % path_len;
                *scroll_offset = 0;
            }
        }
    }

    /// Navigates to the previous step in the decoding path.
    ///
    /// Only has an effect when in the Results state. Wraps around to the
    /// end when at the beginning of the path.
    pub fn prev_step(&mut self) {
        if let AppState::Results {
            result,
            selected_step,
            scroll_offset,
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
}
