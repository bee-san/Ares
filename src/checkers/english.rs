// import storage
use crate::storage;

// given an input, check every item in the array and return true if any of them match
fn check(input: &str) -> bool {
    storage::STORAGE.contains(input)
}

#[cfg(test)]
mod tests {
    use crate::checkers::check;

    #[test]
    fn test_check_and() {
        assert_eq!(true, check("preinterview"));
    }
}