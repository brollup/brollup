use crate::constructive::entry::combinator::combinators::recharge::recharge::Recharge;
use bit_vec::BitVec;

impl Recharge {
    /// Encodes the `Recharge` combinator into a compact bit vector.
    pub fn encode_cpe(&self) -> BitVec {
        // We encode nothing for the recharge combinator.
        // The decoder will retrieve *all* expired vtxos directly from the local storage.

        // Return an empty bit vector.
        BitVec::new()
    }
}
