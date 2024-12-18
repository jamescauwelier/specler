use crate::core::spec::spec_provider::SpecProvider;
use crate::core::spec_error::SpecError;
use crate::core::validated::Validated;

/// # ValueObjectFactory
///
/// A trait to create a new instance of a value object where a spec
/// is first validated to provide an `Ok(T1)` or an `Err(SpecError)`.
pub trait ValueObjectFactory<T1: Clone,T2,S: SpecProvider<T1>>
where
    T1: Into<Validated<T1,S>>
{
    /// Factory method to create a new instance of a type
    /// after validating a spec. Instead of directly returning a
    /// T2, it returns a Result<T2, SpecError>.
    /// Most often, T2 == T1, most importantly this will be the
    /// case in the macro implementation.
    fn create(
        pre_validated_input: impl Into<T1>,
    ) -> Result<T2, SpecError>;
}

/// # ValueObject
///
/// A value object is an object that encapsulates a simple
/// value and provides a getter method to access its internal
/// value, even though you're discouraged to use it directly.
///
/// The value access might be needed though when, for
/// example, mapping the object structures into some kind
/// of storage, like a database. It can also be useful or testing
/// purposes.
pub trait ValueObject<T> {
    /// Returns a reference to the internal value of the value object
    fn value(&self) -> &T;
}