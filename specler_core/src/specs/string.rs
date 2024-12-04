use regex::Regex;
use crate::core::validator::validator_result::ValidatorResult;

/// Validates that a string is not empty
pub fn not_empty() -> Box<dyn Fn(&str) -> ValidatorResult> {
    Box::new(
        move |input: &str| {
            if input.is_empty() {
                ValidatorResult::invalid("cannot be empty")
            } else {
                ValidatorResult::valid()
            }
        }
    )
}

/// Validates a string is shorter than a given length
pub fn no_longer_than(length: usize) -> Box<dyn Fn(&str) -> ValidatorResult> {
    Box::new(
        move |input: &str | {
            if input.len() <= length {
                ValidatorResult::valid()
            } else {
                ValidatorResult::invalid(&format!("cannot be longer than {} characters", length))
            }
        }
    )
}

/// Validates a string is longer than a given length
pub fn no_shorter_than(length: usize) -> Box<dyn Fn(&str) -> ValidatorResult> {
    Box::new(
        move |input: &str| {
            if input.len() >= length {
                ValidatorResult::valid()
            } else {
                ValidatorResult::invalid(&format!("cannot be shorter than {} characters", length))
            }
        }
    )
}

/// Validates a string matches a given regex pattern
pub fn matching(pattern: &str) -> Box<dyn Fn(&str) -> ValidatorResult + '_> {
    Box::new(
        move |input: &str| {
            match Regex::new(pattern) {
                Ok(re) => {
                    if re.is_match(input) {
                        ValidatorResult::valid()
                    } else {
                        ValidatorResult::invalid(format!("does not match the pattern '{}'", pattern).as_str())
                    }
                }
                Err(_) => ValidatorResult::invalid(format!("invalid pattern '{}'", pattern).as_str())
            }
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    mod not_empty {
        use crate::core::validator::validator_result::ValidatorResult;

        #[test]
        fn test_valid() {
            let result = super::not_empty()(&"James".to_string());
            assert_eq!(result, ValidatorResult::valid());
        }

        #[test]
        fn test_invalid() {
            let result = super::not_empty()(&"".to_string());
            assert_eq!(result.error_message(), "cannot be empty");
        }
    }

    mod not_be_longer_than {
        #[test]
        fn test_valid() {
            let input = &"James".to_string();
            let result = super::no_longer_than(6)(input);
            assert!(result.is_valid());
        }

        #[test]
        fn test_invalid() {
            let input = &"James".to_string();
            let result = super::no_longer_than(4)(input);
            assert!(!result.is_valid());
            assert_eq!(result.error_message(), "cannot be longer than 4 characters");
        }
    }

    mod not_be_shorter_than {
        #[test]
        fn test_valid() {
            let input = &"James".to_string();
            let result = super::no_shorter_than(4)(input);
            assert!(result.is_valid());
        }

        #[test]
        fn test_invalid() {
            let input = &"James".to_string();
            let result = super::no_shorter_than(6)(input);
            assert!(!result.is_valid());
            assert_eq!(result.error_message(), "cannot be shorter than 6 characters");
        }
    }

    mod matching {

        const PATTERN: &str = r"^abba$";

        #[test]
        fn test_valid() {
            let valid_input: String = String::from("abba");
            let result = super::matching(PATTERN)(&valid_input);
            assert!(result.is_valid());
        }

        #[test]
        fn test_invalid() {
            let invalid_input: String = String::from("oops123");
            let result = super::matching(r"^[a-zA-Z]+$")(&invalid_input);
            assert!(!result.is_valid());
            assert_eq!(result.error_message(), "does not match the pattern '^[a-zA-Z]+$'");
        }
    }
}