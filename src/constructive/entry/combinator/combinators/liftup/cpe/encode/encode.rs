use crate::constructive::{
    entry::combinator::combinators::liftup::liftup::Liftup,
    valtype::val::short_val::short_val::ShortVal,
};
use bit_vec::BitVec;

impl Liftup {
    pub fn encode_cpe(&self) -> BitVec {
        // Initialize empty bit vector.
        let mut bits = BitVec::new();

        // Represent the number of lifts as ShortVal.
        let num_lifts = self.num_lifts();
        let num_lifts_shortval = ShortVal::new(num_lifts as u32);

        // Encode the number of lifts.
        bits.extend(num_lifts_shortval.encode_cpe());

        // That's it. We're not encoding the lifts themselves.
        // They are read directly from the on-chain transaction.

        // Return the bits.
        bits
    }
}
