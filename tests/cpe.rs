#[cfg(test)]
mod cpe_tests {
    use bit_vec::BitVec;
    use cube::{
        constructive::{
            entity::{account::account::Account, contract::contract::Contract},
            valtype::{
                maybe_common::{
                    common::{
                        common_long::common_long::CommonLongVal,
                        common_short::common_short::CommonShortVal,
                    },
                    maybe_common::maybe_common::MaybeCommon,
                },
                val::{
                    atomic_val::atomic_val::AtomicVal,
                    long_val::long_val::{LongVal, LongValTier},
                    short_val::short_val::{ShortVal, ShortValTier},
                },
            },
        },
        inscriptive::registery::registery::Registery,
        operative::Chain,
    };
    use secp::Point;
    use std::collections::HashMap;

    #[tokio::test]
    async fn cpe_atomic_val_test() -> Result<(), String> {
        // Test 0 with upper bound 1.
        let atomic_val = AtomicVal::new(0, 1);
        let encoded = atomic_val.encode_cpe().unwrap();
        // Expected bitsize length is 1.
        assert_eq!(encoded.len(), 1);
        let mut bit_stream = encoded.iter();
        let decoded = AtomicVal::decode_cpe(&mut bit_stream, 1).unwrap();
        assert_eq!(decoded.value(), 0);
        assert_eq!(decoded.upper_bound(), 1);

        // Test 0 with upper bound 2.
        let atomic_val = AtomicVal::new(0, 2);
        let encoded = atomic_val.encode_cpe().unwrap();
        // Expected bitsize length is 2.
        assert_eq!(encoded.len(), 2);
        let mut bit_stream = encoded.iter();
        let decoded = AtomicVal::decode_cpe(&mut bit_stream, 2).unwrap();
        assert_eq!(decoded.value(), 0);

        // Test 1 with upper bound 4.
        let atomic_val = AtomicVal::new(1, 4);
        let encoded = atomic_val.encode_cpe().unwrap();
        // Expected bitsize length is 3.
        assert_eq!(encoded.len(), 3);
        let mut bit_stream = encoded.iter();
        let decoded = AtomicVal::decode_cpe(&mut bit_stream, 4).unwrap();
        assert_eq!(decoded.value(), 1);

        // Test 3 with upper bound 6.
        let atomic_val = AtomicVal::new(3, 6);
        let encoded = atomic_val.encode_cpe().unwrap();
        // Expected bitsize length is still 3.
        assert_eq!(encoded.len(), 3);
        let mut bit_stream = encoded.iter();
        let decoded = AtomicVal::decode_cpe(&mut bit_stream, 6).unwrap();
        assert_eq!(decoded.value(), 3);

        // Test 11 with upper bound 15.
        let atomic_val = AtomicVal::new(11, 15);
        let encoded = atomic_val.encode_cpe().unwrap();
        // Expected bitsize length is 4.
        assert_eq!(encoded.len(), 4);
        let mut bit_stream = encoded.iter();
        let decoded = AtomicVal::decode_cpe(&mut bit_stream, 15).unwrap();
        assert_eq!(decoded.value(), 11);

        // Test 5 with upper bound 40.
        let atomic_val = AtomicVal::new(5, 40);
        let encoded = atomic_val.encode_cpe().unwrap();
        // Expected bitsize length is 6.
        assert_eq!(encoded.len(), 6);
        let mut bit_stream = encoded.iter();
        let decoded = AtomicVal::decode_cpe(&mut bit_stream, 40).unwrap();
        assert_eq!(decoded.value(), 5);

        // Test 55 with upper bound 80.
        let atomic_val = AtomicVal::new(55, 80);
        let encoded = atomic_val.encode_cpe().unwrap();
        // Expected bitsize length is 7.
        assert_eq!(encoded.len(), 7);
        let mut bit_stream = encoded.iter();
        let decoded = AtomicVal::decode_cpe(&mut bit_stream, 80).unwrap();
        assert_eq!(decoded.value(), 55);

        // Test 30 with upper bound 127.
        let atomic_val = AtomicVal::new(30, 127);
        let encoded = atomic_val.encode_cpe().unwrap();
        // Expected bitsize length is still 7.
        assert_eq!(encoded.len(), 7);
        let mut bit_stream = encoded.iter();
        let decoded = AtomicVal::decode_cpe(&mut bit_stream, 127).unwrap();
        assert_eq!(decoded.value(), 30);

        // Test 199 with upper bound 200.
        let atomic_val = AtomicVal::new(199, 200);
        let encoded = atomic_val.encode_cpe().unwrap();
        // Expected bitsize length is 8.
        assert_eq!(encoded.len(), 8);
        let mut bit_stream = encoded.iter();
        let decoded = AtomicVal::decode_cpe(&mut bit_stream, 200).unwrap();
        assert_eq!(decoded.value(), 199);

        // Test 100 with upper bound 255.
        let atomic_val = AtomicVal::new(100, 255);
        let encoded = atomic_val.encode_cpe().unwrap();
        // Expected bitsize length is still 8.
        assert_eq!(encoded.len(), 8);
        let mut bit_stream = encoded.iter();
        let decoded = AtomicVal::decode_cpe(&mut bit_stream, 255).unwrap();
        assert_eq!(decoded.value(), 100);

        // Test 255 with upper bound 255.
        let atomic_val = AtomicVal::new(255, 255);
        let encoded = atomic_val.encode_cpe().unwrap();
        // Expected bitsize length is still 8.
        assert_eq!(encoded.len(), 8);
        let mut bit_stream = encoded.iter();
        let decoded = AtomicVal::decode_cpe(&mut bit_stream, 255).unwrap();
        assert_eq!(decoded.value(), 255);

        Ok(())
    }

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
        let registery = Registery::new(Chain::Signet).unwrap();

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

