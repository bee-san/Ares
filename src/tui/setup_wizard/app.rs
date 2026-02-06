//! Application state machine for the setup wizard.
//!
//! This module defines the core state management for the setup wizard,
//! handling transitions between different configuration steps.

use std::collections::HashMap;
use std::path::Path;

use super::themes::{ColorScheme, Theme, THEMES};

/// The total number of steps in the wizard (for progress display).
pub const TOTAL_STEPS: usize = 7;

/// Represents the current state of the setup wizard.
#[derive(Debug, Clone)]
pub enum SetupState {
    /// Welcome screen with ASCII art and intro
    Welcome,
    /// Optional tutorial screen
    Tutorial,
    /// Theme/color scheme selection with live preview
    ThemeSelection {
        /// Currently highlighted theme index
        selected: usize,
        /// Whether the user is in custom color input mode
        custom_mode: bool,
        /// Custom color values being edited (if in custom mode)
        custom_colors: CustomColors,
        /// Which custom color field is being edited
        custom_field: usize,
    },
    /// "Ask every time" vs "collect all results" choice
    ResultsMode {
        /// Currently selected option (0 = ask each time, 1 = collect all)
        selected: usize,
    },
    /// Timeout configuration (shown if top_results mode)
    TimeoutConfig {
        /// Current timeout value
        value: u32,
        /// Whether the input field is focused for editing
        editing: bool,
    },
    /// Wordlist configuration
    WordlistConfig {
        /// List of already added custom file paths
        custom_paths: Vec<String>,
        /// Current path being typed (for custom file input)
        current_input: String,
        /// Cursor position in the current input
        cursor: usize,
        /// Selection state for predefined wordlists (indexes into get_predefined_wordlists())
        selected_predefined: Vec<usize>,
        /// Current UI focus
        focus: WordlistFocus,
        /// Custom URL being typed
        custom_url: String,
        /// Custom URL source name
        custom_url_source: String,
        /// Download progress if downloading (None if not downloading)
        download_progress: Option<DownloadProgress>,
    },
    /// Enhanced detection (AI model) configuration
    EnhancedDetection {
        /// Currently selected option (0 = no, 1 = yes)
        selected: usize,
    },
    /// HuggingFace token input (shown if enhanced detection enabled)
    TokenInput {
        /// The token being entered (masked)
        token: String,
        /// Cursor position
        cursor: usize,
    },
    /// Model download progress
    Downloading {
        /// Download progress (0.0 to 1.0)
        progress: f32,
        /// Current status message
        status: String,
        /// Whether download failed
        failed: bool,
        /// Error message if failed
        error: Option<String>,
    },
    /// Easter egg: cute cat!
    CuteCat,
    /// Showing the cute cat for 3 seconds
    ShowingCat,
    /// Setup complete - ready to exit
    Complete,
}

/// Custom color values for the custom theme option.
#[derive(Debug, Clone, Default)]
pub struct CustomColors {
    /// Informational color input
    pub informational: String,
    /// Warning color input
    pub warning: String,
    /// Success color input
    pub success: String,
    /// Error color input
    pub error: String,
    /// Question color input
    pub question: String,
}

impl CustomColors {
    /// Gets the field at the given index.
    pub fn get_field(&self, index: usize) -> &str {
        match index {
            0 => &self.informational,
            1 => &self.warning,
            2 => &self.success,
            3 => &self.error,
            4 => &self.question,
            _ => "",
        }
    }

    /// Gets a mutable reference to the field at the given index.
    pub fn get_field_mut(&mut self, index: usize) -> &mut String {
        match index {
            0 => &mut self.informational,
            1 => &mut self.warning,
            2 => &mut self.success,
            3 => &mut self.error,
            _ => &mut self.question,
        }
    }

    /// Gets the field name at the given index.
    pub fn field_name(index: usize) -> &'static str {
        match index {
            0 => "Informational",
            1 => "Warning",
            2 => "Success",
            3 => "Error",
            4 => "Question",
            _ => "",
        }
    }

    /// Parses the custom colors into a ColorScheme.
    pub fn to_scheme(&self) -> Option<ColorScheme> {
        let info = parse_rgb(&self.informational)?;
        let warn = parse_rgb(&self.warning)?;
        let succ = parse_rgb(&self.success)?;
        let err = parse_rgb(&self.error)?;
        let ques = parse_rgb(&self.question)?;

        Some(ColorScheme {
            informational: info,
            warning: warn,
            success: succ,
            error: err,
            question: ques,
        })
    }
}

