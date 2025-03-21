#[cfg(test)]
mod cpe_tests {
    use bit_vec::BitVec;
    use brollup::{
        cpe::CompactPayloadEncoding,
        entity::account::Account,
        registery::registery::Registery,
        valtype::{
            long::{LongVal, LongValTier},
            short::{ShortVal, ShortValTier},
        },
        Network,
    };
    use secp::Point;

    #[tokio::test]
    async fn cpe_single_short_val_test() -> Result<(), String> {
        // Value 100 (u8) (0 < 100 < 256).
        let short_val = ShortVal::new(100);
        let encoded = short_val.encode();
        let (decoded, _) = ShortVal::decode(&encoded, None).await.unwrap();
        assert_eq!(decoded.tier(), ShortValTier::U8);
        assert_eq!(decoded.value(), 100);

        // Value 5_000 (u16) (256 < 5_000 < 65_536).
        let short_val = ShortVal::new(5000);
        let encoded = short_val.encode();
        let (decoded, _) = ShortVal::decode(&encoded, None).await.unwrap();
        assert_eq!(decoded.tier(), ShortValTier::U16);
        assert_eq!(decoded.value(), 5000);

        // Value 100_000 (u24) (65_536 < 100_000 < 16_777_216).
        let short_val = ShortVal::new(100_000);
        let encoded = short_val.encode();
        let (decoded, _) = ShortVal::decode(&encoded, None).await.unwrap();
        assert_eq!(decoded.tier(), ShortValTier::U24);
        assert_eq!(decoded.value(), 100_000);

        // Value 50_000_000 (u32) (16_777_216 < 50_000_000 < 4_294_967_296).
        let short_val = ShortVal::new(50_000_000);
        let encoded = short_val.encode();
        let (decoded, _) = ShortVal::decode(&encoded, None).await.unwrap();
        assert_eq!(decoded.tier(), ShortValTier::U32);
        assert_eq!(decoded.value(), 50_000_000);

        Ok(())
    }

    #[tokio::test]
    async fn cpe_multi_short_val_test() -> Result<(), String> {
        let mut full = BitVec::new();

        // Insert 100 (u8) (0 < 100 < 256).
        let short_val = ShortVal::new(100);
        let encoded = short_val.encode();
        full.extend(&encoded);

        // Insert 5_000 (u16) (256 < 5_000 < 65_536).
        let short_val = ShortVal::new(5000);
        let encoded = short_val.encode();
        full.extend(&encoded);

        // Insert 100_000 (u24) (65_536 < 100_000 < 16_777_216).
        let short_val = ShortVal::new(100_000);
        let encoded = short_val.encode();
        full.extend(&encoded);

        // Insert 50_000_000 (u32) (16_777_216 < 50_000_000 < 4_294_967_296).
        let short_val = ShortVal::new(50_000_000);
        let encoded = short_val.encode();
        full.extend(&encoded);

        // Insert 5 garbage bits.
        full.push(true);
        full.push(false);
        full.push(false);
        full.push(true);
        full.push(true);

        let (decoded, remaining) = ShortVal::decode(&full, None).await.unwrap();
        assert_eq!(decoded.tier(), ShortValTier::U8);
        assert_eq!(decoded.value(), 100);

        let (decoded, remaining) = ShortVal::decode(&remaining, None).await.unwrap();
        assert_eq!(decoded.tier(), ShortValTier::U16);
        assert_eq!(decoded.value(), 5_000);

        let (decoded, remaining) = ShortVal::decode(&remaining, None).await.unwrap();
        assert_eq!(decoded.tier(), ShortValTier::U24);
        assert_eq!(decoded.value(), 100_000);

        let (decoded, remaining) = ShortVal::decode(&remaining, None).await.unwrap();
        assert_eq!(decoded.tier(), ShortValTier::U32);
        assert_eq!(decoded.value(), 50_000_000);

        assert_eq!(remaining.len(), 5);

        Ok(())
    }

