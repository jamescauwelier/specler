/// Asserts that a `SpecValidationResult` is valid
// #[macro_export]
// macro_rules! assert_spec_valid {
//         ($result:expr) => {
//             match $result {
//                 SpecValidationResult::Valid(_) => (),
//                 SpecValidationResult::Invalid(v, _) => {
//                     panic!("Expected '{}' to pass the specification, but got errors", v)
//                 }
//             }
//         };
//     }

/// Asserts that a `SpecValidationResult` is invalid
#[macro_export]
macro_rules! assert_spec_invalid {
        ($result:expr) => {
            match &$result {
                SpecValidationResult::Invalid(_, _) => (),
                SpecValidationResult::Valid(_) => {
                    panic!("Expected an invalid result, but got valid")
                }
            }
        };
    }

/// Asserts that a `SpecValidationResult` is invalid and contains the expected error
#[macro_export]
macro_rules! assert_spec_error_msg {
        ($result:expr, $expected_err:expr) => {
            match &$result {
                Ok(_) => panic!("Expected an Err, but got Ok"),
                Err(se) => {
                    assert!(se.errors().contains(&$expected_err.to_string()))
                }
            }
        };
    }