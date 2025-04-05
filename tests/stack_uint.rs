#[cfg(test)]
mod stack_uint_tests {

    use brollup::executive::stack::{
        stack_error::StackError,
        stack_item::{
            item::StackItem,
            uint_ext::{StackItemUintExt, StackUint},
        },
    };

    /// Test the stack uint conversion covering all byte-ranges from 0 to 32.
    #[test]
    #[deny(overflowing_literals)]
    fn stack_uint_test() -> Result<(), StackError> {
        // Test 0
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(0));
        assert_eq!(stack_item.to_uint().unwrap(), StackUint::from(0));
        assert_eq!(stack_item.bytes().len(), 0);

        // Test 1
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(1));
        assert_eq!(stack_item.to_uint().unwrap(), StackUint::from(1));
        assert_eq!(stack_item.bytes().len(), 1);

        // Test 2
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(2));
        assert_eq!(stack_item.to_uint().unwrap(), StackUint::from(2));
        assert_eq!(stack_item.bytes().len(), 1);

        // Test 3
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(3));
        assert_eq!(stack_item.to_uint().unwrap(), StackUint::from(3));
        assert_eq!(stack_item.bytes().len(), 1);

        // Test 10
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(10));
        assert_eq!(stack_item.to_uint().unwrap(), StackUint::from(10));
        assert_eq!(stack_item.bytes().len(), 1);

        // Test 100
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(100));
        assert_eq!(stack_item.to_uint().unwrap(), StackUint::from(100));
        assert_eq!(stack_item.bytes().len(), 1);

        // Test 255
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(255));
        assert_eq!(stack_item.to_uint().unwrap(), StackUint::from(255));
        assert_eq!(stack_item.bytes().len(), 1);

        // Test 256 (now we are in the 2-byte range)
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(256));
        assert_eq!(stack_item.to_uint().unwrap(), StackUint::from(256));
        assert_eq!(stack_item.bytes().len(), 2);

        // Test 1_000
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(1000));
        assert_eq!(stack_item.to_uint().unwrap(), StackUint::from(1000));
        assert_eq!(stack_item.bytes().len(), 2);

        // Test 10_000
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(10000));
        assert_eq!(stack_item.to_uint().unwrap(), StackUint::from(10000));
        assert_eq!(stack_item.bytes().len(), 2);

        // Test 65535
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(65535));
        assert_eq!(stack_item.to_uint().unwrap(), StackUint::from(65535));
        assert_eq!(stack_item.bytes().len(), 2);

        // Test 65536 (now we are in the 3-byte range)
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(65536));
        assert_eq!(stack_item.to_uint().unwrap(), StackUint::from(65536));
        assert_eq!(stack_item.bytes().len(), 3);

        // Test 100_000
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(100000));
        assert_eq!(stack_item.to_uint().unwrap(), StackUint::from(100000));
        assert_eq!(stack_item.bytes().len(), 3);

        // Test 1_000_000
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(1000000));
        assert_eq!(stack_item.to_uint().unwrap(), StackUint::from(1000000));
        assert_eq!(stack_item.bytes().len(), 3);

        // Test 16777215
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(16777215));
        assert_eq!(stack_item.to_uint().unwrap(), StackUint::from(16777215));
        assert_eq!(stack_item.bytes().len(), 3);

        // Test 16777216 (now we are in the 4-byte range)
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(16777216));
        assert_eq!(stack_item.to_uint().unwrap(), StackUint::from(16777216));
        assert_eq!(stack_item.bytes().len(), 4);

        // Test 1_000_000_000
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(1000000000));
        assert_eq!(stack_item.to_uint().unwrap(), StackUint::from(1000000000));
        assert_eq!(stack_item.bytes().len(), 4);

        // Test 4,294,967,295
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(4_294_967_295_u64));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(4_294_967_295_u64)
        );
        assert_eq!(stack_item.bytes().len(), 4);

        // Test 4,294,967,296 (now we are in the 5-byte range)
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(4_294_967_296_u64));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(4_294_967_296_u64)
        );
        assert_eq!(stack_item.bytes().len(), 5);

        // Test 1_000_000_000_000
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(1000000000000_i64));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(1000000000000_i64)
        );
        assert_eq!(stack_item.bytes().len(), 5);

        // Test 1_099_511_627_775
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(1099511627775_i64));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(1099511627775_i64)
        );
        assert_eq!(stack_item.bytes().len(), 5);

        // Test 1_099_511_627_776 (now we are in the 6-byte range)
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(1099511627776_i64));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(1099511627776_i64)
        );
        assert_eq!(stack_item.bytes().len(), 6);

        // Test 281474976710655
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(281474976710655_i64));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(281474976710655_i64)
        );
        assert_eq!(stack_item.bytes().len(), 6);

        // Test 281474976710656 (now we are in the 7-byte range)
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(281474976710656_i64));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(281474976710656_i64)
        );
        assert_eq!(stack_item.bytes().len(), 7);

        // Test 72057594037927935
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(72057594037927935_i64));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(72057594037927935_i64)
        );
        assert_eq!(stack_item.bytes().len(), 7);

        // Test 72057594037927936 (now we are in the 8-byte range)
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(72057594037927936_i64));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(72057594037927936_i64)
        );
        assert_eq!(stack_item.bytes().len(), 8);

        // Test 18446744073709551615
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(18446744073709551615_u64));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(18446744073709551615_u64)
        );
        assert_eq!(stack_item.bytes().len(), 8);

        // Test 18446744073709551616 (now we are in the 9-byte range)
        let stack_item: StackItem =
            StackItem::from_uint(StackUint::from(18446744073709551616_u128));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(18446744073709551616_u128)
        );
        assert_eq!(stack_item.bytes().len(), 9);

        // Test 4722366482869645213695
        let stack_item: StackItem =
            StackItem::from_uint(StackUint::from(4722366482869645213695_u128));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(4722366482869645213695_u128)
        );
        assert_eq!(stack_item.bytes().len(), 9);

        // Test 4722366482869645213696 (now we are in the 10-byte range)
        let stack_item: StackItem =
            StackItem::from_uint(StackUint::from(4722366482869645213696_u128));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(4722366482869645213696_u128)
        );
        assert_eq!(stack_item.bytes().len(), 10);

        // Test 1208925819614629174706175
        let stack_item: StackItem =
            StackItem::from_uint(StackUint::from(1208925819614629174706175_u128));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(1208925819614629174706175_u128)
        );
        assert_eq!(stack_item.bytes().len(), 10);

        // Test 1208925819614629174706176 (now we are in the 11-byte range)
        let stack_item: StackItem =
            StackItem::from_uint(StackUint::from(1208925819614629174706176_u128));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(1208925819614629174706176_u128)
        );
        assert_eq!(stack_item.bytes().len(), 11);

        // Test 309485009821345068724781055
        let stack_item: StackItem =
            StackItem::from_uint(StackUint::from(309485009821345068724781055_u128));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(309485009821345068724781055_u128)
        );
        assert_eq!(stack_item.bytes().len(), 11);

        // Test 309485009821345068724781056 (now we are in the 12-byte range)
        let stack_item: StackItem =
            StackItem::from_uint(StackUint::from(309485009821345068724781056_u128));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(309485009821345068724781056_u128)
        );
        assert_eq!(stack_item.bytes().len(), 12);

        // Test 79228162514264337593543950335
        let stack_item: StackItem =
            StackItem::from_uint(StackUint::from(79228162514264337593543950335_u128));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(79228162514264337593543950335_u128)
        );
        assert_eq!(stack_item.bytes().len(), 12);

        // Test 79228162514264337593543950336 (now we are in the 13-byte range)
        let stack_item: StackItem =
            StackItem::from_uint(StackUint::from(79228162514264337593543950336_u128));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(79228162514264337593543950336_u128)
        );
        assert_eq!(stack_item.bytes().len(), 13);

        // Test 20282409603651670423947251286015
        let stack_item: StackItem =
            StackItem::from_uint(StackUint::from(20282409603651670423947251286015_u128));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(20282409603651670423947251286015_u128)
        );
        assert_eq!(stack_item.bytes().len(), 13);

        // Test 20282409603651670423947251286016 (now we are in the 14-byte range)
        let stack_item: StackItem =
            StackItem::from_uint(StackUint::from(20282409603651670423947251286016_u128));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(20282409603651670423947251286016_u128)
        );
        assert_eq!(stack_item.bytes().len(), 14);

        // Test 5192296858534827628530496329220095
        let stack_item: StackItem =
            StackItem::from_uint(StackUint::from(5192296858534827628530496329220095_u128));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(5192296858534827628530496329220095_u128)
        );
        assert_eq!(stack_item.bytes().len(), 14);

        // Test 5192296858534827628530496329220096 (now we are in the 15-byte range)
        let stack_item: StackItem =
            StackItem::from_uint(StackUint::from(5192296858534827628530496329220096_u128));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(5192296858534827628530496329220096_u128)
        );
        assert_eq!(stack_item.bytes().len(), 15);

        // Test 1329227995784915872903807060280344575
        let stack_item: StackItem =
            StackItem::from_uint(StackUint::from(1329227995784915872903807060280344575_u128));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(1329227995784915872903807060280344575_u128)
        );
        assert_eq!(stack_item.bytes().len(), 15);

        // Test 1329227995784915872903807060280344576 (now we are in the 16-byte range)
        let stack_item: StackItem =
            StackItem::from_uint(StackUint::from(1329227995784915872903807060280344576_u128));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(1329227995784915872903807060280344576_u128)
        );
        assert_eq!(stack_item.bytes().len(), 16);

        // Test 340282366920938463463374607431768211455
        let stack_item: StackItem = StackItem::from_uint(StackUint::from(
            340282366920938463463374607431768211455_u128,
        ));
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from(340282366920938463463374607431768211455_u128)
        );
        assert_eq!(stack_item.bytes().len(), 16);

        // Test 340282366920938463463374607431768211456 (now we are in the 17-byte range)
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str("340282366920938463463374607431768211456").unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str("340282366920938463463374607431768211456").unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 17);

        // Test 87112285931760246646623899502532662132735
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str("87112285931760246646623899502532662132735").unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str("87112285931760246646623899502532662132735").unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 17);

        // Test 87112285931760246646623899502532662132736 (now we are in the 18-byte range)
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str("87112285931760246646623899502532662132736").unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str("87112285931760246646623899502532662132736").unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 18);

        // Test 22300745198530623141535718272648361505980415
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str("22300745198530623141535718272648361505980415").unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str("22300745198530623141535718272648361505980415").unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 18);

        // Test 22300745198530623141535718272648361505980416 (now we are in the 19-byte range)
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str("22300745198530623141535718272648361505980416").unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str("22300745198530623141535718272648361505980416").unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 19);

        // Test 5708990770823839524233143877797980545530986495
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str("5708990770823839524233143877797980545530986495").unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str("5708990770823839524233143877797980545530986495").unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 19);

        // Test 5708990770823839524233143877797980545530986496 (now we are in the 20-byte range)
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str("5708990770823839524233143877797980545530986496").unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str("5708990770823839524233143877797980545530986496").unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 20);

        // Test 1461501637330902918203684832716283019655932542975
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str("1461501637330902918203684832716283019655932542975").unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str("1461501637330902918203684832716283019655932542975").unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 20);

        // Test 1461501637330902918203684832716283019655932542976 (now we are in the 21-byte range)
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str("1461501637330902918203684832716283019655932542976").unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str("1461501637330902918203684832716283019655932542976").unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 21);

        // Test 374144419156711147060143317175368453031918731001855
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str("374144419156711147060143317175368453031918731001855").unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str("374144419156711147060143317175368453031918731001855").unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 21);

        // Test 374144419156711147060143317175368453031918731001856 (now we are in the 22-byte range)
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str("374144419156711147060143317175368453031918731001856").unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str("374144419156711147060143317175368453031918731001856").unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 22);

        // Test 95780971304118053647396689196894323976171195136475135
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str("95780971304118053647396689196894323976171195136475135")
                .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str("95780971304118053647396689196894323976171195136475135")
                .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 22);

        // Test 95780971304118053647396689196894323976171195136475136 (now we are in the 23-byte range)
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str("95780971304118053647396689196894323976171195136475136")
                .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str("95780971304118053647396689196894323976171195136475136")
                .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 23);

        // Test 24519928653854221733733552434404946937899825954937634815
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str("24519928653854221733733552434404946937899825954937634815")
                .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str("24519928653854221733733552434404946937899825954937634815")
                .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 23);

        // Test 24519928653854221733733552434404946937899825954937634816 (now we are in the 24-byte range)
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str("24519928653854221733733552434404946937899825954937634816")
                .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str("24519928653854221733733552434404946937899825954937634816")
                .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 24);

        // Test 6277101735386680763835789423207666416102355444464034512895
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str("6277101735386680763835789423207666416102355444464034512895")
                .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str("6277101735386680763835789423207666416102355444464034512895")
                .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 24);

        // Test 6277101735386680763835789423207666416102355444464034512896 (now we are in the 25-byte range)
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str("6277101735386680763835789423207666416102355444464034512896")
                .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str("6277101735386680763835789423207666416102355444464034512896")
                .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 25);

        // Test 1606938044258990275541962092341162602522202993782792835301375
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str(
                "1606938044258990275541962092341162602522202993782792835301375",
            )
            .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str(
                "1606938044258990275541962092341162602522202993782792835301375"
            )
            .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 25);

        // Test 1606938044258990275541962092341162602522202993782792835301376 (now we are in the 26-byte range)
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str(
                "1606938044258990275541962092341162602522202993782792835301376",
            )
            .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str(
                "1606938044258990275541962092341162602522202993782792835301376"
            )
            .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 26);

        // Test 411376139330301510538742295639337626245683966408394965837152255
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str(
                "411376139330301510538742295639337626245683966408394965837152255",
            )
            .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str(
                "411376139330301510538742295639337626245683966408394965837152255"
            )
            .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 26);

        // Test 411376139330301510538742295639337626245683966408394965837152256 (now we are in the 27-byte range)
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str(
                "411376139330301510538742295639337626245683966408394965837152256",
            )
            .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str(
                "411376139330301510538742295639337626245683966408394965837152256"
            )
            .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 27);

        // Test 105312291668557186697918027683670432318895095400549111254310977535
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str(
                "105312291668557186697918027683670432318895095400549111254310977535",
            )
            .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str(
                "105312291668557186697918027683670432318895095400549111254310977535"
            )
            .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 27);

        // Test 105312291668557186697918027683670432318895095400549111254310977536 (now we are in the 28-byte range)
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str(
                "105312291668557186697918027683670432318895095400549111254310977536",
            )
            .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str(
                "105312291668557186697918027683670432318895095400549111254310977536"
            )
            .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 28);

        // Test 26959946667150639794667015087019630673637144422540572481103610249215
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str(
                "26959946667150639794667015087019630673637144422540572481103610249215",
            )
            .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str(
                "26959946667150639794667015087019630673637144422540572481103610249215"
            )
            .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 28);

        // Test 26959946667150639794667015087019630673637144422540572481103610249216 (now we are in the 29-byte range)
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str(
                "26959946667150639794667015087019630673637144422540572481103610249216",
            )
            .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str(
                "26959946667150639794667015087019630673637144422540572481103610249216"
            )
            .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 29);

        // 6901746346790563787434755862277025452451108972170386555162524223799295
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str(
                "6901746346790563787434755862277025452451108972170386555162524223799295",
            )
            .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str(
                "6901746346790563787434755862277025452451108972170386555162524223799295"
            )
            .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 29);

        // Test 6901746346790563787434755862277025452451108972170386555162524223799296 (now we are in the 30-byte range)
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str(
                "6901746346790563787434755862277025452451108972170386555162524223799296",
            )
            .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str(
                "6901746346790563787434755862277025452451108972170386555162524223799296"
            )
            .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 30);

        // 1766847064778384329583297500742918515827483896875618958121606201292619775
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str(
                "1766847064778384329583297500742918515827483896875618958121606201292619775",
            )
            .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str(
                "1766847064778384329583297500742918515827483896875618958121606201292619775"
            )
            .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 30);

        // Test 1766847064778384329583297500742918515827483896875618958121606201292619776 (now we are in the 31-byte range)
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str(
                "1766847064778384329583297500742918515827483896875618958121606201292619776",
            )
            .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str(
                "1766847064778384329583297500742918515827483896875618958121606201292619776"
            )
            .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 31);

        // Test 452312848583266388373324160190187140051835877600158453279131187530910662655
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str(
                "452312848583266388373324160190187140051835877600158453279131187530910662655",
            )
            .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str(
                "452312848583266388373324160190187140051835877600158453279131187530910662655"
            )
            .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 31);

        // Test 452312848583266388373324160190187140051835877600158453279131187530910662656 (now we are in the 32-byte range)
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str(
                "452312848583266388373324160190187140051835877600158453279131187530910662656",
            )
            .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str(
                "452312848583266388373324160190187140051835877600158453279131187530910662656"
            )
            .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 32);

        // Test 115792089237316195423570985008687907853269984665640564039457584007913129639935 (MAX UINT256 VALUE)
        let stack_item: StackItem = StackItem::from_uint(
            StackUint::from_dec_str(
                "115792089237316195423570985008687907853269984665640564039457584007913129639935",
            )
            .unwrap(),
        );
        assert_eq!(
            stack_item.to_uint().unwrap(),
            StackUint::from_dec_str(
                "115792089237316195423570985008687907853269984665640564039457584007913129639935"
            )
            .unwrap()
        );
        assert_eq!(stack_item.bytes().len(), 32);

        // Test 115792089237316195423570985008687907853269984665640564039457584007913129639936 (overflow)
        // Expecting error
        assert!(StackUint::from_dec_str(
            "115792089237316195423570985008687907853269984665640564039457584007913129639936",
        )
        .is_err());

        Ok(())
    }
}
