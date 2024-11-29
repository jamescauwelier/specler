/// Asserts that a `SpecValidatorResult` is valid
#[macro_export]
macro_rules! assert_spec_valid {
        ($result:expr) => {
            match $result {
                SpecValidatorResult::Valid(_) => (),
                SpecValidatorResult::Invalid(v, _) => {
                    panic!("Expected '{}' to pass the specification, but got errors", v)
                }
            }
        };
    }

/// Asserts that a `SpecValidatorResult` is invalid
#[macro_export]
macro_rules! assert_spec_invalid {
        ($result:expr) => {
            match $result {
                SpecValidatorResult::Invalid(_, _) => (),
                SpecValidatorResult::Valid(_) => {
                    panic!("Expected an invalid result, but got valid")
                }
            }
        };
    }

/// Asserts that a `SpecValidatorResult` is invalid and contains the expected error
#[macro_export]
macro_rules! assert_spec_validation_error {
        ($result:expr, $expected_err:expr) => {
            match $result {
                SpecValidatorResult::Valid(_) => panic!("Expected an Err, but got Ok"),
                SpecValidatorResult::Invalid(_, errors) => {
                    assert!(errors.contains(&$expected_err.to_string()))
                }
            }
        };
    }