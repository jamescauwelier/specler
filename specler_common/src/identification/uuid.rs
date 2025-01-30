use specler::prelude::*;

pub struct UuidV4Spec;
impl SpecProvider<String> for UuidV4Spec {
    fn spec() -> Spec<String> {
        Require::<String>::to()
            .be(a_uuid_v4())
    }
    }

pub struct UuidV7Spec;
impl SpecProvider<String> for UuidV7Spec {
    fn spec() -> Spec<String> {
        Require::<String>::to()
            .be(a_uuid_v7())
    }
}

#[cfg(test)]
mod tests {

    mod v4 {
        use specler_macros::*;
        use super::super::*;
        
        #[derive(Debug)]
        #[specled(UuidV4Spec)]
        struct Id(String);


        #[test]
        fn valid_uuid_v4() {
            let valid_v4_string = "2c143023-bb8a-4e18-bcd3-63ce7d988fb1".to_string();
            let id = Id::create(valid_v4_string);
            assert!(id.is_ok());
        }

        #[test]
        fn invalid_uuid_v4() {
            let id = Id::create("abc".to_string());

            match id {
                Err(e) => {
                    assert_eq!(e.to_string(), "'Id' spec failed: invalid UUID v4 string");
                },
                Ok(_) => panic!("Expected this string to be an invalid uuid v4")
            }
        }
    }

    mod v7 {
        use specler_macros::*;
        use super::super::*;

        #[derive(Debug)]
        #[specled(UuidV7Spec)]
        struct Id(String);

        #[test]
        fn valid_uuid_v7() {
            let valid_v7_string = "0194b435-18a3-775d-9de8-8f49be8ee75c".to_string();
            let id = Id::create(valid_v7_string);
            assert!(id.is_ok());
        }

        #[test]
        fn invalid_uuid_v7() {
            let id = Id::create("abc".to_string());
            match id {
                Err(e) => {
                    assert_eq!(e.to_string(), "'Id' spec failed: invalid UUID v7 string");
                },
                Ok(_) => panic!("Expected this string to be an invalid uuid v7")
            }
        }
    }
}