    #[tokio::test]
    async fn cpe_single_long_val_test() -> Result<(), String> {
        // Value 100 (u8) (0 < 100 < 256).
        let long_val = LongVal::new(100);
        let encoded = long_val.encode();
        let (decoded, _) = LongVal::decode(&encoded, None).await.unwrap();
        assert_eq!(decoded.tier(), LongValTier::U8);
        assert_eq!(decoded.value(), 100);

        // Value 5_000 (u16) (256 < 5_000 < 65_536).
        let long_val = LongVal::new(5_000);
        let encoded = long_val.encode();
        let (decoded, _) = LongVal::decode(&encoded, None).await.unwrap();
        assert_eq!(decoded.tier(), LongValTier::U16);
        assert_eq!(decoded.value(), 5_000);

        // Value 100_000 (u24) (65_536 < 100_000 < 16_777_216).
        let long_val = LongVal::new(100_000);
        let encoded = long_val.encode();
        let (decoded, _) = LongVal::decode(&encoded, None).await.unwrap();
        assert_eq!(decoded.tier(), LongValTier::U24);
        assert_eq!(decoded.value(), 100_000);

        // Value 50_000_000 (u32) (16_777_216 < 50_000_000 < 4_294_967_296).
        let long_val = LongVal::new(50_000_000);
        let encoded = long_val.encode();
        let (decoded, _) = LongVal::decode(&encoded, None).await.unwrap();
        assert_eq!(decoded.tier(), LongValTier::U32);
        assert_eq!(decoded.value(), 50_000_000);

        // Value 100_000_000_000 (u40) (4_294_967_296 < 100_000_000_000 < 1_099_511_627_776).
        let long_val = LongVal::new(100_000_000_000);
        let encoded = long_val.encode();
        let (decoded, _) = LongVal::decode(&encoded, None).await.unwrap();
        assert_eq!(decoded.tier(), LongValTier::U40);
        assert_eq!(decoded.value(), 100_000_000_000);

        //281,474,976,710,655
        // Value 100_000_000_000_000 (u48) (1_099_511_627_776 < 100_000_000_000_000 < 2_814_749_767_106_56).
        let long_val = LongVal::new(100_000_000_000_000);
        let encoded = long_val.encode();
        let (decoded, _) = LongVal::decode(&encoded, None).await.unwrap();
        assert_eq!(decoded.tier(), LongValTier::U48);
        assert_eq!(decoded.value(), 100_000_000_000_000);

        // Value 100_000_000_000_000_000 (u56) (2_814_749_767_106_56 < 50_000_000_000_000_00 < 72_057_594_037_927_936).
        let long_val = LongVal::new(50_000_000_000_000_00);
        let encoded = long_val.encode();
        let (decoded, _) = LongVal::decode(&encoded, None).await.unwrap();
        assert_eq!(decoded.tier(), LongValTier::U56);
        assert_eq!(decoded.value(), 50_000_000_000_000_00);

        // Value 100_000_000_000_000_000_000 (u64) (72_057_594_037_927_936 < 100_000_000_000_000_000 < 18,446,744,073,709,551,616).
        let long_val = LongVal::new(100_000_000_000_000_000);
        let encoded = long_val.encode();
        let (decoded, _) = LongVal::decode(&encoded, None).await.unwrap();
        assert_eq!(decoded.tier(), LongValTier::U64);
        assert_eq!(decoded.value(), 100_000_000_000_000_000);

        Ok(())
    }

