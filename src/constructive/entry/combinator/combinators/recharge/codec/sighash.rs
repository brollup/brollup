use crate::{
    constructive::entry::combinator::{
        combinator_type::CombinatorType, combinators::recharge::recharge::Recharge,
    },
    transmutative::{
        hash::{Hash, HashTag},
        secp::authenticable::AuthSighash,
    },
};
use bitcoin::hashes::Hash as _;

/// The sighash for the `Recharge` combinator.
impl AuthSighash for Recharge {
    fn auth_sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        for vtxo in self.recharge_vtxos.iter() {
            match vtxo.outpoint() {
                Some(outpoint) => {
                    preimage.extend(outpoint.txid.to_byte_array());
                    preimage.extend(outpoint.vout.to_le_bytes());
                }
                None => return [0; 32],
            };
        }

        preimage.hash(Some(HashTag::SighashCombinator(CombinatorType::Recharge)))
    }
}
