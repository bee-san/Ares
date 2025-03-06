//! First-run configuration module for Ares
//!
//! This module handles the initial setup of Ares, including color scheme configuration
//! and user preferences. It provides functionality for creating and managing color schemes,
//! handling user input, and converting between different color formats.

use colored::Colorize;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::{self, Write};

/// Represents a color scheme with RGB values for different message types and roles.
/// Each color is stored as a comma-separated RGB string in the format "r,g,b"
/// where r, g, and b are values between 0 and 255.
#[derive(Debug)]
pub struct ColorScheme {
    /// RGB color value for informational messages in format "r,g,b"
    /// Used for general information and status updates
    pub informational: String,
    /// RGB color value for warning messages in format "r,g,b"
    /// Used for non-critical warnings and cautions
    pub warning: String,
    /// RGB color value for success messages in format "r,g,b"
    /// Used for successful operations and confirmations
    pub success: String,
    /// RGB color value for question prompts in format "r,g,b"
    /// Used for interactive prompts and user queries
    pub question: String,
    /// RGB color value for general statements in format "r,g,b"
    /// Used for standard output and neutral messages
    pub statement: String,
}

/// Prints a statement in white color.
///
/// # Arguments
/// * `text` - Any type that implements Display trait to be printed in white
///
/// # Returns
/// * `String` - The input text formatted in white color
fn print_statement<T: Display>(text: T) -> String {
    text.to_string().white().to_string()
}

/// Prints a warning message in red color.
///
/// # Arguments
/// * `text` - Any type that implements Display trait to be printed in red
///
/// # Returns
/// * `String` - The input text formatted in red color
fn print_warning<T: Display>(text: T) -> String {
    text.to_string().red().to_string()
}

/// Prints a question prompt in yellow color.
///
/// # Arguments
/// * `text` - Any type that implements Display trait to be printed in yellow
///
/// # Returns
/// * `String` - The input text formatted in yellow color
fn print_question<T: Display>(text: T) -> String {
    text.to_string().yellow().to_string()
}

/// Prints text in a specified RGB color.
///
/// # Arguments
/// * `text` - The text to be colored
/// * `rgb` - RGB color string in format "r,g,b" where r,g,b are 0-255
///
/// # Returns
/// * `String` - The text colored with the specified RGB values, or uncolored if RGB format is invalid
fn print_rgb(text: &str, rgb: &str) -> String {
    let parts: Vec<&str> = rgb.split(',').collect();
    if parts.len() != 3 {
        return text.to_string();
    }

    if let (Ok(r), Ok(g), Ok(b)) = (
        parts[0].trim().parse::<u8>(),
        parts[1].trim().parse::<u8>(),
        parts[2].trim().parse::<u8>(),
    ) {
        text.truecolor(r, g, b).to_string()
    } else {
        text.to_string()
    }
}

/// Returns the Capptucin color scheme with warm, muted colors.
///
/// # Returns
/// * `ColorScheme` - A color scheme with Capptucin's signature warm colors
fn get_capptucin_scheme() -> ColorScheme {
    ColorScheme {
        informational: "238,212,159".to_string(), // rgb(238, 212, 159)
        warning: "237,135,150".to_string(),       // rgb(237, 135, 150)
        success: "166,218,149".to_string(),       // rgb(166, 218, 149)
        question: "244,219,214".to_string(),      // rgb(244, 219, 214)
        statement: "202,211,245".to_string(),     // rgb(202, 211, 245)
    }
}

/// Returns the Darcula color scheme matching JetBrains Darcula theme.
///
/// # Returns
/// * `ColorScheme` - A color scheme with Darcula's signature colors
fn get_darcula_scheme() -> ColorScheme {
    ColorScheme {
        informational: "241,250,140".to_string(), // rgb(241, 250, 140)
        warning: "255,85,85".to_string(),         // rgb(255, 85, 85)
        success: "80,250,123".to_string(),        // rgb(80, 250, 123)
        question: "139,233,253".to_string(),      // rgb(139, 233, 253)
        statement: "248,248,242".to_string(),     // rgb(248, 248, 242)
    }
}

