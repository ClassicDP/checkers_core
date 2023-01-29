use std::borrow::{Borrow, BorrowMut};
use crate::Moves::{Move, StraightStrike};

#[derive(Clone)]
pub enum MoveItem {
    StrikeChain {
        chain: Vec<StraightStrike>,
        complete: bool,
    },
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
        if let MoveItem::StrikeChain { chain: ref mut _chain, ref mut complete } = self.list[i] {
            *complete = true;
        }
    }
    pub fn pop_chain_link(&mut self) {
        let mut i = self.list.len();
        if i == 0 { return }
        i -= 1;
        if let MoveItem::StrikeChain { chain, complete } = self.list[i].borrow_mut() {
            if !*complete {
                chain.pop();
                if chain.len() == 0 { self.list.pop(); }
            }
        }
    }

    pub fn push_chain_link(&mut self, strike: StraightStrike) {
        if self.list.len() == 0 || *{
            match self.list.last().borrow_mut().unwrap() {
                MoveItem::StrikeChain { chain, complete } => complete,
                _ => &false
            }
        } { self.list.push(MoveItem::StrikeChain { chain: Vec::new(), complete: false }); }
        let i = self.list.len() - 1;
        if let MoveItem::StrikeChain { chain, complete } = self.list[i].borrow_mut() {
            chain.push(strike);
        }
    }
}