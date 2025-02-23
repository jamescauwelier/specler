use proptest::prelude::*;

pub mod prelude {
    pub use crate::SpecStrategies;
    pub use crate::impl_arbitrary;
}

pub trait SpecStrategies<T> {
    fn valid_strategy() -> impl Strategy<Value = T>;
    fn invalid_strategy() -> impl Strategy<Value = T>;
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
            impl proptest::prelude::Arbitrary for $target {
                type Parameters = ();

                fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                    use proptest::prelude::*;
                    use specler::prelude::*;
                    use specler_arbitrary::prelude::*;

                    $spec::valid_strategy()
                        .prop_map(|s| $target::create(s))
                        .prop_map(core::result::Result::unwrap)
                        .boxed()
                }

                type Strategy = proptest::prelude::BoxedStrategy<$target>;
            }
        };
    }