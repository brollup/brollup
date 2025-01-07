#[cfg(test)]
mod authenticable_tests {
    use brollup::{
        hash::{Hash, HashTag},
        into::IntoPoint,
        noist::dkg::{session::DKGSession, sharemap::DKGShareMap},
        schnorr::{Authenticable, Sighash},
        secp_point::SecpPoint,
    };
    use secp::{MaybePoint, Point, Scalar};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
    pub struct DemoStruct {
        pub field1: String,
        pub field2: u32,
    }

    impl Sighash for DemoStruct {
        fn sighash(&self) -> [u8; 32] {
            let mut preimage: Vec<u8> = Vec::<u8>::new();

            preimage.extend(self.field1.as_bytes());
            preimage.extend(self.field2.to_be().to_be_bytes());

            preimage.hash(Some(HashTag::SighashAuthenticable))
        }
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

        let authenticable_bytes = authenticable.serialize();

        println!("authenticable_bytes: {}", hex::encode(&authenticable_bytes));

        let _deserialized: Authenticable<DemoStruct> =
            bincode::deserialize(&authenticable_bytes).unwrap();

        //println!("{}", des.object().field1);

        let my_struct = authenticable.object();

        assert_eq!(my_struct.field1, "Brollup");
        assert_eq!(my_struct.field2, 21);

        Ok(())
    }

    #[test]
    fn authenticable_test_2() -> Result<(), String> {
        #[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
        pub struct DemoStruct {
            pub field1: Scalar,
            pub field2: Point,
        }

        let point = SecpPoint::new(Scalar::one().base_point_mul());

        let bytes = bincode::serialize(&point).unwrap();
        let des: SecpPoint = bincode::deserialize(&bytes).unwrap();

        //let bytes = bincode::serialize(&my_struct).unwrap();
        //let des: DemoStruct = bincode::deserialize(&bytes).unwrap();

        Ok(())
    }
}
