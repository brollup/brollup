#[cfg(test)]
mod cpe_tests {
    use bit_vec::BitVec;
    use brollup::{
        cpe::cpe::CompactPayloadEncoding,
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
    use std::collections::HashMap;

    #[tokio::test]
    async fn cpe_single_short_val_test() -> Result<(), String> {
        // Value 100 (u8) (0 < 100 < 256).
        let short_val = ShortVal::new(100);
        let encoded = short_val.encode_cpe().unwrap();
        let mut bit_stream = encoded.iter();

        let decoded = ShortVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.uncommon_tier(), ShortValTier::U8);
        assert_eq!(decoded.value(), 100);

        // Value 5_000 (u16) (256 < 5_000 < 65_536).
        let short_val = ShortVal::new(5000);
        let encoded = short_val.encode_cpe().unwrap();
        let mut bit_stream = encoded.iter();

        let decoded = ShortVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.uncommon_tier(), ShortValTier::U16);
        assert_eq!(decoded.value(), 5000);

        // Value 100_000 (u24) (65_536 < 100_000 < 16_777_216).
        let short_val = ShortVal::new(100_000);
        let encoded = short_val.encode_cpe().unwrap();
        let mut bit_stream = encoded.iter();

        let decoded = ShortVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.uncommon_tier(), ShortValTier::U24);
        assert_eq!(decoded.value(), 100_000);

        // Value 50_000_000 (u32) (16_777_216 < 50_000_000 < 4_294_967_296).
        let short_val = ShortVal::new(50_000_000);
        let encoded = short_val.encode_cpe().unwrap();
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
        let encoded = short_val.encode_cpe().unwrap();

        assert_eq!(encoded.len(), 10);

        full.extend(&encoded);

        // Insert 5_000 (u16) (256 < 5_000 < 65_536).
        let short_val = ShortVal::new(5000);
        let encoded = short_val.encode_cpe().unwrap();

        assert_eq!(encoded.len(), 18);

        full.extend(&encoded);

        // Insert 100_000 (u24) (65_536 < 100_000 < 16_777_216).
        let short_val = ShortVal::new(100_000);
        let encoded = short_val.encode_cpe().unwrap();

        assert_eq!(encoded.len(), 26);

        full.extend(&encoded);

        // Insert 50_000_000 (u32) (16_777_216 < 50_000_000 < 4_294_967_296).
        let short_val = ShortVal::new(50_000_000);
        let encoded = short_val.encode_cpe().unwrap();

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
        let encoded = long_val.encode_cpe().unwrap();

        let mut bit_stream = encoded.iter();

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U8);
        assert_eq!(decoded.value(), 100);

        // Value 5_000 (u16) (256 < 5_000 < 65_536).
        let long_val = LongVal::new(5_000);
        let encoded = long_val.encode_cpe().unwrap();
        let mut bit_stream = encoded.iter();

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U16);
        assert_eq!(decoded.value(), 5_000);

        // Value 100_000 (u24) (65_536 < 100_000 < 16_777_216).
        let long_val = LongVal::new(100_000);
        let encoded = long_val.encode_cpe().unwrap();
        let mut bit_stream = encoded.iter();

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U24);
        assert_eq!(decoded.value(), 100_000);

        // Value 50_000_000 (u32) (16_777_216 < 50_000_000 < 4_294_967_296).
        let long_val = LongVal::new(50_000_000);
        let encoded = long_val.encode_cpe().unwrap();
        let mut bit_stream = encoded.iter();

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U32);
        assert_eq!(decoded.value(), 50_000_000);

        // Value 100_000_000_000 (u40) (4_294_967_296 < 100_000_000_000 < 1_099_511_627_776).
        let long_val = LongVal::new(100_000_000_000);
        let encoded = long_val.encode_cpe().unwrap();
        let mut bit_stream = encoded.iter();

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U40);
        assert_eq!(decoded.value(), 100_000_000_000);

        //281,474,976,710,655
        // Value 100_000_000_000_000 (u48) (1_099_511_627_776 < 100_000_000_000_000 < 2_814_749_767_106_56).
        let long_val = LongVal::new(100_000_000_000_000);
        let encoded = long_val.encode_cpe().unwrap();
        let mut bit_stream = encoded.iter();

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U48);
        assert_eq!(decoded.value(), 100_000_000_000_000);

        // Value 100_000_000_000_000_000 (u56) (2_814_749_767_106_56 < 50_000_000_000_000_00 < 72_057_594_037_927_936).
        let long_val = LongVal::new(50_000_000_000_000_00);
        let encoded = long_val.encode_cpe().unwrap();
        let mut bit_stream = encoded.iter();

        let decoded = LongVal::decode_cpe(&mut bit_stream).unwrap();
        assert_eq!(decoded.tier(), LongValTier::U56);
        assert_eq!(decoded.value(), 50_000_000_000_000_00);

        // Value 100_000_000_000_000_000_000 (u64) (72_057_594_037_927_936 < 100_000_000_000_000_000 < 18,446,744,073,709,551,616).
        let long_val = LongVal::new(100_000_000_000_000_000);
        let encoded = long_val.encode_cpe().unwrap();
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
        let encoded = long_val.encode_cpe().unwrap();
        full.extend(&encoded);

        // Insert 5_000 (u16) (256 < 5_000 < 65_536).
        let long_val = LongVal::new(5_000);
        let encoded = long_val.encode_cpe().unwrap();
        full.extend(&encoded);

        // Insert 100_000 (u24) (65_536 < 100_000 < 16_777_216).
        let long_val = LongVal::new(100_000);
        let encoded = long_val.encode_cpe().unwrap();
        full.extend(&encoded);

        // Insert 50_000_000 (u32) (16_777_216 < 50_000_000 < 4_294_967_296).
        let long_val = LongVal::new(50_000_000);
        let encoded = long_val.encode_cpe().unwrap();
        full.extend(&encoded);

        // Insert 100_000_000_000 (u40) (4_294_967_296 < 100_000_000_000 < 1_099_511_627_776).
        let long_val = LongVal::new(100_000_000_000);
        let encoded = long_val.encode_cpe().unwrap();
        full.extend(&encoded);

        // Insert 100_000_000_000_000 (u48) (1_099_511_627_776 < 100_000_000_000_000 < 2_814_749_767_106_56).
        let long_val = LongVal::new(100_000_000_000_000);
        let encoded = long_val.encode_cpe().unwrap();
        full.extend(&encoded);

        // Insert 100_000_000_000_000_000 (u56) (2_814_749_767_106_56 < 50_000_000_000_000_00 < 72_057_594_037_927_936).
        let long_val = LongVal::new(50_000_000_000_000_00);
        let encoded = long_val.encode_cpe().unwrap();
        full.extend(&encoded);

        // Insert 100_000_000_000_000_000_000 (u64) (72_057_594_037_927_936 < 100_000_000_000_000_000 < 18,446,744,073,709,551,616).
        let long_val = LongVal::new(100_000_000_000_000_000);
        let encoded = long_val.encode_cpe().unwrap();
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
        let point_1 =
            Point::from_hex("021123864025e2c24bd82e6e19729eaa93cf02c57149bbfc84d239a0369f471316")
                .unwrap();

        let point_2 =
            Point::from_hex("02fbdf138aaa8e1e0446b3641cdc7f2d92b4442a80bbb191b8d96b653ccd270e44")
                .unwrap();

        let point_3 =
            Point::from_hex("0275c9daf737afcf7b430c752614c32bf3353b7581bdbd753f42ced96346016419")
                .unwrap();

        let account_1_to_encode = Account::new(point_1, None, None).unwrap();
        let account_2_to_encode = Account::new(point_2, None, None).unwrap();
        let account_3_to_encode = Account::new(point_3, None, None).unwrap();

        let account_1_encoded = account_1_to_encode.encode_cpe().unwrap();
        let account_2_encoded = account_2_to_encode.encode_cpe().unwrap();
        let account_3_encoded = account_3_to_encode.encode_cpe().unwrap();

        let mut account_1_bit_stream = account_1_encoded.iter();
        let mut account_2_bit_stream = account_2_encoded.iter();
        let mut account_3_bit_stream = account_3_encoded.iter();

        let account_1_decoded = Account::decode_cpe(&mut account_1_bit_stream, &account_registery)
            .await
            .unwrap();
        assert_eq!(account_1_to_encode, account_1_decoded);
        assert_eq!(account_1_to_encode.key(), account_1_decoded.key());
        assert_eq!(
            account_1_to_encode.registery_index(),
            account_1_decoded.registery_index()
        );

        let account_2_decoded = Account::decode_cpe(&mut account_2_bit_stream, &account_registery)
            .await
            .unwrap();
        assert_eq!(account_2_to_encode, account_2_decoded);
        assert_eq!(account_2_to_encode.key(), account_2_decoded.key());

        let account_3_decoded = Account::decode_cpe(&mut account_3_bit_stream, &account_registery)
            .await
            .unwrap();
        assert_eq!(account_3_to_encode, account_3_decoded);
        assert_eq!(account_3_to_encode.key(), account_3_decoded.key());

        // Registered account test

        let point_1 =
            Point::from_hex("02f2a9b1353ad81072c30a72c8efb125ca9fdc498a9b6cbe9d05e7937ac540c3ce")
                .unwrap();

        let point_2 =
            Point::from_hex("02746e29a85dbf1427a1330a743714362f35cc626c8cba1d03b37b989144b6b294")
                .unwrap();

        let point_3 =
            Point::from_hex("028e4a21dbe63689548103831790d6270c58f141648d8703a2e3c75a00cbc21e3b")
                .unwrap();

        // Insert the account into the registery.
        {
            let empty_caller_contracts = HashMap::<Account, u64>::new();

            // multi batc
            let mut _account_registery = account_registery.lock().await;
            let _ = _account_registery
                .batch_update(vec![point_1, point_2, point_3], empty_caller_contracts);
        }

        // Retrieve the account from the registery.
        let account_1 = {
            let _account_registery = account_registery.lock().await;
            _account_registery.account_by_key(point_1).unwrap()
        };

        let account_2 = {
            let _account_registery = account_registery.lock().await;
            _account_registery.account_by_key(point_2).unwrap()
        };

        let account_3 = {
            let _account_registery = account_registery.lock().await;
            _account_registery.account_by_key(point_3).unwrap()
        };

        // Check keys.
        assert_eq!(account_1.key(), point_1);
        assert_eq!(account_2.key(), point_2);
        assert_eq!(account_3.key(), point_3);

        // Check ranks.
        assert_eq!(account_1.rank(), Some(1));
        assert_eq!(account_2.rank(), Some(2));
        assert_eq!(account_3.rank(), Some(3));

        let account_1_encoded = account_1.encode_cpe().unwrap();
        let mut account_1_bit_stream = account_1_encoded.iter();

        let account_2_encoded = account_2.encode_cpe().unwrap();
        let mut account_2_bit_stream = account_2_encoded.iter();

        let account_3_encoded = account_3.encode_cpe().unwrap();
        let mut account_3_bit_stream = account_3_encoded.iter();

        let account_1_decoded = Account::decode_cpe(&mut account_1_bit_stream, &account_registery)
            .await
            .unwrap();

        let account_2_decoded = Account::decode_cpe(&mut account_2_bit_stream, &account_registery)
            .await
            .unwrap();

        let account_3_decoded = Account::decode_cpe(&mut account_3_bit_stream, &account_registery)
            .await
            .unwrap();

        assert_eq!(account_1, account_1_decoded);
        assert_eq!(account_1.key(), account_1_decoded.key());
        assert_eq!(
            account_1.registery_index(),
            account_1_decoded.registery_index()
        );

        assert_eq!(account_2, account_2_decoded);
        assert_eq!(account_2.key(), account_2_decoded.key());
        assert_eq!(
            account_2.registery_index(),
            account_2_decoded.registery_index()
        );

        assert_eq!(account_3, account_3_decoded);
        assert_eq!(account_3.key(), account_3_decoded.key());
        assert_eq!(
            account_3.registery_index(),
            account_3_decoded.registery_index()
        );

        // Contract test

        let contract_id_1 = [0xaau8; 32];
        let contract_id_2 = [0xbbu8; 32];
        let contract_id_3 = [0xccu8; 32];

        // Insert the contract into the registery.
        {
            let empty_called_contracts = HashMap::<Contract, u64>::new();

            let mut _contract_registery = contract_registery.lock().await;
            let _ = _contract_registery.batch_update(
                vec![contract_id_1, contract_id_2, contract_id_3],
                empty_called_contracts,
            );
        }

        // Get the contract #1.
        let contract_1 = {
            let _contract_registery = contract_registery.lock().await;
            _contract_registery
                .contract_by_contract_id(contract_id_1)
                .unwrap()
        };

        // Get the contract #2.
        let contract_2 = {
            let _contract_registery = contract_registery.lock().await;
            _contract_registery
                .contract_by_contract_id(contract_id_2)
                .unwrap()
        };

        // Get the contract #3.
        let contract_3 = {
            let _contract_registery = contract_registery.lock().await;
            _contract_registery
                .contract_by_contract_id(contract_id_3)
                .unwrap()
        };

        // Check contract IDs.
        assert_eq!(contract_1.contract_id(), contract_id_1);
        assert_eq!(contract_2.contract_id(), contract_id_2);
        assert_eq!(contract_3.contract_id(), contract_id_3);

        // Check registery index.
        assert_eq!(contract_1.registery_index(), 1);
        assert_eq!(contract_2.registery_index(), 2);
        assert_eq!(contract_3.registery_index(), 3);

        // Check ranks.
        assert_eq!(contract_1.rank(), Some(1));
        assert_eq!(contract_2.rank(), Some(2));
        assert_eq!(contract_3.rank(), Some(3));

        let contract_1_encoded = contract_1.encode_cpe().unwrap();
        let contract_2_encoded = contract_2.encode_cpe().unwrap();
        let contract_3_encoded = contract_3.encode_cpe().unwrap();

        let mut contract_1_bit_stream = contract_1_encoded.iter();
        let mut contract_2_bit_stream = contract_2_encoded.iter();
        let mut contract_3_bit_stream = contract_3_encoded.iter();

        let contract_1_decoded =
            Contract::decode_cpe(&mut contract_1_bit_stream, &contract_registery)
                .await
                .unwrap();

        let contract_2_decoded =
            Contract::decode_cpe(&mut contract_2_bit_stream, &contract_registery)
                .await
                .unwrap();

        let contract_3_decoded =
            Contract::decode_cpe(&mut contract_3_bit_stream, &contract_registery)
                .await
                .unwrap();

        assert_eq!(contract_1, contract_1_decoded);
        assert_eq!(contract_1.contract_id(), contract_1_decoded.contract_id());
        assert_eq!(
            contract_1.registery_index(),
            contract_1_decoded.registery_index()
        );

        assert_eq!(contract_2, contract_2_decoded);
        assert_eq!(contract_2.contract_id(), contract_2_decoded.contract_id());
        assert_eq!(
            contract_2.registery_index(),
            contract_2_decoded.registery_index()
        );

        assert_eq!(contract_3, contract_3_decoded);
        assert_eq!(contract_3.contract_id(), contract_3_decoded.contract_id());
        assert_eq!(
            contract_3.registery_index(),
            contract_3_decoded.registery_index()
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_common_short_val() -> Result<(), String> {
        // Test 100
        let value = 100;
        let encoded = CommonVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 500
        let value = 500;
        let encoded = CommonVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 1000
        let value = 1000;
        let encoded = CommonVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 5000
        let value = 5000;
        let encoded = CommonVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 25000
        let value = 25000;
        let encoded = CommonVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 50000
        let value = 50000;
        let encoded = CommonVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 75000
        let value = 75000;
        let encoded = CommonVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 100000
        let value = 100000;
        let encoded = CommonVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 1000000
        let value = 1000000;
        let encoded = CommonVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 25000000
        let value = 25000000;
        let encoded = CommonVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(value, decoded.value());

        // Test 50000000
        let value = 50000000;
        let encoded = CommonVal::new(value).unwrap().encode_cpe().unwrap();
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

        assert_eq!(
            common_short_val.value(),
            maybe_common_short_val.inner_val().value()
        );
        assert_eq!(
            uncommon_short_val.value(),
            maybe_common_uncommon_short_val.inner_val().value()
        );

        let maybe_common_short_val_encoded = maybe_common_short_val.encode_cpe().unwrap();
        let maybe_common_uncommon_short_val_encoded =
            maybe_common_uncommon_short_val.encode_cpe().unwrap();

        let mut maybe_common_short_val_bit_stream = maybe_common_short_val_encoded.iter();
        let mut maybe_common_uncommon_short_val_bit_stream =
            maybe_common_uncommon_short_val_encoded.iter();

        let common_short_val_decoded: ShortVal =
            MaybeCommon::<ShortVal>::decode_cpe(&mut maybe_common_short_val_bit_stream)
                .unwrap()
                .inner_val();

        let uncommon_short_val_decoded: ShortVal =
            MaybeCommon::<ShortVal>::decode_cpe(&mut maybe_common_uncommon_short_val_bit_stream)
                .unwrap()
                .inner_val();

        assert_eq!(common_short_val.value(), common_short_val_decoded.value());
        assert_eq!(
            uncommon_short_val.value(),
            uncommon_short_val_decoded.value()
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

        assert_eq!(
            common_long_val.value(),
            maybe_common_long_val.inner_val().value()
        );

        assert_eq!(
            uncommon_long_val.value(),
            maybe_common_uncommon_long_val.inner_val().value()
        );

        let maybe_common_common_long_val_encoded = maybe_common_long_val.encode_cpe().unwrap();
        let maybe_common_uncommon_long_val_encoded =
            maybe_common_uncommon_long_val.encode_cpe().unwrap();

        let mut maybe_common_common_long_val_bit_stream =
            maybe_common_common_long_val_encoded.iter();
        let mut maybe_common_uncommon_long_val_bit_stream =
            maybe_common_uncommon_long_val_encoded.iter();

        let common_long_val_decoded: LongVal =
            MaybeCommon::<LongVal>::decode_cpe(&mut maybe_common_common_long_val_bit_stream)
                .unwrap()
                .inner_val();

        let uncommon_long_val_decoded: LongVal =
            MaybeCommon::<LongVal>::decode_cpe(&mut maybe_common_uncommon_long_val_bit_stream)
                .unwrap()
                .inner_val();

        assert_eq!(common_long_val.value(), common_long_val_decoded.value());
        assert_eq!(uncommon_long_val.value(), uncommon_long_val_decoded.value());

        Ok(())
    }

    #[tokio::test]
    async fn varbytes_0_to_4096_12_bit_test() -> Result<(), String> {
        // 0 to 4095
        for i in 0..=4095 {
            let value: u16 = i;

            let value_bytes = value.to_le_bytes();

            let mut value_bits = BitVec::new();
            for i in 0..12 {
                let byte_idx = i / 8;
                let bit_idx = i % 8;
                value_bits.push((value_bytes[byte_idx] >> bit_idx) & 1 == 1);
            }

            let mut decoded_value = 0u16;
            for i in 0..12 {
                let bit = value_bits[i];
                if bit {
                    decoded_value |= 1 << i;
                }
            }

            assert_eq!(value, decoded_value);
        }

        Ok(())
    }

    #[tokio::test]
    async fn rank_6_bit_test() -> Result<(), String> {
        // 1 to 64
        for i in 0..=63 {
            // encode u8
            let rank_index: u8 = i;

            let rank_bytes = rank_index.to_le_bytes();

            // Convert the rank (u8) into a BitVec.
            let mut rank_bits = BitVec::new();
            for i in 0..6 {
                rank_bits.push((rank_bytes[0] >> i) & 1 == 1);
            }

            // Decode the rank directly from the BitVec.
            let mut decoded_rank = 0u8;
            for i in 0..6 {
                let bit = rank_bits[i];
                if bit {
                    decoded_rank |= 1 << i;
                }
            }

            assert_eq!(rank_index, decoded_rank);
        }

        Ok(())
    }
}
