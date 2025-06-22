use crate::constructive::entry::{branches::uppermost_right_branch::upper_right_branch::right_branch::lower_right_branch::recovery_branch::recovery_branch::RecoveryBranch, combinator::combinators::reserved::Reserved};
use serde::{Deserialize, Serialize};

/// The lower right branch of an entry. Descend from the right branch.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LowerRightBranch {
    RecoveryBranch(RecoveryBranch),
    Reserved(Reserved),
}
