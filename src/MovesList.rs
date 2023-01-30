use crate::Moves::{Move, StraightStrike};

pub type Chain = Vec<StraightStrike>;
#[derive(Clone, Debug)]
pub enum MoveItem {
    StrikeChain(Chain),
    Move(Move),
}

#[derive(Debug)]
pub struct MoveList {
    pub list: Vec<MoveItem>,
    pub current_chain: Chain,
}

impl MoveList {
    pub fn new() -> MoveList {
        MoveList {
            list: Vec::new(),
            current_chain: Vec::new(),
        }
    }
}
