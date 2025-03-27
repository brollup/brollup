use crate::{
    cpe::{
        cpe::CompactPayloadEncoding,
        decode_error::{entry_error::LiftupCPEDecodingError, error::CPEDecodingError},
    },
    entity::account::Account,
    hash::{Hash, HashTag},
    schnorr::Sighash,
    taproot::P2TR,
    txn::txholder::TxHolder,
    txo::lift::Lift,
    valtype::short_val::ShortVal,
    EPOCH_DIRECTORY,
};
use bit_vec::BitVec;
use bitcoin::hashes::Hash as _;
use secp::Point;
use serde::{Deserialize, Serialize};

/// A `Liftup` is a collection of `Lift`s that are being lifted up.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Liftup {
    lift_prevtxos: Vec<Lift>,
}

impl Liftup {
    /// Creates a new `Liftup` from a list of `Lift`s.  
    pub fn new(lifts: Vec<Lift>) -> Option<Liftup> {
        let mut lift_prevtxos = Vec::<Lift>::new();

        for lift in lifts.iter() {
            match lift.outpoint() {
                Some(_) => lift_prevtxos.push(lift.to_owned()),
                None => return None,
            }
        }

        let liftup = Liftup { lift_prevtxos };

        Some(liftup)
    }

    /// Returns the list of `Lift`s in the `Liftup`.
    pub fn lifts(&self) -> Vec<Lift> {
        self.lift_prevtxos.clone()
    }

    /// Returns the number of `Lift`s in the `Liftup`.
    pub fn num_lifts(&self) -> usize {
        self.lift_prevtxos.len()
    }

    /// Serializes the `Liftup` to a byte vector.
    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    /// Validates the `Liftup` against an `Account`.
    pub fn validate_account(&self, account: Account) -> bool {
        for lift in self.lift_prevtxos.iter() {
            if let None = lift.outpoint() {
                return false;
            }

            if lift.account_key() != account.key() {
                return false;
            }
        }

        true
    }

    /// Compact payload decoding for `Liftup`.
    /// Decodes a `Liftup` from a bit stream and `TxHolder`.
    pub async fn decode_cpe<'a>(
        bit_stream: &mut bit_vec::Iter<'a>,
        txholder: &'a mut TxHolder,
        epoch_dir: &'a EPOCH_DIRECTORY,
        account_key: Point,
    ) -> Result<Liftup, CPEDecodingError> {
        // Decode the number of lifts.
        let num_lifts = ShortVal::decode_cpe(bit_stream)?;

        // Initialize empty vector of lifts.
        let mut lifts = Vec::<Lift>::new();

        // Iterate over the number of lifts.
        for _ in 0..num_lifts.value() {
            // Iterate over the TxHolder lift inputs.
            let (txin_outpoint, txin_txout) = match txholder.current_in().await {
                Some((outpoint, txout)) => (outpoint, txout),
                None => {
                    // Unable to find a matching `Lift` at the current transaction input iterator position.
                    let input_iter_position = txholder.input_iter_position();
                    return Err(CPEDecodingError::LiftupCPEDecodingError(
                        LiftupCPEDecodingError::NoLiftAtInputIter(input_iter_position),
                    ));
                }
            };

            // Get the script pubkey of the transaction input being spent.
            let txin_spk = txin_txout.script_pubkey.to_bytes();

            // Check if spk matches to one of the operator key comibinations.
            let epoch_group_keys = {
                let _epoch_dir = epoch_dir.lock().await;
                _epoch_dir.active_group_keys()
            };

            // Iterate over the epoch group keys to see a match.
            for (epoch_lookup_index, epoch_group_key) in epoch_group_keys.iter().enumerate() {
                // Construct the lift.
                let lift = Lift::new(
                    account_key,
                    epoch_group_key.to_owned(),
                    Some(txin_outpoint),
                    Some(txin_txout.value.to_sat()),
                );

                // Return the script pubkey of the lift.
                let lift_spk = match lift.taproot() {
                    Some(taproot) => match taproot.spk() {
                        Some(spk) => spk,
                        None => {
                            // Unable to re-construct `Lift` at the current transaction input iterator position.
                            let input_iter_position = txholder.input_iter_position();
                            return Err(CPEDecodingError::LiftupCPEDecodingError(
                                LiftupCPEDecodingError::LiftReconstructionErrAtInputIter(
                                    input_iter_position,
                                ),
                            ));
                        }
                    },
                    None => {
                        // Unable to re-construct `Lift` at the current transaction input iterator position.
                        let input_iter_position = txholder.input_iter_position();
                        return Err(CPEDecodingError::LiftupCPEDecodingError(
                            LiftupCPEDecodingError::LiftReconstructionErrAtInputIter(
                                input_iter_position,
                            ),
                        ));
                    }
                };

                // Check if the script pubkey matches.
                match txin_spk == lift_spk {
                    true => {
                        lifts.push(lift);

                        // Iterate TxHolder inputs by one.
                        {
                            let _ = txholder.iterate_input();
                        }

                        break;
                    }
                    false => {
                        // Check if this is the last for loop iteration.
                        match epoch_lookup_index == epoch_group_keys.len() - 1 {
                            true => {
                                // Unable to find a matching `Lift` at the current transaction input iterator position.
                                let input_iter_position = txholder.input_iter_position();
                                return Err(CPEDecodingError::LiftupCPEDecodingError(
                                    LiftupCPEDecodingError::NoMatchingLiftAtInputIter(
                                        input_iter_position,
                                    ),
                                ));
                            }
                            false => {
                                // If this is not the last iteration, continue to the next epoch group key.
                                continue;
                            }
                        }
                    }
                }
            }
        }

        // Construct the `Liftup`.
        let liftup = Liftup {
            lift_prevtxos: lifts,
        };

        // Return the `Liftup`.
        Ok(liftup)
    }
}

impl Sighash for Liftup {
    fn sighash(&self) -> [u8; 32] {
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

        preimage.hash(Some(HashTag::SighashCombinator))
    }
}

impl CompactPayloadEncoding for Liftup {
    fn encode_cpe(&self) -> bit_vec::BitVec {
        // Initialize empty bit vector.
        let mut bits = BitVec::new();

        // Represent the number of lifts as ShortVal.
        let num_lifts = self.num_lifts();
        let num_lifts_shortval = ShortVal::new(num_lifts as u32);

        // Encode the number of lifts.
        bits.extend(num_lifts_shortval.encode_cpe());

        // That's it. We're not encoding the lifts themselves.
        // They are read directly from the on-chain transaction.

        bits
    }
}
