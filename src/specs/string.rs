use crate::core::validator::validator_result::ValidatorResult;

/// Validates that a string is not empty
pub fn not_empty(input: &String) -> ValidatorResult {
    if input.is_empty() {
        ValidatorResult::Invalid("String cannot be empty".to_string())
    } else {
        ValidatorResult::Valid
    }
}