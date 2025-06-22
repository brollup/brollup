use crate::constructive::entry::branches::uppermost_right_branch::upper_right_branch::right_branch::{lower_left_branch::lower_left_branch::LowerLeftBranch, lower_right_branch::lower_right_branch::LowerRightBranch};
use serde::{Deserialize, Serialize};

/// The right branch of an entry. Descend from the upper right branch.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RightBranch {
    LowerLeftBranch(LowerLeftBranch),
    LowerRightBranch(LowerRightBranch),
}