        let account_1_encoded = account_1_to_encode.encode_cpe();
        let account_2_encoded = account_2_to_encode.encode_cpe();
        let account_3_encoded = account_3_to_encode.encode_cpe();

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

        let account_1_encoded = account_1.encode_cpe();
        let mut account_1_bit_stream = account_1_encoded.iter();

        let account_2_encoded = account_2.encode_cpe();
        let mut account_2_bit_stream = account_2_encoded.iter();

        let account_3_encoded = account_3.encode_cpe();
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

        let contract_1_encoded = contract_1.encode_cpe();
        let contract_2_encoded = contract_2.encode_cpe();
        let contract_3_encoded = contract_3.encode_cpe();

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
        let encoded = CommonShortVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonShortVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 6);
        assert_eq!(value, decoded.value());

        // Test 500
        let value = 500;
        let encoded = CommonShortVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonShortVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 6);
        assert_eq!(value, decoded.value());

        // Test 1000
        let value = 1000;
        let encoded = CommonShortVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonShortVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 6);
        assert_eq!(value, decoded.value());

        // Test 5000
        let value = 5000;
        let encoded = CommonShortVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonShortVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 6);
        assert_eq!(value, decoded.value());

        // Test 25000
        let value = 25000;
        let encoded = CommonShortVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonShortVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 6);
        assert_eq!(value, decoded.value());

        // Test 50000
        let value = 50000;
        let encoded = CommonShortVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonShortVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 6);
        assert_eq!(value, decoded.value());

        // Test 75000
        let value = 75000;
        let encoded = CommonShortVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonShortVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 6);
        assert_eq!(value, decoded.value());

        // Test 100000
        let value = 100000;
        let encoded = CommonShortVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonShortVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 6);
        assert_eq!(value, decoded.value());

        // Test 1000000
        let value = 1000000;
        let encoded = CommonShortVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonShortVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 6);
        assert_eq!(value, decoded.value());

        // Test 25000000
        let value = 25000000;
        let encoded = CommonShortVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonShortVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 6);
        assert_eq!(value, decoded.value());

        // Test 50000000
        let value = 50000000;
        let encoded = CommonShortVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonShortVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 6);
        assert_eq!(value, decoded.value());

        // Test 100_000_000
        let value = 100_000_000;
        let encoded = CommonShortVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonShortVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 6);
        assert_eq!(value, decoded.value());

        Ok(())
    }

    #[tokio::test]
    async fn test_common_long_val() -> Result<(), String> {
        // Test 100
        let value = 100;
        let encoded = CommonLongVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonLongVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 7);
        assert_eq!(value, decoded.value());

        // Test 500
        let value = 500;
        let encoded = CommonLongVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonLongVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 7);
        assert_eq!(value, decoded.value());

        // Test 1000
        let value = 1000;
        let encoded = CommonLongVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonLongVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 7);
        assert_eq!(value, decoded.value());

        // Test 5000
        let value = 5000;
        let encoded = CommonLongVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonLongVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 7);
        assert_eq!(value, decoded.value());

        // Test 25000
        let value = 25000;
        let encoded = CommonLongVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonLongVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 7);
        assert_eq!(value, decoded.value());

        // Test 50000
        let value = 50000;
        let encoded = CommonLongVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonLongVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 7);
        assert_eq!(value, decoded.value());

        // Test 75000
        let value = 75000;
        let encoded = CommonLongVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonLongVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 7);
        assert_eq!(value, decoded.value());

        // Test 100000
        let value = 100000;
        let encoded = CommonLongVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonLongVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 7);
        assert_eq!(value, decoded.value());

        // Test 1000000
        let value = 1000000;
        let encoded = CommonLongVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonLongVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 7);
        assert_eq!(value, decoded.value());

        // Test 25000000
        let value = 25000000;
        let encoded = CommonLongVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonLongVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 7);
        assert_eq!(value, decoded.value());

        // Test 50000000
        let value = 50000000;
        let encoded = CommonLongVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonLongVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 7);
        assert_eq!(value, decoded.value());

        // Test 100_000_000
        let value = 100_000_000;
        let encoded = CommonShortVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonShortVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 6);
        assert_eq!(value, decoded.value());

        // Test 150_000_000
        let value = 150_000_000;
        let encoded = CommonLongVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonLongVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 7);
        assert_eq!(value, decoded.value());

        // Tetst 600_000_000_000
        let value = 600_000_000_000;
        let encoded = CommonLongVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonLongVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 7);
        assert_eq!(value, decoded.value());

        // Test 10_000_000_000_000
        let value = 10_000_000_000_000;
        let encoded = CommonLongVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonLongVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 7);
        assert_eq!(value, decoded.value());

        // Test 750_000_000_000_000
        let value = 750_000_000_000_000;
        let encoded = CommonLongVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonLongVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 7);
        assert_eq!(value, decoded.value());

        // Test 10_000_000_000_000_000
        let value = 10_000_000_000_000_000;
        let encoded = CommonLongVal::new(value).unwrap().encode_cpe().unwrap();
        let decoded = CommonLongVal::decode_cpe(&mut encoded.iter()).unwrap();

        assert_eq!(encoded.len(), 7);
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
            maybe_common_short_val.value().value()
        );
        assert_eq!(
            uncommon_short_val.value(),
            maybe_common_uncommon_short_val.value().value()
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
                .value();

        let uncommon_short_val_decoded: ShortVal =
            MaybeCommon::<ShortVal>::decode_cpe(&mut maybe_common_uncommon_short_val_bit_stream)
                .unwrap()
                .value();

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
            maybe_common_long_val.value().value()
        );

        assert_eq!(
            uncommon_long_val.value(),
            maybe_common_uncommon_long_val.value().value()
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
                .value();

        let uncommon_long_val_decoded: LongVal =
            MaybeCommon::<LongVal>::decode_cpe(&mut maybe_common_uncommon_long_val_bit_stream)
                .unwrap()
                .value();

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
    async fn atomic_val_4_bit_test() -> Result<(), String> {
        // 0 to 15
        for i in 0..=15 {
            // encode u8
            let val: u8 = i;

            let val_bytes = val.to_le_bytes();

            // Convert the rank (u8) into a BitVec.
            // 4 bits
            let mut val_bits = BitVec::new();
            for i in 0..4 {
                val_bits.push((val_bytes[0] >> i) & 1 == 1);
            }

            // Decode the rank directly from the BitVec.
            // 4 bits
            let mut decoded_val = 0u8;
            for i in 0..4 {
                let bit = val_bits[i];
                if bit {
                    decoded_val |= 1 << i;
                }
            }

            assert_eq!(val, decoded_val);
        }

        Ok(())
    }
}
