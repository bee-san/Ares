pub struct CheckObject{
    /// If our checkers return success, we change this bool to True
    pub is_identified: bool,
    /// text is the text before we check it.
    pub text: String,
    /// Checker is the function we used to check the text
    pub checker: String,
    /// Description about identifications
    pub description: String,
    /// Link is a link to more info about the checker
    pub link: String,
}