use std::marker::PhantomData;
use crate::core::spec::Spec;

pub struct Require<T> {
    phantom_data: PhantomData<T>
}

impl <T> Require<T> {
    pub fn to() -> Spec<T> {
        Spec::new()
    }
}
