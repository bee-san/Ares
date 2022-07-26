/// Checker_type is a type used to define checkers
/// This means that we can standardise the way we check for plaintext
use crate::checkers::checker_result::CheckResult;
use lemmeknow::Identify;

/// Every checker is of type CheckerType
/// This will let us pick & choose which checkers to use
/// at runtime.
pub struct Checker<Type> {
    /// The name of the checker
    pub name: &'static str,
    /// The description of the checker
    /// you can take the first line from Wikipedia
    /// Sometimes our checkers do not exist on Wikipedia so we write our own.
    pub description: &'static str,
    /// The link to the checker's website
    /// Wikipedia link, articles, github etc
    pub link: &'static str,
    /// The tags of the checker
    pub tags: Vec<&'static str>,
    /// The expected runtime of the checker
    /// We get this by bench marking the code
    pub expected_runtime: f32,
    /// The popularity of the checker
    pub popularity: f32,
    /// lemmeknow config object
    pub lemmeknow_config: Identify,
    pub _phatom: std::marker::PhantomData<Type>,
}

/// Every checker must implement this trait
/// Which checks the given text to see if its plaintext
/// and returns CheckResult, which is our results object.
pub trait Check {
    fn new() -> Self
    where
        Self: Sized;
    fn check(&self, text: &str) -> CheckResult;
}
