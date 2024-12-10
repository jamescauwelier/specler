use crate::core::validator::validator_result::ValidatorResult;

pub(crate) mod validator_result;
pub mod validator_fn;

pub type Validator<T> = Box<dyn Fn(T) -> ValidatorResult>;
