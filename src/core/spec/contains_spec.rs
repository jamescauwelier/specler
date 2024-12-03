use crate::core::spec::Spec;

pub trait ContainsSpec<T> {
    fn spec() -> Spec<T>;
}