use crate::{
    hash::{sha256, Hash, HashTag},
    taproot::P2TR,
    txo::projector::Projector,
};

pub fn sigmsg_txn_1(
    outpoint: [u8; 36],
    prev_spk: Vec<u8>,
    projector: Projector,
) -> Option<[u8; 32]> {
    let projector_spk = projector.spk()?;

    let spending_amt = hex::decode("b80b000000000000").unwrap(); // 3000

    let mut sighash_preimage = Vec::<u8>::new();

    sighash_preimage.push(0x00);
    sighash_preimage.push(0x00);

    sighash_preimage.extend(hex::decode("01000000").unwrap()); //nver
    sighash_preimage.extend(hex::decode("00000000").unwrap()); //nLocktime

    let sha_prevouts = {
        let mut sha_prevouts = Vec::<u8>::new();
        sha_prevouts.extend(&outpoint);
        sha256(&sha_prevouts)
    };

    sighash_preimage.extend(sha_prevouts);

    let sha_amounts = {
        let mut sha_amounts = Vec::<u8>::new();
        sha_amounts.extend(&spending_amt);
        sha256(&sha_amounts)
    };

    sighash_preimage.extend(sha_amounts);

    let sha_scriptpubkeys = {
        let mut sha_scriptpubkeys = Vec::<u8>::new();
        sha_scriptpubkeys.push(0x22);
        sha_scriptpubkeys.extend(&prev_spk);

        sha256(&sha_scriptpubkeys)
    };

    sighash_preimage.extend(sha_scriptpubkeys);

    let sha_sequences = {
        let mut sha_sequences = Vec::<u8>::new();
        sha_sequences.extend(&hex::decode("ffffffff").unwrap());
        sha256(&sha_sequences)
    };

    sighash_preimage.extend(sha_sequences);

    let sha_outputs = {
        let mut txn = Vec::<u8>::new();
        // VTXO projector
        txn.extend(hex::decode("d007000000000000").unwrap()); // 2000 sats amt
        txn.push(0x22); // spk len
        txn.extend(&projector_spk);

        // Connector projector
        txn.extend(hex::decode("f401000000000000").unwrap()); // 500 sats amt
        txn.push(0x22); // spk len
        txn.extend(&projector_spk);
        sha256(&txn)
    };

    sighash_preimage.extend(sha_outputs);

    sighash_preimage.push(0x00);
    sighash_preimage.extend(&hex::decode("00000000").unwrap()); // spending input

    Some(sighash_preimage.hash(Some(HashTag::TapSighash)))
}

pub fn sigmsg_txn_2(
    txid: [u8; 32],
    prev_spk: Vec<u8>,
    key_1: [u8; 32],
    key_2: [u8; 32],
    key_3: [u8; 32],
) -> Option<[u8; 32]> {
    let spending_amt = hex::decode("d007000000000000").unwrap(); // 2000

    let mut sighash_preimage = Vec::<u8>::new();

    sighash_preimage.push(0x00);
    sighash_preimage.push(0x00);

    sighash_preimage.extend(hex::decode("01000000").unwrap()); //nver
    sighash_preimage.extend(hex::decode("00000000").unwrap()); //nLocktime

    let sha_prevouts = {
        let mut sha_prevouts = Vec::<u8>::new();
        sha_prevouts.extend(&txid);
        sha_prevouts.extend(hex::decode("00000000").unwrap());
        sha256(&sha_prevouts)
    };

    sighash_preimage.extend(sha_prevouts);

    let sha_amounts = {
        let mut sha_amounts = Vec::<u8>::new();
        sha_amounts.extend(&spending_amt);
        sha256(&sha_amounts)
    };

    sighash_preimage.extend(sha_amounts);

    let sha_scriptpubkeys = {
        let mut sha_scriptpubkeys = Vec::<u8>::new();
        sha_scriptpubkeys.push(0x22);
        sha_scriptpubkeys.extend(&prev_spk);

        sha256(&sha_scriptpubkeys)
    };

    sighash_preimage.extend(sha_scriptpubkeys);

    let sha_sequences = {
        let mut sha_sequences = Vec::<u8>::new();
        sha_sequences.extend(&hex::decode("ffffffff").unwrap());
        sha256(&sha_sequences)
    };

    sighash_preimage.extend(sha_sequences);

    let sha_outputs = {
        let mut txn = Vec::<u8>::new();
        // VTXO projector
        txn.extend(hex::decode("f401000000000000").unwrap()); // 500 sats amt
        txn.push(0x22); // spk len
        txn.push(0x51);
        txn.push(0x20);
        txn.extend(&key_1);
        txn.extend(hex::decode("f401000000000000").unwrap()); // 500 sats amt
        txn.push(0x22); // spk len
        txn.push(0x51);
        txn.push(0x20);
        txn.extend(&key_2);
        txn.extend(hex::decode("f401000000000000").unwrap()); // 500 sats amt
        txn.push(0x22); // spk len
        txn.push(0x51);
        txn.push(0x20);
        txn.extend(&key_3);
        sha256(&txn)
    };

    sighash_preimage.extend(sha_outputs);

    sighash_preimage.push(0x00);
    sighash_preimage.extend(&hex::decode("00000000").unwrap()); // spending input

    Some(sighash_preimage.hash(Some(HashTag::TapSighash)))
}

