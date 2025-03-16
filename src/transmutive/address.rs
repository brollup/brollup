use crate::Network;
use bech32::{segwit, Hrp};

type ScriptPubKey = Vec<u8>;

/// Returns the Human-Readable Part (HRP) for a given network.
///
/// This function takes a network type and returns the corresponding HRP.
///
/// # Arguments
///

fn hrp_from_network(network: Network) -> Option<Hrp> {
    match network {
        Network::Signet => Hrp::parse("tb").ok(),
        Network::Mainnet => Hrp::parse("bc").ok(),
    }
}

/// Encodes a Taproot key into a Bech32-encoded Bitcoin address.
///
/// This function takes a network type and a Taproot key, and returns the corresponding
/// Bech32-encoded Bitcoin address if the key is valid.
///
/// # Arguments
///
pub fn encode_p2tr(network: Network, taproot_key: [u8; 32]) -> Option<String> {
    let hrp = hrp_from_network(network)?;
    let address = segwit::encode(hrp, segwit::VERSION_1, &taproot_key).ok()?;

    Some(address)
}

/// Encodes a P2WSH witness program into a Bech32-encoded Bitcoin address.
///
/// This function takes a network type and a P2WSH witness program, and returns the corresponding
/// Bech32-encoded Bitcoin address if the witness program is valid.
///
/// # Arguments
///
pub fn encode_p2wsh(network: Network, witness_program: [u8; 32]) -> Option<String> {
    let hrp = hrp_from_network(network)?;
    let address = segwit::encode(hrp, segwit::VERSION_0, &witness_program).ok()?;

    Some(address)
}

/// Encodes a P2WPKH witness program into a Bech32-encoded Bitcoin address.
///
/// This function takes a network type and a P2WPKH witness program, and returns the corresponding
/// Bech32-encoded Bitcoin address if the witness program is valid.
///
/// # Arguments
///
pub fn encode_p2wpkh(network: Network, witness_program: [u8; 20]) -> Option<String> {
    let hrp = hrp_from_network(network)?;
    let address = segwit::encode(hrp, segwit::VERSION_0, &witness_program).ok()?;

    Some(address)
}

/// Decodes a Bech32-encoded Bitcoin address into a ScriptPubKey.
///
/// This function takes a Bitcoin address and a network type, and returns the corresponding
/// ScriptPubKey if the address is valid. Legacy addresses are not supported.
///
/// # Arguments
///
/// * `network` - The network type of the address.
/// * `address` - The Bech32-encoded Bitcoin address to decode.
///
/// # Returns
///

pub fn address_to_spk(network: Network, address: &str) -> Option<ScriptPubKey> {
    let mut spk = Vec::<u8>::new();

    let (hrp, version, program) = match segwit::decode(&address) {
        Ok(result) => result,
        Err(_) => return None,
    };

    // Check if the network is valid
    match network {
        Network::Signet => {
            if hrp != Hrp::parse("tb").expect("invalid hrp") {
                return None;
            }
        }
        Network::Mainnet => {
            if hrp != Hrp::parse("bc").expect("invalid hrp") {
                return None;
            }
        }
    }

    // Match the version
    match version {
        segwit::VERSION_0 => {
            // Segwit version 0
            spk.push(0x00);

            match program.len() {
                20 => {
                    // P2WPKH
                    spk.push(0x14);
                    spk.extend(&program);
                }
                32 => {
                    // P2WSH
                    spk.push(0x20);
                    spk.extend(&program);
                }
                _ => {
                    return None;
                }
            }
        }
        segwit::VERSION_1 => {
            // Segwit version 1
            spk.push(0x51);

            match program.len() {
                32 => {
                    // P2TR
                    spk.push(0x20);
                    spk.extend(&program);
                }
                _ => {
                    return None;
                }
            }
        }
        _ => {
            return None;
        }
    }

    Some(spk)
}
