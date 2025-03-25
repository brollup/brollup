#[cfg(test)]
mod cpe_tests {
    use bit_vec::BitVec;
    use brollup::{
        cpe::CompactPayloadEncoding,
        entity::{account::Account, contract::Contract},
        registery::registery::Registery,
        valtype::{
            long_val::{LongVal, LongValTier},
            maybe_common::{common_val::CommonVal, maybe_common::MaybeCommon},
            short_val::{ShortVal, ShortValTier},
        },
        Network,
    };
    use secp::Point;

    #[tokio::test]
    async fn cpe_single_short_val_test() -> Result<(), String> {
        // Value 100 (u8) (0 < 100 < 256).
        let short_val = ShortVal::new(100);
        let encoded = short_val.encode_cpe();
        let mut bit_stream = encoded.iter();

        let decoded = ShortVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.uncommon_tier(), ShortValTier::U8);
        assert_eq!(decoded.value(), 100);

        // Value 5_000 (u16) (256 < 5_000 < 65_536).
        let short_val = ShortVal::new(5000);
        let encoded = short_val.encode_cpe();
        let mut bit_stream = encoded.iter();

        let decoded = ShortVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.uncommon_tier(), ShortValTier::U16);
        assert_eq!(decoded.value(), 5000);

        // Value 100_000 (u24) (65_536 < 100_000 < 16_777_216).
        let short_val = ShortVal::new(100_000);
        let encoded = short_val.encode_cpe();
        let mut bit_stream = encoded.iter();

        let decoded = ShortVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.uncommon_tier(), ShortValTier::U24);
        assert_eq!(decoded.value(), 100_000);

        // Value 50_000_000 (u32) (16_777_216 < 50_000_000 < 4_294_967_296).
        let short_val = ShortVal::new(50_000_000);
        let encoded = short_val.encode_cpe();
        let mut bit_stream = encoded.iter();

        let decoded = ShortVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.uncommon_tier(), ShortValTier::U32);
        assert_eq!(decoded.value(), 50_000_000);

        Ok(())
    }

    #[tokio::test]
    async fn cpe_multi_short_val_test() -> Result<(), String> {
        let mut full = BitVec::new();

        // Insert 100 (u8) (0 < 100 < 256).
        let short_val = ShortVal::new(100);
        let encoded = short_val.encode_cpe();

        assert_eq!(encoded.len(), 10);

        full.extend(&encoded);

        // Insert 5_000 (u16) (256 < 5_000 < 65_536).
        let short_val = ShortVal::new(5000);
        let encoded = short_val.encode_cpe();

        assert_eq!(encoded.len(), 18);

        full.extend(&encoded);

        // Insert 100_000 (u24) (65_536 < 100_000 < 16_777_216).
        let short_val = ShortVal::new(100_000);
        let encoded = short_val.encode_cpe();

        assert_eq!(encoded.len(), 26);

        full.extend(&encoded);

        // Insert 50_000_000 (u32) (16_777_216 < 50_000_000 < 4_294_967_296).
        let short_val = ShortVal::new(50_000_000);
        let encoded = short_val.encode_cpe();

        assert_eq!(encoded.len(), 34);

        full.extend(&encoded);

        // Insert 5 garbage bits.
        full.push(true);
        full.push(false);
        full.push(false);
        full.push(true);
        full.push(true);

        let mut bit_stream = full.iter();

        let decoded = ShortVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.uncommon_tier(), ShortValTier::U8);
        assert_eq!(decoded.value(), 100);

        let decoded = ShortVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.uncommon_tier(), ShortValTier::U16);
        assert_eq!(decoded.value(), 5_000);

        let decoded = ShortVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.uncommon_tier(), ShortValTier::U24);
        assert_eq!(decoded.value(), 100_000);

        let decoded = ShortVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.uncommon_tier(), ShortValTier::U32);
        assert_eq!(decoded.value(), 50_000_000);

        // 5 garbage bits left.
        assert_eq!(bit_stream.len(), 5);

        Ok(())
    }

    #[tokio::test]
    async fn cpe_single_long_val_test() -> Result<(), String> {
        // Value 100 (u8) (0 < 100 < 256).
        let long_val = LongVal::new(100);
        let encoded = long_val.encode_cpe();

        let mut bit_stream = encoded.iter();

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U8);
        assert_eq!(decoded.value(), 100);

        // Value 5_000 (u16) (256 < 5_000 < 65_536).
        let long_val = LongVal::new(5_000);
        let encoded = long_val.encode_cpe();
        let mut bit_stream = encoded.iter();

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U16);
        assert_eq!(decoded.value(), 5_000);

        // Value 100_000 (u24) (65_536 < 100_000 < 16_777_216).
        let long_val = LongVal::new(100_000);
        let encoded = long_val.encode_cpe();
        let mut bit_stream = encoded.iter();

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U24);
        assert_eq!(decoded.value(), 100_000);

        // Value 50_000_000 (u32) (16_777_216 < 50_000_000 < 4_294_967_296).
        let long_val = LongVal::new(50_000_000);
        let encoded = long_val.encode_cpe();
        let mut bit_stream = encoded.iter();

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U32);
        assert_eq!(decoded.value(), 50_000_000);

        // Value 100_000_000_000 (u40) (4_294_967_296 < 100_000_000_000 < 1_099_511_627_776).
        let long_val = LongVal::new(100_000_000_000);
        let encoded = long_val.encode_cpe();
        let mut bit_stream = encoded.iter();

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U40);
        assert_eq!(decoded.value(), 100_000_000_000);

        //281,474,976,710,655
        // Value 100_000_000_000_000 (u48) (1_099_511_627_776 < 100_000_000_000_000 < 2_814_749_767_106_56).
        let long_val = LongVal::new(100_000_000_000_000);
        let encoded = long_val.encode_cpe();
        let mut bit_stream = encoded.iter();

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U48);
        assert_eq!(decoded.value(), 100_000_000_000_000);

        // Value 100_000_000_000_000_000 (u56) (2_814_749_767_106_56 < 50_000_000_000_000_00 < 72_057_594_037_927_936).
        let long_val = LongVal::new(50_000_000_000_000_00);
        let encoded = long_val.encode_cpe();
        let mut bit_stream = encoded.iter();

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U56);
        assert_eq!(decoded.value(), 50_000_000_000_000_00);

        // Value 100_000_000_000_000_000_000 (u64) (72_057_594_037_927_936 < 100_000_000_000_000_000 < 18,446,744,073,709,551,616).
        let long_val = LongVal::new(100_000_000_000_000_000);
        let encoded = long_val.encode_cpe();
        let mut bit_stream = encoded.iter();

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U64);
        assert_eq!(decoded.value(), 100_000_000_000_000_000);

        Ok(())
    }

    #[tokio::test]
    async fn cpe_multi_long_val_test() -> Result<(), String> {
        let mut full = BitVec::new();

        // Insert 100 (u8) (0 < 100 < 256).
        let long_val = LongVal::new(100);
        let encoded = long_val.encode_cpe();
        full.extend(&encoded);

        // Insert 5_000 (u16) (256 < 5_000 < 65_536).
        let long_val = LongVal::new(5_000);
        let encoded = long_val.encode_cpe();
        full.extend(&encoded);

        // Insert 100_000 (u24) (65_536 < 100_000 < 16_777_216).
        let long_val = LongVal::new(100_000);
        let encoded = long_val.encode_cpe();
        full.extend(&encoded);

        // Insert 50_000_000 (u32) (16_777_216 < 50_000_000 < 4_294_967_296).
        let long_val = LongVal::new(50_000_000);
        let encoded = long_val.encode_cpe();
        full.extend(&encoded);

        // Insert 100_000_000_000 (u40) (4_294_967_296 < 100_000_000_000 < 1_099_511_627_776).
        let long_val = LongVal::new(100_000_000_000);
        let encoded = long_val.encode_cpe();
        full.extend(&encoded);

        // Insert 100_000_000_000_000 (u48) (1_099_511_627_776 < 100_000_000_000_000 < 2_814_749_767_106_56).
        let long_val = LongVal::new(100_000_000_000_000);
        let encoded = long_val.encode_cpe();
        full.extend(&encoded);

        // Insert 100_000_000_000_000_000 (u56) (2_814_749_767_106_56 < 50_000_000_000_000_00 < 72_057_594_037_927_936).
        let long_val = LongVal::new(50_000_000_000_000_00);
        let encoded = long_val.encode_cpe();
        full.extend(&encoded);

        // Insert 100_000_000_000_000_000_000 (u64) (72_057_594_037_927_936 < 100_000_000_000_000_000 < 18,446,744,073,709,551,616).
        let long_val = LongVal::new(100_000_000_000_000_000);
        let encoded = long_val.encode_cpe();
        full.extend(&encoded);

        // Insert 5 garbage bits.
        full.push(true);
        full.push(false);
        full.push(false);
        full.push(true);
        full.push(true);

        let mut bit_stream = full.iter();

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U8);
        assert_eq!(decoded.value(), 100);

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U16);
        assert_eq!(decoded.value(), 5_000);

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U24);
        assert_eq!(decoded.value(), 100_000);

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U32);
        assert_eq!(decoded.value(), 50_000_000);

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U40);
        assert_eq!(decoded.value(), 100_000_000_000);

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U48);
        assert_eq!(decoded.value(), 100_000_000_000_000);

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U56);
        assert_eq!(decoded.value(), 50_000_000_000_000_00);

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U64);
        assert_eq!(decoded.value(), 100_000_000_000_000_000);

        assert_eq!(bit_stream.len(), 5);

        Ok(())
    }

    #[tokio::test]
    async fn account_and_contract_test() -> Result<(), String> {
        // Get the registery.
        let registery = Registery::new(Network::Signet).unwrap();

        // Get the account registery.
        let account_registery = {
            let mut _registery = registery.lock().await;
            _registery.account_registery()
        };

        // Get the contract registery.
        let contract_registery = {
            let mut _registery = registery.lock().await;
            _registery.contract_registery()
        };

        // Unregistered account test
        let point =
            Point::from_hex("021123864025e2c24bd82e6e19729eaa93cf02c57149bbfc84d239a0369f471316")
                .unwrap();

        let account_to_encode = Account::new(point, None).unwrap();
        let encoded = account_to_encode.encode_cpe();

        let mut bit_stream = encoded.iter();

        let account_decoded = Account::decode_cpe(&mut bit_stream, &account_registery)
            .await
            .unwrap();
        assert_eq!(account_to_encode, account_decoded);
        assert_eq!(account_to_encode.key(), account_decoded.key());
        assert_eq!(
            account_to_encode.registery_index(),
            account_decoded.registery_index()
        );

        // Registered account test

        let point =
            Point::from_hex("022d69e8ef6a06ed3efcf433ee24dbe55e8e6dec5804957326b07c3902960af1f9")
                .unwrap();

        // Insert the account into the registery.
        {
            let mut _account_registery = account_registery.lock().await;
            _account_registery.insert(point);
        }

        let account = {
            let _account_registery = account_registery.lock().await;
            _account_registery.account_by_key_registered(point).unwrap()
        };

        let encoded = account.encode_cpe();
        let mut bit_stream = encoded.iter();

        let account_decoded = Account::decode_cpe(&mut bit_stream, &account_registery)
            .await
            .unwrap();
        assert_eq!(account, account_decoded);
        assert_eq!(account.key(), account_decoded.key());
        assert_eq!(account.registery_index(), account_decoded.registery_index());

        // Contract test

        let contract_id = [0xffu8; 32];

        // Insert the contract into the registery.
        {
            let mut _contract_registery = contract_registery.lock().await;
            _contract_registery.insert(contract_id);
        }

        // Get the contract.
        let contract = {
            let _contract_registery = contract_registery.lock().await;
            _contract_registery.contract_by_id(contract_id).unwrap()
        };

        let encoded = contract.encode_cpe();

        let mut bit_stream = encoded.iter();

        let decoded_contract = Contract::decode_cpe(&mut bit_stream, &contract_registery)
            .await
            .unwrap();
        assert_eq!(contract, decoded_contract);
        assert_eq!(contract.contract_id(), decoded_contract.contract_id());
        assert_eq!(
            contract.registery_index(),
            decoded_contract.registery_index()
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_common_short_val() -> Result<(), String> {
        // Test 100
        let value = 100;
        let encoded = CommonVal::new(value).unwrap().encode_cpe();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 500
        let value = 500;
        let encoded = CommonVal::new(value).unwrap().encode_cpe();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 1000
        let value = 1000;
        let encoded = CommonVal::new(value).unwrap().encode_cpe();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 5000
        let value = 5000;
        let encoded = CommonVal::new(value).unwrap().encode_cpe();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 25000
        let value = 25000;
        let encoded = CommonVal::new(value).unwrap().encode_cpe();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 50000
        let value = 50000;
        let encoded = CommonVal::new(value).unwrap().encode_cpe();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 75000
        let value = 75000;
        let encoded = CommonVal::new(value).unwrap().encode_cpe();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 100000
        let value = 100000;
        let encoded = CommonVal::new(value).unwrap().encode_cpe();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 1000000
        let value = 1000000;
        let encoded = CommonVal::new(value).unwrap().encode_cpe();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 25000000
        let value = 25000000;
        let encoded = CommonVal::new(value).unwrap().encode_cpe();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 50000000
        let value = 50000000;
        let encoded = CommonVal::new(value).unwrap().encode_cpe();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        Ok(())
    }

    #[tokio::test]
    async fn test_maybe_common_short_val() -> Result<(), String> {
        let common_short_val = ShortVal::new(1000);
        let uncommon_short_val = ShortVal::new(34567);

        let maybe_common_short_val: MaybeCommon<ShortVal> = MaybeCommon::new(common_short_val);
        let maybe_common_uncommon_short_val: MaybeCommon<ShortVal> =
            MaybeCommon::new(uncommon_short_val);

        assert_eq!(maybe_common_short_val.is_common(), true);
        assert_eq!(maybe_common_uncommon_short_val.is_common(), false);

        assert_eq!(common_short_val.value(), maybe_common_short_val.value());
        assert_eq!(
            uncommon_short_val.value(),
            maybe_common_uncommon_short_val.value()
        );

        let maybe_common_short_val_encoded = maybe_common_short_val.encode_cpe();
        let maybe_common_uncommon_short_val_encoded = maybe_common_uncommon_short_val.encode_cpe();

        let mut maybe_common_short_val_bit_stream = maybe_common_short_val_encoded.iter();
        let mut maybe_common_uncommon_short_val_bit_stream =
            maybe_common_uncommon_short_val_encoded.iter();

        let maybe_common_short_val_decoded =
            MaybeCommon::<ShortVal>::decode_cpe(&mut maybe_common_short_val_bit_stream).unwrap();

        let maybe_common_uncommon_short_val_decoded =
            MaybeCommon::<ShortVal>::decode_cpe(&mut maybe_common_uncommon_short_val_bit_stream)
                .unwrap();

        assert_eq!(
            common_short_val.value(),
            maybe_common_short_val_decoded.value()
        );
        assert_eq!(
            uncommon_short_val.value(),
            maybe_common_uncommon_short_val_decoded.value()
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_maybe_common_long_val() -> Result<(), String> {
        let common_long_val = LongVal::new(1000000);
        let uncommon_long_val = LongVal::new(9274537803189203421);

        let maybe_common_long_val: MaybeCommon<LongVal> = MaybeCommon::new(common_long_val);
        let maybe_common_uncommon_long_val: MaybeCommon<LongVal> =
            MaybeCommon::new(uncommon_long_val);

        assert_eq!(maybe_common_long_val.is_common(), true);
        assert_eq!(maybe_common_uncommon_long_val.is_common(), false);

        assert_eq!(common_long_val.value(), maybe_common_long_val.value());
        assert_eq!(
            uncommon_long_val.value(),
            maybe_common_uncommon_long_val.value()
        );

        let maybe_common_common_long_val_encoded = maybe_common_long_val.encode_cpe();
        let maybe_common_uncommon_long_val_encoded = maybe_common_uncommon_long_val.encode_cpe();

        let mut maybe_common_common_long_val_bit_stream =
            maybe_common_common_long_val_encoded.iter();
        let mut maybe_common_uncommon_long_val_bit_stream =
            maybe_common_uncommon_long_val_encoded.iter();

        let maybe_common_common_long_val_decoded =
            MaybeCommon::<LongVal>::decode_cpe(&mut maybe_common_common_long_val_bit_stream)
                .unwrap();

        let maybe_common_uncommon_long_val_decoded =
            MaybeCommon::<LongVal>::decode_cpe(&mut maybe_common_uncommon_long_val_bit_stream)
                .unwrap();

        assert_eq!(
            common_long_val.value(),
            maybe_common_common_long_val_decoded.value()
        );
        assert_eq!(
            uncommon_long_val.value(),
            maybe_common_uncommon_long_val_decoded.value()
        );

        Ok(())
    }
}
