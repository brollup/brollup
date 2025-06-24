use crate::constructive::{
    entity::contract::contract::Contract, valtype::val::short_val::short_val::ShortVal,
};
use bit_vec::BitVec;

impl Contract {
    /// Encodes the `Contract` as a bit vector.
    pub fn encode_cpe(&self) -> BitVec {
        // Initialize the bitvec.
        let mut bits = BitVec::new();

        // Get rank. Returns None if the contract has no given rank.
        let rank = self.rank.unwrap_or(ShortVal::new(0));

        // Extend rank bits.
        bits.extend(rank.encode_cpe());

        // Return the bits.
        bits
    }
}
