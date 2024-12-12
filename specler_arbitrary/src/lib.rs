use proptest::prelude::*;

pub mod prelude {
    pub use crate::ArbitraryValidSpecValue;
    pub use crate::ArbitraryInvalidSpecValue;
    pub use crate::impl_arbitrary;
}

pub trait ArbitraryValidSpecValue<T> {
    fn any_valid_value() -> BoxedStrategy<T>;
}

pub trait ArbitraryInvalidSpecValue<T> {
    fn any_invalid_value() -> BoxedStrategy<T>;
}

#[macro_export]
macro_rules! impl_arbitrary {
        (
            /// The target type to generate arbitrary values for
            $target:ident,
            /// The spec that knows how to generate valid input values
            /// to provide to the factory method of the target type
            $spec:ident
        ) => {
            impl Arbitrary for $target {
                type Parameters = ();

                fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                    $spec::any_valid_value()
                        .prop_map(|s| $target::create(s))
                        .prop_map(Result::unwrap)
                        .boxed()
                }

                type Strategy = BoxedStrategy<$target>;
            }
        };
    }