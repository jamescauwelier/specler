use email_address::EmailAddress;
use regex::Regex;
use crate::core::validator::validator_result::ValidatorResult;

type ValidatorFn<T> = Box<dyn Fn(T) -> ValidatorResult>;

/// Validates that a string is not empty
pub fn not_empty() -> ValidatorFn<String> {
    Box::new(
        move |input: String| {
            if input.is_empty() {
                ValidatorResult::invalid("cannot be empty")
            } else {
                ValidatorResult::valid()
            }
        }
    )
}

/// Validates a string is shorter than a given length
pub fn no_longer_than(length: usize) -> ValidatorFn<String> {
    Box::new(
        move |input: String | {
            if input.len() <= length {
                ValidatorResult::valid()
            } else {
                ValidatorResult::invalid(&format!("cannot be longer than {} characters", length))
            }
        }
    )
}

/// Validates a string is longer than a given length
pub fn no_shorter_than(length: usize) -> ValidatorFn<String> {
    Box::new(
        move |input: String| {
            if input.len() >= length {
                ValidatorResult::valid()
            } else {
                ValidatorResult::invalid(&format!("cannot be shorter than {} characters", length))
            }
        }
    )
}

/// Validates a string matches a given regex pattern
pub fn matching(pattern: &str) -> Box<dyn Fn(String) -> ValidatorResult + '_> {
    Box::new(
        move |input: String| {
            match Regex::new(pattern) {
                Ok(re) => {
                    if re.is_match(&input) {
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

/// Validates a string is a valid email address
pub fn an_email() -> ValidatorFn<String> {
    Box::new(
        move |input: String| {
            match EmailAddress::is_valid(&input) {
                true => ValidatorResult::valid(),
                false => ValidatorResult::invalid("is not a valid email address")
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
            let result = super::not_empty()("James".to_string());
            assert_eq!(result, ValidatorResult::valid());
        }

        #[test]
        fn test_invalid() {
            let result = super::not_empty()("".to_string());
            assert_eq!(result.error_message(), "cannot be empty");
        }
    }

    mod not_be_longer_than {
        #[test]
        fn test_valid() {
            let input = "James".to_string();
            let result = super::no_longer_than(6)(input);
            assert!(result.is_valid());
        }

        #[test]
        fn test_invalid() {
            let input = "James".to_string();
            let result = super::no_longer_than(4)(input);
            assert!(!result.is_valid());
            assert_eq!(result.error_message(), "cannot be longer than 4 characters");
        }
    }

    mod not_be_shorter_than {
        #[test]
        fn test_valid() {
            let input = "James".to_string();
            let result = super::no_shorter_than(4)(input);
            assert!(result.is_valid());
        }

        #[test]
        fn test_invalid() {
            let input = "James".to_string();
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
            let result = super::matching(PATTERN)(valid_input);
            assert!(result.is_valid());
        }

        #[test]
        fn test_invalid() {
            let pattern = r"^[a-zA-Z]+$";
            let invalid_input: String = String::from("oops123");
            let result = super::matching(pattern)(invalid_input);
            assert!(!result.is_valid());
            assert_eq!(result.error_message(), "does not match the pattern '^[a-zA-Z]+$'");
        }
    }

    mod email {

        mod test_valid {

            macro_rules! test_valid_email {
                ($test_id:ident, $email:expr) => {
                    #[test]
                    fn $test_id() {
                        let valid_input: String = String::from($email);
                        let result = crate::specs::string::an_email()(valid_input.clone());
                        assert!(result.is_valid(), "'{}' should be a valid email address, but is not", valid_input);
                    }
                };
            }

            test_valid_email!(test_valid_email_1, "a@b.something");
            test_valid_email!(test_valid_email_2, "john.doe@some.domain.com");
            test_valid_email!(test_valid_email_3, "john.doe+a45d8f89@abc123.com");
        }

        mod test_invalid {

            macro_rules! test_invalid_email {
                ($test_id:ident, $email:expr) => {
                    #[test]
                    fn $test_id() {
                        let valid_input: String = String::from($email);
                        let result = crate::specs::string::an_email()(valid_input.clone());
                        assert!(!result.is_valid(), "'{}' should be an invalid email address, but is marked as valid", valid_input);
                    }
                };
            }

            test_invalid_email!(test_invalid_email_1, "a");
            test_invalid_email!(test_invalid_email_2, "@b");
        }
    }
}