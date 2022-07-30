use crate::checkers::checker_result::CheckResult;
use lemmeknow::{Data, Identifier};

use super::checker_type::{Check, Checker};

/// The Lemmeknow checker configuration struct
const IDENTIFIER: Identifier = Identifier {
    min_rarity: 0.1,
    max_rarity: 1.0,
    tags: vec![],
    exclude_tags: vec![],
    file_support: false,
    boundaryless: false,
};

/// The LemmeKnow Checker checks if the text matches a known Regex pattern.
/// This is the struct for it.
pub struct LemmeKnow;

impl Check for Checker<LemmeKnow> {
    fn new() -> Self {
        Checker {
            // TODO: Update fields with proper values
            name: "LemmeKnow Checker",
            description: "Uses LemmeKnow to check for regex matches",
            link: "https://swanandx.github.io/lemmeknow-frontend/",
            tags: vec!["lemmeknow", "regex"],
            expected_runtime: 0.01,
            popularity: 1.0,
            lemmeknow_config: Identifier::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn check(&self, text: &str) -> CheckResult {
        let lemmeknow_result = IDENTIFIER.identify(text);
        let mut is_identified = false;
        let mut description = "".to_string();
        if !lemmeknow_result.is_empty() {
            is_identified = true;
            description = format_data_result(&lemmeknow_result[0].data)
        }

        CheckResult {
            is_identified,
            text: text.to_owned(),
            checker_name: self.name,
            checker_description: self.description,
            // Returns a vector of matches
            description,
            link: self.link,
        }
    }
}

/// Formats the data result to a string
/// This is used to display the result in the UI
fn format_data_result(input: &Data) -> String {
    /*
    Input contains these:
        println!("{}", input.name);
    println!("{}", input.regex);
    println!("{}", input.plural_name);
    println!("{}", input.description);
    println!("{}", input.rarity);
    println!("{}", input.url);
    println!("{:?}", input.tags);
    In the future we'd want to include more advanced things like URL. */
    format!("The plaintext is {}", input.name) // removed .to_string() from here
}
