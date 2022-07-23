use crate::checkers::checker_type::CheckerType;

pub struct CheckResult<'text> {
    /// If our checkers return success, we change this bool to True
    pub is_identified: bool,
    /// text is the text before we check it.
    pub text: &'text str,
    /// Checker is the function we used to check the text
    pub checker: CheckerType,
}
