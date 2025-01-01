#[cfg(test)]
mod schnorr_tests {
    use brollup::schnorr::Authenticable;
    use hex;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
    pub struct DemoStruct {
        pub field1: String,
        pub field2: u32,
    }

    #[test]
    fn authenticable_test() -> Result<(), String> {
        let my_struct = DemoStruct {
            field1: "Brollup".to_string(),
            field2: 21,
        };

        let secret_key: [u8; 32] =
            hex::decode("7c341c752c061be9c820f556cbf3b1b2e4e01eb757df126f3750a5125f18a786")
                .unwrap()
                .try_into()
                .unwrap();

        let authenticable = Authenticable::new(my_struct, secret_key).unwrap();

        assert_eq!(
            hex::encode(authenticable.key()),
            "de8f0861ec3b9488d5a75042d246a011e1e1736d791d9d664b73a47375ab122f"
        );

        let authenticable_bytes = bincode::serialize(&authenticable).unwrap();

        let authenticable: Authenticable<DemoStruct> =
            bincode::deserialize(&authenticable_bytes).unwrap();

        if !authenticable.authenticate() {
            return Err("Authentication failed.".into());
        }

        let my_struct = authenticable.object();

        assert_eq!(my_struct.field1, "Brollup");
        assert_eq!(my_struct.field2, 21);

        Ok(())
    }
}