use serde::{Deserialize, Serialize};
use crate::moves::{QuietMove, StraightStrike};
use crate::piece::Piece;
use wasm_bindgen::prelude::wasm_bindgen;
use ts_rs::TS;


#[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(TS)]
#[ts(export)]
pub struct Strike {
    pub vec: Vec<StraightStrike>,
    pub took_pieces: Vec<Piece>,
    pub king_move: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(TS)]
#[ts(export)]
pub struct MoveItem {
    pub strike: Option<Strike>,
    pub mov: Option<QuietMove>,
}



#[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(TS)]
#[ts(export)]
pub struct MoveList {
    pub list: Vec<MoveItem>,
    pub current_chain: Strike,
}

impl MoveList {
    pub fn new() -> MoveList {
        MoveList {
            list: Vec::new(),
            current_chain: Strike { vec: Vec::new(), took_pieces: Vec::new(), king_move: false },
        }
    }
}


