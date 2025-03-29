use super::combinator::{
    add::Add, call::Call, claim::Claim, combinator::Combinator, deploy::Deploy, liftup::Liftup,
    r#move::Move, recharge::Recharge, reserved::Reserved, revive::Revive, sub::Sub,
    swapout::Swapout,
};
use crate::{
    constructive::entity::account::Account,
    transmutive::{
        hash::{Hash, HashTag},
        schnorr::Sighash,
    },
};
use serde::{Deserialize, Serialize};

/// The uppermost left branch of an entry.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UppermostLeftBranch {
    liftup: Option<Liftup>,
    recharge: Option<Recharge>,
}

impl UppermostLeftBranch {
    pub fn new(liftup: Option<Liftup>, recharge: Option<Recharge>) -> Self {
        Self { liftup, recharge }
    }

    pub fn new_liftup(liftup: Liftup) -> Self {
        Self {
            liftup: Some(liftup),
            recharge: None,
        }
    }

    pub fn new_recharge(recharge: Recharge) -> Self {
        Self {
            liftup: None,
            recharge: Some(recharge),
        }
    }

    pub fn new_liftup_and_recharge(liftup: Liftup, recharge: Recharge) -> Self {
        Self {
            liftup: Some(liftup),
            recharge: Some(recharge),
        }
    }

    pub fn liftup(&self) -> Option<Liftup> {
        self.liftup.clone()
    }

    pub fn recharge(&self) -> Option<Recharge> {
        self.recharge.clone()
    }
}

/// The uppermost right branch of an entry.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UppermostRightBranch {
    TransactiveBranch(TransactiveBranch),
    UpperRightBranch(UpperRightBranch),
}

/// The transactive branch of an entry. Descend from the uppermost right branch.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactiveBranch {
    Move(Move),
    Call(Call),
}

/// The uppermost right branch of an entry. Descend from the uppermost right branch.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UpperRightBranch {
    LiquidityBranch(LiquidityBranch),
    RightBranch(RightBranch),
}

/// The liquidity branch of an entry. Descend from the upper right branch.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LiquidityBranch {
    Add(Add),
    Sub(Sub),
}

/// The right branch of an entry. Descend from the upper right branch.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RightBranch {
    LowerLeftBranch(LowerLeftBranch),
    LowerRightBranch(LowerRightBranch),
}

/// The lower left branch of an entry. Descend from the right branch.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LowerLeftBranch {
    Deploy(Deploy),
    Swapout(Swapout),
}

/// The lower right branch of an entry. Descend from the right branch.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LowerRightBranch {
    RecoveryBranch(RecoveryBranch),
    Reserved(Reserved),
}

/// The recovery branch of an entry. Descend from the lower right branch.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecoveryBranch {
    Revive(Revive),
    Claim(Claim),
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

/// The entry represents a transaction, containing one or more combinators.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    account: Account,
    uppermost_left_branch: Option<UppermostLeftBranch>,
    uppermost_right_branch: Option<UppermostRightBranch>,
}

impl Entry {
    /// Creates a new entry.
    fn new(
        account: Account,
        uppermost_left_branch: Option<UppermostLeftBranch>,
        uppermost_right_branch: Option<UppermostRightBranch>,
    ) -> Entry {
        Self {
            account,
            uppermost_left_branch,
            uppermost_right_branch,
        }
    }

