use std::marker::PhantomData;
use crate::core::spec::contains_spec::ContainsSpec;
use crate::core::spec_error::SpecError;

pub enum Validated<T,S>
where
    T: Clone,
    S: ContainsSpec<T>
{
    Valid {
        value: T,
        _spec: PhantomData<S>
    },
    Invalid {
        error: SpecError,
        _spec: PhantomData<S>
    }
}

impl <T,S> From<T> for Validated<T,S>
where
    T: Clone,
    S: ContainsSpec<T>
{
    fn from(value: T) -> Validated<T,S> {
        let spec = S::spec();
        match spec.validate(value) {
            Ok(v) => Validated::Valid {
                value: v,
                _spec: PhantomData
            },
            Err(e) => Validated::Invalid {
                error: e,
                _spec: PhantomData
            }
        }
    }
}