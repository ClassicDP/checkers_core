use std::collections::HashSet;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::HashRcWrap::HashRcWrap;
use crate::Moves::{QuietMove, StraightStrike};
use crate::MovesList::MoveItem::{Move, StrikeChain};
use crate::Piece;

#[derive(Clone, Debug)]
pub struct Chain {
    pub vec: Vec<StraightStrike>,
    pub took_pieces: Vec<Piece>,
    pub(crate) king_move: bool,
}

#[derive(Clone, Debug)]
pub enum MoveItem {
    StrikeChain(Chain),
    Move(QuietMove),
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
            current_chain: Chain { vec: Vec::new(), took_pieces: Vec::new() , king_move: false },
        }
    }
}
