use crate::constructive::entry::combinator::combinators::add::Add;
use crate::constructive::entry::combinator::combinators::sub::Sub;
use serde::{Deserialize, Serialize};

/// The liquidity branch of an entry. Descend from the upper right branch.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LiquidityBranch {
    Add(Add),
    Sub(Sub),
}