/// Parses an RGB string like "255,128,64" into a tuple.
fn parse_rgb(s: &str) -> Option<(u8, u8, u8)> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 3 {
        return None;
    }
    let r = parts[0].trim().parse().ok()?;
    let g = parts[1].trim().parse().ok()?;
    let b = parts[2].trim().parse().ok()?;
    Some((r, g, b))
}

/// Focus state for the wordlist configuration screen.
#[derive(Debug, Clone, PartialEq)]
pub enum WordlistFocus {
    /// Navigating predefined wordlist checkboxes.
    /// The cursor field tracks which wordlist is currently highlighted.
    PredefinedList {
        /// Index of the currently highlighted wordlist in the list.
        cursor: usize,
    },
    /// Typing custom file path
    CustomInput,
    /// Typing custom URL
    CustomUrlInput,
    /// Typing custom URL source name
    CustomUrlSource,
    /// Focused on Done button
    Done,
}

/// Download progress information for wordlist downloads.
#[derive(Debug, Clone)]
pub struct DownloadProgress {
    /// Current wordlist index being downloaded
    pub current: usize,
    /// Total wordlists to download
    pub total: usize,
    /// Status message
    pub status: String,
    /// List of failed downloads (wordlist names)
    pub failed: Vec<String>,
}

/// Main setup wizard application struct.
#[derive(Debug)]
pub struct SetupApp {
    /// Current state of the wizard
    pub state: SetupState,
    /// Whether the user chose to quit/skip
    pub should_quit: bool,
    /// Whether setup was skipped (use defaults)
    pub skipped: bool,
    /// Animation tick counter
    pub tick: usize,

    // Configuration values collected during setup
    /// Selected theme
    pub selected_theme: Option<Theme>,
    /// Custom color scheme (if custom was chosen)
    pub custom_scheme: Option<ColorScheme>,
    /// Whether to use top_results mode
    pub top_results: bool,
    /// Timeout value in seconds
    pub timeout: u32,
    /// Wordlist paths (multiple allowed)
    pub wordlist_paths: Vec<String>,
    /// Selected predefined wordlist indexes
    pub selected_predefined_wordlists: Vec<usize>,
    /// Whether enhanced detection is enabled
    pub enhanced_detection: bool,
    /// HuggingFace token (not stored, just used for download)
    pub hf_token: Option<String>,
    /// Model path for enhanced detection
    pub model_path: Option<String>,
    /// Whether the user wants to see the cute cat
    pub show_cat: bool,
}

impl SetupApp {
    /// Creates a new SetupApp in the Welcome state.
    pub fn new() -> Self {
        Self {
            state: SetupState::Welcome,
            should_quit: false,
            skipped: false,
            tick: 0,
            selected_theme: None,
            custom_scheme: None,
            top_results: false,
            timeout: 5,
            wordlist_paths: Vec::new(),
            selected_predefined_wordlists: Vec::new(),
            enhanced_detection: false,
            hf_token: None,
            model_path: None,
            show_cat: false,
        }
    }

