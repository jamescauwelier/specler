use crate::core::spec::Spec;

/// # SpecProvider
///
/// Trait to provide a specification on a type `T`.
pub trait SpecProvider<T> {
    /// Provides the spec
    fn spec() -> Spec<T>;
}