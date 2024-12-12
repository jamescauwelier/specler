pub(crate) mod test {
    #[macro_export]
    macro_rules! verify_invalid_input {
        ($name: ident, $input:expr, $factory:expr) => {
            #[test]
            fn $name() {
                match $factory($input) {
                    Ok(v) => panic!("{:?}'' is not a valid string, but passed spec anyway", v),
                    Err(_) => {}
                }
            }
        };
    }

    #[macro_export]
    macro_rules! verify_valid_input {
        ($name: ident, $input:expr, $factory:expr) => {
            #[test]
            fn $name() {
                match $factory($input) {
                    Ok(_) => {},
                    Err(_) => panic!("{:?}'' should pass the spec, but did not", $input),
                }
            }
        };
    }
}

