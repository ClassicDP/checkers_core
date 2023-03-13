use std::borrow::Borrow;
use std::rc::Rc;
use serde::{Deserialize, Serialize};
use crate::moves::{BoardPos, ChainPieceMove, PieceMove, QuietMove, StraightStrike};
use ts_rs::TS;
use wasm_bindgen::prelude::wasm_bindgen;


#[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(TS)]
#[ts(export)]
pub struct Strike {
    pub vec: Vec<StraightStrike>,
    pub king_move: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(TS)]
#[ts(export)]
#[wasm_bindgen]
pub struct MoveItem {
    #[wasm_bindgen(skip)]
    pub strike: Option<Strike>,
    #[wasm_bindgen(skip)]
    pub mov: Option<QuietMove>,
}

impl MoveItem {
    pub fn get_chain_piece_move(&self) -> &dyn ChainPieceMove {
        match &self.strike {
            Some(x) => x,
            _ => {
                match &self.mov {
                    Some(x) => x,
                    _ => { panic!("move item error") }
                }
            }
        }
    }
    pub fn from(&self) -> BoardPos {
        self.get_chain_piece_move().from()
    }

    pub fn to(&self) -> BoardPos {
        self.get_chain_piece_move().to()
    }


    pub fn is_king_move(&self) -> bool {
        self.get_chain_piece_move().is_king_move()
    }
}

pub struct MoveItemIter<'a> {
    list: Vec<Rc<&'a dyn PieceMove>>,
    ind: usize,
}


impl<'a> Iterator for MoveItemIter<'a> {
    type Item = Rc<&'a dyn PieceMove>;

    fn next(&mut self) -> Option<Rc<&'a dyn PieceMove>> {
        if self.ind < self.list.len() {
            self.ind += 1;
            Some(self.list[self.ind - 1].clone())
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a MoveItem {
    type Item = Rc<&'a dyn PieceMove>;
    type IntoIter = MoveItemIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        MoveItemIter {
            list: {
                let mut v: Vec<Rc<&'a dyn PieceMove>> = Vec::new();
                if self.mov.is_some() { v.push(Rc::new(self.borrow().mov.as_ref().unwrap())); } else {
                    for x in &self.borrow().strike.as_ref().unwrap().vec {
                        v.push(Rc::new(x))
                    }
                }
                v
            },
            ind: 0,
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(TS)]
#[ts(export)]
#[wasm_bindgen]
pub struct MoveList {
    #[wasm_bindgen(skip)]
    pub list: Vec<MoveItem>,
    #[wasm_bindgen(skip)]
    pub current_chain: Strike,
}


impl MoveList {
    pub fn new() -> MoveList {
        MoveList {
            list: Vec::new(),
            current_chain: Strike { vec: Vec::new(), king_move: false },
        }
    }
}


