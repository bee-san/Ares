pub struct CheckObject{
    /// If our checkers return success, we change this bool to True
    is_identified: bool,
    /// text is the text _before_ we check it.
    text: &'static str,
    /// Checker is the function we used to check the text
    checker: &'static str,
    /// Description about identifications
    description: &'static str,
    /// Link is a link to more info about the checker
    link: &'static str,
}