/// Returns Autumn's personal Girly Pop theme with pink and pastel colors.
///
/// # Returns
/// * `ColorScheme` - A color scheme with Girly Pop's signature pastel colors
fn get_girly_pop_scheme() -> ColorScheme {
    ColorScheme {
        informational: "237,69,146".to_string(), // rgb(237,69,146)
        warning: "241,218,165".to_string(),      // rgb(241, 218, 165)
        success: "243,214,243".to_string(),      // rgb(243, 214, 243)
        question: "255,128,177".to_string(),     // rgb(255, 128, 177)
        statement: "255,148,219".to_string(),    // rgb(255, 148, 219)
    }
}

/// Returns the default color scheme with standard terminal colors.
///
/// # Returns
/// * `ColorScheme` - A color scheme with standard, high-contrast colors
fn get_default_scheme() -> ColorScheme {
    ColorScheme {
        informational: "255,215,0".to_string(), // Gold yellow
        warning: "255,0,0".to_string(),         // Red
        success: "0,255,0".to_string(),         // Green
        question: "255,215,0".to_string(),      // Gold yellow (same as informational)
        statement: "255,255,255".to_string(),   // White
    }
}

/// Runs the first-time setup wizard for Ares, allowing users to configure their color scheme.
///
/// This function presents users with color scheme options and handles their selection,
/// including support for custom color schemes. It guides users through the setup process
/// with clear prompts and visual examples of each color scheme.
///
/// # Returns
/// * `HashMap<String, String>` - A mapping of role names to their RGB color values
pub fn run_first_time_setup() -> HashMap<String, String> {
    println!(
        "\n{}",
        print_statement("🤠 Howdy! This is your first time running Ares.")
    );
    println!("{}\n", print_statement("Let me help you configure Ares."));

    // Ask if the user wants a custom color scheme
    let want_custom = ask_yes_no_question("Do you want a custom colour scheme?", false);

    if !want_custom {
        // User doesn't want a custom color scheme, use default
        return color_scheme_to_hashmap(get_default_scheme());
    }

    // Show color scheme options
    println!(
        "\n{}",
        print_statement("What colour scheme looks best to you?")
    );

    println!("1. Capptucin");
    let capptucin = get_capptucin_scheme();
    print!("   ");
    print!(
        "{} | ",
        print_rgb("Informational", &capptucin.informational)
    );
    print!("{} | ", print_rgb("Warning", &capptucin.warning));
    print!("{} | ", print_rgb("Success", &capptucin.success));
    print!("{} | ", print_rgb("Questions", &capptucin.question));
    println!("{}\n", print_rgb("Statements", &capptucin.statement));

    println!("2. Darcula");
    let darcula = get_darcula_scheme();
    print!("   ");
    print!("{} | ", print_rgb("Informational", &darcula.informational));
    print!("{} | ", print_rgb("Warning", &darcula.warning));
    print!("{} | ", print_rgb("Success", &darcula.success));
    print!("{} | ", print_rgb("Questions", &darcula.question));
    println!("{}\n", print_rgb("Statements", &darcula.statement));

    println!("3. 💖✨💐 Girly Pop");
    let girly = get_girly_pop_scheme();
    print!("   ");
    print!("{} | ", print_rgb("Informational", &girly.informational));
    print!("{} | ", print_rgb("Warning", &girly.warning));
    print!("{} | ", print_rgb("Success", &girly.success));
    print!("{} | ", print_rgb("Questions", &girly.question));
    println!("{}\n", print_rgb("Statements", &girly.statement));

    println!("4. Default");
    let default = get_default_scheme();
    print!("   ");
    print!("{} | ", print_rgb("Informational", &default.informational));
    print!("{} | ", print_rgb("Warning", &default.warning));
    print!("{} | ", print_rgb("Success", &default.success));
    print!("{} | ", print_rgb("Questions", &default.question));
    println!("{}\n", print_rgb("Statements", &default.statement));

    // For the Custom option, show format instructions
    println!("5. Custom");
    println!("   Format: r,g,b (e.g., 255,0,0 for red)");
    println!("   Values must be between 0 and 255");
    println!("   You'll be prompted to enter RGB values for each color.\n");

    // Get user's choice
    let choice = get_user_input_range("Enter your choice (1-5): ", 1, 5);

    match choice {
        1 => color_scheme_to_hashmap(get_capptucin_scheme()),
        2 => color_scheme_to_hashmap(get_darcula_scheme()),
        3 => color_scheme_to_hashmap(get_girly_pop_scheme()),
        4 => color_scheme_to_hashmap(get_default_scheme()),
        5 => {
            // Custom color scheme
            println!(
                "\n{}",
                print_statement("Enter RGB values for each color (format: r,g,b)")
            );

            let informational = get_user_input_rgb("Informational: ");
            let warning = get_user_input_rgb("Warning: ");
            let success = get_user_input_rgb("Success: ");
            let question = get_user_input_rgb("Questions: ");
            let statement = get_user_input_rgb("Statements: ");

            let custom_scheme = ColorScheme {
                informational,
                warning,
                success,
                question,
                statement,
            };

            color_scheme_to_hashmap(custom_scheme)
        }
        _ => color_scheme_to_hashmap(get_default_scheme()), // This should never happen due to input validation
    }
}

