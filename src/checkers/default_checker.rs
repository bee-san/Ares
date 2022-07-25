use lemmeknow::Identify;

use super::checker_type::Checker;

/// The default checker is used to check if the text is plaintext
/// Based on what the Ares team has found to be the best checker.

pub struct DefaultChecker;

impl Default for Checker<DefaultChecker> {
    fn default() -> Self {
        Checker {
            name: "Template checker",
            description: "This is a default template checker. If you're seeing this, it's an error. Please contact us on Discord http://discord.skerritt.blog",
            link: "http://discord.skerritt.blog",
            tags: vec![],
            expected_runtime: 0.0,
            popularity: 0.0,
            lemmeknow_config: Identify::default(),
            _phatom: std::marker::PhantomData,
        }
    }
}
