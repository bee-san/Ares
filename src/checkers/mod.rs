mod english;
mod lemmeknow;
mod checkerObject;

pub fn check(input: &str) -> bool {
    // Uses lemmeknow to check if any regexes match
    // import and call lemmeknow.rs
    if lemmeknow::check_lemmeknow(input).is_some() {
        return true;
    }

    if english::check_english(input).is_some() {
        return true;
    }

    false
}

// test
#[cfg(test)]
mod tests {
    use crate::checkers::check;

    #[test]
    fn test_check_ip_address() {
        assert_eq!(true, check("192.168.0.1"));
    }
}
