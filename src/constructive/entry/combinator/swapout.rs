use crate::{
    constructive::entity::account::Account,
    transmutative::{
        hash::{Hash, HashTag},
        secp::authenticable::AuthSighash,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SwapoutType {
    P2WPKH([u8; 20]),
    P2WSH([u8; 32]),
    P2TR([u8; 32]),
}

/// A swapout is a transaction output that contains a scriptPubKey and an amount.
/// Legacy addresses are not supported.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Swapout {
    account: Account,
    amount: u32,
    swapout_type: SwapoutType,
}

impl Swapout {
    /// Creates a new swapout for a P2TR address.
    pub fn new_p2tr(account: Account, amount: u32, taproot_key: [u8; 32]) -> Swapout {
        Swapout {
            account,
            amount,
            swapout_type: SwapoutType::P2TR(taproot_key),
        }
    }

    /// Creates a new swapout for a P2WSH address.
    pub fn new_p2wsh(account: Account, amount: u32, witness_program: [u8; 32]) -> Swapout {
        Swapout {
            account,
            amount,
            swapout_type: SwapoutType::P2WSH(witness_program),
        }
    }

    /// Creates a new swapout for a P2WPKH address.
    pub fn new_p2wpkh(account: Account, amount: u32, witness_program: [u8; 20]) -> Swapout {
        Swapout {
            account,
            amount,
            swapout_type: SwapoutType::P2WPKH(witness_program),
        }
    }

    /// Creates a new swapout from a scriptPubKey.
    pub fn from_spk(account: Account, amount: u32, spk: Vec<u8>) -> Option<Swapout> {
        let swapout_type = match spk.len() {
            22 => SwapoutType::P2WPKH(spk[2..].try_into().ok()?),
            34 => match spk[0] {
                0x00 => SwapoutType::P2WSH(spk[2..].try_into().ok()?),
                0x51 => SwapoutType::P2TR(spk[2..].try_into().ok()?),
                _ => return None,
            },
            _ => return None,
        };

        Some(Swapout {
            account,
            amount,
            swapout_type,
        })
    }

    /// Returns the witness version for the swapout.
    pub fn witness_version(&self) -> u8 {
        match &self.swapout_type {
            SwapoutType::P2WPKH(_) => 0,
            SwapoutType::P2WSH(_) => 0,
            SwapoutType::P2TR(_) => 1,
        }
    }

    /// Returns the witness program for the swapout.
    pub fn witness_program(&self) -> &[u8] {
        match &self.swapout_type {
            SwapoutType::P2WPKH(witness_program) => witness_program,
            SwapoutType::P2WSH(witness_program) => witness_program,
            SwapoutType::P2TR(witness_program) => witness_program,
        }
    }

    /// Returns the scriptPubKey for the swapout.
    pub fn spk(&self) -> Vec<u8> {
        let mut spk = Vec::<u8>::new();

        match &self.swapout_type {
            SwapoutType::P2WPKH(witness_program) => {
                spk.push(0x00);
                spk.push(0x14);
                spk.extend(witness_program);
            }
            SwapoutType::P2WSH(witness_program) => {
                spk.push(0x00);
                spk.push(0x20);
                spk.extend(witness_program);
            }
            SwapoutType::P2TR(witness_program) => {
                spk.push(0x51);
                spk.push(0x20);
                spk.extend(witness_program);
            }
        }

        spk
    }

    /// Returns the account for the swapout.
    pub fn account(&self) -> Account {
        self.account
    }

    /// Returns the amount for the swapout.
    pub fn amount(&self) -> u32 {
        self.amount
    }

    /// Returns the swapout type for the swapout.
    pub fn swapout_type(&self) -> SwapoutType {
        self.swapout_type.clone()
    }

    /// Serializes the swapout to a byte vector.
    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    /// Validates the account for the swapout.
    ///
    /// This function checks if the account key matches the account key in the swapout.
    pub fn validate_account(&self, account: Account) -> bool {
        self.account.key() == account.key()
    }
}

impl AuthSighash for Swapout {
    fn auth_sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        preimage.extend(self.account.key().serialize_xonly());
        preimage.extend(self.amount.to_le_bytes());
        preimage.extend(self.witness_program());

        preimage.hash(Some(HashTag::SighashCombinator))
    }
}
