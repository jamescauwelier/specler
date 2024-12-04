use crate::core::spec::contains_spec::ContainsSpec;
use crate::core::spec_error::SpecError;
use crate::core::validated::Validated;

/// # CreateWithSpecification
///
/// A trait to create a new instance of a type where a spec
/// is first validated to provide an `Ok(T1)` or an `Err(SpecError)`.
pub trait CreateWithSpecification<T1: Clone,T2,S: ContainsSpec<T1>>
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