use crate::core::validator::validator_result::ValidatorResult;

/// Validates that a string is not empty
pub fn not_empty(input: &String) -> ValidatorResult {
    if input.is_empty() {
        ValidatorResult::Invalid("String cannot be empty".to_string())
    } else {
        ValidatorResult::Valid
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
            assert_eq!(result, ValidatorResult::Valid);
        }

        #[test]
        fn test_invalid() {
            let result = super::not_empty(&"".to_string());
            assert_eq!(result, ValidatorResult::Invalid("String cannot be empty".to_string()));
        }
    }
}