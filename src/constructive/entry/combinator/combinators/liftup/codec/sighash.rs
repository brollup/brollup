use crate::{
    constructive::entry::combinator::{
        combinator_type::CombinatorType, combinators::liftup::liftup::Liftup,
    },
    transmutative::{
        hash::{Hash, HashTag},
        secp::authenticable::AuthSighash,
    },
};
use bitcoin::hashes::Hash as _;

impl AuthSighash for Liftup {
    fn auth_sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        for prevtxo in self.lift_prevtxos.iter() {
            match prevtxo.outpoint() {
                Some(outpoint) => {
                    preimage.extend(outpoint.txid.to_byte_array());
                    preimage.extend(outpoint.vout.to_le_bytes());
                }
                None => return [0; 32],
            }
        }

        preimage.hash(Some(HashTag::SighashCombinator(CombinatorType::Liftup)))
    }
}
