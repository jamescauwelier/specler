use crate::core::validator::validator_result::ValidatorResult;

/// Validates that a string is not empty
pub fn not_empty(input: &String) -> ValidatorResult {
    if input.is_empty() {
        ValidatorResult::invalid("cannot be empty")
    } else {
        ValidatorResult::valid()
    }
}

/// Validates a string is shorter than a given length
pub fn not_be_longer_than(input: &String, length: usize) -> ValidatorResult {
    if input.len() <= length {
        ValidatorResult::valid()
    } else {
        ValidatorResult::invalid(&format!("cannot be longer than {} characters", length))
    }
}

/// Validates a string is longer than a given length
pub fn not_be_shorter_than(input: &String, length: usize) -> ValidatorResult {
    if input.len() >= length {
        ValidatorResult::valid()
    } else {
        ValidatorResult::invalid(&format!("cannot be shorter than {} characters", length))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod not_empty {
        use crate::core::validator::validator_result::ValidatorResult;

        #[test]
        fn test_valid() {
            let result = super::not_empty(&"James".to_string());
            assert_eq!(result, ValidatorResult::valid());
        }

        #[test]
        fn test_invalid() {
            let result = super::not_empty(&"".to_string());
            assert_eq!(result.error_message(), "cannot be empty");
        }
    }

    mod not_be_longer_than {
        #[test]
        fn test_valid() {
            let result = super::not_be_longer_than(&"James".to_string(), 6);
            assert!(result.is_valid());
        }

        #[test]
        fn test_invalid() {
            let result = super::not_be_longer_than(&"James".to_string(), 4);
            assert!(!result.is_valid());
            assert_eq!(result.error_message(), "cannot be longer than 4 characters");
        }
    }

    mod not_be_shorter_than {
        #[test]
        fn test_valid() {
            let result = super::not_be_shorter_than(&"James".to_string(), 4);
            assert!(result.is_valid());
        }

        #[test]
        fn test_invalid() {
            let result = super::not_be_shorter_than(&"James".to_string(), 6);
            assert!(!result.is_valid());
            assert_eq!(result.error_message(), "cannot be shorter than 6 characters");
        }
    }
}