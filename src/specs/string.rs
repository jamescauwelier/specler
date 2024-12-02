use crate::core::validator::validator_result::ValidatorResult;

/// Validates that a string is not empty
pub fn not_empty(input: &String) -> ValidatorResult {
    if input.is_empty() {
        ValidatorResult::invalid("cannot be empty")
    } else {
        ValidatorResult::valid()
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
}