    /// Advances the animation tick.
    pub fn tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }

    /// Gets the current step number (1-indexed) for display.
    pub fn current_step(&self) -> usize {
        match &self.state {
            SetupState::Welcome => 1,
            SetupState::Tutorial => 1,
            SetupState::ThemeSelection { .. } => 2,
            SetupState::ResultsMode { .. } => 3,
            SetupState::TimeoutConfig { .. } => 4,
            SetupState::WordlistConfig { .. } => 5,
            SetupState::EnhancedDetection { .. } => 6,
            SetupState::TokenInput { .. } => 6,
            SetupState::Downloading { .. } => 6,
            SetupState::CuteCat => 7,
            SetupState::ShowingCat => 7,
            SetupState::Complete => 7,
        }
    }

    /// Navigates to the next step in the wizard.
    pub fn next_step(&mut self) {
        self.state = match &self.state {
            SetupState::Welcome => SetupState::Tutorial,
            SetupState::Tutorial => SetupState::ThemeSelection {
                selected: 0,
                custom_mode: false,
                custom_colors: CustomColors::default(),
                custom_field: 0,
            },
            SetupState::ThemeSelection {
                selected,
                custom_mode,
                custom_colors,
                ..
            } => {
                // Save the selected theme
                if *custom_mode {
                    if let Some(scheme) = custom_colors.to_scheme() {
                        self.custom_scheme = Some(scheme);
                    }
                } else if *selected < THEMES.len() {
                    self.selected_theme = Some(THEMES[*selected].clone());
                }
                SetupState::ResultsMode { selected: 0 }
            }
            SetupState::ResultsMode { selected } => {
                self.top_results = *selected == 1;
                if self.top_results {
                    SetupState::TimeoutConfig {
                        value: 3,
                        editing: true,
                    }
                } else {
                    SetupState::WordlistConfig {
                        custom_paths: Vec::new(),
                        current_input: String::new(),
                        cursor: 0,
                        selected_predefined: Vec::new(),
                        focus: WordlistFocus::PredefinedList { cursor: 0 },
                        custom_url: String::new(),
                        custom_url_source: String::new(),
                        download_progress: None,
                    }
                }
            }
            SetupState::TimeoutConfig { value, .. } => {
                self.timeout = *value;
                SetupState::WordlistConfig {
                    custom_paths: Vec::new(),
                    current_input: String::new(),
                    cursor: 0,
                    selected_predefined: Vec::new(),
                    focus: WordlistFocus::PredefinedList { cursor: 0 },
                    custom_url: String::new(),
                    custom_url_source: String::new(),
                    download_progress: None,
                }
            }
            SetupState::WordlistConfig {
                custom_paths,
                selected_predefined,
                ..
            } => {
                // Check if we need to download anything
                let total_downloads = selected_predefined.len() + custom_paths.len();

                if total_downloads > 0 {
                    // Start downloads - update state to show progress
                    let paths_clone = custom_paths.clone();
                    let selected_clone = selected_predefined.clone();

                    SetupState::WordlistConfig {
                        custom_paths: paths_clone.clone(),
                        current_input: String::new(),
                        cursor: 0,
                        selected_predefined: selected_clone.clone(),
                        focus: WordlistFocus::Done,
                        custom_url: String::new(),
                        custom_url_source: String::new(),
                        download_progress: Some(DownloadProgress {
                            current: 0,
                            total: total_downloads,
                            status: "Preparing downloads...".to_string(),
                            failed: Vec::new(),
                        }),
                    }
                } else {
                    // No downloads needed, go straight to next step
                    self.wordlist_paths = custom_paths.clone();
                    self.selected_predefined_wordlists = selected_predefined.clone();
                    SetupState::EnhancedDetection { selected: 0 }
                }
            }
            SetupState::EnhancedDetection { selected } => {
                if *selected == 1 {
                    self.enhanced_detection = true;
                    SetupState::TokenInput {
                        token: String::new(),
                        cursor: 0,
                    }
                } else {
                    SetupState::CuteCat
                }
            }
            SetupState::TokenInput { token, .. } => {
                self.hf_token = Some(token.clone());
                // Start the download
                SetupState::Downloading {
                    progress: 0.0,
                    status: "Preparing download...".to_string(),
                    failed: false,
                    error: None,
                }
            }
            SetupState::Downloading { failed, .. } => {
                if *failed {
                    // On failure, continue anyway
                    self.enhanced_detection = false;
                }
                SetupState::CuteCat
            }
            SetupState::CuteCat => {
                // If user said yes to seeing cat, go to ShowingCat; otherwise skip to Complete
                if self.show_cat {
                    SetupState::ShowingCat
                } else {
                    SetupState::Complete
                }
            }
            SetupState::ShowingCat => SetupState::Complete,
            SetupState::Complete => SetupState::Complete,
        };
    }

    /// Navigates to the previous step in the wizard.
    pub fn prev_step(&mut self) {
        self.state = match &self.state {
            SetupState::Welcome => SetupState::Welcome,
            SetupState::Tutorial => SetupState::Welcome,
            SetupState::ThemeSelection { .. } => SetupState::Tutorial,
            SetupState::ResultsMode { .. } => SetupState::ThemeSelection {
                selected: self.get_theme_index(),
                custom_mode: self.custom_scheme.is_some(),
                custom_colors: CustomColors::default(),
                custom_field: 0,
            },
            SetupState::TimeoutConfig { .. } => SetupState::ResultsMode {
                selected: if self.top_results { 1 } else { 0 },
            },
            SetupState::WordlistConfig { .. } => {
                if self.top_results {
                    SetupState::TimeoutConfig {
                        value: self.timeout,
                        editing: false,
                    }
                } else {
                    SetupState::ResultsMode {
                        selected: if self.top_results { 1 } else { 0 },
                    }
                }
            }
            SetupState::EnhancedDetection { .. } => SetupState::WordlistConfig {
                custom_paths: self.wordlist_paths.clone(),
                current_input: String::new(),
                cursor: 0,
                selected_predefined: self.selected_predefined_wordlists.clone(),
                focus: WordlistFocus::PredefinedList { cursor: 0 },
                custom_url: String::new(),
                custom_url_source: String::new(),
                download_progress: None,
            },
            SetupState::TokenInput { .. } => SetupState::EnhancedDetection { selected: 1 },
            SetupState::Downloading { .. } => SetupState::TokenInput {
                token: self.hf_token.clone().unwrap_or_default(),
                cursor: 0,
            },
            SetupState::CuteCat => SetupState::EnhancedDetection {
                selected: if self.enhanced_detection { 1 } else { 0 },
            },
            SetupState::ShowingCat => SetupState::CuteCat,
            SetupState::Complete => SetupState::CuteCat,
        };
    }

    /// Gets the index of the currently selected theme.
    fn get_theme_index(&self) -> usize {
        if let Some(ref theme) = self.selected_theme {
            THEMES
                .iter()
                .position(|t| t.name == theme.name)
                .unwrap_or(0)
        } else {
            0
        }
    }

    /// Skips the setup and uses defaults.
    pub fn skip_setup(&mut self) {
        self.skipped = true;
        self.should_quit = true;
    }

    /// Gets the current color scheme (either selected theme or custom).
    pub fn get_current_scheme(&self) -> ColorScheme {
        if let Some(ref scheme) = self.custom_scheme {
            scheme.clone()
        } else if let Some(ref theme) = self.selected_theme {
            theme.scheme.clone()
        } else {
            // Return scheme based on current selection in ThemeSelection state
            if let SetupState::ThemeSelection {
                selected,
                custom_mode,
                custom_colors,
                ..
            } = &self.state
            {
                if *custom_mode {
                    custom_colors.to_scheme().unwrap_or_default()
                } else if *selected < THEMES.len() {
                    THEMES[*selected].scheme.clone()
                } else {
                    ColorScheme::default()
                }
            } else {
                ColorScheme::default()
            }
        }
    }

    /// Validates the wordlist path.
    pub fn validate_wordlist_path(path: &str) -> Result<(), String> {
        if path.is_empty() {
            return Ok(()); // Empty is valid (disabled)
        }
        let p = Path::new(path);
        if !p.exists() {
            return Err("File does not exist".to_string());
        }
        if !p.is_file() {
            return Err("Path is not a file".to_string());
        }
        // Try to open the file to check read permissions
        std::fs::File::open(p).map_err(|e| format!("Cannot read file: {}", e))?;
        Ok(())
    }

    /// Builds the final configuration HashMap.
    pub fn build_config(&self) -> HashMap<String, String> {
        let mut config = HashMap::new();

        // Add color scheme
        let scheme = self.get_current_scheme();
        for (key, value) in scheme.to_config_strings() {
            config.insert(key, value);
        }

        // Add top_results
        config.insert("top_results".to_string(), self.top_results.to_string());

        // Add timeout
        config.insert("timeout".to_string(), self.timeout.to_string());

        // Add wordlist paths if any (use first one for now, config supports single path)
        // TODO: Update config to support multiple wordlist paths
        if let Some(first_path) = self.wordlist_paths.first() {
            config.insert("wordlist_path".to_string(), first_path.clone());
        }

        // Add enhanced detection settings
        config.insert(
            "enhanced_detection".to_string(),
            self.enhanced_detection.to_string(),
        );
        if let Some(ref path) = self.model_path {
            config.insert("model_path".to_string(), path.clone());
        }

        config
    }
}

impl Default for SetupApp {
    fn default() -> Self {
        Self::new()
    }
}
