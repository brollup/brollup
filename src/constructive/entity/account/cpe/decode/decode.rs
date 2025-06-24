use crate::constructive::entity::account::account::Account;
use crate::constructive::entity::account::cpe::decode::decode_error::AccountCPEDecodingError;
use crate::constructive::valtype::val::short_val::short_val::ShortVal;
use crate::inscriptive::registery::account_registery::ACCOUNT_REGISTERY;
use bit_vec::BitVec;
use secp::Point;

impl Account {
    /// Decodes an `Account` from a bit stream.  
    pub async fn decode_cpe<'a>(
        bit_stream: &mut bit_vec::Iter<'a>,
        account_registery: &ACCOUNT_REGISTERY,
    ) -> Result<Account, AccountCPEDecodingError> {
        // Decode the rank value.
        let rank = ShortVal::decode_cpe(bit_stream)
            .map_err(|e| AccountCPEDecodingError::RankAsShortValDecodeError(e))?
            .value();

        // Match the rank value to determine if the account is registered or not.
        // If rank is zero, then we interpret this as an unregistered account, otherwise it is a registered account.
        match rank {
            0 => {
                // Unregistered account.

                // Collect exactly 256 bits for the public key.
                let public_key_bits: BitVec = bit_stream.by_ref().take(256).collect();

                // Ensure the collected bits are the correct length.
                if public_key_bits.len() != 256 {
                    return Err(AccountCPEDecodingError::PublicKeyBitsLengthError);
                }

                // Convert public key bits to an even public key bytes.
                let mut public_key_bytes = vec![0x02];
                public_key_bytes.extend(public_key_bits.to_bytes());

                // Construct the public key.
                let public_key = Point::from_slice(&public_key_bytes)
                    .map_err(|_| AccountCPEDecodingError::PublicKeyPointFromSliceError)?;

                // Check if the key is already registered.
                let is_registered = {
                    let _account_registery = account_registery.lock().await;
                    _account_registery.is_registered(public_key)
                };

                // If the key is already registered, return an error.
                if is_registered {
                    return Err(AccountCPEDecodingError::KeyAlreadyRegisteredError);
                }

                // Construct the unregistered account.
                let account = Account {
                    key: public_key,
                    registery_index: None,
                    rank: None,
                };

                // Return the `Account`.
                return Ok(account);
            }
            _ => {
                // Registered account.

                // Retrieve the account given rank value.
                let account = {
                    let _account_registery = account_registery.lock().await;
                    _account_registery.account_by_rank(rank).ok_or(
                        AccountCPEDecodingError::FailedToLocateAccountGivenRank(rank),
                    )?
                };

                // Return the `Account`.
                return Ok(account);
            }
        }
    }
}