    // Liftup and/or recharge ONLY.
    pub fn new_nop(account: Account, liftup: Option<Liftup>, recharge: Option<Recharge>) -> Entry {
        let uppermost_left_branch = {
            if liftup.is_some() && recharge.is_some() {
                Some(UppermostLeftBranch::new(liftup, recharge))
            } else {
                None
            }
        };

        let uppermost_right_branch = None;
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    /// Creates a new move entry.
    pub fn new_move(
        account: Account,
        r#move: Move,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
    ) -> Entry {
        let uppermost_left_branch = {
            if liftup.is_some() || recharge.is_some() {
                Some(UppermostLeftBranch::new(liftup, recharge))
            } else {
                None
            }
        };

        let uppermost_right_branch = Some(UppermostRightBranch::TransactiveBranch(
            TransactiveBranch::Move(r#move),
        ));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    pub fn new_call(
        account: Account,
        call: Call,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
    ) -> Entry {
        let uppermost_left_branch = {
            if liftup.is_some() || recharge.is_some() {
                Some(UppermostLeftBranch::new(liftup, recharge))
            } else {
                None
            }
        };
        let uppermost_right_branch = Some(UppermostRightBranch::TransactiveBranch(
            TransactiveBranch::Call(call),
        ));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    /// Creates a new add entry.
    pub fn new_add(
        account: Account,
        add: Add,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
    ) -> Entry {
        let uppermost_left_branch = {
            if liftup.is_some() || recharge.is_some() {
                Some(UppermostLeftBranch::new(liftup, recharge))
            } else {
                None
            }
        };
        let uppermost_right_branch = Some(UppermostRightBranch::UpperRightBranch(
            UpperRightBranch::LiquidityBranch(LiquidityBranch::Add(add)),
        ));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    /// Creates a new sub entry.
    pub fn new_sub(
        account: Account,
        sub: Sub,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
    ) -> Entry {
        let uppermost_left_branch = {
            if liftup.is_some() || recharge.is_some() {
                Some(UppermostLeftBranch::new(liftup, recharge))
            } else {
                None
            }
        };
        let uppermost_right_branch = Some(UppermostRightBranch::UpperRightBranch(
            UpperRightBranch::LiquidityBranch(LiquidityBranch::Sub(sub)),
        ));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    /// Creates a new swapout entry.
    pub fn new_swapout(
        account: Account,
        swapout: Swapout,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
    ) -> Entry {
        let uppermost_left_branch = {
            if liftup.is_some() || recharge.is_some() {
                Some(UppermostLeftBranch::new(liftup, recharge))
            } else {
                None
            }
        };
        let uppermost_right_branch = Some(UppermostRightBranch::UpperRightBranch(
            UpperRightBranch::RightBranch(RightBranch::LowerLeftBranch(LowerLeftBranch::Swapout(
                swapout,
            ))),
        ));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    /// Creates a new reserved entry.
    pub fn new_reserved(
        account: Account,
        reserved: Reserved,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
    ) -> Entry {
        let uppermost_left_branch = {
            if liftup.is_some() || recharge.is_some() {
                Some(UppermostLeftBranch::new(liftup, recharge))
            } else {
                None
            }
        };
        let uppermost_right_branch = Some(UppermostRightBranch::UpperRightBranch(
            UpperRightBranch::RightBranch(RightBranch::LowerRightBranch(
                LowerRightBranch::Reserved(reserved),
            )),
        ));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    /// Returns the account of the entry.
    pub fn account(&self) -> Account {
        self.account.clone()
    }

    /// Returns the liftup of the entry.
    pub fn liftup(&self) -> Option<Liftup> {
        let uppermost_left_branch = match &self.uppermost_left_branch {
            Some(uppermost_left_branch) => uppermost_left_branch,
            None => return None,
        };

        uppermost_left_branch.liftup()
    }

    /// Returns the recharge of the entry.
    pub fn recharge(&self) -> Option<Recharge> {
        let uppermost_left_branch = match &self.uppermost_left_branch {
            Some(uppermost_left_branch) => uppermost_left_branch,
            None => return None,
        };

        uppermost_left_branch.recharge()
    }

    /// Returns the main combinator of the entry.
    pub fn main_combinator(&self) -> Option<Combinator> {
        let uppermost_right_branch = match &self.uppermost_right_branch {
            Some(uppermost_right_branch) => uppermost_right_branch,
            None => return None,
        };

        Some(uppermost_right_branch.main_combinator())
    }

    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    /// Validates the account of the entry.
    pub fn validate_account(&self) -> bool {
        let account = self.account();

        if let None = self.uppermost_left_branch {
            if let None = self.uppermost_right_branch {
                return false;
            }
        }

        if let Some(uppermost_left_branch) = &self.uppermost_left_branch {
            if let Some(liftup) = &uppermost_left_branch.liftup {
                if !liftup.validate_account(account) {
                    return false;
                }
            }

            if let Some(recharge) = &uppermost_left_branch.recharge {
                if !recharge.validate_account(account) {
                    return false;
                }
            }
        }

        if let Some(main_combinator) = self.main_combinator() {
            match main_combinator {
                Combinator::Move(r#move) => {
                    if !r#move.validate_account(account) {
                        return false;
                    }
                }
                Combinator::Call(call) => {
                    if !call.validate_account(account) {
                        return false;
                    }
                }
                Combinator::Add(add) => {
                    if !add.validate_account(account) {
                        return false;
                    }
                }
                Combinator::Sub(sub) => {
                    if !sub.validate_account(account) {
                        return false;
                    }
                }
                Combinator::Deploy(deploy) => {
                    if !deploy.validate_account(account) {
                        return false;
                    }
                }
                Combinator::Swapout(swapout) => {
                    if !swapout.validate_account(account) {
                        return false;
                    }
                }
                Combinator::Revive(revive) => {
                    if !revive.validate_account(account) {
                        return false;
                    }
                }
                Combinator::Claim(claim) => {
                    if !claim.validate_account(account) {
                        return false;
                    }
                }
                // Reserved fails the validation.
                Combinator::Reserved(_) => return false,
                // Main combinator cannot be of liftup or recharge.
                Combinator::Liftup(_) => return false,
                Combinator::Recharge(_) => return false,
            }
        }

        true
    }
}

impl Sighash for Entry {
    fn sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        // Account
        preimage.extend(self.account.key().serialize_xonly());

        match &self.uppermost_left_branch {
            Some(uppermost_left_branch) => {
                preimage.push(0x01);

                match &uppermost_left_branch.liftup {
                    Some(liftup) => {
                        preimage.push(0x01);
                        preimage.extend(liftup.sighash());
                    }
                    None => preimage.push(0x00),
                }

                match &uppermost_left_branch.recharge {
                    Some(recharge) => {
                        preimage.push(0x01);
                        preimage.extend(recharge.sighash());
                    }
                    None => preimage.push(0x00),
                }
            }
            None => preimage.push(0x00),
        }

        match &self.uppermost_right_branch {
            Some(uppermost_right_branch) => {
                preimage.push(0x01);

                match &uppermost_right_branch.main_combinator() {
                    Combinator::Move(r#move) => preimage.extend(r#move.sighash()),
                    Combinator::Call(call) => preimage.extend(call.sighash()),
                    Combinator::Add(add) => preimage.extend(add.sighash()),
                    Combinator::Sub(sub) => preimage.extend(sub.sighash()),
                    Combinator::Deploy(deploy) => preimage.extend(deploy.sighash()),
                    Combinator::Swapout(swapout) => preimage.extend(swapout.sighash()),
                    Combinator::Revive(revive) => preimage.extend(revive.sighash()),
                    Combinator::Claim(claim) => preimage.extend(claim.sighash()),
                    // Reserved is not covered.
                    Combinator::Reserved(_) => return [0xffu8; 32],
                    // Liftup and recharge belong to the uppermost left branch.
                    Combinator::Liftup(_) => return [0xffu8; 32],
                    Combinator::Recharge(_) => return [0xffu8; 32],
                }
            }
            None => preimage.push(0x00),
        }

        preimage.hash(Some(HashTag::SighashEntry))
    }
}
