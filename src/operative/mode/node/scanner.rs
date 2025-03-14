use crate::{
    baked::{MAINNET_LIFT_SCAN_HEIGHT_START, SIGNET_LIFT_SCAN_HEIGHT_START},
    key::KeyHolder,
    rpc::bitcoin_rpc::{get_block, get_chain_height},
    rpcholder::RPCHolder,
    taproot::P2TR,
    txn::outpoint::Outpoint,
    txo::lift::Lift,
    Network, EPOCH_DIRECTORY, LIFT_WALLET,
};
use bitcoincore_rpc::bitcoin::hashes::Hash;
use colored::Colorize;
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

pub async fn scan_lifts(
    network: Network,
    key_holder: &KeyHolder,
    rpc_holder: &RPCHolder,
    epoch_dir: &EPOCH_DIRECTORY,
    lift_wallet: &LIFT_WALLET,
) {
    let sync_start_height = match network {
        Network::Signet => SIGNET_LIFT_SCAN_HEIGHT_START,
        Network::Mainnet => MAINNET_LIFT_SCAN_HEIGHT_START,
    };

    loop {
        let (wallet_sync_height, self_lifts) = {
            let _lift_wallet = lift_wallet.lock().await;
            (_lift_wallet.height(), _lift_wallet.set())
        };

        // Retrieve chain height.
        let chain_height = {
            match get_chain_height(rpc_holder) {
                Ok(tip) => tip,
                Err(err) => {
                    println!("{} {}", "Error retrieving chain tip: ".red(), err);
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
            }
        };

        println!("chain_height: {}", chain_height);

        match wallet_sync_height == chain_height {
            true => {
                // Lift wallet is fully synced.
                sleep(Duration::from_secs(10)).await;
            }
            false => {
                // Lift wallet is not fully synced.
                let height_to_sync = match wallet_sync_height < sync_start_height {
                    true => sync_start_height,
                    false => wallet_sync_height + 1,
                };

                let block = match get_block(rpc_holder, height_to_sync) {
                    Ok(block) => block,
                    Err(err) => {
                        println!("{} {}", "Error retrieving block: ".red(), err);
                        sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                };

                let lift_spks_to_scan = match lifts_spks_to_scan(key_holder, epoch_dir).await {
                    Some(spks) => spks,
                    None => {
                        println!("{}", "Unexpected retrieving lift spks error.".red());
                        sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                };

                // Scan block..
                for transaction in block.txdata.iter() {
                    let inputs = transaction.input.clone();
                    let outputs = transaction.output.clone();

                    // Remove spent lifts from wallet.
                    for input in inputs.iter() {
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
                                        let mut _lift_wallet = lift_wallet.lock().await;
                                        _lift_wallet.remove(lift);
                                    }
                                }
                            }
                        }
                    }

                    // Add new lifts to wallet.
                    for (index, output) in outputs.iter().enumerate() {
                        let txn_output_spk = output.script_pubkey.as_bytes().to_vec();

                        // Compare to lift spks to scan.
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

                                let lift =
                                    Lift::new(self_key, operator_key, Some(outpoint), Some(value));

                                // Add to lift wallet.
                                {
                                    let mut _lift_wallet = lift_wallet.lock().await;
                                    _lift_wallet.insert(&lift);
                                }
                            }
                        }
                    }
                }

                // Set the new Lift wallet height.
                {
                    let mut _lift_wallet = lift_wallet.lock().await;
                    _lift_wallet.set_height(height_to_sync);
                }
            }
        }
    }
}
