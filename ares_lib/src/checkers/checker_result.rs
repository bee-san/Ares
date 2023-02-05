use super::checker_type::Checker;

/// The checkerResult struct is used to store the results of a checker.
pub struct CheckResult {
    /// If our checkers return success, we change this bool to True
    pub is_identified: bool,
    /// text is the text before we check it.
    // we can make this &'text str
    // but then crack requires lifetime annotations.
    pub text: String,
    /// Description of the checked text.
    pub description: String,
    /// Name of the Checker we are using
    pub checker_name: &'static str,
    /// Description of the Checker we are using
    pub checker_description: &'static str,
    /// Link to more info about checker
    pub link: &'static str,
}

/// To save time we have a default
/// for checkResult in case we fail
/// I do not believe the checker is important if failed
/// as we will not use it. To save time we will return a default
/// checker.
impl CheckResult {
    /// Creates a default CheckResult
    pub fn new<Type>(checker_used: &Checker<Type>) -> CheckResult {
        CheckResult {
            is_identified: false,
            text: "".to_string(),
            checker_name: checker_used.name,
            checker_description: checker_used.description,
            description: "".to_string(),
            link: checker_used.link,
        }
    }
}
