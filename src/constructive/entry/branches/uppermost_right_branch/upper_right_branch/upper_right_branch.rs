use crate::constructive::entry::branches::uppermost_right_branch::upper_right_branch::{
    liquidity_branch::liquidity_branch::LiquidityBranch, right_branch::right_branch::RightBranch,
};
use serde::{Deserialize, Serialize};

/// The uppermost right branch of an entry. Descend from the uppermost right branch.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UpperRightBranch {
    LiquidityBranch(LiquidityBranch),
    RightBranch(RightBranch),
}
