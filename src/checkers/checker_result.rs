use crate::checkers::checker_type::CheckerType;

pub struct CheckResult {
    /// If our checkers return success, we change this bool to True
    pub is_identified: bool,
    /// text is the text before we check it.
    pub text: &'static str,
    /// Checker is the function we used to check the text
    pub checker: CheckerType,
}


/// To save time we have a default 
/// for checkResult in case we fail
/// I do not believe the checker is important if failed
/// as we will not use it. To save time we will return a default
/// checker. 
impl CheckResult{
    fn New(checker_used: CheckerType) -> CheckResult {
        CheckResult {
            is_identified: false,
            text: "",
            checker: checker_used,
        }
    }
}