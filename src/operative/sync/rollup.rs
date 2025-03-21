use crate::{
    baked,
    key::KeyHolder,
    registery::registery::REGISTERY,
    rpc::bitcoin_rpc::{get_block, get_chain_height},
    rpcholder::RPCHolder,
    taproot::P2TR,
    txn::outpoint::Outpoint,
    txo::lift::Lift,
    wallet::wallet::WALLET,
    Network, EPOCH_DIRECTORY, LP_DIRECTORY, ROLLUP_DIRECTORY,
};
use async_trait::async_trait;
use bitcoin::hashes::Hash;
use secp::Point;
use std::time::Duration;
use tokio::time::sleep;

type LiftSPK = Vec<u8>;

/// Returns the list of Taproot scriptpubkeys to scan.
pub async fn lifts_spks_to_scan(
    key_holder: &KeyHolder,
    epoch_dir: &EPOCH_DIRECTORY,
) -> Option<Vec<(LiftSPK, Point)>> {
    let mut spks = Vec::<(LiftSPK, Point)>::new();

    let self_key = key_holder.public_key();

    let group_keys = {
        let _epoch_dir = epoch_dir.lock().await;
        _epoch_dir.group_keys()
    };

    for operator_group_key in group_keys.iter() {
        let lift = Lift::new(self_key, operator_group_key.to_owned(), None, None);
        let taproot = lift.taproot()?;
        let spk = taproot.spk()?;

        spks.push((spk, operator_group_key.to_owned()));
    }

    Some(spks)
}

#[async_trait]
pub trait RollupSync {
    /// Continuously syncs the rollup.
    async fn sync(
        &self,
        network: Network,
        rpc_holder: &RPCHolder,
        key_holder: &KeyHolder,
        _epoch_dir: &EPOCH_DIRECTORY,
        _lp_dir: &LP_DIRECTORY,
        _registery: &REGISTERY,
        wallet: Option<&WALLET>,
    );

    /// Awaits the rollup to be synced to the latest Bitcoin chain tip.
    async fn await_sync(&self);
}

#[async_trait]
impl RollupSync for ROLLUP_DIRECTORY {
    async fn await_sync(&self) {
        loop {
            let is_fully_synced = {
                let _self = self.lock().await;
                _self.is_synced()
            };

            match is_fully_synced {
                true => break,
                false => sleep(Duration::from_secs(5)).await,
            }
        }
    }

    async fn sync(
        &self,
        network: Network,
        rpc_holder: &RPCHolder,
        key_holder: &KeyHolder,
        epoch_dir: &EPOCH_DIRECTORY,
        _lp_dir: &LP_DIRECTORY,
        _registery: &REGISTERY,
        wallet: Option<&WALLET>,
    ) {
        let mut synced: bool = false;

        let rollup_dir: &ROLLUP_DIRECTORY = self;

        let sync_start_height = match network {
            Network::Signet => baked::SIGNET_SYNC_START_HEIGHT,
            Network::Mainnet => baked::MAINNET_SYNC_START_HEIGHT,
        };

        loop {
            let rollup_bitcoin_sync_height = {
                let _rollup_dir = rollup_dir.lock().await;
                _rollup_dir.bitcoin_sync_height()
            };

            let self_lifts = {
                match wallet {
                    Some(wallet) => {
                        let lift_wallet = {
                            let _wallet = wallet.lock().await;
                            _wallet.lift_wallet()
                        };

                        let _lift_wallet = lift_wallet.lock().await;
                        _lift_wallet.lifts()
                    }
                    None => vec![],
                }
            };

            // Retrieve chain height.
            let chain_height = {
                match get_chain_height(rpc_holder) {
                    Ok(tip) => tip,
                    Err(_) => {
                        sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                }
            };

            match rollup_bitcoin_sync_height == chain_height {
                true => {
                    // Rollup is fully synced.
                    if !synced {
                        {
                            let mut _rollup_dir = rollup_dir.lock().await;
                            _rollup_dir.set_synced(true);
                        }
                        synced = true;
                    }

                    sleep(Duration::from_secs(10)).await;
                }
                false => {
                    // Rollup is not fully synced.
                    let height_to_sync = match rollup_bitcoin_sync_height < sync_start_height {
                        true => sync_start_height,
                        false => rollup_bitcoin_sync_height + 1,
                    };

                    let block = match get_block(rpc_holder, height_to_sync) {
                        Ok(block) => block,
                        Err(_) => {
                            sleep(Duration::from_secs(5)).await;
                            continue;
                        }
                    };

                    let lift_spks_to_scan = match wallet {
                        Some(_) => match lifts_spks_to_scan(key_holder, epoch_dir).await {
                            Some(spks) => spks,
                            None => vec![],
                        },
                        None => vec![],
                    };

                    // Scan block..
                    for transaction in block.txdata.iter() {
                        let inputs = transaction.input.clone();
                        let outputs = transaction.output.clone();

                        // Iterate over inputs.
                        for input in inputs.iter() {
                            // Remove spent lifts from wallet.
                            if let Some(wallet) = wallet {
                                let txn_input_outpoint = {
                                    let prev: [u8; 32] =
                                        input.previous_output.txid.to_raw_hash().to_byte_array();
                                    let vout = input.previous_output.vout;
                                    Outpoint::new(prev, vout)
                                };

                                // Compare to self lift outpoints.
                                for lift in self_lifts.iter() {
                                    if let Some(self_lift_outpoint) = lift.outpoint() {
                                        if txn_input_outpoint == self_lift_outpoint {
                                            // Remove from lift wallet.
                                            {
                                                let lift_wallet = {
                                                    let _wallet = wallet.lock().await;
                                                    _wallet.lift_wallet()
                                                };

                                                let mut _lift_wallet = lift_wallet.lock().await;
                                                _lift_wallet.remove_lift(lift);
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Iterate over outputs.
                        for (index, output) in outputs.iter().enumerate() {
                            let txn_output_spk = output.script_pubkey.as_bytes().to_vec();

                            // Compare to lift spks to scan.
                            if let Some(wallet) = wallet {
                                for (lift_spk, operator_group_key) in lift_spks_to_scan.iter() {
                                    if &txn_output_spk == lift_spk {
                                        let outpoint = {
                                            let txhash: [u8; 32] =
                                                transaction.compute_txid().to_byte_array();
                                            let vout = index as u32;
                                            Outpoint::new(txhash, vout)
                                        };

                                        let value = output.value.to_sat();

                                        let self_key = key_holder.public_key();
                                        let operator_key = operator_group_key.to_owned();

                                        let lift = Lift::new(
                                            self_key,
                                            operator_key,
                                            Some(outpoint),
                                            Some(value),
                                        );

                                        // Add to lift wallet.
                                        {
                                            let lift_wallet = {
                                                let _wallet = wallet.lock().await;
                                                _wallet.lift_wallet()
                                            };

                                            let mut _lift_wallet = lift_wallet.lock().await;
                                            _lift_wallet.insert_lift(&lift);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Set the new rollup bitcoin sync height.
                    {
                        let mut _rollup_dir = rollup_dir.lock().await;
                        _rollup_dir.set_bitcoin_sync_height(height_to_sync);
                    }

                    // TODO set the new rollup sync height.

                    println!("Synced height #{}.", height_to_sync);
                }
            }
        }
    }
}
