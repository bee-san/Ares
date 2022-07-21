pub struct CheckObject<'text>{
    /// If our checkers return success, we change this bool to True
    pub is_identified: bool,
    /// text is the text before we check it.
    pub text: &'text str,
    /// Checker is the function we used to check the text
    pub checker: &'static str,
    /// Description about identifications
    pub description: String,
    /// Link is a link to more info about the checker
    pub link: &'static str,
}