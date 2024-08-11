#[cfg(test)]
mod txo_tests {
    use brollup::{
        entry::{entry::Entry, transfer::Transfer},
        encoding::cpe::CompactPayloadEncoding,
        taproot::P2TR,
        txo::{
            connector::Connector,
            lift::Lift,
            projector::{Projector, ProjectorTag},
            vtxo::VTXO,
        },
        valtype::{account::Account, value::ShortVal},
    };
    use musig2::secp256k1::{self, Parity, PublicKey, XOnlyPublicKey};

    #[test]
    fn test_lift() -> Result<(), secp256k1::Error> {
        let self_key: XOnlyPublicKey =
            "b2d9fb51db445564f1d4e754f644597b11ff191d12c2a582fb598e509cd72421"
                .parse()
                .unwrap();

        let lift_txo = Lift::new(self_key);

        let tap_tree = lift_txo
            .taproot()?
            .tree()
            .expect("lift_txo is not a valid tap_tree");

        let exit_path = tap_tree.leaves()[0].tap_script();

        let exit_path_expected = hex::decode(
            "0350cd00b27520b2d9fb51db445564f1d4e754f644597b11ff191d12c2a582fb598e509cd72421ac",
        )
        .unwrap();

        assert_eq!(exit_path, exit_path_expected);

        let spk = lift_txo.taproot()?.spk()?;
        let spk_expected =
            hex::decode("5120768f26cecaee8a061b981a1562cac68d1a21e673c70647111a95afbdc0e484c2")
                .unwrap();

        assert_eq!(spk, spk_expected);

        Ok(())
    }

    #[test]
    fn test_vtxo() -> Result<(), secp256k1::Error> {
        let self_key: XOnlyPublicKey =
            "255ac1b59bafb50b4fead46fd8bf07884a9e23b6cd82a5e348a756b66973082e"
                .parse()
                .unwrap();

        let lift_txo = VTXO::new(self_key);

        let tap_tree = lift_txo
            .taproot()?
            .tree()
            .expect("lift_txo is not a valid tap_tree");

        let exit_path = tap_tree.leaves()[0].tap_script();

        let exit_path_expected = hex::decode(
            "02a032b27520255ac1b59bafb50b4fead46fd8bf07884a9e23b6cd82a5e348a756b66973082eac",
        )
        .unwrap();

        assert_eq!(exit_path, exit_path_expected);

        let spk = lift_txo.taproot()?.spk()?;
        let spk_expected =
            hex::decode("51202ed65f7f5936aa7a51c39ae0b38df3f339e52b5c6cbcfa8f37c082025e06d46f")
                .unwrap();

        assert_eq!(spk, spk_expected);

        Ok(())
    }

    #[test]
    fn test_connector() -> Result<(), secp256k1::Error> {
        let self_key: XOnlyPublicKey =
            "f28c4676022feba41258aeebcd82ec67c73e7b391fae3b702a61cc28ef3a541d"
                .parse()
                .unwrap();

        let connector_txo = Connector::new(self_key);

        let spk = connector_txo.taproot()?.spk()?;
        let spk_expected =
            hex::decode("5120ac55373c7c33dd80720b58440c1f957717585a423e467373dbabfc77cff21e4b")
                .unwrap();

        assert_eq!(spk, spk_expected);

        Ok(())
    }

    #[test]
    fn test_projector() -> Result<(), secp256k1::Error> {
        let public_key_1: XOnlyPublicKey =
            "9dde15a45d76d940f90188537d52136ba5e86c8fb2f521f53be794410352798f"
                .parse()
                .unwrap();
        let public_key_2: XOnlyPublicKey =
            "cf77e4bb66c0a1ce2cd04cd2838ea5d4210e1474fabe717c47237a1da77b81bc"
                .parse()
                .unwrap();
        let public_key_3: XOnlyPublicKey =
            "ea3d7da21468ef105ad5f3fef1710dd2c759f0014563fd9df922ec7456a9f811"
                .parse()
                .unwrap();

        let mut pubkeys: [PublicKey; 3] = [
            public_key_1.public_key(Parity::Even),
            public_key_2.public_key(Parity::Even),
            public_key_3.public_key(Parity::Even),
        ];
        pubkeys.sort();

        let pubkeys = vec![public_key_1, public_key_2, public_key_3];

        let projector = Projector::new(pubkeys, ProjectorTag::VTXOProjector);

        let agg_key: XOnlyPublicKey = projector.agg_key().unwrap();

        println!("agg_key {}", agg_key);

        let sweep_path = projector
            .taproot()?
            .tree()
            .expect("no projector tap_tree found.")
            .leaves()[0]
            .tap_script();

        let expected_sweep_path = hex::decode(
            "02a032b27520fe44f87e8dcf65392e213f304bee1e3a31e562bc1061830d6f2e9539496c46f2ac",
        )
        .unwrap();

        assert_eq!(sweep_path, expected_sweep_path);

        let spk = projector.spk().unwrap();
        let expected_spk =
            hex::decode("51205b2675abdef4752c84d30cccb6249503c199a15e9ef0cf48ee8c6c477762dcd7")
                .unwrap();

        assert_eq!(expected_spk, spk);

        Ok(())
    }

    #[test]
    fn test_txo_transfer() {
        let key: XOnlyPublicKey =
            "b2d9fb51db445564f1d4e754f644597b11ff191d12c2a582fb598e509cd72421"
                .parse()
                .unwrap();

        let acct = Account::new(key);
        let amt = ShortVal::new(23423);

        let trs = Transfer::new_uncommon(acct, acct, amt);

        let txo = Entry::Transfer(trs);

        println!("bits {}", txo.to_cpe());
    }
}