pub fn tx_2_build(
    txid: [u8; 32],
    key_1: [u8; 32],
    key_2: [u8; 32],
    key_3: [u8; 32],
    sig: [u8; 64],
) -> Option<Vec<u8>> {
    let mut txn = Vec::<u8>::new();

    txn.extend(hex::decode("01000000").unwrap()); //nver

    txn.push(0x00); // marker
    txn.push(0x01); // flag

    txn.push(0x01); // num inputs

    txn.extend(txid); // outpoint
    txn.extend(hex::decode("00000000").unwrap()); // nSequence

    txn.push(0x00); // scriptsig

    txn.extend(hex::decode("ffffffff").unwrap()); // nSequence

    txn.push(0x03); // num outs

    {
        // Key 1
        txn.extend(hex::decode("f401000000000000").unwrap()); // 500 sats amt
        txn.push(0x22); // spk len
        txn.push(0x51); // spk len
        txn.push(0x20); // spk len
        txn.extend(&key_1);
    }

    {
        // Key 1
        txn.extend(hex::decode("f401000000000000").unwrap()); // 500 sats amt
        txn.push(0x22); // spk len
        txn.push(0x51); // spk len
        txn.push(0x20); // spk len
        txn.extend(&key_2);
    }

    {
        // Key 1
        txn.extend(hex::decode("f401000000000000").unwrap()); // 500 sats amt
        txn.push(0x22); // spk len
        txn.push(0x51); // spk len
        txn.push(0x20); // spk len
        txn.extend(&key_3);
    }

    txn.push(0x01); // num witness elements
    txn.push(0x40); // sig len
    txn.extend(sig); // sig

    txn.extend(hex::decode("00000000").unwrap()); // locktime

    Some(txn)
}

pub fn tx_1_build(outpoint: [u8; 36], projector: Projector, sig: [u8; 64]) -> Option<Vec<u8>> {
    let projector_spk = projector.spk()?;

    let mut txn = Vec::<u8>::new();

    txn.extend(hex::decode("01000000").unwrap()); //nver

    txn.push(0x00); // marker
    txn.push(0x01); // flag

    txn.push(0x01); // num inputs

    txn.extend(outpoint); // outpoint

    txn.push(0x00); // scriptsig

    txn.extend(hex::decode("ffffffff").unwrap()); // nSequence

    txn.push(0x02); // num outs

    // VTXO projector
    txn.extend(hex::decode("d007000000000000").unwrap()); // 2000 sats amt
    txn.push(0x22); // spk len
    txn.extend(&projector_spk);

    // Connector projector
    txn.extend(hex::decode("f401000000000000").unwrap()); // 2000 sats amt
    txn.push(0x22); // spk len
    txn.extend(&projector_spk);

    txn.push(0x01); // num witness elements
    txn.push(0x40); // sig len
    txn.extend(sig); // sig

    txn.extend(hex::decode("00000000").unwrap()); // locktime

    Some(txn)
}

pub fn tx_1_id(outpoint: [u8; 36], projector: Projector) -> Option<[u8; 32]> {
    let projector_spk = projector.spk()?;

    let mut txn = Vec::<u8>::new();

    txn.extend(hex::decode("01000000").unwrap()); //nver

    txn.push(0x01); // num inputs

    txn.extend(outpoint); // outpoint

    txn.push(0x00); // scriptsig

    txn.extend(hex::decode("ffffffff").unwrap()); // nSequence

    txn.push(0x02); // num outs

    // VTXO projector
    txn.extend(hex::decode("d007000000000000").unwrap()); // 2000 sats amt
    txn.push(0x22); // spk len
    txn.extend(&projector_spk);

    // Connector projector
    txn.extend(hex::decode("f401000000000000").unwrap()); // 2000 sats amt
    txn.push(0x22); // spk len
    txn.extend(&projector_spk);

    txn.extend(hex::decode("00000000").unwrap()); // locktime

    Some(sha256(&sha256(&txn)))
}
