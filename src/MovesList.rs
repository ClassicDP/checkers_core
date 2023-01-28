use std::borrow::{Borrow, BorrowMut};
use crate::Moves::{Move, StraightStrike};

#[derive(Clone)]
pub enum MoveItem {
    StrikeChain(Vec<StraightStrike>),
    Move(Move),
}


pub struct MoveList {
    list: Vec<MoveItem>,
}

impl MoveList {
    pub fn new() -> MoveList {
        MoveList { list: Vec::new() }
    }
    pub fn complete_chain(&mut self) {
        let mut i = self.list.len();
        if i == 0 { return; }
        i -= 1;
        match &self.list[i] {
            MoveItem::StrikeChain(chain) => {
                if chain.len() > 0 { self.list.push(self.list[i].clone()); }
            }
            _ => {}
        }
    }
    pub fn pop_chain_link(&mut self) -> Option<StraightStrike> {
        let mut i = self.list.len();
        if i == 0 { return None; }
        i -= 1;
        if let MoveItem::StrikeChain(chain) = self.list[i].borrow_mut() {
            chain.pop()
        } else { None }
    }

    pub fn push_chain_link(&mut self, strike: StraightStrike) {
        let mut move_item = self.list.pop();
        if move_item.is_none() { move_item = Some(MoveItem::StrikeChain(Vec::new())); }
        self.list.push(move_item.unwrap());
        let i = self.list.len() - 1;
        if let MoveItem::StrikeChain(mi) = self.list[i].borrow_mut() {
            mi.push(strike);
        }
    }
}