    #[tokio::test]
    async fn cpe_multi_long_val_test() -> Result<(), String> {
        let mut full = BitVec::new();

        // Insert 100 (u8) (0 < 100 < 256).
        let long_val = LongVal::new(100);
        let encoded = long_val.encode();
        full.extend(&encoded);

        // Insert 5_000 (u16) (256 < 5_000 < 65_536).
        let long_val = LongVal::new(5_000);
        let encoded = long_val.encode();
        full.extend(&encoded);

        // Insert 100_000 (u24) (65_536 < 100_000 < 16_777_216).
        let long_val = LongVal::new(100_000);
        let encoded = long_val.encode();
        full.extend(&encoded);

        // Insert 50_000_000 (u32) (16_777_216 < 50_000_000 < 4_294_967_296).
        let long_val = LongVal::new(50_000_000);
        let encoded = long_val.encode();
        full.extend(&encoded);

        // Insert 100_000_000_000 (u40) (4_294_967_296 < 100_000_000_000 < 1_099_511_627_776).
        let long_val = LongVal::new(100_000_000_000);
        let encoded = long_val.encode();
        full.extend(&encoded);

        // Insert 100_000_000_000_000 (u48) (1_099_511_627_776 < 100_000_000_000_000 < 2_814_749_767_106_56).
        let long_val = LongVal::new(100_000_000_000_000);
        let encoded = long_val.encode();
        full.extend(&encoded);

        // Insert 100_000_000_000_000_000 (u56) (2_814_749_767_106_56 < 50_000_000_000_000_00 < 72_057_594_037_927_936).
        let long_val = LongVal::new(50_000_000_000_000_00);
        let encoded = long_val.encode();
        full.extend(&encoded);

        // Insert 100_000_000_000_000_000_000 (u64) (72_057_594_037_927_936 < 100_000_000_000_000_000 < 18,446,744,073,709,551,616).
        let long_val = LongVal::new(100_000_000_000_000_000);
        let encoded = long_val.encode();
        full.extend(&encoded);

        // Insert 5 garbage bits.
        full.push(true);
        full.push(false);
        full.push(false);
        full.push(true);
        full.push(true);

        let (decoded, remaining) = LongVal::decode(&full, None).await.unwrap();
        assert_eq!(decoded.tier(), LongValTier::U8);
        assert_eq!(decoded.value(), 100);

        let (decoded, remaining) = LongVal::decode(&remaining, None).await.unwrap();
        assert_eq!(decoded.tier(), LongValTier::U16);
        assert_eq!(decoded.value(), 5_000);

        let (decoded, remaining) = LongVal::decode(&remaining, None).await.unwrap();
        assert_eq!(decoded.tier(), LongValTier::U24);
        assert_eq!(decoded.value(), 100_000);

        let (decoded, remaining) = LongVal::decode(&remaining, None).await.unwrap();
        assert_eq!(decoded.tier(), LongValTier::U32);
        assert_eq!(decoded.value(), 50_000_000);

        let (decoded, remaining) = LongVal::decode(&remaining, None).await.unwrap();
        assert_eq!(decoded.tier(), LongValTier::U40);
        assert_eq!(decoded.value(), 100_000_000_000);

        let (decoded, remaining) = LongVal::decode(&remaining, None).await.unwrap();
        assert_eq!(decoded.tier(), LongValTier::U48);
        assert_eq!(decoded.value(), 100_000_000_000_000);

        let (decoded, remaining) = LongVal::decode(&remaining, None).await.unwrap();
        assert_eq!(decoded.tier(), LongValTier::U56);
        assert_eq!(decoded.value(), 50_000_000_000_000_00);

        let (decoded, remaining) = LongVal::decode(&remaining, None).await.unwrap();
        assert_eq!(decoded.tier(), LongValTier::U64);
        assert_eq!(decoded.value(), 100_000_000_000_000_000);

        assert_eq!(remaining.len(), 5);

        Ok(())
    }

    #[tokio::test]
    async fn unregistered_account_test() -> Result<(), String> {
        let point =
            Point::from_hex("022d69e8ef6a06ed3efcf433ee24dbe55e8e6dec5804957326b07c3902960af1f9")
                .unwrap();

        let account_to_encode = Account::new(point, None).unwrap();
        let encoded = account_to_encode.encode();

        let (account_decoded, _) = Account::decode(&encoded, None).await.unwrap();
        assert_eq!(account_to_encode, account_decoded);
        assert_eq!(account_to_encode.key(), account_decoded.key());
        assert_eq!(
            account_to_encode.registery_index(),
            account_decoded.registery_index()
        );
        Ok(())
    }

    #[tokio::test]
    async fn registered_account_test() -> Result<(), String> {
        let registery = Registery::new(Network::Signet).unwrap();

        let account_registery = {
            let mut _registery = registery.lock().await;
            _registery.account_registery()
        };

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

        let encoded = account.encode();
        let (decoded, _) = Account::decode(&encoded, Some(&registery)).await.unwrap();
        assert_eq!(account, decoded);
        assert_eq!(account.key(), decoded.key());
        assert_eq!(account.registery_index(), decoded.registery_index());

        Ok(())
    }
}
