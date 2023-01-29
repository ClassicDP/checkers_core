use std::borrow::{Borrow, BorrowMut};
use crate::Moves::{Move, StraightStrike};

#[derive(Clone, Debug)]
pub enum MoveItem {
    StrikeChain {
        chain: Vec<StraightStrike>,
        complete: bool,
    },
    Move(Move),
}

#[derive(Debug)]
pub struct MoveList {
    list: Vec<MoveItem>,
    pub current_depth: usize,
}

impl MoveList {
    pub fn new() -> MoveList {
        MoveList { list: Vec::new(), current_depth: 0 }
    }

    pub fn complete_chain(&mut self) {
        let mut i = self.list.len();
        if i == 0 { return; }
        i -= 1;
        if let MoveItem::StrikeChain { chain: ref mut _chain, ref mut complete } = self.list[i] {
            *complete = true;
        }
    }
    pub fn pop_chain_link(&mut self) {
        let mut i = self.list.len();
        if i == 0 { return; }
        i -= 1;
        if let MoveItem::StrikeChain { chain, complete } = self.list[i].borrow_mut() {
            if !*complete {
                chain.pop();
                if chain.len() == 0 { self.list.pop(); }
            }
        }
    }

    pub fn push_chain_link(&mut self, strike: StraightStrike, current_depth: usize) {
        if let MoveItem::StrikeChain { chain, complete } = {
            if self.list.len() == 0 {
                self.list.push(MoveItem::StrikeChain { chain: Vec::new(), complete: false });
                self.list[0].borrow_mut()
            } else {
                if let MoveItem::StrikeChain { chain, complete } = self.list.last().unwrap() {
                    if *complete {
                        let mut new_chain = Vec::from_iter(chain[0..current_depth - 1].iter().cloned());
                        self.list.push(MoveItem::StrikeChain { chain: new_chain, complete: false })
                    }
                }
                let i = self.list.len() - 1;
                self.list[i].borrow_mut()
            }
        } {
            chain.push(strike);
        }
    }
}