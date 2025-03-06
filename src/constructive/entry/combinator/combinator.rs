use serde::{Deserialize, Serialize};

use super::{
    add::Add, call::Call, liftup::Liftup, r#move::Move, recharge::Recharge, remove::Remove,
    reserved::Reserved, swapout::Swapout,
};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Combinator {
    Liftup(Liftup),
    Recharge(Recharge),
    Move(Move),
    Call(Call),
    Add(Add),
    Remove(Remove),
    Swapout(Swapout),
    Reserved(Reserved),
}

impl Combinator {
    pub fn new_liftup(liftup: Liftup) -> Combinator {
        Combinator::Liftup(liftup)
    }

    pub fn new_recharge(recharge: Recharge) -> Combinator {
        Combinator::Recharge(recharge)
    }

    pub fn new_move(r#move: Move) -> Combinator {
        Combinator::Move(r#move)
    }

    pub fn new_call(call: Call) -> Combinator {
        Combinator::Call(call)
    }

    pub fn new_add(add: Add) -> Combinator {
        Combinator::Add(add)
    }

    pub fn new_remove(remove: Remove) -> Combinator {
        Combinator::Remove(remove)
    }

    pub fn new_swapout(swapout: Swapout) -> Combinator {
        Combinator::Swapout(swapout)
    }

    pub fn new_reserved(reserved: Reserved) -> Combinator {
        Combinator::Reserved(reserved)
    }

    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }
}
