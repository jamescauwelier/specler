use crate::core::validator::validator_fn::ValidatorFn;
use crate::core::validator::validator_result::ValidatorResult;
use ::uuid::Uuid;

fn a_uuid(version: usize) -> ValidatorFn<String> {
    Box::new(
        move |input: String| {
            match Uuid::parse_str(&input) {
                Ok(uuid) => {
                    if uuid.get_version_num() == version {
                        ValidatorResult::valid()
                    } else {
                        ValidatorResult::invalid(format!("valid UUID string, but not a UUID v{}", version).as_str())
                    }
                },
                Err(_) => ValidatorResult::invalid(format!("invalid UUID v{} string", version).as_str())
            }
        }
    )
}

/// Produces a validator for UUID v4 strings, which are random
/// and not sortable.
pub fn a_uuid_v4() -> ValidatorFn<String> {
    a_uuid(4)
}

/// Produces a validator for UUID v7 strings, which are random
/// but time sortable. These are best for database identifiers.
pub fn a_uuid_v7() -> ValidatorFn<String> {
    a_uuid(7)
}

#[cfg(test)]
mod uuid {
    const UUID_V4: &str = "d79c2c06-4656-4579-83f3-293e1a398a72";
    const UUID_V7: &str = "0193b1b9-a68c-7e91-9fc4-19dfe321bd4f";
    const INVALID_UUID: &str = "abc";

    mod v4 {
        use crate::specs::uuid::a_uuid_v4;

        #[test]
        fn test_valid() {
            let result = a_uuid_v4()(super::UUID_V4.into());
            assert!(result.is_valid());
        }

        #[test]
        fn test_invalid_type_7() {
            let result = a_uuid_v4()(super::UUID_V7.into());
            assert!(!result.is_valid());
        }

        #[test]
        fn test_not_a_uuid() {
            let result = a_uuid_v4()(super::INVALID_UUID.into());
            assert!(!result.is_valid());
        }
    }

    mod v7 {
        use crate::specs::uuid::a_uuid_v7;

        #[test]
        fn test_valid() {
            let result = a_uuid_v7()(super::UUID_V7.into());
            assert!(result.is_valid());
        }

        #[test]
        fn test_invalid_type_4() {
            let result = a_uuid_v7()(super::UUID_V4.into());
            assert!(!result.is_valid());
        }

        #[test]
        fn test_not_a_uuid() {
            let result = a_uuid_v7()(super::INVALID_UUID.into());
            assert!(!result.is_valid());
        }
    }
}