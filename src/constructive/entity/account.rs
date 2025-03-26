use crate::{
    cpe::{AccountCPEDecodingError, CPEDecodingError, CompactPayloadEncoding},
    registery::account_registery::ACCOUNT_REGISTERY,
    valtype::short_val::ShortVal,
};
use async_trait::async_trait;
use bit_vec::BitVec;
use secp::Point;
use serde::{Deserialize, Serialize};

/// Represents an account; a user of the system.
#[derive(Clone, Copy, Serialize, Deserialize, Hash, Debug)]
pub struct Account {
    key: Point,
    registery_index: Option<ShortVal>,
}

impl Account {
    /// Creates a new account.
    pub fn new(key: Point, registery_index: Option<u32>) -> Option<Account> {
        let is_odd: bool = key.parity().into();

        if is_odd {
            return None;
        }

        // Convert the registery index to a ShortVal.
        let registery_index = match registery_index {
            Some(index) => Some(ShortVal::new(index)),
            None => None,
        };

        let account = Account {
            key,
            registery_index,
        };

        Some(account)
    }

    /// Sets the registery index of the account.
    pub fn set_registery_index(&mut self, registery_index: u32) {
        self.registery_index = Some(ShortVal::new(registery_index));
    }

    /// Returns the key of the account.
    pub fn key(&self) -> Point {
        self.key
    }

    /// Returns the registery index of the account.
    pub fn registery_index(&self) -> Option<u32> {
        Some(self.registery_index?.value())
    }

    /// Returns true if the key is odd.
    pub fn is_odd_key(&self) -> bool {
        self.key.parity().into()
    }

    /// Serializes the account.
    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    /// Compact payload decoding for `Account`.
    /// Decodes an `Account` from a bit stream.  
    pub async fn decode_cpe<'a>(
        bit_stream: &mut bit_vec::Iter<'a>,
        account_registery: &ACCOUNT_REGISTERY,
    ) -> Result<Account, CPEDecodingError> {
        // Check if the account is registered.
        let is_registered = bit_stream
            .next()
            .ok_or(CPEDecodingError::AccountCPEDecodingError(
                AccountCPEDecodingError::FailedToIterateIsRegisteredBit,
            ))?;

        match is_registered {
            true => {
                // Account is registered.

                // Decode registery index.
                let registery_index = ShortVal::decode_cpe(bit_stream).map_err(|_| {
                    CPEDecodingError::AccountCPEDecodingError(
                        AccountCPEDecodingError::FailedToDecodeRegisteryIndex,
                    )
                })?;

                // Retrieve the account given registery index.
                let account = {
                    let _account_registery = account_registery.lock().await;
                    _account_registery
                        .account_by_index(registery_index.value())
                        .ok_or(CPEDecodingError::AccountCPEDecodingError(
                            AccountCPEDecodingError::UnableToLocateAccountKeyGivenIndex(
                                registery_index.value(),
                            ),
                        ))?
                };

                // Return the `Account`.
                Ok(account)
            }
            false => {
                // Account is not registered. This means we need to construct a new key and register it.

                // Collect exactly 256 bits for the public key.
                let public_key_bits: BitVec = bit_stream.by_ref().take(256).collect();

                // Ensure the collected bits are the correct length.
                if public_key_bits.len() != 256 {
                    return Err(CPEDecodingError::AccountCPEDecodingError(
                        AccountCPEDecodingError::UnableToConstructNewKey,
                    ));
                }

                // Convert public key bits to an even public key bytes.
                let mut public_key_bytes = vec![0x02];
                public_key_bytes.extend(public_key_bits.to_bytes());

                // Construct the public key.
                let public_key = Point::from_slice(&public_key_bytes).map_err(|_| {
                    CPEDecodingError::AccountCPEDecodingError(
                        AccountCPEDecodingError::UnableToConstructNewKey,
                    )
                })?;

                // Check if the key is already registered.
                let is_registered = {
                    let _account_registery = account_registery.lock().await;
                    _account_registery.is_registered(public_key)
                };

                // If the key is already registered, return an error.
                if is_registered {
                    return Err(CPEDecodingError::AccountCPEDecodingError(
                        AccountCPEDecodingError::AccountKeyAlreadyRegistered(
                            public_key.serialize_xonly(),
                        ),
                    ));
                }

                // Construct the unregistered account.
                let account = Account {
                    key: public_key,
                    registery_index: None,
                };

                // Return the `Account`.
                Ok(account)
            }
        }
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Account {}

#[async_trait]
impl CompactPayloadEncoding for Account {
    fn encode_cpe(&self) -> BitVec {
        let mut bits = BitVec::new();

        // Check registery status.
        match self.registery_index {
            Some(registery_index) => {
                // True for registered.
                bits.push(true);

                // Registery index bits.
                let registery_index_bits = registery_index.encode_cpe();

                // Extend registery index bits.
                bits.extend(registery_index_bits);
            }
            None => {
                // False for unregistered.
                bits.push(false);

                // Public key bits.
                let public_key_bits = BitVec::from_bytes(&self.key.serialize_xonly());

                // Extend public key bits.
                bits.extend(public_key_bits);
            }
        }

        bits
    }
}
