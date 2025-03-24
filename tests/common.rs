#[cfg(test)]
mod common_value_test {
    use brollup::valtype::common::CommonInt;

    #[tokio::test]
    async fn common_value_test() -> Result<(), String> {
        // Test 1
        let value = 1;
        let encoded = CommonInt::encode(value).unwrap();
        let decoded = CommonInt::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded);

        // Test 2
        let value = 2;
        let encoded = CommonInt::encode(value).unwrap();
        let decoded = CommonInt::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded);

        // Test 3
        let value = 3;
        let encoded = CommonInt::encode(value).unwrap();
        let decoded = CommonInt::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded);

        // Test 5
        let value = 5;
        let encoded = CommonInt::encode(value).unwrap();
        let decoded = CommonInt::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded);

        // Test 100
        let value = 100;
        let encoded = CommonInt::encode(value).unwrap();
        let decoded = CommonInt::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded);

        // Test 500
        let value = 500;
        let encoded = CommonInt::encode(value).unwrap();
        let decoded = CommonInt::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded);

        // Test 1000
        let value = 1000;
        let encoded = CommonInt::encode(value).unwrap();
        let decoded = CommonInt::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded);

        // Test 5000
        let value = 5000;
        let encoded = CommonInt::encode(value).unwrap();
        let decoded = CommonInt::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded);

        // Test 25000
        let value = 25000;
        let encoded = CommonInt::encode(value).unwrap();
        let decoded = CommonInt::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded);

        // Test 50000
        let value = 50000;
        let encoded = CommonInt::encode(value).unwrap();
        let decoded = CommonInt::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded);

        // Test 75000
        let value = 75000;
        let encoded = CommonInt::encode(value).unwrap();
        let decoded = CommonInt::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded);

        // Test 100000
        let value = 100000;
        let encoded = CommonInt::encode(value).unwrap();
        let decoded = CommonInt::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded);

        // Test 1000000
        let value = 1000000;
        let encoded = CommonInt::encode(value).unwrap();
        let decoded = CommonInt::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded);

        // Test 25000000
        let value = 25000000;
        let encoded = CommonInt::encode(value).unwrap();
        let decoded = CommonInt::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded);

        // Test 50000000
        let value = 50000000;
        let encoded = CommonInt::encode(value).unwrap();
        let decoded = CommonInt::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded);

        // Test 1000000000
        let value = 1000000000;
        let encoded = CommonInt::encode(value).unwrap();
        let decoded = CommonInt::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded);

        Ok(())
    }
}
