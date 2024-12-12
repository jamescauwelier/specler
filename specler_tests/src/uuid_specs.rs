use specler::prelude::*;
use specler::specs::uuid::a_uuid_v7;
use specler_macros::value_object;

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
mod tests {
    use crate::{verify_invalid_input, verify_valid_input};
    use super::*;

    verify_valid_input!(valid_uuid_v7, "0193b205-ab8e-7b34-a13e-e18b9941dc05", Id::create);

    verify_invalid_input!(invalid_because_empty, "", Id::create);
    verify_invalid_input!(invalid_because_v4, "0df7a521-d96e-4732-90c3-19aec144415c", Id::create);
}