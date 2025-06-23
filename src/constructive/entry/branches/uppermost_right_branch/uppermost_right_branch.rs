use crate::constructive::entry::combinator::combinators::{
    add::Add, call::call::Call, deploy::Deploy, r#move::Move, reserved::Reserved, revive::Revive,
    sub::Sub, swapout::Swapout,
};
use crate::constructive::entry::{
    branches::uppermost_right_branch::{
        transactive_branch::transactive_branch::TransactiveBranch,
        upper_right_branch::{
            liquidity_branch::liquidity_branch::LiquidityBranch,
            right_branch::{
                lower_left_branch::lower_left_branch::LowerLeftBranch,
                lower_right_branch::{
                    lower_right_branch::LowerRightBranch,
                    recovery_branch::recovery_branch::RecoveryBranch,
                },
                right_branch::RightBranch,
            },
            upper_right_branch::UpperRightBranch,
        },
    },
    combinator::{combinator::Combinator, combinators::claim::Claim},
};
use serde::{Deserialize, Serialize};

/// The uppermost right branch of an entry.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UppermostRightBranch {
    TransactiveBranch(TransactiveBranch),
    UpperRightBranch(UpperRightBranch),
}

impl UppermostRightBranch {
    /// Create a branch containing a `Move` combinator.
    pub fn new_move(r#move: Move) -> Self {
        Self::TransactiveBranch(TransactiveBranch::Move(r#move))
    }

    /// Create a branch containing a `Call` combinator.
    pub fn new_call(call: Call) -> Self {
        Self::TransactiveBranch(TransactiveBranch::Call(call))
    }

    /// Create a branch containing a `Add` combinator.
    pub fn new_add(add: Add) -> Self {
        Self::UpperRightBranch(UpperRightBranch::LiquidityBranch(LiquidityBranch::Add(add)))
    }

    /// Create a branch containing a `Sub` combinator.
    pub fn new_sub(sub: Sub) -> Self {
        Self::UpperRightBranch(UpperRightBranch::LiquidityBranch(LiquidityBranch::Sub(sub)))
    }

    /// Create a branch containing a `Deploy` combinator.
    pub fn new_deploy(deploy: Deploy) -> Self {
        Self::UpperRightBranch(UpperRightBranch::RightBranch(RightBranch::LowerLeftBranch(
            LowerLeftBranch::Deploy(deploy),
        )))
    }

    /// Create a branch containing a `Swapout` combinator.
    pub fn new_swapout(swapout: Swapout) -> Self {
        Self::UpperRightBranch(UpperRightBranch::RightBranch(RightBranch::LowerLeftBranch(
            LowerLeftBranch::Swapout(swapout),
        )))
    }

    /// Create a branch containing a `Revive` combinator.
    pub fn new_revive(revive: Revive) -> Self {
        Self::UpperRightBranch(UpperRightBranch::RightBranch(
            RightBranch::LowerRightBranch(LowerRightBranch::RecoveryBranch(
                RecoveryBranch::Revive(revive),
            )),
        ))
    }

    /// Create a branch containing a `Claim` combinator.
    pub fn new_claim(claim: Claim) -> Self {
        Self::UpperRightBranch(UpperRightBranch::RightBranch(
            RightBranch::LowerRightBranch(LowerRightBranch::RecoveryBranch(RecoveryBranch::Claim(
                claim,
            ))),
        ))
    }

    /// Create a branch containing a `Reserved` combinator.
    pub fn new_reserved(reserved: Reserved) -> Self {
        Self::UpperRightBranch(UpperRightBranch::RightBranch(
            RightBranch::LowerRightBranch(LowerRightBranch::Reserved(reserved)),
        ))
    }

    /// Returns the main combinator of the branch.
    pub fn main_combinator(&self) -> Combinator {
        match self {
            Self::TransactiveBranch(transactive_branch) => match transactive_branch {
                TransactiveBranch::Move(r#move) => Combinator::Move(r#move.clone()),
                TransactiveBranch::Call(call) => Combinator::Call(call.clone()),
            },
            Self::UpperRightBranch(upper_right_branch) => match upper_right_branch {
                // Liquidity branch.
                UpperRightBranch::LiquidityBranch(liquidity_branch) => match liquidity_branch {
                    LiquidityBranch::Add(add) => Combinator::Add(add.clone()),
                    LiquidityBranch::Sub(sub) => Combinator::Sub(sub.clone()),
                },
                // Right branch.
                UpperRightBranch::RightBranch(right_branch) => match right_branch {
                    // Lower left branch.
                    RightBranch::LowerLeftBranch(lower_left_branch) => match lower_left_branch {
                        LowerLeftBranch::Deploy(deploy) => Combinator::Deploy(deploy.clone()),
                        LowerLeftBranch::Swapout(swapout) => Combinator::Swapout(swapout.clone()),
                    },
                    // Lower right branch.
                    RightBranch::LowerRightBranch(lower_right_branch) => match lower_right_branch {
                        // Recovery branch.
                        LowerRightBranch::RecoveryBranch(recovery_branch) => {
                            match recovery_branch {
                                RecoveryBranch::Revive(revive) => {
                                    Combinator::Revive(revive.clone())
                                }
                                RecoveryBranch::Claim(claim) => Combinator::Claim(claim.clone()),
                            }
                        }
                        LowerRightBranch::Reserved(reserved) => {
                            Combinator::Reserved(reserved.clone())
                        }
                    },
                },
            },
        }
    }
}
