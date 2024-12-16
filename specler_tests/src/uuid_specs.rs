use specler::prelude::*;
use specler::specs::uuid::a_uuid_v7;
use specler_macros::value_object;

#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
use specler_arbitrary::prelude::*;
#[cfg(test)]
use uuid::Uuid;

#[derive(Debug)]
#[value_object(UuidSpecs)]
struct Id (String);

struct UuidSpecs;
impl SpecProvider<String> for UuidSpecs
{
    fn spec() -> Spec<String> {
        Require::<String>::to()
            .be(a_uuid_v7())
    }
}

#[cfg(test)]
impl SpecStrategies<String> for UuidSpecs {
    fn valid_strategy() -> impl Strategy<Value=String> {
        // note that we could generate a more distributed set of
        // valid UUIDs by using randome timestamps for the
        // context seed
        Just(Uuid::now_v7().to_string()).boxed()
    }

    fn invalid_strategy() -> impl Strategy<Value=String> {
        ".*"
            .prop_filter("String should not be a valid UUIDv7",
                         move |s| {
                             match Uuid::parse_str(s) {
                                 // we filter out uuids that are of version 7
                                 Ok(uuid) => uuid.get_version_num() != 7,
                                 // non-uuids are always invalid input
                                 Err(_) => true
                             }
                         }
            )
            .boxed()
    }
}

#[cfg(test)]
impl_arbitrary!(Id, UuidSpecs);

#[cfg(test)]
mod tests {
    use super::*;

    proptest!{
        #[test]
        fn can_be_created_using_valid_input(s in UuidSpecs::valid_strategy()) {
            let id = Id::create(s);
            assert!(id.is_ok());
        }

        #[test]
        fn cannot_be_created_using_invalid_input(s in UuidSpecs::invalid_strategy()) {
            let id = Id::create(s);
            assert!(!id.is_ok());
        }

        #[test]
        fn is_a_value_object(input: Id) {
            input.value();
        }
    }
}