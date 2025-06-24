use secp::Point;

use crate::{
    constructive::{
        entry::combinator::combinators::liftup::{
            cpe::decode::decode_error::LiftupCPEDecodingError, liftup::Liftup,
        },
        taproot::P2TR,
        txn::txholder::TxHolder,
        txo::lift::Lift,
        valtype::val::short_val::short_val::ShortVal,
    },
    inscriptive::epoch::dir::EPOCH_DIRECTORY,
};

impl Liftup {
    /// Compact payload decoding for `Liftup`.
    /// Decodes a `Liftup` from a bit stream and `TxHolder`.
    pub async fn decode_cpe<'a>(
        bit_stream: &mut bit_vec::Iter<'a>,
        txholder: &'a mut TxHolder,
        epoch_dir: &'a EPOCH_DIRECTORY,
        account_key: Point,
    ) -> Result<Liftup, LiftupCPEDecodingError> {
        // Decode the number of lifts.
        let num_lifts = ShortVal::decode_cpe(bit_stream)
            .map_err(|e| LiftupCPEDecodingError::NumLiftsCPEDecodeError(e))?;

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
                    return Err(LiftupCPEDecodingError::NoLiftAtInputIter(
                        input_iter_position,
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
                            return Err(LiftupCPEDecodingError::LiftReconstructionErrAtInputIter(
                                input_iter_position,
                            ));
                        }
                    },
                    None => {
                        // Unable to re-construct `Lift` at the current transaction input iterator position.
                        let input_iter_position = txholder.input_iter_position();
                        return Err(LiftupCPEDecodingError::LiftReconstructionErrAtInputIter(
                            input_iter_position,
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
                                return Err(LiftupCPEDecodingError::NoMatchingLiftAtInputIter(
                                    input_iter_position,
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
