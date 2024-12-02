#[derive(Debug, PartialEq)]
pub enum ValidatorResult {
    Valid,
    Invalid(String)
}