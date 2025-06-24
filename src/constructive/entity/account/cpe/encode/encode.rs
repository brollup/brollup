use crate::constructive::entity::account::account::Account;
use crate::constructive::valtype::val::short_val::short_val::ShortVal;
use bit_vec::BitVec;

impl Account {
    /// Encodes the `Account` as a bit vector.
    pub fn encode_cpe(&self) -> BitVec {
        let mut bits = BitVec::new();

        // Match on the rank value.
        match self.rank {
            // If the rank is set, then we interpret this as a registered account.
            Some(rank) => {
                // Extend rank bits.
                bits.extend(rank.encode_cpe());
            }
            // If the rank is not set, then we interpret this as an unregistered account.
            None => {
                // Extend with the rank value zero.
                bits.extend(ShortVal::new(0).encode_cpe());

                // Public key bits.
                let public_key_bits = BitVec::from_bytes(&self.key.serialize_xonly());

                // Extend public key bits.
                bits.extend(public_key_bits);
            }
        };

        // Return the bits.
        bits
    }
}