/// Prompts the user with a yes/no question and returns their response.
///
/// # Arguments
/// * `question` - The question to display to the user
/// * `default_yes` - Whether the default answer (when user presses enter) should be yes
///
/// # Returns
/// * `bool` - true for yes, false for no
fn ask_yes_no_question(question: &str, default_yes: bool) -> bool {
    let prompt = if default_yes {
        format!("{} (Y/n): ", question)
    } else {
        format!("{} (y/N): ", question)
    };

    print!("{}", print_question(&prompt));
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_lowercase();

    if input.is_empty() {
        return default_yes;
    }

    match input.as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => {
            println!(
                "{}",
                print_warning("Invalid input. Please enter 'y' or 'n'.")
            );
            ask_yes_no_question(question, default_yes)
        }
    }
}

/// Gets user input within a specified numeric range.
///
/// # Arguments
/// * `prompt` - The prompt to display to the user
/// * `min` - The minimum acceptable value (inclusive)
/// * `max` - The maximum acceptable value (inclusive)
///
/// # Returns
/// * `u32` - The user's input within the specified range
fn get_user_input_range(prompt: &str, min: u32, max: u32) -> u32 {
    print!("{}", print_question(prompt));
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();

    match input.parse::<u32>() {
        Ok(num) if num >= min && num <= max => num,
        _ => {
            println!(
                "{}",
                print_warning(format!(
                    "Invalid input. Please enter a number between {} and {}.",
                    min, max
                ))
            );
            get_user_input_range(prompt, min, max)
        }
    }
}

/// Gets user input for RGB color values.
///
/// # Arguments
/// * `prompt` - The prompt to display to the user
///
/// # Returns
/// * `String` - A validated RGB color string in format "r,g,b"
fn get_user_input_rgb(prompt: &str) -> String {
    print!("{}", print_question(prompt));
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();

    // Validate RGB format (r,g,b)
    if let Some(rgb) = parse_rgb_input(input) {
        rgb
    } else {
        println!(
            "{}",
            print_warning("Invalid RGB format. Please use the format 'r,g,b' (e.g., '255,0,0').")
        );
        get_user_input_rgb(prompt)
    }
}

/// Parses and validates an RGB input string.
///
/// # Arguments
/// * `input` - The RGB string to parse in format "r,g,b"
///
/// # Returns
/// * `Option<String>` - Some(rgb) if valid, None if invalid
fn parse_rgb_input(input: &str) -> Option<String> {
    let parts: Vec<&str> = input.split(',').collect();

    if parts.len() != 3 {
        return None;
    }

    let r = parts[0].trim().parse::<u8>().ok()?;
    let g = parts[1].trim().parse::<u8>().ok()?;
    let b = parts[2].trim().parse::<u8>().ok()?;

    Some(format!("{},{},{}", r, g, b))
}

/// Converts a ColorScheme struct to a HashMap for configuration storage.
///
/// # Arguments
/// * `scheme` - The ColorScheme to convert
///
/// # Returns
/// * `HashMap<String, String>` - A mapping of role names to their RGB values
fn color_scheme_to_hashmap(scheme: ColorScheme) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("informational".to_string(), scheme.informational);
    map.insert("warning".to_string(), scheme.warning);
    map.insert("success".to_string(), scheme.success);
    map.insert("question".to_string(), scheme.question);
    map.insert("statement".to_string(), scheme.statement);
    map
}
