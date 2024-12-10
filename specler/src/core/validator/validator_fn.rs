use crate::core::validator::validator_result::ValidatorResult;

pub type ValidatorFn<T> = Box<dyn Fn(T) -> ValidatorResult>;