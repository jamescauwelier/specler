use std::marker::PhantomData;
use crate::core::spec::Spec;

/// A marker struct to indicate that a validation is required
pub struct Require<T: Clone> {
    phantom_data: PhantomData<T>
}

impl <T: Clone> Require<T> {
    /// Starts the creation of a new validation spec
    pub fn to() -> Spec<T> {
        Spec::<T>::new()
    }
}
