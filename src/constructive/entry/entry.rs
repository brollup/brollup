use crate::hash::{Hash, HashTag};
use crate::{
    combinator::{
        add::Add, call::Call, combinator::Combinator, liftup::Liftup, r#move::Move,
        recharge::Recharge, remove::Remove, reserved::Reserved, swapout::Swapout,
    },
    schnorr::Sighash,
    valtype::account::Account,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UppermostLeftBranch {
    liftup: Option<Liftup>,
    recharge: Option<Recharge>,
}

impl UppermostLeftBranch {
    pub fn new(liftup: Option<Liftup>, recharge: Option<Recharge>) -> Self {
        Self { liftup, recharge }
    }

    pub fn liftup(&self) -> Option<Liftup> {
        self.liftup.clone()
    }

    pub fn recharge(&self) -> Option<Recharge> {
        self.recharge.clone()
    }

    pub fn on(&self) -> bool {
        self.liftup.is_some() || self.recharge.is_some()
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Transact {
    Move(Move),
    Call(Call),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Liquidity {
    Add(Add),
    Remove(Remove),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RightBranch {
    Swapout(Swapout),
    Reserved(Reserved),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UpperRightBranch {
    Liquidity(Liquidity),
    RightBranch(RightBranch),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UppermostRightBranch {
    Transact(Transact),
    UpperRightBranch(UpperRightBranch),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    account: Account,
    uppermost_left_branch: Option<UppermostLeftBranch>,
    uppermost_right_branch: Option<UppermostRightBranch>,
}

impl Entry {
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
            if liftup.is_some() || recharge.is_some() {
                Some(UppermostLeftBranch::new(liftup, recharge))
            } else {
                None
            }
        };

        let uppermost_right_branch = None;
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

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

        let uppermost_right_branch = Some(UppermostRightBranch::Transact(Transact::Move(r#move)));
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
        let uppermost_right_branch = Some(UppermostRightBranch::Transact(Transact::Call(call)));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

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
            UpperRightBranch::Liquidity(Liquidity::Add(add)),
        ));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    pub fn new_remove(
        account: Account,
        remove: Remove,
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
            UpperRightBranch::Liquidity(Liquidity::Remove(remove)),
        ));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

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
            UpperRightBranch::RightBranch(RightBranch::Swapout(swapout)),
        ));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

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
            UpperRightBranch::RightBranch(RightBranch::Reserved(reserved)),
        ));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    pub fn account(&self) -> Account {
        self.account.clone()
    }

    pub fn liftup(&self) -> Option<Liftup> {
        let uppermost_left_branch = match &self.uppermost_left_branch {
            Some(uppermost_left_branch) => uppermost_left_branch,
            None => return None,
        };

        uppermost_left_branch.liftup()
    }

    pub fn recharge(&self) -> Option<Recharge> {
        let uppermost_left_branch = match &self.uppermost_left_branch {
            Some(uppermost_left_branch) => uppermost_left_branch,
            None => return None,
        };

        uppermost_left_branch.recharge()
    }

    pub fn main_combinator(&self) -> Option<Combinator> {
        let uppermost_right_branch = match &self.uppermost_right_branch {
            Some(uppermost_right_branch) => uppermost_right_branch,
            None => return None,
        };

        match uppermost_right_branch {
            UppermostRightBranch::Transact(transact) => match transact {
                Transact::Move(r#move) => Some(Combinator::Move(r#move.clone())),
                Transact::Call(call) => Some(Combinator::Call(call.clone())),
            },
            UppermostRightBranch::UpperRightBranch(upper_right_branch) => {
                match upper_right_branch {
                    UpperRightBranch::Liquidity(liquidity) => match liquidity {
                        Liquidity::Add(add) => Some(Combinator::Add(add.clone())),
                        Liquidity::Remove(remove) => Some(Combinator::Remove(remove.clone())),
                    },
                    UpperRightBranch::RightBranch(right_branch) => match right_branch {
                        RightBranch::Swapout(swapout) => Some(Combinator::Swapout(swapout.clone())),
                        RightBranch::Reserved(reserved) => {
                            Some(Combinator::Reserved(reserved.clone()))
                        }
                    },
                }
            }
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

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
                Combinator::Remove(remove) => {
                    if !remove.validate_account(account) {
                        return false;
                    }
                }
                Combinator::Swapout(swapout) => {
                    if !swapout.validate_account(account) {
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

                match &self.main_combinator() {
                    Some(main_combinator) => match main_combinator {
                        Combinator::Move(r#move) => preimage.extend(r#move.sighash()),
                        Combinator::Call(call) => preimage.extend(call.sighash()),
                        Combinator::Add(add) => preimage.extend(add.sighash()),
                        Combinator::Remove(remove) => preimage.extend(remove.sighash()),
                        Combinator::Swapout(swapout) => preimage.extend(swapout.sighash()),
                        Combinator::Reserved(_) => return [0xffu8; 32],
                        Combinator::Liftup(_) => return [0xffu8; 32],
                        Combinator::Recharge(_) => return [0xffu8; 32],
                    },
                    None => return [0xffu8; 32],
                }
            }
            None => preimage.push(0x00),
        }

        preimage.hash(Some(HashTag::SighashEntry))
    }
}
